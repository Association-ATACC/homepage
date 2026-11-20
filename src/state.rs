//! Shared application state exposed to Axum routes and Leptos server
//! functions.

use axum::extract::FromRef;
use leptos::prelude::LeptosOptions;
use sqlx::SqlitePool;

use crate::server::config::SmtpConfig;

/// Application state shared across every Axum route and Leptos server
/// function.
#[derive(Clone, FromRef)]
pub struct AppState {
  /// Leptos configuration (site address, output directory, reload port, etc.)
  /// used by the SSR handler and the file server.
  pub leptos_options: LeptosOptions,
  /// Connection pool for the application's SQLite database.
  pub pool: SqlitePool,
  /// SMTP settings used when sending outgoing emails.
  pub smtp: SmtpConfig,
  /// Public base URL of the application (e.g. `https://example.com`), used to
  /// build absolute links such as those in emails.
  pub public_url: String,
}
