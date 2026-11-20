//! Email verification page (`/verify?token=...`): the target of the link sent
//! in the confirmation email.
//!
//! - `page`: the page component.
//! - `api`: the `verify_email` server function.
//! - `logic`: the server-only database work behind it.

mod api;
#[cfg(feature = "ssr")]
mod logic;
pub mod page;
