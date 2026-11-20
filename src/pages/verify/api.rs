//! Server function of the verification page: confirms an email address.

// `#[server]` generates an `async fn` stub without `.await` for the client
// (`hydrate`) build; the lint cannot be silenced on the macro call itself.
#![allow(clippy::unused_async_trait_impl)]

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Result of a verification attempt, sent back to the client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerifyOutcome {
  /// The token matched an unverified user, who is now verified.
  Verified {
    /// First name(s) of the user, for display.
    first_names: String,
  },
  /// The token matched a user who was already verified.
  AlreadyVerified {
    /// First name(s) of the user, for display.
    first_names: String,
  },
  /// No user owns this token.
  NotFound,
}

/// Marks the user owning `token` as verified, if not already.
///
/// # Errors
///
/// Returns a [`ServerFnError`] with a generic French message when the
/// database query fails. The technical details are logged on the server.
#[server]
pub async fn verify_email(
  /// Verification token taken from the link in the email.
  token: String,
) -> Result<VerifyOutcome, ServerFnError> {
  use crate::state::AppState;

  let state = expect_context::<AppState>();

  super::logic::verify(&state, &token).await.map_err(|_| {
    ServerFnError::new(
      "Une erreur interne est survenue. Réessaie dans quelques instants.",
    )
  })
}
