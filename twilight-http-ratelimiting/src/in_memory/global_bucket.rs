//! Bucket implementation for a global ratelimit.

use super::bucket::BucketQueue;
use crate::RatelimitHeaders;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;

/// seconds per period
const PERIOD: u64 = 1;
/// requests per period
const REQUESTS: u32 = 50;

/// Global bucket. Keeps track of the global rate limit.
#[derive(Debug, Clone)]
pub struct GlobalBucket(Arc<InnerGlobalBucket>);

impl GlobalBucket {
    /// Creates a new global bucket using custom ratelimit values.
    ///
    /// `period` is given in seconds.
    ///
    /// `requests` indicates the amount of requests per period.
    #[must_use]
    pub fn with_ratelimit(period: u64, requests: u32) -> Self {
        Self(InnerGlobalBucket::new(period, requests))
    }

    /// Queue of global ratelimit requests.
    pub fn queue(&self) -> &BucketQueue {
        &self.0.queue
    }

    /// Whether the global ratelimit is exhausted.
    pub fn is_locked(&self) -> bool {
        self.0.is_locked.load(Ordering::Relaxed)
    }
}

impl Default for GlobalBucket {
    fn default() -> Self {
        Self(InnerGlobalBucket::new(PERIOD, REQUESTS))
    }
}

/// Inner struct to allow [`GlobalBucket`] to return an [`Arc`].
#[derive(Debug)]
struct InnerGlobalBucket {
    /// Queue to receive rate limit requests.
    pub queue: BucketQueue,
    /// currently waiting for capacity.
    is_locked: AtomicBool,
}

impl InnerGlobalBucket {
    /// Creates a new bucket and starts a task processing incoming requests.
    fn new(period: u64, requests: u32) -> Arc<Self> {
        let this = Self {
            queue: BucketQueue::default(),
            is_locked: AtomicBool::default(),
        };
        let this = Arc::new(this);

        tokio::spawn(run_global_queue_task(this.clone(), period, requests));

        this
    }
}

#[tracing::instrument(name = "background global queue task", skip_all)]
async fn run_global_queue_task(bucket: Arc<InnerGlobalBucket>, period: u64, requests: u32) {
    let mut time = Instant::now();

    while let Some(queue_tx) = bucket.queue.pop().await {
        wait_if_needed(bucket.as_ref(), &mut time, period, requests).await;

        let ticket_headers = if let Some(ticket_headers) = queue_tx.available() {
            ticket_headers
        } else {
            continue;
        };

        if let Ok(Some(RatelimitHeaders::Global(headers))) = ticket_headers.await {
            tracing::debug!(seconds = headers.retry_after(), "globally ratelimited");

            bucket.is_locked.store(true, Ordering::Release);
            tokio::time::sleep(Duration::from_secs(headers.retry_after())).await;
            bucket.is_locked.store(false, Ordering::Release);
        }
    }
}

/// Checks and sleeps in case a request needs to wait before proceeding.
async fn wait_if_needed(
    bucket: &InnerGlobalBucket,
    time: &mut Instant,
    period: u64,
    requests: u32,
) {
    let period = Duration::from_secs(period);
    let fill_rate = period / requests;

    let now = Instant::now();
    // base contingent of 1 period worth of requests
    let base = now - period;
    // reset to base if no request came in for long enough
    if base > *time {
        *time = base;
    }

    // we request one request worth of rate limit consumption
    *time += fill_rate;

    // if time > now, wait until there is capacity available again
    if *time > now {
        bucket.is_locked.store(true, Ordering::Release);
        tokio::time::sleep_until(*time).await;
        bucket.is_locked.store(false, Ordering::Release);
    }
}
