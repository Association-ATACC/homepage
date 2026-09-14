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
    use crate::server::db;
    use crate::server::email::send_verification_email;
    use crate::state::AppState;

    let state = expect_context::<AppState>();

    let first_names = first_names.trim().to_string();
    let last_names = last_names.trim().to_string();
    let email = email.trim().to_lowercase();
    let phone = phone.map(|p| p.trim().to_string()).filter(|p| !p.is_empty());
    let student_number = student_number
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    if first_names.is_empty() || last_names.is_empty() {
        return Err(ServerFnError::new(
            "Merci de renseigner ton prénom et ton nom.",
        ));
    }
    if !email.contains('@') || !email.contains('.') {
        return Err(ServerFnError::new(
            "Cette adresse email ne semble pas valide.",
        ));
    }

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
