//! Outgoing email: builds and sends the confirmation message for a pending
//! registration over SMTP.

use std::fmt;

use lettre::address::AddressError;
use lettre::message::{
  Mailbox, Message, MultiPart, SinglePart, header::ContentType,
};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};

use super::config::{SmtpConfig, SmtpSecurity};

/// Error returned when a verification email cannot be built or sent.
#[derive(Debug)]
pub enum EmailError {
  /// An email address could not be parsed as a mailbox.
  Address {
    /// The address that failed to parse.
    address: String,
    /// Underlying address parsing error.
    source: AddressError,
  },
  /// The message could not be assembled (missing header, invalid body, etc.).
  Build(lettre::error::Error),
  /// The SMTP transport could not be configured for the given host.
  Transport {
    /// Hostname the transport was being configured for.
    host: String,
    /// Underlying SMTP configuration error.
    source: lettre::transport::smtp::Error,
  },
  /// The SMTP server rejected the message or the delivery failed.
  Send(lettre::transport::smtp::Error),
}

impl fmt::Display for EmailError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Address { address, source } => {
        write!(f, "invalid email address {address:?}: {source}")
      }
      Self::Build(source) => {
        write!(f, "failed to build the email message: {source}")
      }
      Self::Transport { host, source } => {
        write!(
          f,
          "failed to configure the SMTP transport for {host}: {source}"
        )
      }
      Self::Send(source) => {
        write!(f, "failed to send the email: {source}")
      }
    }
  }
}

impl core::error::Error for EmailError {
  fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
    match self {
      Self::Address { source, .. } => Some(source),
      Self::Build(source) => Some(source),
      Self::Transport { source, .. } | Self::Send(source) => Some(source),
    }
  }
}

/// Parses `address` as a mailbox, keeping the offending string in the error.
fn parse_mailbox(address: &str) -> Result<Mailbox, EmailError> {
  address.parse().map_err(|source| EmailError::Address {
    address: address.to_owned(),
    source,
  })
}

/// Escapes the characters that are significant in HTML text and attribute
/// values, so user-provided input can be embedded in an HTML body safely.
fn escape_html(input: &str) -> String {
  let mut escaped = String::with_capacity(input.len());
  for c in input.chars() {
    match c {
      '&' => escaped.push_str("&amp;"),
      '<' => escaped.push_str("&lt;"),
      '>' => escaped.push_str("&gt;"),
      '"' => escaped.push_str("&quot;"),
      '\'' => escaped.push_str("&#39;"),
      _ => escaped.push(c),
    }
  }
  escaped
}

/// Sends the "confirm your email" message for a pending registration.
///
/// The message is multipart: a plain-text part and an HTML part, both
/// containing the verification link built from `public_url` and `token`.
///
/// # Errors
///
/// Returns [`EmailError::Address`] if `smtp.from_address` or `to_email` is not
/// a valid mailbox, [`EmailError::Build`] if the message cannot be assembled,
/// [`EmailError::Transport`] if the SMTP transport cannot be configured from
/// `smtp.host`, and [`EmailError::Send`] if the server rejects or fails to
/// deliver the message.
pub async fn send_verification_email(
  smtp: &SmtpConfig,
  public_url: &str,
  to_email: &str,
  first_names: &str,
  token: &str,
) -> Result<(), EmailError> {
  let verify_link = format!(
    "{}/verify?token={}",
    public_url.trim_end_matches('/'),
    token
  );
  let safe_first_names = escape_html(first_names);

  let html_body = format!(
    "<p>Salut {safe_first_names},</p>\
     <p>Merci de ton inscription à l'ATACC ! Pour confirmer ton adresse email, clique sur le lien ci-dessous :</p>\
     <p><a href=\"{verify_link}\">Confirmer mon inscription</a></p>\
     <br>\
     <p>Si tu n'es pas à l'origine de cette inscription, tu peux ignorer ce message.</p>\
     <p>À bientôt,<br>L'équipe ATACC</p>"
  );

  let text_body = format!(
    "Salut {first_names},\n\n\
     Merci de ton inscription à l'ATACC ! Pour confirmer ton adresse email, copie et colle ce lien dans ton navigateur :\n\n\
     {verify_link}\n\n\
     Si tu n'es pas à l'origine de cette inscription, tu peux ignorer ce message.\n\n\
     À bientôt,\n\
     L'équipe ATACC"
  );

  let email = Message::builder()
    .from(parse_mailbox(&smtp.from_address)?)
    .to(parse_mailbox(to_email)?)
    .subject("Confirme ton inscription à l'ATACC")
    .multipart(
      MultiPart::alternative()
        .singlepart(
          SinglePart::builder()
            .header(ContentType::TEXT_PLAIN)
            .body(text_body),
        )
        .singlepart(
          SinglePart::builder()
            .header(ContentType::TEXT_HTML)
            .body(html_body),
        ),
    )
    .map_err(EmailError::Build)?;

  let transport_error = |source| EmailError::Transport {
    host: smtp.host.clone(),
    source,
  };

  let builder = match smtp.security {
    SmtpSecurity::None => {
      AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp.host)
        .port(smtp.port)
    }
    SmtpSecurity::Smtps => {
      AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp.host)
        .map_err(transport_error)?
        .port(smtp.port)
    }
    SmtpSecurity::Starttls => {
      AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp.host)
        .map_err(transport_error)?
        .port(smtp.port)
    }
  };

  let mailer = match &smtp.password {
    Some(pwd) => {
      let creds = Credentials::new(smtp.username.clone(), pwd.to_owned());
      builder.credentials(creds).build()
    }
    None => builder.build(),
  };

  mailer.send(email).await.map_err(EmailError::Send)?;

  Ok(())
}
