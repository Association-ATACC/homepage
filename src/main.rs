//! Binary entry point: loads the configuration, connects to the database and
//! starts the Axum server that serves the Leptos application.

/// Starts the HTTP server.
///
/// The path of the configuration file is read from the `ATACC_CONFIG`
/// environment variable and defaults to `config.toml`. The process exits with
/// status 1 if the configuration cannot be loaded.
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
  use atacc_homepage::app::{App, shell};
  use atacc_homepage::server::{config::AppConfig, db};
  use atacc_homepage::state::AppState;
  use axum::Router;
  use leptos::config::get_configuration;
  use leptos::prelude::*;
  use leptos_axum::{LeptosRoutes, generate_route_list};

  let config_path =
    std::env::var("ATACC_CONFIG").unwrap_or_else(|_| "config.toml".to_owned());
  let app_config = AppConfig::load(&config_path).unwrap_or_else(|e| {
    eprintln!("Failed to load the configuration ({config_path}): {e}");
    eprintln!(
      "Copy config.example.toml to config.toml and adjust it before restarting."
    );
    std::process::exit(1);
  });

  let pool = db::connect(&app_config.database.path)
    .await
    .expect("failed to connect to the database or apply the migrations");

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

  println!("ATACC listening on http://{addr}");

  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  axum::serve(listener, app.into_make_service())
    .await
    .unwrap();
}

/// No-op entry point for builds without the `ssr` feature.
///
/// The wasm/hydrate build artifact comes from the *library* target, built
/// separately by cargo-leptos. This function only needs to exist so that
/// `cargo build` without `--features ssr` doesn't fail.
#[cfg(not(feature = "ssr"))]
pub fn main() {}
