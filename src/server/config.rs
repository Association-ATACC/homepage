//! Application configuration: the structures deserialized from the TOML
//! config file and the loader that reads them.

use core::{error, fmt};
use serde::Deserialize;

/// Root of the configuration file.
#[derive(Clone, Deserialize)]
pub struct AppConfig {
  /// HTTP server settings.
  pub server: ServerSection,
  /// Database settings.
  pub database: DatabaseSection,
  /// Outgoing email settings.
  pub smtp: SmtpConfig,
}

/// `[server]` section of the configuration file.
#[derive(Clone, Deserialize)]
pub struct ServerSection {
  /// Public base URL of the application (e.g. `https://example.com`), used
  /// to build absolute links such as those in emails.
  pub public_url: String,
}

/// `[database]` section of the configuration file.
#[derive(Clone, Deserialize)]
pub struct DatabaseSection {
  /// Filesystem path of the SQLite database file.
  pub path: String,
}

/// Transport security used when connecting to the SMTP server.
///
/// Deserialized from the lowercase values `"none"`, `"smtps"` and
/// `"starttls"`.
#[derive(Clone, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SmtpSecurity {
  /// No encryption. Only suitable for local or trusted relays.
  None,
  /// Implicit TLS: the connection is encrypted from the start (usually
  /// port 465).
  Smtps,
  /// Plain connection upgraded to TLS with the `STARTTLS` command (usually
  /// port 587).
  #[default]
  Starttls,
}

/// `[smtp]` section of the configuration file.
#[derive(Clone, Deserialize)]
pub struct SmtpConfig {
  /// Hostname of the SMTP server.
  pub host: String,
  /// TCP port of the SMTP server.
  pub port: u16,
  /// Username used to authenticate with the SMTP server.
  pub username: String,
  /// Password used to authenticate, if the server requires one.
  pub password: Option<String>,
  /// Address shown in the `From` header of outgoing emails.
  pub from_address: String,
  /// Transport security. Defaults to [`SmtpSecurity::Starttls`].
  #[serde(default)]
  pub security: SmtpSecurity,
}

/// Error returned when the configuration file cannot be loaded.
///
/// The `Display` output is in French, ready to be shown to the operator.
#[derive(Debug)]
pub enum ConfigError {
  /// The file could not be read from disk.
  Read {
    /// Path of the file that could not be read.
    path: String,
    /// Underlying I/O error.
    source: std::io::Error,
  },
  /// The file was read but its content is not valid TOML matching
  /// [`AppConfig`].
  Parse {
    /// Path of the invalid file.
    path: String,
    /// Underlying TOML deserialization error.
    source: toml::de::Error,
  },
}

impl fmt::Display for ConfigError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Read { path, source } => {
        write!(f, "failed to read {path}: {source}")
      }
      Self::Parse { path, source } => {
        write!(f, "invalid config file ({path}): {source}")
      }
    }
  }
}

impl error::Error for ConfigError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      Self::Read { source, .. } => Some(source),
      Self::Parse { source, .. } => Some(source),
    }
  }
}

impl AppConfig {
  /// Loads and parses the config file at `path` (see
  /// `config.example.toml` for the expected shape).
  ///
  /// # Errors
  ///
  /// Returns [`ConfigError::Read`] if the file cannot be read, and
  /// [`ConfigError::Parse`] if its content is not valid TOML matching
  /// [`AppConfig`].
  pub fn load(path: &str) -> Result<Self, ConfigError> {
    let raw =
      std::fs::read_to_string(path).map_err(|source| ConfigError::Read {
        path: path.to_owned(),
        source,
      })?;
    toml::from_str(&raw).map_err(|source| ConfigError::Parse {
      path: path.to_owned(),
      source,
    })
  }
}
