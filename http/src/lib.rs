#![deny(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unused,
    warnings,
)]
#![allow(clippy::module_name_repetitions)]

pub mod client;
pub mod error;
pub mod ratelimiting;
pub mod request;
pub mod routing;

mod pending;

pub use crate::{
    client::Client,
    error::{Error, Result},
};
