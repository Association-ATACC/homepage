//! View of the email verification page.

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_query_map;

use super::api::{VerifyOutcome, verify_email};

/// Heading and text displayed to the visitor.
struct Message {
  /// Heading of the page.
  title: &'static str,
  /// Explanation shown below the heading.
  body: String,
}

/// Builds the message matching the result of the verification request.
///
/// `None` means the link had no token, so no request was made.
fn describe(outcome: Option<Result<VerifyOutcome, ServerFnError>>) -> Message {
  match outcome {
    None => Message {
      title: "Lien invalide",
      body: "\
        Ce lien de confirmation est incomplet. Vérifie que tu as bien copié \
        l'intégralité du lien reçu par email.\
      "
      .to_owned(),
    },
    Some(Ok(VerifyOutcome::Verified { first_names })) => Message {
      title: "Adresse confirmée !",
      body: format!(
        "\
          Merci {first_names}, ton adresse email est confirmée. Ton \
          inscription à l'ATACC est maintenant complète.\
        "
      ),
    },
    Some(Ok(VerifyOutcome::AlreadyVerified { first_names })) => Message {
      title: "Déjà confirmée",
      body: format!(
        "\
          Salut {first_names}, cette adresse était déjà confirmée. Il n'y a \
          rien d'autre à faire.\
        "
      ),
    },
    Some(Ok(VerifyOutcome::NotFound)) => Message {
      title: "Lien invalide ou expiré",
      body: "\
        Ce lien de confirmation n'est plus valide. Réinscris-toi depuis le \
        site pour recevoir un nouveau lien.\
      "
      .to_owned(),
    },
    Some(Err(_)) => Message {
      title: "Erreur",
      body: "\
        Une erreur est survenue de notre côté. Réessaie un peu plus tard.\
      "
      .to_owned(),
    },
  }
}

/// Email verification page: confirms the address behind the token found in
/// the query string of the link (`/verify?token=...`).
///
/// The text inserted by `view!` is escaped by Leptos, so the visitor's first
/// name can be displayed without any manual escaping.
#[component]
pub fn VerifyPage() -> impl IntoView {
  let query = use_query_map();
  let token = move || query.with(|params| params.get("token"));

  let outcome = Resource::new(token, |token| async move {
    match token {
      Some(token) => Some(verify_email(token).await),
      None => None,
    }
  });

  view! {
    <Title text="Confirmation de l'adresse - ATACC" />
    <section class="verify-page">
      <Suspense fallback=|| {
        view! { <p>"Vérification en cours…"</p> }
      }>
        {move || Suspend::new(async move {
          let message = describe(outcome.await);
          view! {
            <h1>{message.title}</h1>
            <p>{message.body}</p>
          }
        })}
      </Suspense>
      <p>
        <a href="/">"Retour à l'accueil"</a>
      </p>
    </section>
  }
}
