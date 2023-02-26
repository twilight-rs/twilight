//! Bucket implementation for a global ratelimit.

use super::bucket::BucketQueue;
use crate::ticket::TicketNotifier;
use crate::RatelimitHeaders;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, Mutex, Semaphore};
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
        self.0.is_locked.try_lock().is_err()
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
    is_locked: Mutex<()>,
}

impl InnerGlobalBucket {
    /// Creates a new bucket and starts a task processing incoming requests.
    fn new(period: u64, requests: u32) -> Arc<Self> {
        let this = Self {
            queue: BucketQueue::default(),
            is_locked: Mutex::default(),
        };
        let this = Arc::new(this);

        tokio::spawn(run_global_queue_task(this.clone(), period, requests));

        this
    }
}

#[tracing::instrument(name = "background global queue task", skip_all)]
async fn run_global_queue_task(bucket: Arc<InnerGlobalBucket>, period: u64, requests: u32) {
    let mut time = Instant::now();
    let semaphore = Arc::new(Semaphore::new(requests as usize));
    let (penalty_tx, mut penalty_rx) = mpsc::channel(requests as usize);

    while let Some(queue_tx) = bucket.queue.pop().await {
        wait_if_needed(
            bucket.as_ref(),
            &mut time,
            period,
            requests,
            &mut penalty_rx,
        )
        .await;

        tokio::spawn(process_request(
            semaphore.clone(),
            queue_tx,
            penalty_tx.clone(),
        ));
    }
}

#[tracing::instrument(name = "process request", skip_all)]
async fn process_request(
    semaphore: Arc<Semaphore>,
    queue_tx: TicketNotifier,
    penalties: Sender<Instant>,
) {
    // This error should never occur, but if it does, do not lock up
    let _permit = semaphore.acquire().await;

    let ticket_headers = if let Some(ticket_headers) = queue_tx.available() {
        ticket_headers
    } else {
        return;
    };

    if let Ok(Some(RatelimitHeaders::Global(headers))) = ticket_headers.await {
        tracing::debug!(seconds = headers.retry_after(), "globally ratelimited");

        let deadline = Instant::now() + Duration::from_secs(headers.retry_after());
        penalties.send(deadline).await.ok();
    }
}

/// Checks and sleeps in case a request needs to wait before proceeding.
async fn wait_if_needed(
    bucket: &InnerGlobalBucket,
    time: &mut Instant,
    period: u64,
    requests: u32,
    penalties: &mut Receiver<Instant>,
) {
    let period = Duration::from_secs(period);
    let fill_rate = period / requests;

    let now = Instant::now();
    // maximum requests at once is 1 period worth of requests
    let base = now - period;
    // if the bucket currently holds more requests than maximum, set to maximum
    if base > *time {
        *time = base;
    }

    // deduct one request from current capacity
    *time += fill_rate;

    // if time > now, then the bucket is exhausted. wait until a request is available again
    if *time > now {
        let _guard = bucket.is_locked.lock().await;
        tokio::time::sleep_until(*time).await;
    }

    // wait for penalties
    while let Ok(deadline) = penalties.try_recv() {
        let _guard = bucket.is_locked.lock().await;
        tokio::time::sleep_until(deadline).await;
    }
}
