//! # twilight-lavalink
//!
//! `twilight-lavalink` is a client for [Lavalink] as part of the twilight
//! ecosystem.
//!
//! It includes support for managing multiple nodes, a player manager for
//! conveniently using players to send events and retrieve information for each
//! guild, and an HTTP module for creating requests using the [`http`] crate and
//! providing models to deserialize their responses.
//!
//! ## Features
//!
//! Included is the `http-support` feature.
//!
//! The `http-support` feature adds support for the `http` module to return
//! request types from the [`http`] crate. This is enabled by default.
//!
//! ## Examples
//!
//! Create a [client], add a [node], and give events to the client to [process]
//! events:
//!
//! ```no_run
//! use futures_util::stream::StreamExt;
//! use std::{
//!     convert::TryInto,
//!     env,
//!     error::Error,
//!     future::Future,
//!     net::SocketAddr,
//!     str::FromStr,
//! };
//! use twilight_gateway::{Event, Shard};
//! use twilight_http::Client as HttpClient;
//! use twilight_lavalink::{http::LoadedTracks, model::Play, Lavalink};
//! use twilight_model::{
//!     channel::Message,
//!     gateway::payload::MessageCreate,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//!     let token = env::var("DISCORD_TOKEN")?;
//!     let lavalink_host = SocketAddr::from_str(&env::var("LAVALINK_HOST")?)?;
//!     let lavalink_auth = env::var("LAVALINK_AUTHORIZATION")?;
//!     let shard_count = 1u64;
//!
//!     let http = HttpClient::new(&token);
//!     let user_id = http.current_user().await?.id;
//!
//!     let lavalink = Lavalink::new(user_id, shard_count);
//!     lavalink.add(lavalink_host, lavalink_auth).await?;
//!
//!     let shard = Shard::new(token).await?;
//!
//!     let mut events = shard.events().await;
//!
//!     while let Some(event) = events.next().await {
//!         lavalink.process(&event).await?;
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! [Lavalink]: https://github.com/Frederikam/Lavalink
//! [client]: client/struct.Lavalink.html
//! [node]: node/struct.Node.html
//! [process]: client/struct.Lavalink.html#method.process
//! [`http`]: https://crates.io/crates/http

#![deny(
    clippy::all,
    future_incompatible,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    unused,
    warnings
)]

pub mod client;
pub mod model;
pub mod node;
pub mod player;

#[cfg(feature = "http")]
pub mod http;

pub use self::{client::Lavalink, node::Node, player::PlayerManager};
