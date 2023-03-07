//! Queue for large bots.

use super::{LocalQueue, Queue, WAIT_BETWEEN_REQUESTS};
use std::{
    error::Error,
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    sync::{Mutex, RwLock},
    time::{sleep, sleep_until, Instant},
};
use twilight_http::Client;

/// An implementation of [`Queue`] for large bots.
///
/// Permits an `IDENTIFY` command every 5 seconds per bucket (`max_concurrency`)
/// and accounts for the daily `IDENTIFY` limit.
///
/// Calculates and recalculates the number of buckets on startup and when
/// resetting the daily `IDENTIFY` limit.
///
/// Does not syncronozie across processes, refer to the [module-level]
/// documentation for how to work around this.
///
/// Use [`LocalQueue`] if your `max_concurrency` is 1.
///
/// [module-level]: crate
/// [`LocalQueue`]: crate::LocalQueue
#[derive(Debug)]
pub struct LargeBotQueue {
    /// List of buckets.
    buckets: RwLock<Vec<LocalQueue>>,
    /// HTTP client.
    client: Arc<Client>,
    /// When the daily `IDENTIFY` limit will be reset.
    reset_at: Mutex<Instant>,
    /// Number of `IDENTIFY` commands remaining in the current day.
    remaining: AtomicU16,
}

impl LargeBotQueue {
    /// Create a new large bot queue.
    ///
    /// Requests the [`GetGatewayAuthed`] endpoint on creation and once a day.
    ///
    /// # Errors
    ///
    /// Errors if requesting the [`GetGatewayAuthed`] endpoint fails. Daily
    /// request errors are logged, but ignored.
    ///
    /// [`GetGatewayAuthed`]: twilight_http::request::GetGatewayAuthed
    pub async fn new(http: Arc<Client>) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let bot_info = http.gateway().authed().await?.model().await?;
        let max_concurrency = bot_info.session_start_limit.max_concurrency;
        let buckets = (0..max_concurrency)
            .map(|_| LocalQueue::new())
            .collect::<Vec<_>>();

        let reset_after = Duration::from_millis(bot_info.session_start_limit.reset_after);
        let remaining = AtomicU16::new(bot_info.session_start_limit.remaining);

        Ok(Self {
            buckets: RwLock::new(buckets),
            client: http,
            reset_at: Mutex::new(Instant::now() + reset_after),
            remaining,
        })
    }

    /// Update the daily `IDENTIFY` limit.
    ///
    /// Decrements the remaining count and requests [`GetGatewayAuthed`] to
    /// reset it count and re-creates the buckets if the count reaches zero.
    async fn day_limit(&self) {
        // Decrement the remaining count and check if it's zero.
        let decrement_remaining = || {
            self.remaining
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
                    (x > 0).then_some(x - 1)
                })
        };

        if let Ok(old) = decrement_remaining() {
            tracing::debug!(remaining = old);
            return;
        }
        // Multiple tasks may queue here, first one to get the lock will reset
        // the remaining count.
        let mut reset_at = self.reset_at.lock().await;
        if let Ok(old) = decrement_remaining() {
            tracing::debug!(remaining = old);
            return;
        }
        // Critical section, no new request is granted until the remaining count
        // is reset.
        sleep_until(*reset_at).await;
        if let Ok(new) = Self::new(Arc::clone(&self.client)).await {
            *reset_at = new.reset_at.into_inner();
            let remaining = new.remaining.into_inner();
            self.remaining.store(remaining, Ordering::Relaxed);

            let mut buckets_guard = self.buckets.write().await;
            let new_buckets = new.buckets.into_inner();
            if buckets_guard.len() != new_buckets.len() {
                tracing::info!(
                    new = buckets_guard.len(),
                    old = new_buckets.len(),
                    "session start limit changed, re-creating buckets"
                );

                // Running concurrently offers no speed increase as it's limited
                // by the slowest bucket.
                for bucket in buckets_guard.iter() {
                    bucket.tx.downgrade();
                    bucket.notify_ready.notified().await;
                }

                // Wait an extra cycle in case a bucket just approved a request.
                sleep(WAIT_BETWEEN_REQUESTS).await;

                *buckets_guard = new_buckets;
            }

            tracing::debug!(limit = remaining, "rest daily identify limit");
            return;
        }

        tracing::warn!("unable to get new session limits");
    }
}

impl Queue for LargeBotQueue {
    fn request(&self, shard_id: u32) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            self.day_limit().await;

            let guard = self.buckets.read().await;
            #[allow(clippy::cast_possible_truncation)]
            let bucket = (shard_id % (guard.len() as u32)) as usize;
            guard[bucket].request(shard_id).await;
        })
    }
}
