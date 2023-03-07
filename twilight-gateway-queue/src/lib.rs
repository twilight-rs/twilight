#![cfg_attr(docsrs, feature(doc_auto_cfg))]
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

#[cfg(feature = "twilight-http")]
mod large_bot_queue;

#[cfg(feature = "twilight-http")]
pub use large_bot_queue::LargeBotQueue;

use std::{
    fmt::Debug,
    future::{self, Future},
    pin::Pin,
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{mpsc, Notify},
    time::sleep,
};

/// Required wait duration between `IDENTIFY` commands.
const WAIT_BETWEEN_REQUESTS: Duration = Duration::from_secs(5);

/// Queue for shards to request the ability to send an `IDENTIFY` command to
/// initialize a new session with the gateway.
///
/// In general, you should not need to implement this trait yourself. Refer to
/// the [module-level] documentation for more information.
///
/// [module-level]: crate
pub trait Queue: Debug + Send + Sync {
    /// Request to send an `IDENTIFY` command.
    ///
    /// The returned future will resolve when the shard should send the command.
    fn request<'a>(&'a self, shard_id: u32) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}

/// A local, in-process implementation of [`Queue`] for bots with a
/// `max_concurrency` of 1.
///
/// Permits an `IDENTIFY` command every 5 seconds, regardless of the bot's
/// `max_concurrency`, and does not account for the daily `IDENTIFY` limit. See
/// [`LargeBotQueue`] for an implementation with these features.
#[derive(Debug)]
pub struct LocalQueue {
    /// Notifier for when an identify command can be sent or when the associated
    /// task shutdown.
    notify_ready: Arc<Notify>,
    /// Seder to associated task when upon receiving a request.
    tx: mpsc::UnboundedSender<()>,
}

impl Default for LocalQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalQueue {
    /// Creates a new local queue.
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        let notify = Arc::new(Notify::new());

        tokio::spawn(waiter(rx, Arc::clone(&notify)));

        Self {
            notify_ready: notify,
            tx,
        }
    }
}

/// Notifies requests as ready each [`WAIT_BETWEEN_REQUESTS`].
///
/// To not wait forever, this exits when the channel closes.
async fn waiter(mut rx: mpsc::UnboundedReceiver<()>, notify_ready: Arc<Notify>) {
    while rx.recv().await.is_some() {
        notify_ready.notify_one();
        sleep(WAIT_BETWEEN_REQUESTS).await;
    }

    // LargeBotQueue reuses this and needs to know when the task is shutdown.
    notify_ready.notify_one();
}

impl Queue for LocalQueue {
    fn request(&self, _: u32) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            self.tx.send(()).expect("channel not closed");

            self.notify_ready.notified().await;
        })
    }
}

/// An implementation of [`Queue`] that instantly permits requests.
///
/// Useful when running behind a proxy gateway. Running without a
/// functional queue **will** get you ratelimited.
#[derive(Debug, Default)]
pub struct NoOpQueue;

impl Queue for NoOpQueue {
    fn request(&self, _: u32) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(future::ready(()))
    }
}

#[cfg(test)]
mod tests {
    use super::{LocalQueue, NoOpQueue, Queue, WAIT_BETWEEN_REQUESTS};
    use static_assertions::{assert_impl_all, assert_obj_safe};
    use std::{fmt::Debug, time::Duration};
    use tokio::time;

    #[cfg(feature = "twilight-http")]
    assert_impl_all!(super::LargeBotQueue: Debug, Queue, Send, Sync);
    assert_impl_all!(LocalQueue: Debug, Default, Queue, Send, Sync);
    assert_impl_all!(NoOpQueue: Debug, Default, Queue, Send, Sync);
    assert_impl_all!(dyn Queue: Debug, Send, Sync);
    assert_obj_safe!(Queue);

    async fn cancel_dropped_request(queue: impl Queue) {
        let now = time::Instant::now();
        queue.request(0).await;

        drop(queue.request(0));

        queue.request(0).await;
        assert!(now.elapsed() < 2 * WAIT_BETWEEN_REQUESTS);
    }

    async fn cancel_polled_request(queue: impl Queue) {
        let now = time::Instant::now();
        queue.request(0).await;

        // Force cancel request.
        time::timeout(Duration::from_secs(1), queue.request(0))
            .await
            .unwrap_err();

        queue.request(0).await;
        assert!(now.elapsed() < 2 * WAIT_BETWEEN_REQUESTS);
    }

    #[cfg(feature = "twilight-http")]
    #[tokio::test(start_paused = true)]
    #[ignore]
    async fn large_bot_queue() {
        let token = std::env::var("DISCORD_TOKEN").unwrap();
        let client = std::sync::Arc::new(twilight_http::Client::new(token));

        cancel_dropped_request(super::LargeBotQueue::new(client.clone()).await.unwrap()).await;
        cancel_polled_request(super::LargeBotQueue::new(client).await.unwrap()).await;
    }

    #[tokio::test(start_paused = true)]
    async fn local_queue() {
        cancel_dropped_request(LocalQueue::new()).await;
        cancel_polled_request(LocalQueue::new()).await;
    }
}
