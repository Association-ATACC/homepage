//! Build script for the crate.
//!
//! This script performs two primary tasks during the Cargo build process:
//! 1. Overrides the package version if built within a Nix environment via
//!    `NIX_RELEASE_VERSION`.

fn main() {
  if let Ok(val) = std::env::var("NIX_RELEASE_VERSION") {
    println!("cargo:rustc-env=CARGO_PKG_VERSION={val}");
  }
  println!("cargo:rerun-if-env-changed=NIX_RELEASE_VERSION");
}
