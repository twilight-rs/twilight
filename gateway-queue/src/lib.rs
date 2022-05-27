#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![deny(unsafe_code)]
#![doc = include_str!("../README.md")]

#[cfg(any(
    feature = "native",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
))]
mod day_limiter;
#[cfg(any(
    feature = "native",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
))]
mod large_bot_queue;

#[cfg(any(
    feature = "native",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
))]
pub use large_bot_queue::LargeBotQueue;

use std::{
    fmt::Debug,
    future::{self, Future},
    pin::Pin,
    time::Duration,
};
use tokio::{
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot::{self, Sender},
    },
    time::sleep,
};

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
        let (tx, rx) = unbounded_channel();

        tokio::spawn(waiter(rx));

        Self(tx)
    }
}

async fn waiter(mut rx: UnboundedReceiver<Sender<()>>) {
    const DUR: Duration = Duration::from_secs(6);
    while let Some(req) = rx.recv().await {
        if let Err(source) = req.send(()) {
            tracing::warn!("skipping, send failed: {source:?}");
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

            if let Err(source) = self.0.send(tx) {
                tracing::warn!("skipping, send failed: {source:?}");
                return;
            }

            tracing::info!("shard {id}/{total} waiting for allowance");

            let _ = rx.await;
        })
    }
}

/// An implementation of [`Queue`] that instantly allows requests.
///
/// Useful when running behind a proxy gateway. Running without a
/// functional queue **will** get you ratelimited.
#[derive(Debug)]
pub struct NoOpQueue;

impl Queue for NoOpQueue {
    fn request(&'_ self, [_id, _total]: [u64; 2]) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(future::ready(()))
    }
}

#[cfg(test)]
mod tests {
    use super::{LocalQueue, NoOpQueue, Queue};
    use static_assertions::{assert_impl_all, assert_obj_safe};
    use std::fmt::Debug;

    assert_impl_all!(LocalQueue: Clone, Debug, Queue, Send, Sync);
    assert_impl_all!(NoOpQueue: Debug, Queue, Send, Sync);
    assert_impl_all!(dyn Queue: Debug, Send, Sync);
    assert_obj_safe!(Queue);
}
