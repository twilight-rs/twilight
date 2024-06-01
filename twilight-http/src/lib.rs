#![doc = include_str!("../README.md")]
#![warn(clippy::missing_const_for_fn, clippy::pedantic, unsafe_code)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::unnecessary_wraps
)]

pub mod api_error;
pub mod client;
pub mod error;
pub mod request;
pub mod response;
pub mod routing;

mod json;
mod query_formatter;

/// Discord API version used by this crate.
pub const API_VERSION: u8 = 10;

pub use crate::{client::Client, error::Error, response::Response};
