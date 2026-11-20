//! ATACC homepage: a Leptos application served with Axum.
//!
//! The crate is built twice: with the `ssr` feature for the server (database,
//! email, shared state) and with the `hydrate` feature for the WebAssembly
//! bundle that takes over the server-rendered page in the browser.

#![recursion_limit = "256"]

pub mod app;
mod pages;

#[cfg(feature = "ssr")]
pub mod server;
#[cfg(feature = "ssr")]
pub mod state;

/// Browser entry point: attaches the client-side application to the HTML
/// rendered by the server.
///
/// Called automatically by the generated JavaScript glue once the WebAssembly
/// module is loaded.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  use crate::app::App;
  console_error_panic_hook::set_once();
  leptos::mount::hydrate_body(App);
}
