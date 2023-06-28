//! Memory based [`Queue`] implementation and supporting items.

use super::{Queue, IDENTIFY_DELAY, LIMIT_PERIOD};
use std::{collections::VecDeque, fmt::Debug, iter};
use tokio::{
    sync::{mpsc, oneshot},
    task::yield_now,
    time::{sleep_until, Duration, Instant},
};

/// Possible messages from the [`InMemoryQueue`] to the [`runner`].
#[derive(Debug)]
enum Message {
    /// Request a permit.
    Request {
        /// For this shard.
        shard: u32,
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
    reset_after: Duration,
    /// The number of permits to reset to.
    total: u16,
}

/// [`InMemoryQueue`]'s background task runner.
///
/// Buckets requests such that only one timer is necessary.
async fn runner(
    mut rx: mpsc::UnboundedReceiver<Message>,
    Settings {
        max_concurrency,
        mut remaining,
        reset_after,
        mut total,
    }: Settings,
) {
    let (interval, reset_at) = {
        let now = Instant::now();
        (sleep_until(now), sleep_until(now + reset_after))
    };
    tokio::pin!(interval, reset_at);

    let mut queues = iter::repeat_with(VecDeque::new)
        .take(max_concurrency.into())
        .collect::<Box<_>>();

    loop {
        tokio::select! {
            biased;
            _ = &mut reset_at, if remaining != total => {
                remaining = total;
            }
            message = rx.recv() => {
                match message {
                    Some(Message::Request { shard, tx }) => {
                        if queues.is_empty() {
                            _ = tx.send(());
                        } else {
                            queues[shard as usize % queues.len()]
                                .push_back((shard, tx));
                        }
                    }
                    Some(Message::Update(update)) => {
                        let (max_concurrency, reset_after);
                        Settings {
                            max_concurrency,
                            remaining,
                            reset_after,
                            total,
                        } = update;

                        if remaining != total {
                            reset_at.as_mut().reset(Instant::now() + reset_after);
                        }

                        if max_concurrency as usize != queues.len() {
                            let unbalanced = queues.into_vec().into_iter().flatten();
                            queues = iter::repeat_with(VecDeque::new)
                                .take(max_concurrency.into())
                                .collect();
                            for (shard, tx) in unbalanced {
                                queues[(shard % u32::from(max_concurrency)) as usize]
                                    .push_back((shard, tx));
                            }
                        }
                    }
                    None => break,
                }
            }
            _ = &mut interval, if queues.iter().any(|queue| !queue.is_empty()) => {
                let span = tracing::info_span!("bucket", capacity = %queues.len());
                let now = Instant::now();
                interval.as_mut().reset(now + IDENTIFY_DELAY);

                if remaining == total {
                    reset_at.as_mut().reset(now + LIMIT_PERIOD);
                }

                for (ratelimit_key, queue) in queues.iter_mut().enumerate() {
                    if remaining == 0 {
                        let duration = reset_at.deadline().saturating_duration_since(now);
                        tracing::debug!(?duration, "sleeping until remaining count refills");
                        (&mut reset_at).await;
                        remaining = total;

                        break;
                    }

                    while let Some((id, tx)) = queue.pop_front() {
                        let calculated_ratelimit_key = (id % u32::from(max_concurrency)) as usize;
                        debug_assert_eq!(ratelimit_key, calculated_ratelimit_key);

                        if tx.send(()).is_err() {
                            continue;
                        }
                        tracing::debug!(parent: &span, ratelimit_key, "allowing shard {id}");
                        remaining -= 1;

                        // Reschedule behind shard for ordering correctness.
                        yield_now().await;
                        break;
                    }
                }
            }
        }
    }
}

/// Memory based [`Queue`] implementation backed by an efficient background task.
///
/// [`InMemoryQueue::update`] allows for dynamically changing the queue's
/// settings.
///
/// Cloning the queue is cheap and just increments a reference counter.
///
/// **Note:** A `max_concurrency` of `0` processes all requests instantly,
/// effectively disabling the queue.
#[derive(Clone, Debug)]
pub struct InMemoryQueue {
    /// Sender to communicate with the background [task runner].
    ///
    /// [task runner]: runner
    tx: mpsc::UnboundedSender<Message>,
}

impl InMemoryQueue {
    /// Creates a new `InMemoryQueue` with custom settings.
    ///
    /// # Panics
    ///
    /// Panics if `total` < `remaining`.
    pub fn new(max_concurrency: u8, remaining: u16, reset_after: Duration, total: u16) -> Self {
        assert!(total >= remaining);
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(runner(
            rx,
            Settings {
                max_concurrency,
                remaining,
                reset_after,
                total,
            },
        ));

        Self { tx }
    }

    /// Update the queue with new info from the [Get Gateway Bot] endpoint.
    ///
    /// May be regularly called as the bot joins/leaves guilds.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use twilight_gateway_queue::InMemoryQueue;
    /// # let rt = tokio::runtime::Builder::new_current_thread()
    /// #     .enable_time()
    /// #     .build()
    /// #     .unwrap();
    /// use std::time::Duration;
    /// use twilight_http::Client;
    ///
    /// # rt.block_on(async {
    /// # let queue = InMemoryQueue::default();
    /// # let token = String::new();
    /// let client = Client::new(token);
    /// let session = client
    ///     .gateway()
    ///     .authed()
    ///     .await?
    ///     .model()
    ///     .await?
    ///     .session_start_limit;
    /// queue.update(
    ///     session.max_concurrency,
    ///     session.remaining,
    ///     Duration::from_millis(session.reset_after),
    ///     session.total,
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `total` < `remaining`.
    ///
    /// [Get Gateway Bot]: https://discord.com/developers/docs/topics/gateway#get-gateway-bot
    pub fn update(&self, max_concurrency: u8, remaining: u16, reset_after: Duration, total: u16) {
        assert!(total >= remaining);

        self.tx
            .send(Message::Update(Settings {
                max_concurrency,
                remaining,
                reset_after,
                total,
            }))
            .expect("receiver dropped after sender");
    }
}

impl Default for InMemoryQueue {
    /// Creates a new `InMemoryQueue` with Discord's default settings.
    ///
    /// Currently these are:
    ///
    /// * `max_concurrency`: 1
    /// * `remaining`: 1000
    /// * `reset_after`: [`LIMIT_PERIOD`]
    /// * `total`: 1000.
    fn default() -> Self {
        Self::new(1, 1000, LIMIT_PERIOD, 1000)
    }
}

impl Queue for InMemoryQueue {
    fn enqueue(&self, shard: u32) -> oneshot::Receiver<()> {
        let (tx, rx) = oneshot::channel();

        self.tx
            .send(Message::Request { shard, tx })
            .expect("receiver dropped after sender");

        rx
    }
}

#[cfg(test)]
mod tests {
    use super::InMemoryQueue;
    use crate::Queue;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(InMemoryQueue: Clone, Debug, Default, Send, Sync, Queue);
}
