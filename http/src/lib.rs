//! # twilight-http
//!
//! HTTP support for the twilight ecosystem.
//!
//! ## Features
//!
//! ### Deserialization
//!
//! `twilight-http` supports `serde_json` and `simd-json` for deserializing
//! responses. `serde_json` is enabled by default.
//!
//! #### `simd-json`
//!
//! The `simd-json` feature enables [`simd-json`] support to use simd features of
//! the modern cpus to deserialize responses faster. It is not enabled by
//! default, and instead the `serde_json` feature is enabled by default.
//!
//! To use this feature you need to also add these lines to
//! `<project root>/.cargo/config`:
//! ```toml
//! [build]
//! rustflags = ["-C", "target-cpu=native"]
//! ```
//!
//! You can also set the environment variable
//! `RUSTFLAGS="-C target-cpu=native"`. If you enable both `serde_json` and
//! `simd-json` at the same time, then `simd-json` will be used.
//!
//! #### `serde_json`
//!
//! `serde_json` is the inverse of `simd-json` and will use the `serde_json`
//! crate to deserialize responses.
//!
//! ### Runtimes
//!
//! `twilight-http` supports some of the popular async runtimes. The
//! `tokio-runtime` feature is enabled by default.
//!
//! #### `smol-runtime`
//!
//! Use [`smol`] as an asynchronous executor for background tasks. The
//! [`futures-timer`] crate will be used for asynchronous timing. If you're
//! using `smol` in your runtime, then you'll want to disable the
//! `tokio-runtime` feature and enable this, like so:
//!
//! ```toml
//! [dependencies.twilight-http]
//! default-features = false
//! features = ["serde_json", "smol-runtime"]
//! git = "https://github.com/twilight-rs/twilight"
//! ```
//!
//! #### `tokio-runtime`
//!
//! Use [`tokio`] as an asynchronous executor for background tasks and
//! asynchronous timing. If you're using `tokio` in your application, this is
//! what you want and you don't need to change anything in your dependencies.
//!
//! [`futures-timer`]: https://crates.io/crates/futures-timer
//! [`simd-json`]: https://crates.io/crates/simd-json
//! [`smol`]: https://crates.io/crates/smol
//! [`tokio`]: https://crates.io/crates/tokio

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

#[cfg(any(
    all(not(feature = "smol-runtime"), not(feature = "tokio-runtime")),
    all(feature = "smol-runtime", feature = "tokio-runtime"),
))]
compile_error!(
    "You must enable feature `smol-runtime` or `tokio-runtime`, but not neither or both"
);

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

use std::{future::Future, time::Duration};

#[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
use serde_json::Result as JsonResult;
#[cfg(feature = "simd-json")]
use simd_json::Result as JsonResult;

pub fn json_from_slice<'a, T: serde::de::Deserialize<'a>>(s: &'a mut [u8]) -> JsonResult<T> {
    #[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
    return serde_json::from_slice(s);
    #[cfg(feature = "simd-json")]
    return simd_json::from_slice(s);
}

#[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
pub use serde_json::to_vec as json_to_vec;
#[cfg(feature = "simd-json")]
pub use simd_json::to_vec as json_to_vec;

fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    #[cfg(feature = "smol")]
    smol::Task::spawn(fut).detach();

    #[cfg(feature = "tokio")]
    tokio::spawn(fut);
}

async fn delay(duration: Duration) {
    #[cfg(feature = "smol")]
    {
        use futures_timer::Delay;

        Delay::new(duration).await
    }

    #[cfg(feature = "tokio")]
    {
        tokio::time::delay_for(duration).await
    }
}

async fn timeout<T, F: Future<Output = T> + Unpin>(duration: Duration, future: F) -> Option<T> {
    #[cfg(feature = "smol")]
    {
        use futures::future::{self, Either};
        use futures_timer::Delay;

        match future::select(future, Delay::new(duration)).await {
            Either::Left((res, _)) => Some(res),
            Either::Right(_) => None,
        }
    }

    #[cfg(feature = "tokio")]
    {
        tokio::time::timeout(duration, future).await.ok()
    }
}
