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
pub mod http;
pub mod model;
pub mod node;
pub mod player;

pub use self::{client::Lavalink, node::Node, player::PlayerManager};

/// Lavalink API version used by this crate.
pub const API_VERSION: u8 = 4;
