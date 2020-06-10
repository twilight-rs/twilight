//! # twilight-gateway
//!
//! `twilight-gateway` is an implementation of Discord's sharding gateway sessions.
//! This is responsible for receiving stateful events in real-time from Discord
//! and sending *some* stateful information.
//!
//! It includes two primary types: the Shard and Cluster.
//!
//! The Shard handles a single websocket connection and can manage up to 2500
//! guilds. If you manage a small bot in under about 2000 guilds, then this is
//! what you use. See the [Discord docs][docs:discord:sharding] for more
//! information on sharding.
//!
//! The Cluster is an interface which manages the health of the shards it
//! manages and proxies all of their events under one unified stream. This is
//! useful to use if you have a large bot in over 1000 or 2000 guilds.
//!
//! ## Features
//!
//! `twilight-gateway` includes only a feature: `simd-json`.
//!
//! `simd` feature enables [simd-json] support to use simd features of the modern cpus
//! to deserialize json data faster. It is not enabled by default since not every cpu has those features.
//! To use this feature you need to also add these lines to a file in `<project root>/.cargo/config`
//! ```toml
//! [build]
//! rustflags = ["-C", "target-cpu=native"]
//! ```
//! you can also use this environment variable `RUSTFLAGS="-C target-cpu=native"`. If you enable both
//! `serde_json` and `simd-json` at the same time; this crate uses `simd-json`. But it is recommended to
//! disable `serde_json` if you are going to use `simd-json`. It is easy to switch to out:
//!
//! ```toml
//! [dependencies]
//! twilight-gateway = { default-features = false, features = ["simd-json"], git = "https://github.com/twilight-rs/twilight" }
//! ```
//!
//! [simd-json]: https://github.com/simd-lite/simd-json

#![deny(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unsafe_code
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    // This issue made it into a stable clippy:
    //
    // <https://github.com/rust-lang/rust-clippy/issues/5360>
    clippy::used_underscore_binding
)]

pub mod cluster;
pub mod queue;
pub mod shard;

mod event;
mod listener;

pub use self::{
    cluster::{Cluster, ClusterConfig},
    event::EventTypeFlags,
    shard::{Shard, ShardConfig},
};
pub use twilight_model::gateway::event::{Event, EventType};

#[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
use serde_json::Result as JsonResult;
#[cfg(feature = "simd-json")]
use simd_json::Result as JsonResult;

pub(crate) fn json_from_slice<'a, T: serde::de::Deserialize<'a>>(s: &'a mut [u8]) -> JsonResult<T> {
    #[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
    return serde_json::from_slice(s);
    #[cfg(feature = "simd-json")]
    return simd_json::from_slice(s);
}
pub(crate) fn json_from_str<'a, T: serde::de::Deserialize<'a>>(s: &'a mut str) -> JsonResult<T> {
    #[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
    return serde_json::from_str(s);
    #[cfg(feature = "simd-json")]
    return simd_json::from_str(s);
}

#[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
pub(crate) use serde_json::to_vec as json_to_vec;
#[cfg(feature = "simd-json")]
pub(crate) use simd_json::to_vec as json_to_vec;

#[cfg(all(feature = "serde_json", not(feature = "simd-json")))]
pub(crate) use serde_json::to_string as json_to_string;
#[cfg(feature = "simd-json")]
pub(crate) use simd_json::to_string as json_to_string;
