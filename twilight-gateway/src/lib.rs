#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![warn(
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

pub mod error;
pub mod stream;

mod channel;
mod command;
mod config;
mod connection;
mod event;
mod future;
#[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
mod inflater;
mod json;
mod latency;
mod message;
mod ratelimiter;
mod session;
mod shard;
mod tls;

#[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
pub use self::inflater::Inflater;
pub use self::{
    channel::MessageSender,
    command::Command,
    config::{Config, ConfigBuilder},
    event::EventTypeFlags,
    json::parse,
    latency::Latency,
    message::Message,
    ratelimiter::CommandRatelimiter,
    session::Session,
    shard::{ConnectionStatus, Shard},
};
pub use twilight_model::gateway::{CloseFrame, Intents, ShardId};

#[doc(no_inline)]
pub use twilight_gateway_queue as queue;
#[doc(no_inline)]
pub use twilight_model::gateway::event::{Event, EventType};

/// Discord Gateway API version used by this crate.
pub const API_VERSION: u8 = 10;
