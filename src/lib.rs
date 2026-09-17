#![recursion_limit = "256"]

pub mod api;
pub mod app;
pub mod components;

// These modules use server-only crates (sqlx, lettre, tokio, ...) and are
// only compiled into the "ssr" binary, never into the wasm/hydrate bundle.
#[cfg(feature = "ssr")]
pub mod server;
#[cfg(feature = "ssr")]
pub mod state;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
