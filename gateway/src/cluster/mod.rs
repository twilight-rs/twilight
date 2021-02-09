//! The cluster is a manager for running and maintaining multiple shards,
//! bringing their event streams into one unified stream.
//!
//! # Examples
//!
//! Start a cluster of 10 shards and print when a shard is connected,
//! disconnected, and when new message commands come in:
//!
//! ```no_run
//! use twilight_gateway::{Cluster, Event, Intents};
//! use futures::StreamExt;
//! use std::env;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let token = env::var("DISCORD_TOKEN")?;
//!     let intents = Intents::GUILD_BANS | Intents::GUILD_EMOJIS | Intents::GUILD_MESSAGES;
//!     let cluster = Cluster::new(token, intents).await?;
//!
//!     cluster.up().await;
//!
//!     let mut events = cluster.events();
//!
//!     while let Some((shard_id, event)) = events.next().await {
//!         tokio::spawn(handle_event(cluster.clone(), shard_id, event));
//!     }
//!
//!     println!("Cluster is now shutdown");
//!
//!     Ok(())
//! }
//!
//! async fn handle_event(cluster: Cluster, shard_id: u64, event: Event) {
//!     match event {
//!         Event::ShardConnected { .. } => {
//!             println!("Shard {} is now connected", shard_id);
//!         },
//!         Event::ShardDisconnected { .. } => {
//!             println!("Shard {} is now disconnected", shard_id);
//!         },
//!         Event::MessageCreate(msg) if msg.content == "!latency" => {
//!             if let Some(shard) = cluster.shard(shard_id) {
//!                 if let Ok(info) = shard.info() {
//!                     println!("Shard {}'s latency is {:?}", shard_id, info.latency());
//!                 }
//!             }
//!         },
//!         Event::MessageCreate(msg) if msg.content == "!shutdown" => {
//!             println!("Got a shutdown request from shard {}", shard_id);
//!
//!             cluster.down();
//!         },
//!         _ => {},
//!     }
//! }
//! ```

mod builder;
mod config;
mod r#impl;

pub use self::{
    builder::{ClusterBuilder, ShardScheme, ShardSchemeRangeError, ShardSchemeRangeErrorType},
    config::Config,
    r#impl::{
        Cluster, ClusterCommandError, ClusterCommandErrorType, ClusterStartError,
        ClusterStartErrorType,
    },
};
