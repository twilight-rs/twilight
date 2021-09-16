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
//! Shards are configurable through the [`ShardBuilder`], which provides a clean
//! interface for correctly configuring a shard.
//!
//! [`Event`]: ::twilight_model::gateway::event::Event
//! [`Disconnected`]: Stage::Disconnected
//! [`Resuming`]: Stage::Resuming
//! [channel deletions]: ::twilight_model::gateway::event::Event::ChannelDelete
//! [information about itself]: Shard::info
//! [new messages]: ::twilight_model::gateway::event::Event::MessageCreate

pub mod raw_message;
pub mod stage;

mod builder;
mod command;
mod config;
mod emitter;
mod event;
mod r#impl;
mod json;
mod processor;

pub use self::{
    builder::{
        LargeThresholdError, LargeThresholdErrorType, ShardBuilder, ShardIdError, ShardIdErrorType,
    },
    command::Command,
    config::Config,
    event::Events,
    processor::heartbeat::Latency,
    r#impl::{
        CommandError, CommandErrorType, Information, ResumeSession, SendError, SendErrorType,
        SessionInactiveError, Shard, ShardStartError, ShardStartErrorType,
    },
    stage::Stage,
};

use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

type ShardStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
