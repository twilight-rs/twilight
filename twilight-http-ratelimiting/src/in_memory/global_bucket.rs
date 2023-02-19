use super::bucket::BucketQueue;
use crate::RatelimitHeaders;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;

/// seconds per period
const PERIOD: u64 = 1;
/// requests per period
const REQUESTS: u64 = 50;

/// Global bucket. Keeps track of the global rate limit.
#[derive(Debug, Clone)]
pub struct GlobalBucket(Arc<InnerGlobalBucket>);

impl GlobalBucket {
    /// Queue of global ratelimit requests
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
        Self(InnerGlobalBucket::new())
    }
}

#[derive(Debug)]
struct InnerGlobalBucket {
    pub queue: BucketQueue,
    /// currently waiting for capacity
    is_locked: AtomicBool,
}

impl InnerGlobalBucket {
    fn new() -> Arc<Self> {
        let this = Self {
            queue: Default::default(),
            is_locked: Default::default(),
        };
        let this = Arc::new(this);

        tokio::spawn(run_global_queue_task(this.clone()));

        this
    }
}

#[tracing::instrument(name = "background global queue task", skip_all)]
async fn run_global_queue_task(bucket: Arc<InnerGlobalBucket>) {
    let mut time = Instant::now();

    while let Some(queue_tx) = bucket.queue.pop().await {
        wait_if_needed(bucket.as_ref(), &mut time).await;

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

async fn wait_if_needed(bucket: &InnerGlobalBucket, time: &mut Instant) {
    let period = Duration::from_secs(PERIOD);
    let fill_rate = period / REQUESTS as u32;

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
