#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use atacc_inscription::app::{shell, App};
    use atacc_inscription::server::{config::AppConfig, db};
    use atacc_inscription::state::AppState;
    use axum::{routing::get, Router};
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let config_path =
        std::env::var("ATACC_CONFIG").unwrap_or_else(|_| "config.toml".to_string());
    let app_config = AppConfig::load(&config_path).unwrap_or_else(|e| {
        eprintln!("Impossible de charger la configuration ({config_path}) : {e}");
        eprintln!("Copie config.example.toml vers config.toml et adapte-le avant de relancer.");
        std::process::exit(1);
    });

    let pool = db::connect(&app_config.database.path)
        .await
        .expect("impossible de se connecter à la base de données / d'appliquer les migrations");

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        pool,
        smtp: app_config.smtp,
        public_url: app_config.server.public_url,
    };

    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/verify", get(verify::verify_handler))
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            {
                let leptos_options = app_state.leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    println!("ATACC écoute sur http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

/// Plain (non-Leptos) route that handles the link sent in the verification
/// email. Kept as a regular Axum handler — simpler than wiring up a second
/// hydrated page just to read one query parameter and show a message.
#[cfg(feature = "ssr")]
mod verify {
    use atacc_inscription::server::db;
    use atacc_inscription::state::AppState;
    use axum::extract::{Query, State};
    use axum::response::Html;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct VerifyParams {
        token: Option<String>,
    }

    pub async fn verify_handler(
        State(state): State<AppState>,
        Query(params): Query<VerifyParams>,
    ) -> Html<String> {
        let body = match params.token {
            None => page(
                "Lien invalide",
                "Ce lien de confirmation est incomplet. Vérifie que tu as bien copié l'intégralité du lien reçu par email.",
            ),
            Some(token) => match db::verify_token(&state.pool, &token).await {
                Ok(db::VerificationResult::Verified { first_names }) => page(
                    "Adresse confirmée !",
                    &format!(
                        "Merci {first_names}, ton adresse email est confirmée. Ton inscription à l'ATACC est maintenant complète."
                    ),
                ),
                Ok(db::VerificationResult::AlreadyVerified { first_names }) => page(
                    "Déjà confirmée",
                    &format!(
                        "Salut {first_names}, cette adresse était déjà confirmée. Il n'y a rien d'autre à faire."
                    ),
                ),
                Ok(db::VerificationResult::NotFound) => page(
                    "Lien invalide ou expiré",
                    "Ce lien de confirmation n'est plus valide. Réinscris-toi depuis le site pour recevoir un nouveau lien.",
                ),
                Err(_) => page(
                    "Erreur",
                    "Une erreur est survenue de notre côté. Réessaie un peu plus tard.",
                ),
            },
        };

        Html(body)
    }

    fn page(title: &str, message: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="fr">
<head>
<meta charset="utf-8"/>
<meta name="viewport" content="width=device-width, initial-scale=1"/>
<title>{title} — ATACC</title>
<link rel="stylesheet" href="/pkg/atacc-inscription.css"/>
</head>
<body>
<main class="verify-page">
<h1>{title}</h1>
<p>{message}</p>
<p><a href="/">Retour à l'accueil</a></p>
</main>
</body>
</html>"#
        )
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // No-op: the binary only does something under the "ssr" feature.
    // (The wasm/hydrate build artifact comes from the *library* target,
    // built separately by cargo-leptos — this fn just needs to exist so
    // `cargo build` without --features ssr doesn't error out.)
}
