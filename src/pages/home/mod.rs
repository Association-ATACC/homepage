//! Home page (`/`): header, registration form and weekly schedule.
//!
//! - `page`: the page component.
//! - `components`: the building blocks used by the page.
//! - `api`: the `register_user` server function.
//! - `logic`: the server-only validation and persistence behind it.

mod api;
mod components;
#[cfg(feature = "ssr")]
mod logic;
pub mod page;
