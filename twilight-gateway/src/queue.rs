//! Ratelimiter for gateway `IDENTIFY` commands.
//!
//! Discord limits how often shards can send `IDENTIFY` commands to once every 5
//! seconds per bucket, with a global daily limit.

use std::collections::BTreeMap;
use tokio::{
    sync::{mpsc, oneshot},
    time::{self, Duration, Instant},
};

/// Delay between `IDENTIFY` commands.
const IDENTIFY_DELAY: Duration = Duration::from_secs(5);

/// Possible messages from the [`Queue`] to the [`runner`].
#[derive(Debug)]
enum Message {
    /// Request a permit.
    Request {
        /// For this shard.
        shard: u64,
        /// Indicate readiness through this sender.
        tx: oneshot::Sender<()>,
    },
    /// Update the runner's settings.
    Update(Settings),
}

/// [`runner`]'s settings.
#[derive(Debug)]
struct Settings {
    /// The maximum number of concurrent permits to grant. `0` instantly grants
    /// all permits.
    max_concurrency: u8,
    /// Remaining daily permits.
    remaining: u16,
    /// Time until the daily permits reset.
    reset_at: Instant,
    /// The number of permits to reset to.
    total: u16,
}

/// Readiess permits each [`IDENTIFY_DELAY`].
async fn runner(
    mut rx: mpsc::UnboundedReceiver<Message>,
    mut max_concurrency: u8,
    mut remaining: u16,
    reset_at: Instant,
    mut total: u16,
) {
    let reset_at = time::sleep_until(reset_at);
    let delay = time::sleep(Duration::ZERO);
    tokio::pin! {
        reset_at,
        delay
    };
    let mut requests = BTreeMap::new();

    'outer: loop {
        tokio::select! {
            biased;
            _ = &mut reset_at => {
                remaining = total;
                reset_at.as_mut().reset(Instant::now() + Duration::from_secs(60 * 60 * 24));
            }
            message = rx.recv() => {
                match message {
                    Some(Message::Request{shard, tx}) => {
                        if max_concurrency == 0 {
                            _ = tx.send(());
                        } else {
                            requests.insert(shard, tx);
                        }
                    }
                    Some(Message::Update(update)) => {
                        let deadline;
                        Settings {max_concurrency, remaining, reset_at: deadline, total} = update;
                        reset_at.as_mut().reset(deadline);
                    }
                    None => break,
                }
            }
            _ = &mut delay, if !requests.is_empty() => {
                let mut removed = 0;
                while removed < max_concurrency {
                    if remaining == 0 {
                        (&mut reset_at).await;
                        remaining = total;
                        reset_at.as_mut().reset(Instant::now() + Duration::from_secs(60 * 60 * 24));
                        continue 'outer;
                    }
                    if let Some((_, tx)) = requests.pop_first() {
                        if tx.is_closed() {
                            continue;
                        }
                        _ = tx.send(());
                        remaining -= 1;
                        removed += 1;
                    }
                }
                delay.as_mut().reset(Instant::now() + IDENTIFY_DELAY);
            }
        }
    }
}

/// Queue for shards to request the ability to send `IDENTIFY` commands to
/// initialize new gateway sessions.
///
/// The queue resets the `remaining` amount to `total` after `reset_after` and
/// then every 24 hours. [`Queue::update`] can override these values after the
/// queue's creation.
///
/// Setting `max_concurrency` to `0` instantly allows all requests.
///
/// Cloning a queue is cheap and just increments a reference counter.
#[derive(Clone, Debug)]
pub struct Queue {
    /// Sender to communicate with the background [`runner`].
    tx: mpsc::UnboundedSender<Message>,
}

impl Queue {
    /// Creates a new [`Queue`] with custom settings.
    pub fn new(max_concurrency: u8, remaining: u16, reset_after: Duration, total: u16) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(runner(
            rx,
            max_concurrency,
            remaining,
            Instant::now() + reset_after,
            total,
        ));

        Self { tx }
    }

    /// Update the queue with new info from the [Get Gateway Bot] endpoint.
    ///
    /// May be regularly called as the bot joins/leaves guilds.
    ///
    /// [Get Gateway Bot]: https://discord.com/developers/docs/topics/gateway#get-gateway-bot
    pub fn update(&self, max_concurrency: u8, remaining: u16, reset_after: Duration, total: u16) {
        self.tx
            .send(Message::Update(Settings {
                max_concurrency,
                remaining,
                reset_at: Instant::now() + reset_after,
                total,
            }))
            .expect("receiver dropped after sender");
    }

    /// Reserve a permit from the queue.
    ///
    /// Duplicate requests drop the previous request.
    ///
    /// Closing the channel causes the shard to retry.
    pub(crate) fn request(&self, shard: u64) -> oneshot::Receiver<()> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Message::Request { shard, tx })
            .expect("receiver dropped after sender");

        rx
    }
}

impl Default for Queue {
    /// Creates a new queue with the default settings of:
    ///
    /// * `max_concurrency`: 1
    /// * `remaining`: 1000
    /// * `reset_after`: 24 hours
    /// * `total`: 1000.
    fn default() -> Self {
        Self::new(1, 1000, Duration::from_secs(60 * 60 * 24), 1000)
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;
    use static_assertions::assert_impl_all;
    use std::{
        fmt::Debug,
        future::{poll_fn, Future},
        pin::Pin,
    };

    assert_impl_all!(Queue: Debug, Default, Send, Sync);

    #[tokio::test(start_paused = true)]
    async fn duplicate_requests_cancelled() {
        let queue = Queue::default();

        let mut t1 = queue.request(10);
        // Race.
        _ = poll_fn(|cx| Pin::new(&mut t1).poll(cx)).await;
        let mut t2 = queue.request(10);
        let mut t3 = queue.request(10);

        // Canceled by t3.
        assert!(poll_fn(|cx| Pin::new(&mut t2).poll(cx)).await.is_err());
        // Completes.
        assert!(poll_fn(|cx| Pin::new(&mut t3).poll(cx)).await.is_ok());
    }
}
