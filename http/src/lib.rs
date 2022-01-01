#![doc = include_str!("../README.md")]
#![deny(
    clippy::all,
    clippy::missing_const_for_fn,
    clippy::pedantic,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    unsafe_code,
    unused,
    warnings
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::semicolon_if_nothing_returned,
    clippy::unnecessary_wraps
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod api_error;
pub mod client;
pub mod error;
pub mod request;
pub mod response;
pub mod routing;

mod json;

/// Discord API version used by this crate.
pub const API_VERSION: u8 = 9;

pub use crate::{client::Client, error::Error, response::Response};

#[cfg(not(any(
    feature = "native",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
)))]
compile_error!(
    "Either the `native`, `rustls-native-roots` or `rustls-webpki-roots` feature must be enabled."
);
