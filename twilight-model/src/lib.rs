#![deny(
    clippy::all,
    clippy::missing_const_for_fn,
    clippy::pedantic,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    unsafe_code,
    unused
)]
#![doc = include_str!("../README.md")]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::semicolon_if_nothing_returned,
    clippy::used_underscore_binding
)]

pub mod application;
pub mod channel;
pub mod gateway;
pub mod guild;
pub mod http;
pub mod id;
pub mod oauth;
pub mod user;
pub mod util;
pub mod voice;

mod visitor;

#[cfg(test)]
mod test;
