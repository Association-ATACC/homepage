CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    first_names TEXT NOT NULL,
    last_names TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    phone TEXT,
    student_number TEXT,
    verification_token TEXT NOT NULL,
    verified INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    verified_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_users_verification_token
    ON users (verification_token);
