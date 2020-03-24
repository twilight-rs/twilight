//! [![license badge][]][license link] [![rust badge]][rust link]
//! # twilight
//!
//! `twilight` is an asynchronous, simple, and extensible set of libraries which can
//! be used separately or in combination for the Discord API.
//!
//! The ecosystem of first-class crates includes `twilight-cache`,
//! `twilight-command-parser`, `twilight-gateway`, `twilight-http`, `twilight-model`,
//! and more. These are explained in detail below.
//!
//! The main `twilight` crate is a "skeleton crate": it includes all of the
//! non-vendor-specific crates in the `twilight` ecosystem.
//!
//! ## Installation
//!
//! Most of twilight requires at least 1.39+ (rust beta).
//!
//! Add this to your `Cargo.toml`'s `[dependencies]` section:
//!
//! ```toml
//! twilight = "0.0.1-alpha.0"
//! ```
//!
//! ## Crates
//!
//! These are crates that can work together for a full application experience.
//! You may not need all of these - such as `twilight-cache` - but they can be
//! mixed together to accomplish just what you need.
//!
//! ### `twilight-model`
//!
//! `twilight-model` is a set of models defining structures, enums, and bitflags
//! for the entirety of the Discord API. It is split into a number of
//! sub-modules, such as `gateway` for containing the WebSocket gateway types,
//! `guild` for containing types owned by guilds (servers), `voice` containing
//! the types used by the Voice WebSocket API, and more.
//!
//! These are all in a single crate so that you can use `gateway` models without
//! depending on `twilight-gateway`. One use case is if you write your own WebSocket
//! gateway implementation.
//!
//! ### `twilight-cache`
//!
//! `twilight-cache` is based on a single trait which can be implemented to use
//! custom third-party backends with a single ubiquitous interface. The Cache is
//! responsible for holding information about things like guilds, channels, role
//! information, voice states, and any other data that comes from Discord.
//!
//! Included by default is an `InMemoryCache` backend, which caches within the
//! process's memory.
//!
//! ### `twilight-gateway`
//!
//! `twilight-gateway` is an implementation of Discord's sharding gateway sessions.
//! This is responsible for receiving stateful events in real-time from Discord
//! and sending *some* stateful information.
//!
//! It includes two primary types: the Shard and Cluster.
//!
//! The Shard handles a single WebSocket connection and can manage up to 2500
//! guilds. If you manage a small bot in under about 2000 guilds, then this is
//! what you use. See the [Discord docs][docs:discord:sharding] for more
//! information on sharding.
//!
//! The Cluster is an interface which manages the health of the shards it
//! manages and proxies all of their events under one unified stream. This is
//! useful to use if you have a large bot in over 1000 or 2000 guilds.
//!
//! ### `twilight-command-parser`
//!
//! `twilight-command-parser` is a crate for parsing commands out of messages
//! received over the gateway. It finds messages commanding your bot and parses
//! the arguments out.
//!
//! ### `twilight-http`
//!
//! `twilight-http` is an HTTP client supporting all of the Discord REST API. It is
//! based on `hyper`. It meets Discord's ratelimiting requirements and supports
//! proxying.
//!
//!
//! ## Examples
//!
//! ```no_run
//! use twilight::{
//!     gateway::{shard::Event, Cluster, ClusterConfig},
//!     http::Client as HttpClient,
//! };
//! use futures::StreamExt;
//! use std::{env, error::Error};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
//!     let token = env::var("DISCORD_TOKEN")?;
//!     let http = HttpClient::new(&token);
//!
//!     let cluster_config = ClusterConfig::builder(&token).build();
//!     let cluster = Cluster::new(cluster_config);
//!     cluster.up().await?;
//!
//!     let mut events = cluster.events().await;
//!
//!     while let Some(event) = events.next().await {
//!         tokio::spawn(handle_event(event, http.clone()));
//!     }
//!
//!     Ok(())
//! }
//!
//! async fn handle_event(
//!     event: (u64, Event),
//!     http: HttpClient,
//! ) -> Result<(), Box<dyn Error + Send + Sync>> {
//!     match event {
//!         (id, Event::Ready(_)) => {
//!             println!("Connected on shard {}", id);
//!         }
//!         (_, Event::MessageCreate(msg)) => {
//!             if msg.content == "!ping" {
//!                 http.create_message(msg.channel_id).content("Pong!").await?;
//!             }
//!         }
//!         _ => {}
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## License
//!
//! All first-party crates are licensed under [ISC][LICENSE.md]
//!
//! [LICENSE.md]: https://github.com/twilight-rs/twilight/blob/master/LICENSE.md
//! [docs:discord:sharding]: https://discordapp.com/developers/docs/topics/gateway#sharding
//! [license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
//! [license link]: https://opensource.org/licenses/ISC
//! [logo]: https://raw.githubusercontent.com/twilight-rs/twilight/master/logo.png
//! [rust badge]: https://img.shields.io/badge/rust-1.39+%20(beta)-93450a.svg?style=flat-square
//! [rust link]: https://github.com/rust-lang/rust/milestone/66

#[cfg(feature = "cache")]
pub extern crate twilight_cache as cache;

#[cfg(feature = "command-parser")]
pub extern crate twilight_command_parser as command_parser;

#[cfg(feature = "gateway")]
pub extern crate twilight_gateway as gateway;

#[cfg(feature = "http")]
pub extern crate twilight_http as http;

#[cfg(feature = "model")]
pub extern crate twilight_model as model;
