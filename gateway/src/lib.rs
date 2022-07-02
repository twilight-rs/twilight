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
    clippy::let_unit_value,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::semicolon_if_nothing_returned,
    clippy::used_underscore_binding,
    unused
)]

pub mod config;
pub mod error;
pub mod latency;
pub mod message;

mod channel;
mod command;
mod compression;
mod event;
mod future;
mod json;
mod ratelimiter;
mod session;
mod shard;
pub mod stream;
mod tls;

pub use self::{
    channel::ShardMessageSender,
    command::Command,
    event::EventTypeFlags,
    ratelimiter::CommandRatelimiter,
    session::Session,
    shard::{ConnectionStatus, Shard},
};
pub use twilight_model::gateway::Intents;

#[doc(no_inline)]
pub use twilight_gateway_queue as queue;
#[doc(no_inline)]
pub use twilight_model::gateway::event::{Event, EventType};

use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

/// Discord Gateway API version used by this crate.
pub const API_VERSION: u8 = 10;

/// URL of the Discord gateway.
const GATEWAY_URL: &str = "wss://gateway.discord.gg";

/// [`tokio_tungstenite`] library Websocket connection.
///
/// Connections are used by [`Shard`]s when [initially connecting] and when
/// reconnecting.
///
/// [initially connecting]: Shard::with_config
type Connection = WebSocketStream<MaybeTlsStream<TcpStream>>;
