//! # twilight-http
//!
//! HTTP support for the twilight ecosystem.
//!
//! ## Features
//!
//! ### Deserialization
//!
//! `twilight-http` supports [`serde_json`] and [`simd-json`] for deserializing
//! responses.
//!
//! #### `simd-json`
//!
//! The `simd-json` feature enables [`simd-json`] support to use simd features
//! of modern cpus to deserialize responses faster. It is not enabled by
//! default.
//!
//! To use this feature you need to also add these lines to
//! `<project root>/.cargo/config`:
//!
//! ```toml
//! [build]
//! rustflags = ["-C", "target-cpu=native"]
//! ```
//!
//! You can also set the environment variable
//! `RUSTFLAGS="-C target-cpu=native"`. If you enable both `serde_json` and
//! `simd-json` at the same time, then `simd-json` will be used.
//!
//! To enable `simd-json`, do something like this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! twilight-http = { branch = "trunk", default-features = false, features = ["rustls", "simd-json"], git = "https://github.com/twilight-rs/twilight" }
//! ```
//!
//! ### TLS
//!
//! `twilight-http` has features to enable [`reqwest`]'s TLS features. These
//! features are mutually exclusive. `rustls` is enabled by default.
//!
//! #### `native`
//!
//! The `native` feature enables [`reqwest`]'s `default-tls`
//! feature, which is mostly equivalent to using [`native-tls`].
//!
//! To enable `native`, do something like this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! twilight-http = { branch = "trunk", default-features = false, features = ["native"], git = "https://github.com/twilight-rs/twilight" }
//! ```
//!
//! #### `rustls`
//!
//! The `rustls` feature enables [`reqwest`]'s `rustls` feature, which uses
//! [`rustls`] as the TLS backend.
//!
//! This is enabled by default.
//!
//! [`native-tls`]: https://crates.io/crates/native-tls
//! [`reqwest`]: https://crates.io/crates/reqwest
//! [`rustls`]: https://crates.io/crates/rustls
//! [`serde_json`]: https://crates.io/crates/serde_json
//! [`simd-json`]: https://crates.io/crates/simd-json

#![deny(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unused,
    warnings
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::option_option,
    clippy::pub_enum_variant_names,
    clippy::must_use_candidate,
    clippy::missing_errors_doc
)]

pub mod api_error;
pub mod client;
pub mod error;
pub mod ratelimiting;
pub mod request;
pub mod routing;

pub use crate::{
    client::Client,
    error::{Error, Result},
};

#[cfg(not(feature = "simd-json"))]
use serde_json::Result as JsonResult;
#[cfg(feature = "simd-json")]
use simd_json::Result as JsonResult;

pub(crate) fn json_from_slice<'a, T: serde::de::Deserialize<'a>>(s: &'a mut [u8]) -> JsonResult<T> {
    #[cfg(not(feature = "simd-json"))]
    return serde_json::from_slice(s);
    #[cfg(feature = "simd-json")]
    return simd_json::from_slice(s);
}

#[cfg(not(feature = "simd-json"))]
pub(crate) use serde_json::to_vec as json_to_vec;
#[cfg(feature = "simd-json")]
pub(crate) use simd_json::to_vec as json_to_vec;

#[cfg(not(any(feature = "native", feature = "rustls")))]
compile_error!("Either the `native` or `rustls` feature must be enabled.");
