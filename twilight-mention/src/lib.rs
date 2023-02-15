#![deny(
    clippy::missing_const_for_fn,
    clippy::pedantic,
    missing_docs,
    unsafe_code
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::unnecessary_wraps,
    clippy::used_underscore_binding
)]
#![doc = include_str!("../README.md")]

pub mod fmt;
pub mod parse;
pub mod timestamp;

#[doc(no_inline)]
pub use fmt::Mention;

#[doc(no_inline)]
pub use parse::ParseMention;
