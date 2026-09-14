use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use super::config::{SmtpConfig, SmtpSecurity};

/// Sends the "confirm your email" message for a pending registration.
pub async fn send_verification_email(
    smtp: &SmtpConfig,
    public_url: &str,
    to_email: &str,
    first_names: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let verify_link = format!(
        "{}/verify?token={}",
        public_url.trim_end_matches('/'),
        token
    );

    let html_body = format!(
        "<p>Salut {first_names},</p>\
         <p>Merci de ton inscription à l'ATACC ! Pour confirmer ton adresse email, clique sur le lien ci-dessous :</p>\
         <p><a href=\"{verify_link}\">Confirmer mon inscription</a></p>\
         <br>\
         <p>Si tu n'es pas à l'origine de cette inscription, tu peux ignorer ce message.</p>\
         <p>À bientôt,<br>L'équipe ATACC</p>"
    );

    let email = Message::builder()
        .from(smtp.from_address.parse()?)
        .to(to_email.parse()?)
        .subject("Confirme ton inscription à l'ATACC")
        .header(ContentType::TEXT_HTML)
        .body(html_body)?;

    let builder = match smtp.security {
        SmtpSecurity::None => {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp.host).port(smtp.port)
        }
        SmtpSecurity::Smtps => {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp.host)?.port(smtp.port)
        }
        SmtpSecurity::Starttls => {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp.host)?.port(smtp.port)
        }
    };

    let mailer = match &smtp.password {
        Some(pwd) => {
            let creds = Credentials::new(smtp.username.clone(), pwd.to_owned());
            builder.credentials(creds).build()
        }
        None => builder.build(),
    };

    mailer.send(email).await?;

    Ok(())
}
