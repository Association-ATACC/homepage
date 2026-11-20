//! Pages of the website, one folder per route.
//!
//! Each folder holds the page component (`page.rs`), the server function it
//! calls (`api.rs`) and, when needed, the server-only logic behind that
//! function (`logic.rs`).

pub mod home;
pub mod verify;
