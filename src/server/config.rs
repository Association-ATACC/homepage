use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerSection,
    pub database: DatabaseSection,
    pub smtp: SmtpConfig,
}

#[derive(Clone, Deserialize)]
pub struct ServerSection {
    pub public_url: String,
}

#[derive(Clone, Deserialize)]
pub struct DatabaseSection {
    pub path: String,
}

#[derive(Clone, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SmtpSecurity {
    None,
    Smtps,
    #[default]
    Starttls,
}

/// SMTP connection details. Deliberately does not derive `Debug` so a
/// stray `{:?}` never leaks the password into logs.
#[derive(Clone, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub from_address: String,
    #[serde(default)]
    pub security: SmtpSecurity,
}

impl AppConfig {
    /// Loads and parses the config file at `path` (see
    /// `config.example.toml` for the expected shape).
    pub fn load(path: &str) -> Result<Self, String> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| format!("impossible de lire {path} : {e}"))?;
        toml::from_str(&raw).map_err(|e| format!("fichier de config invalide ({path}) : {e}"))
    }
}
