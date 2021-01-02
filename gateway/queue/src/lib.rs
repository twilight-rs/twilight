//! Ratelimiting functionality for queueing new gateway sessions.
//!
//! The gateway ratelimits how often clients can initialize new sessions.
//! Instances of a queue are given to shards so that they can request to
//! initialize a session.
//!
//! Queue implementations must point to the same broker so that all shards
//! across all clusters, processes, and other forms of multi-serviced
//! applications, can work together and use the same ratelimiting source. That
//! is, if you for example have two clusters in two different processes, then
//! the two processes must use some unified form of ratelimiting: this can
//! either mean using IPC to communicate ratelimiting or a broker.
//!
//! # Provided queues
//!
//! Most users only need the [`LocalQueue`]: it's a single-process queue for
//! smaller bots. Larger bots need the [`LargeBotQueue`], which supports
//! single-process [Sharding for Very Large Bots] through the use of bucket
//! releasing.
//!
//! By default, the gateway's `Cluster` and `Shard`s use the [`LocalQueue`]. You
//! can override this in the `ClusterBuilder::queue` and `ShardBuilder::queue`
//! configuration methods.
//!
//! # Advanced use cases
//!
//! Large bots, and smaller bots out of design, may need to implement their own
//! queue. The most common reason to need this is if you have clusters in
//! multiple processes. You'll need a broker to manage ratelimiting across them
//! all so a [`Queue`] trait is provided that shards can use to make requests to
//! create sessions.
//!
//! [Sharding for Very Large Bots]: https://discord.com/developers/docs/topics/gateway#sharding-for-very-large-bots

mod day_limiter;
mod large_bot_queue;

pub use large_bot_queue::LargeBotQueue;

use day_limiter::DayLimiter;
use futures_channel::{
    mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    oneshot::{self, Sender},
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::{fmt::Debug, future::Future, pin::Pin, time::Duration};
use tokio::time::sleep;

/// Queue for shards to request the ability to initialize new sessions with the
/// gateway.
///
/// This will usually only need to be implemented when you have a multi-process
/// cluster setup. Refer to the [module-level] documentation for more
/// information.
///
/// [module-level]: crate
pub trait Queue: Debug + Send + Sync {
    /// A shard has requested the ability to request a session initialization
    /// with the gateway.
    ///
    /// The returned future must resolve only when the shard can initiate the
    /// session.
    fn request<'a>(&'a self, shard_id: [u64; 2]) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}

/// A local, in-process implementation of a [`Queue`] which manages the
/// connection attempts of one or more shards.
///
/// The queue will take incoming requests and then queue them, releasing one of
/// the requests every 6 seconds. The queue is necessary because there's a
/// ratelimit on how often shards can initiate sessions.
///
/// You usually won't need to handle this yourself, because the `Cluster` will
/// do that for you when managing multiple shards.
///
/// # When not to use this
///
/// This queue implementation is "local", meaning it's intended to be used if
/// you manage shards only in this process. If you run shards in multiple
/// different processes (do you utilize microservices a lot?), then you **must
/// not** use this implementation. Shards across multiple processes may
/// create new sessions at the same time, which is bad.
///
/// It should also not be used for very large sharding, for that the
/// [`LargeBotQueue`] can be used.
///
/// If you can't use this, look into an alternative implementation of the
/// [`Queue`], such as the [`gateway-queue`] broker.
///
/// [`gateway-queue`]: https://github.com/twilight-rs/gateway-queue
#[derive(Clone, Debug)]
pub struct LocalQueue(UnboundedSender<Sender<()>>);

impl Default for LocalQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalQueue {
    /// Creates a new local queue.
    pub fn new() -> Self {
        let (tx, rx) = unbounded();

        tokio::spawn(waiter(rx));

        Self(tx)
    }
}

async fn waiter(mut rx: UnboundedReceiver<Sender<()>>) {
    const DUR: Duration = Duration::from_secs(6);
    while let Some(req) = rx.next().await {
        if let Err(err) = req.send(()) {
            tracing::warn!("skipping, send failed: {:?}", err);
        }
        sleep(DUR).await;
    }
}

impl Queue for LocalQueue {
    /// Request to be able to identify with the gateway. This will place this
    /// request behind all other requests, and the returned future will resolve
    /// once the request has been completed.
    fn request(&'_ self, [id, total]: [u64; 2]) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            let (tx, rx) = oneshot::channel();

            if let Err(err) = self.0.clone().send(tx).await {
                tracing::warn!("skipping, send failed: {:?}", err);
                return;
            }

            tracing::info!("shard {}/{} waiting for allowance", id, total);

            let _ = rx.await;
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{LocalQueue, Queue};
    use static_assertions::{assert_impl_all, assert_obj_safe};
    use std::fmt::Debug;

    assert_impl_all!(LocalQueue: Clone, Debug, Queue, Send, Sync);
    assert_impl_all!(dyn Queue: Debug, Send, Sync);
    assert_obj_safe!(Queue);
}
