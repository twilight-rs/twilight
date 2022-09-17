#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![deny(
    clippy::all,
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::pedantic,
    future_incompatible,
    missing_docs,
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
    clippy::unnecessary_wraps,
    clippy::used_underscore_binding
)]

pub mod error;
pub mod message;
pub mod stream;

mod channel;
mod command;
mod compression;
mod config;
mod connection;
mod event;
mod future;
mod json;
mod latency;
mod ratelimiter;
mod session;
mod shard;
mod tls;

pub use self::{
    channel::MessageSender,
    command::Command,
    config::{Config, ConfigBuilder, ShardId},
    event::EventTypeFlags,
    latency::Latency,
    ratelimiter::CommandRatelimiter,
    session::Session,
    shard::{ConnectionStatus, Shard},
};
pub use twilight_model::gateway::Intents;

#[doc(no_inline)]
pub use twilight_gateway_queue as queue;
#[doc(no_inline)]
pub use twilight_model::gateway::event::{Event, EventType};

/// Discord Gateway API version used by this crate.
pub const API_VERSION: u8 = 10;
