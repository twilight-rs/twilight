#![deny(
    clippy::all,
    clippy::missing_const_for_fn,
    clippy::pedantic,
    future_incompatible,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    unsafe_code,
    unused
)]
#![doc = include_str!("../README.md")]
#![allow(
    clippy::let_unit_value,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::semicolon_if_nothing_returned,
    clippy::used_underscore_binding
)]

pub mod cluster;
pub mod shard;

mod event;

pub use self::event::EventTypeFlags;
pub use twilight_model::gateway::Intents;

#[doc(no_inline)]
pub use self::{
    cluster::{Cluster, Config as ClusterConfig},
    shard::{Config as ShardConfig, Shard},
};
#[doc(no_inline)]
pub use twilight_gateway_queue as queue;
#[doc(no_inline)]
pub use twilight_model::gateway::event::{Event, EventType};

/// Discord API version used by this crate.
pub const API_VERSION: u8 = 10;
