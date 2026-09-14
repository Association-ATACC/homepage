use chrono::Utc;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

/// Opens (creating if needed) the SQLite database at `database_path` and
/// runs any pending migrations from `./migrations`.
pub async fn connect(database_path: &str) -> Result<SqlitePool, sqlx::Error> {
    let url = format!("sqlite://{database_path}?mode=rwc");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

pub enum PendingRegistration {
    /// Brand new registration; email should be sent with this token.
    New { token: String },
    /// An unverified registration already existed for this email; details
    /// were refreshed and a new token issued — email should be resent.
    Resent { token: String },
    /// This email was already verified; nothing to do.
    AlreadyVerified,
}

#[derive(FromRow)]
struct ExistingUserRow {
    id: String,
    verified: i64,
}

/// Creates a new pending registration for `email`, or — if a row for that
/// email already exists and isn't verified yet — updates its details and
/// issues a fresh verification token so the email can be resent.
///
/// Uses the runtime-checked `sqlx::query`/`query_as` API (not the
/// `query!`/`query_as!` macros), so building this project doesn't require
/// a live database connection or an offline query cache at compile time.
pub async fn upsert_pending_registration(
    pool: &SqlitePool,
    first_names: &str,
    last_names: &str,
    email: &str,
    phone: Option<&str>,
    student_number: Option<&str>,
) -> Result<PendingRegistration, sqlx::Error> {
    let existing: Option<ExistingUserRow> =
        sqlx::query_as("SELECT id, verified FROM users WHERE email = ?1")
            .bind(email)
            .fetch_optional(pool)
            .await?;

    if let Some(row) = existing {
        if row.verified != 0 {
            return Ok(PendingRegistration::AlreadyVerified);
        }

        let token = Uuid::new_v4().to_string();
        sqlx::query(
            "UPDATE users
             SET first_names = ?1, last_names = ?2, phone = ?3, student_number = ?4, verification_token = ?5
             WHERE id = ?6",
        )
        .bind(first_names)
        .bind(last_names)
        .bind(phone)
        .bind(student_number)
        .bind(&token)
        .bind(&row.id)
        .execute(pool)
        .await?;

        return Ok(PendingRegistration::Resent { token });
    }

    let id = Uuid::new_v4().to_string();
    let token = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO users
             (id, first_names, last_names, email, phone, student_number, verification_token, verified, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, ?8)",
    )
    .bind(&id)
    .bind(first_names)
    .bind(last_names)
    .bind(email)
    .bind(phone)
    .bind(student_number)
    .bind(&token)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(PendingRegistration::New { token })
}

pub enum VerificationResult {
    Verified { first_names: String },
    AlreadyVerified { first_names: String },
    NotFound,
}

#[derive(FromRow)]
struct TokenRow {
    id: String,
    first_names: String,
    verified: i64,
}

/// Marks the user owning `token` as verified, if not already.
pub async fn verify_token(pool: &SqlitePool, token: &str) -> Result<VerificationResult, sqlx::Error> {
    let row: Option<TokenRow> =
        sqlx::query_as("SELECT id, first_names, verified FROM users WHERE verification_token = ?1")
            .bind(token)
            .fetch_optional(pool)
            .await?;

    let Some(row) = row else {
        return Ok(VerificationResult::NotFound);
    };

    if row.verified != 0 {
        return Ok(VerificationResult::AlreadyVerified {
            first_names: row.first_names,
        });
    }

    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE users SET verified = 1, verified_at = ?1 WHERE id = ?2")
        .bind(&now)
        .bind(&row.id)
        .execute(pool)
        .await?;

    Ok(VerificationResult::Verified {
        first_names: row.first_names,
    })
}
