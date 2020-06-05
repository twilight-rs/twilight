#![deny(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unsafe_code
)]
#![allow(
    clippy::explicit_deref_methods,
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
