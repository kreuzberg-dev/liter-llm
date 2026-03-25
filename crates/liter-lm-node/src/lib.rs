#![deny(clippy::all)]

use napi_derive::napi;

/// Returns the version of the liter-lm library.
#[napi]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
