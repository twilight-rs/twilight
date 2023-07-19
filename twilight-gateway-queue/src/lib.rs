#![doc = include_str!("../README.md")]
#![warn(
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::pedantic,
    missing_docs,
    unsafe_code
)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

mod in_memory;

pub use in_memory::InMemoryQueue;

use tokio::{sync::oneshot, time::Duration};

/// Period between buckets.
pub const IDENTIFY_DELAY: Duration = Duration::from_secs(5);

/// Duration from the first identify until the remaining count resets to the
/// total count.
pub const LIMIT_PERIOD: Duration = Duration::from_secs(60 * 60 * 24);

/// Abstraction for types processing gateway identify requests.
pub trait Queue {
    /// Enqueue a shard with this ID.
    ///
    /// Send `()` to signal the shard to proceed. Note that shards may have
    /// dropped the receiver prior.
    ///
    /// Closing the channel should causes the shard to requeue.
    fn enqueue(&self, id: u32) -> oneshot::Receiver<()>;
}

impl<T> Queue for &T
where
    T: Queue,
{
    fn enqueue(&self, shard: u32) -> oneshot::Receiver<()> {
        (**self).enqueue(shard)
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;
    use static_assertions::assert_obj_safe;

    assert_obj_safe!(Queue);
}
