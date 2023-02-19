//! In-memory based default [`Ratelimiter`] implementation used in `twilight-http`.

mod bucket;
mod global_bucket;

use self::bucket::{Bucket, BucketQueueTask};
pub use self::global_bucket::GlobalBucket;
use super::{
    ticket::{self, TicketNotifier},
    Bucket as InfoBucket, Ratelimiter,
};
use crate::{
    request::Path, GetBucketFuture, GetTicketFuture, HasBucketFuture, IsGloballyLockedFuture,
};
use futures_util::future;
use std::{
    collections::hash_map::{Entry, HashMap},
    sync::{Arc, Mutex},
    time::Duration,
};

/// Default ratelimiter implementation used in twilight that
/// stores ratelimit information in an in-memory mapping.
///
/// This will meet most users' needs for simple ratelimiting,
/// but for multi-processed bots, consider either implementing
/// your own [`Ratelimiter`] that uses a shared storage backend
/// or use the [HTTP proxy].
///
/// [HTTP proxy]: https://twilight.rs/chapter_2_multi-serviced_approach.html#http-proxy-ratelimiting
#[derive(Clone, Debug, Default)]
pub struct InMemoryRatelimiter {
    /// Mapping of [`Path`]s to their associated [`Bucket`]s.
    buckets: Arc<Mutex<HashMap<Path, Arc<Bucket>>>>,
    /// Global ratelimit data.
    global: GlobalBucket,
}

impl InMemoryRatelimiter {
    /// Create a new in-memory ratelimiter.
    ///
    /// This is used by HTTP client to queue requests in order to avoid
    /// hitting the API's ratelimits.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new in-memory ratelimiter using custom ratelimit values.
    ///
    /// `period` is given in seconds.
    ///
    /// `requests` indicates the amount of requests per period.
    #[must_use]
    pub fn with_global_ratelimit(period: u64, requests: u32) -> Self {
        Self {
            global: GlobalBucket::with_ratelimit(period, requests),
            ..Self::default()
        }
    }

    /// Enqueue the [`TicketNotifier`] to the [`Path`]'s [`Bucket`].
    ///
    /// Returns the new [`Bucket`] if none existed.
    fn entry(&self, path: Path, tx: TicketNotifier) -> Option<Arc<Bucket>> {
        let mut buckets = self.buckets.lock().expect("buckets poisoned");

        match buckets.entry(path.clone()) {
            Entry::Occupied(bucket) => {
                tracing::debug!("got existing bucket: {path:?}");

                bucket.get().queue.push(tx);

                tracing::debug!("added request into bucket queue: {path:?}");

                None
            }
            Entry::Vacant(entry) => {
                tracing::debug!("making new bucket for path: {path:?}");

                let bucket = Bucket::new(path);
                bucket.queue.push(tx);

                let bucket = Arc::new(bucket);
                entry.insert(Arc::clone(&bucket));

                Some(bucket)
            }
        }
    }
}

impl Ratelimiter for InMemoryRatelimiter {
    fn bucket(&self, path: &Path) -> GetBucketFuture {
        self.buckets
            .lock()
            .expect("buckets poisoned")
            .get(path)
            .map_or_else(
                || Box::pin(future::ok(None)),
                |bucket| {
                    let started_at = bucket.started_at.lock().expect("bucket poisoned");
                    let reset_after = Duration::from_millis(bucket.reset_after());

                    Box::pin(future::ok(Some(InfoBucket::new(
                        bucket.limit(),
                        bucket.remaining(),
                        reset_after,
                        *started_at,
                    ))))
                },
            )
    }

    fn is_globally_locked(&self) -> IsGloballyLockedFuture {
        Box::pin(future::ok(self.global.is_locked()))
    }

    fn has(&self, path: &Path) -> HasBucketFuture {
        let has = self
            .buckets
            .lock()
            .expect("buckets poisoned")
            .contains_key(path);

        Box::pin(future::ok(has))
    }

    fn ticket(&self, path: Path) -> GetTicketFuture {
        tracing::debug!("getting bucket for path: {path:?}");

        let (tx, rx) = ticket::channel();

        if let Some(bucket) = self.entry(path.clone(), tx) {
            tokio::spawn(
                BucketQueueTask::new(bucket, Arc::clone(&self.buckets), self.global.clone(), path)
                    .run(),
            );
        }

        Box::pin(future::ok(rx))
    }
}
