use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Result of a registration attempt, sent back to the client.
/// Deliberately does not carry the verification token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistrationOutcome {
    EmailSent,
    AlreadyVerified,
}

/// Registers (or re-registers) a user and sends them a verification email.
///
/// The body below only compiles on the server: the `#[server]` macro
/// replaces it with a network call when this crate is built for the
/// browser (the `hydrate` feature), so it's safe to reference server-only
/// modules (`crate::server`, `crate::state`) directly inside the function.
#[server]
pub async fn register_user(
    first_names: String,
    last_names: String,
    email: String,
    phone: Option<String>,
    student_number: Option<String>,
) -> Result<RegistrationOutcome, ServerFnError> {
    use regex::Regex;
    use std::sync::LazyLock;

    use crate::server::db;
    use crate::server::email::send_verification_email;
    use crate::state::AppState;

    let state = expect_context::<AppState>();

    static EMAIL_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

    static STUDENT_NUM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{8}$").unwrap());

    static LOCAL_PHONE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^0\d{9}$").unwrap());

    static INTL_PHONE_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\+\d{9,15}$").unwrap());

    let first_names = first_names.trim().to_string();
    let last_names = last_names.trim().to_string();
    let email = email.trim().to_lowercase();
    let student_number = student_number
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    if first_names.is_empty() || last_names.is_empty() {
        return Err(ServerFnError::new(
            "Merci de renseigner ton prénom et ton nom.",
        ));
    }

    if !EMAIL_REGEX.is_match(&email) {
        return Err(ServerFnError::new(
            "Cette adresse email ne semble pas valide.",
        ));
    }

    if let Some(ref num) = student_number {
        if !STUDENT_NUM_REGEX.is_match(num) {
            return Err(ServerFnError::new(
                "Le numéro étudiant doit comporter exactement 8 chiffres.",
            ));
        }
    }

    let phone = match phone
        .map(|p| p.replace([' ', '.', '-'], "").trim().to_string())
        .filter(|p| !p.is_empty())
    {
        Some(p) if LOCAL_PHONE_REGEX.is_match(&p) => {
            // Drop the leading '0' and prepend '+33'
            Some(format!("+33{}", &p[1..]))
        }
        Some(p) if INTL_PHONE_REGEX.is_match(&p) => {
            // Keep the +XX international number as is
            Some(p)
        }
        Some(_) => {
            return Err(ServerFnError::new(
                "Le format du numéro de téléphone est invalide (attendu: 06... ou +33...).",
            ));
        }
        None => None,
    };

    let pending = db::upsert_pending_registration(
        &state.pool,
        &first_names,
        &last_names,
        &email,
        phone.as_deref(),
        student_number.as_deref(),
    )
    .await
    .map_err(|e| ServerFnError::new(format!("Erreur base de données : {e}")))?;

    match pending {
        db::PendingRegistration::New { token } | db::PendingRegistration::Resent { token } => {
            send_verification_email(&state.smtp, &state.public_url, &email, &first_names, &token)
                .await
                .map_err(|e| ServerFnError::new(format!("Erreur d'envoi d'email : {e}")))?;
            Ok(RegistrationOutcome::EmailSent)
        }
        db::PendingRegistration::AlreadyVerified => Ok(RegistrationOutcome::AlreadyVerified),
    }
}
