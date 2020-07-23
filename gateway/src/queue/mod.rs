mod day_limiter;
mod large_bot_queue;

pub use large_bot_queue::LargeBotQueue;

use async_trait::async_trait;
use day_limiter::DayLimiter;
use futures_channel::{
    mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    oneshot::{self, Sender},
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::{fmt::Debug, time::Duration};
use tokio::time::delay_for;

#[async_trait]
pub trait Queue: Debug + Send + Sync {
    async fn request(&self, shard_id: [u64; 2]);
}

/// A local, in-process implementation of a [`Queue`] which manages the
/// connection attempts of one or more [`Shard`]s.
///
/// The queue will take incoming requests and then queue them, releasing one of
/// the requests every 6 seconds. The queue is necessary because there's a
/// ratelimit on how often shards can initiate sessions.
///
/// You usually won't need to handle this yourself, because the [`Cluster`] will
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
/// [`LargeBotQueue`]: ./struct.LargeBotQueue.html
/// [`Cluster`]: ../cluster/struct.Cluster.html
/// [`Queue`]: trait.Queue.html
/// [`Shard`]: ../shard/struct.Shard.html
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
        delay_for(DUR).await;
    }
}

#[async_trait]
impl Queue for LocalQueue {
    /// Request to be able to identify with the gateway. This will place this
    /// request behind all other requests, and the returned future will resolve
    /// once the request has been completed.
    async fn request(&self, [id, total]: [u64; 2]) {
        let (tx, rx) = oneshot::channel();

        if let Err(err) = self.0.clone().send(tx).await {
            tracing::warn!("skipping, send failed: {:?}", err);
            return;
        }

        tracing::info!("shard {}/{} waiting for allowance", id, total);

        let _ = rx.await;
    }
}
