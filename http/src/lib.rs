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
    clippy::missing_errors_doc
)]

pub mod client;
pub mod error;
pub mod ratelimiting;
pub mod request;
pub mod routing;

mod api_error;

pub use crate::{
    api_error::ApiError,
    client::Client,
    error::{Error, Result},
};
