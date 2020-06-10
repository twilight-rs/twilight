//! # twilight-http
//!
//! HTTP support for the twilight ecosystem.
//!
//! ## Features
//!
//! `twilight-http` includes three features: `native`, `simd` and `rustls`. `native` is
//! enabled by default. `native` will enable `reqwest`'s `default-tls` feature,
//! which will use the TLS library native to your OS (for example, OpenSSL on
//! Linux). `rustls` will enable `reqwest`'s `rustls-tls` feature, which will use
//! [rustls].
//!
//! If you want to use Rustls instead of your native library, it's easy to switch it
//! out:
//!
//! ```toml
//! [dependencies]
//! twilight-http = { default-features = false, features = ["rustls"], git = "https://github.com/twilight-rs/twilight" }
//! ```
//!
//! You can also choose to use neither feature. This is only useful if you provide
//! your own configured Reqwest client to the HTTP client, otherwise you will
//! encounter TLS errors.
//!
//! `simd` feature enables [simd-json] support to use simd features of the modern cpus
//! to deserialize json data faster. It is not enabled by default since not every cpu has those features.
//! To use this feature you need to also add these lines to a file in `<project root>/.cargo/config`
//! ```toml
//! [build]
//! rustflags = ["-C", "target-cpu=native"]
//! ```
//! you can also use this environment variable `RUSTFLAGS="-C target-cpu=native"`. If you enable both
//! `serde_json` and `simd-json` at the same time; this crate uses `simd-json`. But it is recommended to
//! disable `serde_json` if you are going to use `simd-json`. It is easy to switch to out:
//!
//! ```toml
//! [dependencies]
//! twilight-gateway = { default-features = false, features = ["simd-json"], git = "https://github.com/twilight-rs/twilight" }
//! ```
//!
//! [rustls]: https://github.com/ctz/rustls
//! [simd-json]: https://github.com/simd-lite/simd-json

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
    clippy::pub_enum_variant_names,
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    // This issue made it into a stable clippy:
    //
    // <https://github.com/rust-lang/rust-clippy/issues/5360>
    clippy::used_underscore_binding
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

#[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
use serde_json::Result as JsonResult;
#[cfg(feature = "simd-json")]
use simd_json::Result as JsonResult;

pub(crate) fn json_from_slice<'a, T: serde::de::Deserialize<'a>>(s: &'a mut [u8]) -> JsonResult<T> {
    #[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
    return serde_json::from_slice(s);
    #[cfg(feature = "simd-json")]
    return simd_json::from_slice(s);
}

#[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
pub(crate) use serde_json::to_vec as json_to_vec;
#[cfg(feature = "simd-json")]
pub(crate) use simd_json::to_vec as json_to_vec;
