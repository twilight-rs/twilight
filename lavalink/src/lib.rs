#![deny(
    clippy::all,
    clippy::missing_const_for_fn,
    future_incompatible,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    unsafe_code,
    unused
)]
#![doc = include_str!("../README.md")]

pub mod client;
pub mod model;
pub mod node;
pub mod player;

#[cfg(feature = "http-support")]
pub mod http;

pub use self::{client::Lavalink, node::Node, player::PlayerManager};
