#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![warn(
    clippy::missing_const_for_fn,
    clippy::pedantic,
    missing_docs,
    unsafe_code
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::unnecessary_wraps
)]

pub mod client;
pub mod model;
pub mod node;
pub mod player;

#[cfg(feature = "http-support")]
pub mod http;

pub use self::{client::Lavalink, node::Node, player::PlayerManager};
