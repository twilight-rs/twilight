//! Types for working with and running connections to the gateway.
//!
//! At the heart of the `shard` module is the [`Shard`] itself: it's the
//! interface used to start a shard, send messages to the gateway, and receive
//! [`Event`]s from it, such as [new messages] or [channel deletions].
//!
//! Once running, the shard maintains [information about itself] that you can
//! obtain through it. This is information such as the latency or the current
//! [`Stage`] of the connection, like whether it's [disconnected] or [resuming]
//! the connection.
//!
//! Shards are configurable through the [`ConfigBuilder`] struct, which
//! provides a clean interface for correctly building a [`Config`].
//!
//! [`Config`]: config/struct.Config.html
//! [`ConfigBuilder`]: config/struct.ConfigBuilder.html
//! [`Event`]: event/enum.Event.html
//! [`Shard`]: struct.Shard.html
//! [`Stage`]: struct.Stage.html
//! [channel deletions]: event/enum.Event.html#variant.ChannelDelete
//! [new messages]: event/enum.Event.html#variant.MessageCreate

pub mod config;
pub mod error;
pub mod event;
pub mod stage;

mod connect;
mod heartbeat;
mod r#impl;
mod inflater;
mod processor;
mod session;
mod sink;
mod socket_forwarder;

pub use self::{
    config::Config,
    error::{Error, Result},
    event::{Event, EventType},
    heartbeat::Latency,
    r#impl::{Information, Shard},
    sink::ShardSink,
    stage::Stage,
};

use tokio_net::tcp::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

type ShardStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
