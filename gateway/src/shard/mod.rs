//! Types for working with and running connections to the gateway.
//!
//! At the heart of the `shard` module is the [`Shard`] itself: it's the
//! interface used to start a shard, send messages to the gateway, and receive
//! [`Event`]s from it, such as [new messages] or [channel deletions].
//!
//! Once running, the shard maintains [information about itself] that you can
//! obtain through it. This is information such as the latency or the current
//! [`Stage`] of the connection, like whether it's [`Disconnected`] or
//! [`Resuming`] the connection.
//!
//! Shards are configurable through the [`ConfigBuilder`] struct, which
//! provides a clean interface for correctly building a [`Config`].
//!
//! [`Config`]: config/struct.Config.html
//! [`ConfigBuilder`]: config/struct.ConfigBuilder.html
//! [`Event`]: event/enum.Event.html
//! [`Shard`]: struct.Shard.html
//! [`Stage`]: enum.Stage.html
//! [`Disconnected`]: enum.Stage.html#variant.Disconnected
//! [`Resuming`]: enum.Stage.html#variant.Resuming
//! [channel deletions]: event/enum.Event.html#variant.ChannelDelete
//! [new messages]: event/enum.Event.html#variant.MessageCreate

pub mod config;
pub mod error;
pub mod event;
pub mod stage;

mod r#impl;
mod processor;
mod sink;

pub use self::{
    config::Config,
    error::{Error, Result},
    event::{Event, EventType},
    processor::heartbeat::Latency,
    r#impl::{Information, Shard},
    sink::ShardSink,
    stage::Stage,
};

use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

type ShardStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
