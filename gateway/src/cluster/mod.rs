//! The [`Cluster`] is a manager for running and maintaing multiple shards,
//! bringing their event streams into one unified stream.
//!
//! # Examples
//!
//! Start a cluster of 10 shards and print when a shard is connected,
//! disconnected, and when new commands come in:
///
/// ```no_run
/// use twilight_gateway::{cluster::{Cluster, ClusterConfig}, Event};
/// use futures::StreamExt;
/// use std::env;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     let token = env::var("DISCORD_TOKEN")?;
///     let cluster = Cluster::new(token);
///
///     cluster.up().await?;
///
///     let mut events = cluster.events().await;
///
///     while let Some((shard_id, event)) = events.next().await {
///         tokio::spawn(handle_event(cluster.clone(), shard_id, event));
///     }
///
///     println!("Cluster is now shutdown");
///
///     Ok(())
/// }
///
/// async fn handle_event(cluster: Cluster, shard_id: u64, event: Event) {
///     match event {
///         Event::ShardConnected { .. } => {
///             println!("Shard {} is now connected", shard_id);
///         },
///         Event::ShardDisconnected { .. } => {
///             println!("Shard {} is now disconnected", shard_id);
///         },
///         Event::MessageCreate(msg) if msg.content == "!latency" => {
///             if let Some(shard) = cluster.shard(shard_id).await {
///                 let info = shard.info().await;
///
///                 println!("Shard {}'s latency is {:?}", shard_id, info.latency());
///             }
///         },
///         Event::MessageCreate(msg) if msg.content == "!shutdown" => {
///             println!("Got a shutdown request from shard {}", shard_id);
///
///             cluster.down().await;
///         },
///         _ => {},
///     }
/// }
/// ```
///
/// [`Cluster`]: struct.Cluster.html
pub mod config;
pub mod error;

mod r#impl;

pub use self::{
    config::ClusterConfig,
    error::{Error, Result},
    r#impl::Cluster,
};
