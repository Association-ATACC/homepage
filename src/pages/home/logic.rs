//! Server-only logic of the home page: validation of the registration form,
//! storage of the pending registration and sending of the confirmation email.

use core::fmt;
use std::sync::LazyLock;

use regex::Regex;

use super::api::RegistrationOutcome;
use crate::server::db::{self, PendingRegistration};
use crate::server::email::{EmailError, send_verification_email};
use crate::state::AppState;

/// Matches a plausible email address.
static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

/// Matches a student number: exactly 8 digits.
static STUDENT_NUM_REGEX: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^\d{8}$").unwrap());

/// Matches a French local phone number (`0` followed by 9 digits).
static LOCAL_PHONE_REGEX: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^0\d{9}$").unwrap());

/// Matches an E.164 international phone number.
static INTL_PHONE_REGEX: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^\+[1-9]\d{8,14}$").unwrap());

/// Error returned by [`register`].
///
/// The `Display` output is in French because it is shown to the visitor
/// in the registration form. The `Database` and `Email` variants display
/// a generic message; their sources are logged instead.
#[derive(Debug)]
pub enum RegistrationError {
  /// The first name or the last name is empty.
  MissingName,
  /// The email address does not look valid.
  InvalidEmail,
  /// The student number is not exactly 8 digits.
  InvalidStudentNumber,
  /// The phone number is neither a French local nor an international one.
  InvalidPhone,
  /// A database query failed.
  Database(sqlx::Error),
  /// The confirmation email could not be built or sent.
  Email(EmailError),
}

impl fmt::Display for RegistrationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let message = match self {
      Self::MissingName => "Merci de renseigner ton prénom et ton nom.",
      Self::InvalidEmail => "Cette adresse email ne semble pas valide.",
      Self::InvalidStudentNumber => {
        "Le numéro étudiant doit comporter exactement 8 chiffres."
      }
      Self::InvalidPhone => {
        "\
          Le format du numéro de téléphone est invalide (attendu: 06... ou \
          +33...).\
        "
      }
      Self::Database(_) => {
        "Une erreur interne est survenue. Réessaie dans quelques instants."
      }
      Self::Email(_) => {
        "\
          L'email de confirmation n'a pas pu être envoyé. Réessaie dans \
          quelques instants.\
        "
      }
    };
    f.write_str(message)
  }
}

impl core::error::Error for RegistrationError {
  fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
    match self {
      Self::Database(source) => Some(source),
      Self::Email(source) => Some(source),
      Self::MissingName
      | Self::InvalidEmail
      | Self::InvalidStudentNumber
      | Self::InvalidPhone => None,
    }
  }
}

/// Normalizes a phone number to the international format.
///
/// Spaces, dots and dashes are removed, then `06...` becomes `+336...`.
/// Returns `None` if the result is not a valid phone number.
fn normalize_phone(raw: &str) -> Option<String> {
  let cleaned = raw.replace([' ', '.', '-'], "");
  let cleaned = cleaned.trim();

  if LOCAL_PHONE_REGEX.is_match(cleaned) {
    Some(format!("+33{}", &cleaned[1..]))
  } else if INTL_PHONE_REGEX.is_match(cleaned) {
    Some(cleaned.to_owned())
  } else {
    None
  }
}

/// Validates the form input, stores the pending registration and sends the
/// verification email when needed.
///
/// # Errors
///
/// Returns [`RegistrationError`] if an input is invalid, if a database
/// query fails, or if the email cannot be sent.
pub async fn register(
  state: &AppState,
  first_names: &str,
  last_names: &str,
  email: &str,
  phone: &str,
  student_number: &str,
) -> Result<RegistrationOutcome, RegistrationError> {
  let first_names = first_names.trim();
  let last_names = last_names.trim();
  let email = email.trim();
  let student_number = student_number.trim();

  if first_names.is_empty() || last_names.is_empty() {
    return Err(RegistrationError::MissingName);
  }

  if !EMAIL_REGEX.is_match(email) {
    return Err(RegistrationError::InvalidEmail);
  }

  if !STUDENT_NUM_REGEX.is_match(student_number) {
    return Err(RegistrationError::InvalidStudentNumber);
  }

  let phone = normalize_phone(phone).ok_or(RegistrationError::InvalidPhone)?;

  let pending = db::upsert_pending_registration(
    &state.pool,
    first_names,
    last_names,
    email,
    &phone,
    student_number,
  )
  .await
  .map_err(|err| {
    leptos::logging::error!("registration database error: {err}");
    RegistrationError::Database(err)
  })?;

  match pending {
    PendingRegistration::New { token }
    | PendingRegistration::Resent { token } => {
      send_verification_email(
        &state.smtp,
        &state.public_url,
        email,
        first_names,
        &token,
      )
      .await
      .map_err(|err| {
        leptos::logging::error!("verification email error: {err}");
        RegistrationError::Email(err)
      })?;
      Ok(RegistrationOutcome::EmailSent)
    }
    PendingRegistration::AlreadyVerified => {
      Ok(RegistrationOutcome::AlreadyVerified)
    }
  }
}
