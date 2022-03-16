#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]
#![deny(
    clippy::all,
    clippy::missing_const_for_fn,
    clippy::pedantic,
    future_incompatible,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    unsafe_code,
    unused,
    warnings
)]
#![deprecated(
    since = "0.8.1",
    note = "use interactions via `twilight-http` or `twilight-gateway`"
)]
#![doc = include_str!("../README.md")]

pub mod config;

mod arguments;
mod casing;
mod parser;

pub use self::{
    arguments::Arguments,
    config::CommandParserConfig,
    parser::{Command, Parser},
};
