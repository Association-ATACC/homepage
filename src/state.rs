use axum::extract::FromRef;
use leptos::prelude::LeptosOptions;
use sqlx::SqlitePool;

use crate::server::config::SmtpConfig;

/// Application state shared across every Axum route and Leptos server
/// function. `FromRef` lets Axum hand out just the `LeptosOptions` substate
/// where that's all a handler needs (e.g. `leptos_axum::file_and_error_handler`).
#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: SqlitePool,
    pub smtp: SmtpConfig,
    pub public_url: String,
}
