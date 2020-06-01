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
