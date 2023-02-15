#![deny(
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
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

pub mod channel;
pub mod command;
pub mod component;
pub mod embed;
pub mod message;
pub mod request;
pub mod sticker;
