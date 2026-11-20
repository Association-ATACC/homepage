//! Server function of the home page: user registration.

// `#[server]` generates an `async fn` stub without `.await` for the client
// (`hydrate`) build; the lint cannot be silenced on the macro call itself.
#![allow(clippy::unused_async_trait_impl)]

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Result of a registration attempt, sent back to the client.
/// Deliberately does not carry the verification token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistrationOutcome {
  /// A confirmation email was sent (new or refreshed registration).
  EmailSent,
  /// The email address was already registered and confirmed.
  AlreadyVerified,
}

/// Registers (or re-registers) a user and sends them a verification email.
///
/// The body below only compiles on the server: the `#[server]` macro
/// replaces it with a network call when this crate is built for the
/// browser (the `hydrate` feature).
///
/// # Errors
///
/// Returns a [`ServerFnError`] carrying a French message meant for the
/// visitor when an input is invalid, when the database query fails, or when
/// the confirmation email cannot be sent. Technical details of the last two
/// are logged on the server and not sent to the client.
#[server]
// `#[server]` generates an `async fn` stub without `.await` for the client
// (`hydrate`) build; the lint cannot be silenced on the macro call itself.
#[allow(clippy::unused_async_trait_impl)]
pub async fn register_user(
  /// First name(s) of the visitor.
  first_names: String,
  /// Last name(s) of the visitor.
  last_names: String,
  /// Email address to confirm.
  email: String,
  /// Phone number, either French local (`06...`) or international (`+33...`).
  phone: String,
  /// Student number, exactly 8 digits.
  student_number: String,
) -> Result<RegistrationOutcome, ServerFnError> {
  use crate::state::AppState;

  let state = expect_context::<AppState>();

  super::logic::register(
    &state,
    &first_names,
    &last_names,
    &email,
    &phone,
    &student_number,
  )
  .await
  .map_err(|err| ServerFnError::new(err.to_string()))
}
