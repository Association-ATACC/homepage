//! Server-only logic of the verification page: database access and mapping
//! of its result to the type sent to the client.

use super::api::VerifyOutcome;
use crate::server::db::{self, VerificationResult};
use crate::state::AppState;

/// Marks the user owning `token` as verified and describes what happened.
///
/// # Errors
///
/// Returns the database error if the lookup or the update fails. The error is
/// logged here, so callers only need to show a generic message.
pub async fn verify(
  state: &AppState,
  token: &str,
) -> Result<VerifyOutcome, sqlx::Error> {
  let result = db::verify_token(&state.pool, token.trim())
    .await
    .inspect_err(|err| {
      leptos::logging::error!("verification database error: {err}");
    })?;

  Ok(match result {
    VerificationResult::Verified { first_names } => {
      VerifyOutcome::Verified { first_names }
    }
    VerificationResult::AlreadyVerified { first_names } => {
      VerifyOutcome::AlreadyVerified { first_names }
    }
    VerificationResult::NotFound => VerifyOutcome::NotFound,
  })
}
