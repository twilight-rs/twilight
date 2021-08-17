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
//! use std::{env, sync::Arc};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let token = env::var("DISCORD_TOKEN")?;
//!     let intents = Intents::GUILD_BANS | Intents::GUILD_EMOJIS | Intents::GUILD_MESSAGES;
//!     let (cluster, mut events) = Cluster::new(token, intents).await?;
//!     let cluster = Arc::new(cluster);
//!     cluster.up().await;
//!
//!     while let Some((shard_id, event)) = events.next().await {
//!         tokio::spawn(handle_event(Arc::clone(&cluster), shard_id, event));
//!     }
//!
//!     println!("Cluster is now shutdown");
//!
//!     Ok(())
//! }
//!
//! async fn handle_event(cluster: Arc<Cluster>, shard_id: u64, event: Event) {
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
//!
//! Start bucket 1 of a very large bot with 320 shards and a maximum concurrency
//! of 16:
//!
//! ```no_run
//! use twilight_gateway::{cluster::ShardScheme, Cluster, Event, Intents};
//! use futures::StreamExt;
//! use std::{convert::TryFrom, env};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let token = env::var("DISCORD_TOKEN")?;
//!     let intents = Intents::GUILD_MESSAGES;
//!     let scheme = ShardScheme::try_from((1, 16, 320))?;
//!     let (cluster, mut events) = Cluster::builder(token, intents)
//!         .shard_scheme(scheme)
//!         .build()
//!         .await?;
//!
//!     cluster.up().await;
//!
//!     while let Some((shard_id, event)) = events.next().await {
//!         println!("got event type {:?}", event.kind());
//!     }
//!
//!     println!("Cluster is now shutdown");
//!
//!     Ok(())
//! }
//! ```

pub mod scheme;

mod builder;
mod config;
mod event;
mod r#impl;

pub use self::{
    builder::ClusterBuilder,
    config::Config,
    event::Events,
    r#impl::{
        Cluster, ClusterCommandError, ClusterCommandErrorType, ClusterStartError,
        ClusterStartErrorType, Shards,
    },
    scheme::{ShardScheme, ShardSchemeRangeError, ShardSchemeRangeErrorType},
};
