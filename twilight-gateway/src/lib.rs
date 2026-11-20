#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![warn(
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::pedantic,
    missing_docs,
    unsafe_code
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::unnecessary_wraps
)]

pub mod error;

mod channel;
mod command;
#[cfg(any(feature = "zlib", feature = "zstd"))]
mod compression;
mod config;
mod event;
mod json;
mod latency;
mod message;
mod ratelimiter;
mod session;
mod shard;
mod stream;

pub use self::{
    channel::MessageSender,
    command::Command,
    config::{Config, ConfigBuilder},
    event::EventTypeFlags,
    json::parse,
    latency::Latency,
    message::Message,
    ratelimiter::CommandRatelimiter,
    session::Session,
    shard::{Shard, ShardState},
    stream::StreamExt,
};
pub use twilight_model::gateway::{CloseFrame, Intents, ShardId};

#[doc(no_inline)]
pub use twilight_gateway_queue as queue;
#[doc(no_inline)]
pub use twilight_model::gateway::event::{Event, EventType};

/// Discord Gateway API version used by this crate.
pub const API_VERSION: u8 = 10;

/// Creates an iterator of a single bucket's worth of shard identifiers.
///
/// Each bucket holds a consecutive range of identifiers and all identifiers
/// share the same `total` value.
///
/// # Strategy
///
/// Shards may be bucketed per-thread for a thread-per-core architecture and/or
/// per-machine for horizontal scaling.
///
/// # Examples
///
/// Create 1 bucket with the recommended shard count:
///
/// ```no_run
/// use std::env;
/// use twilight_http::Client;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> anyhow::Result<()> {
/// let http = Client::new(env::var("TOKEN")?);
/// let info = http.gateway().authed().await?.model().await?;
///
/// let shards = twilight_gateway::bucket(0, 1, info.shards);
/// assert_eq!(shards.len(), info.shards as usize);
/// # anyhow::Ok(())
/// # }
/// ```
///
/// Create 2 buckets with 25 identifiers:
///
/// ```
/// let bucket_1 = twilight_gateway::bucket(0, 2, 25);
/// let bucket_2 = twilight_gateway::bucket(1, 2, 25);
///
/// assert_eq!(bucket_1.len(), 13);
/// assert_eq!(bucket_2.len(), 12);
/// ```
///
/// # Panics
///
/// Panics if the bucket id is greater than or equal to the total number of
/// buckets.
#[inline]
#[track_caller]
pub fn bucket(
    bucket_id: u16,
    buckets: u16,
    shards: u32,
) -> impl DoubleEndedIterator<Item = ShardId> + ExactSizeIterator {
    let bucket_id = u32::from(bucket_id);
    let buckets = u32::from(buckets);
    assert!(bucket_id < buckets, "bucket_id must be less than buckets");

    let (q, r) = (shards / buckets, shards % buckets);

    let len = q + u32::from(bucket_id < r);
    let start = bucket_id * q + r.min(bucket_id);

    // Compiler hint deduced from mathematical proof.
    if shards < start + len {
        std::process::abort();
    }
    (start..start + len).map(move |id| ShardId::new(id, shards))
}
