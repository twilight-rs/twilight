#![deny(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unsafe_code
)]
#![allow(clippy::module_name_repetitions)]

pub mod cluster;
pub mod queue;
pub mod shard;

mod event;
mod listener;

pub use self::shard::{Config as ShardConfig, Shard};
