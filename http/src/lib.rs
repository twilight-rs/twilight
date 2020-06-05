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
