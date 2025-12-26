//! Rate limiting state manager.

use crate::{Endpoint, GLOBAL_LIMIT_PERIOD, RateLimitHeaders};
use hashbrown::{HashTable, hash_table::Entry as TableEntry};
use std::{
    collections::{HashMap, VecDeque, hash_map::Entry as MapEntry},
    future::poll_fn,
    hash::{BuildHasher, Hash, Hasher as _, RandomState},
    mem,
    pin::pin,
};
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinSet,
    time::{Duration, Instant, sleep},
};
use tokio_util::time::delay_queue::{DelayQueue, Key};
use tracing::Span;

/// Rate limiter hasher.
#[derive(Debug)]
struct Hasher(RandomState);

impl Hasher {
    /// Hashes a bucket and an endpoint's top-level resources.
    ///
    /// The resulting hash is globally unique.
    fn bucket(&self, bucket: &[u8], endpoint: &Endpoint) -> u64 {
        let mut hasher = self.0.build_hasher();
        endpoint.hash_resources(&mut hasher);
        bucket.hash(&mut hasher);
        hasher.finish()
    }

    /// Hashes an endpoint's top-level resources.
    fn endpoint(&self, endpoint: &Endpoint) -> u64 {
        let mut hasher = self.0.build_hasher();
        endpoint.hash_resources(&mut hasher);
        hasher.finish()
    }
}

/// Pending permit request state.
#[derive(Debug)]
pub struct Message {
    /// Endpoint the permit is for, mapping to a [`Queue`].
    pub endpoint: Endpoint,
    /// Completion handle.
    pub notifier: oneshot::Sender<oneshot::Sender<Option<RateLimitHeaders>>>,
}

/// Grouped pending permits holder.
///
/// Grouping is based on previous permits' response headers: by bucket (if known) or endpoint.
///
/// May not be rate limited, in which case [`limit`], [`reset`], and [`remaining`] are unused.
///
/// [`limit`]: Self::limit
/// [`reset`]: Self::reset
/// [`remaining`]: Self::remaining
#[derive(Debug, Default)]
struct Queue {
    /// Whether the queue has a request in flight.
    in_flight: bool,
    /// List of pending permit requests.
    pending: VecDeque<Message>,
    /// Total number of permits until the queue becomes exhausted.
    limit: u16,
    /// Key mapping to an [`Instant`] when the queue resets, if rate limited.
    reset: Option<Key>,
    /// Number of remaining permits until the queue becomes exhausted.
    remaining: u16,
}

impl Queue {
    /// Whether the queue is exhausted.
    const fn is_exhausted(&self) -> bool {
        self.reset.is_some() && self.remaining == 0
    }

    /// Convert the queue into a bucket.
    fn to_bucket(&self, f: impl FnOnce(Key) -> Instant) -> Option<crate::Bucket> {
        self.reset.map(|key| crate::Bucket {
            limit: self.limit,
            remaining: self.remaining,
            reset_at: f(key).into(),
        })
    }
}

impl From<VecDeque<Message>> for Queue {
    fn from(pending: VecDeque<Message>) -> Self {
        Self {
            in_flight: false,
            pending,
            limit: 0,
            reset: None,
            remaining: 0,
        }
    }
}

/// Interval at which to prune stale queues.
const GC_INTERVAL: Duration = Duration::from_secs(60 * 60 * 6);

/// Rate limiter actor runner.
#[allow(clippy::too_many_lines)]
pub async fn runner(
    global_limit: u16,
    mut rx: mpsc::UnboundedReceiver<(Message, Option<crate::Predicate>)>,
) {
    let mut buckets = HashMap::<Endpoint, Vec<u8>>::new();
    let mut gc_interval = pin!(sleep(GC_INTERVAL));
    let mut global_remaining = global_limit;
    let mut global_timer = pin!(sleep(Duration::ZERO));
    let hasher = Hasher(RandomState::new());
    let mut in_flight = JoinSet::<(Endpoint, Result<Option<RateLimitHeaders>, ()>, Span)>::new();
    let mut queues = HashTable::<(u64, Queue)>::new();
    let mut resets = DelayQueue::<u64>::new();

    /// Updates global rate limit state.
    macro_rules! on_global {
        () => {{
            if global_remaining == global_limit {
                global_timer
                    .as_mut()
                    .reset(Instant::now() + GLOBAL_LIMIT_PERIOD);
            } else if global_remaining == 1 {
                tracing::info!(
                    reset_after = ?global_timer.deadline().saturating_duration_since(Instant::now()),
                    "globally exhausted"
                );
            }
            global_remaining -= 1;
        }};
    }

    /// Tries to pop a pending request off a queue.
    macro_rules! try_pop {
        ($queue:ident) => {{
            while let Some(req) = $queue.pending.front()
                && (global_remaining != 0 || req.endpoint.is_interaction())
                && let Some(req) = $queue.pending.pop_front()
            {
                if req.notifier.is_closed() {
                    continue;
                }

                let (tx, rx) = oneshot::channel();
                if req.notifier.send(tx).is_err() {
                    continue;
                }

                let span = tracing::info_span!("token", endpoint = %req.endpoint);
                span.in_scope(|| tracing::debug!("permitted"));
                if !req.endpoint.is_interaction() {
                    on_global!();
                }
                in_flight.spawn(async move { (req.endpoint, rx.await.map_err(|_| ()), span) });
                $queue.in_flight = true;
                break;
            }
        }};
    }

    loop {
        tokio::select! {
            biased;
            () = &mut gc_interval => {
                buckets.retain(|endpoint, bucket| {
                    let hash = hasher.bucket(bucket, endpoint);
                    let Ok(entry) = queues.find_entry(hash, |&(key, _)| key == hash) else {
                        // Already removed.
                        return false;
                    };
                    let (_, queue) = entry.get();

                    let retain = queue.in_flight || !queue.pending.is_empty() || queue.reset.is_some();
                    if !retain {
                        entry.remove();
                    }

                    retain
                });
                gc_interval.as_mut().reset(Instant::now() + GC_INTERVAL);
            }
            () = &mut global_timer, if global_remaining != global_limit => {
                let globally_exhausted = global_remaining == 0;
                global_remaining = global_limit;
                if globally_exhausted {
                    // Resume stopped queues.
                    queues
                        .iter_mut()
                        .map(|(_, queue)| queue)
                        .filter(|queue| !queue.in_flight && !queue.is_exhausted())
                        .for_each(|queue| try_pop!(queue));
                }
            }
            Some(hash) = poll_fn(|cx| resets.poll_expired(cx)) => {
                let hash = hash.into_inner();
                let (_, queue) = queues.find_mut(hash, |&(key, _)| key == hash).unwrap();

                debug_assert!(!queue.in_flight);
                // Note that non-exhausted queues are not stopped.
                if queue.is_exhausted() {
                    try_pop!(queue);
                }
                queue.reset = None;
            }
            Some(Ok((endpoint, headers, span))) = in_flight.join_next() => {
                if let Ok(Some(headers)) = headers {
                    span.in_scope(|| tracing::trace!(?headers));

                    let hash = hasher.bucket(&headers.bucket, &endpoint);
                    let queue = match buckets.entry(endpoint) {
                        MapEntry::Occupied(entry) if *entry.get() == headers.bucket => {
                            &mut queues.find_mut(hash, |&(key, _)| key == hash).unwrap().1
                        }
                        entry => {
                            let old_hash = match &entry {
                                MapEntry::Occupied(entry) => hasher.bucket(entry.get(), entry.key()),
                                MapEntry::Vacant(entry) => hasher.endpoint(entry.key()),
                            };
                            let entry = entry.insert_entry(headers.bucket);
                            let endpoint = entry.key();

                            // Retrieve this endpoint's requests.
                            let (_, old_queue) = queues.find_mut(old_hash, |&(key, _)| key == old_hash).unwrap();
                            old_queue.in_flight = false;
                            let (pending, old_pending) = mem::take(&mut old_queue.pending)
                                .into_iter()
                                .filter(|req| !req.notifier.is_closed())
                                .partition::<VecDeque<_>, _>(|req| req.endpoint == *endpoint);
                            old_queue.pending = old_pending;
                            try_pop!(old_queue);

                            // And move them into the new queue.
                            match queues.entry(hash, |&(key, _)| key == hash, |&(key, _)| key) {
                                TableEntry::Occupied(entry) => {
                                    let (_, queue) = entry.into_mut();
                                    queue.pending.extend(pending);

                                    // Yield to existing driver.
                                    if queue.in_flight {
                                        continue;
                                    }

                                    queue
                                }
                                TableEntry::Vacant(entry) => &mut entry.insert((hash, Queue::from(pending))).into_mut().1,
                            }
                        }
                    };

                    queue.in_flight = false;
                    queue.limit = headers.limit;
                    queue.remaining = headers.remaining;
                    match &queue.reset {
                        Some(key) => resets.reset_at(key, headers.reset_at.into()),
                        None => queue.reset = Some(resets.insert_at(hash, headers.reset_at.into())),
                    }

                    if queue.is_exhausted() {
                        span.in_scope(|| tracing::info!(
                            reset_after = ?headers.reset_at.saturating_duration_since(Instant::now().into()),
                            "exhausted"
                        ));
                    } else {
                        try_pop!(queue);
                    }
                } else {
                    if headers.is_err() {
                        span.in_scope(|| tracing::debug!("cancelled"));
                        if global_remaining != global_limit {
                            global_remaining += 1;
                        }
                    } else {
                        span.in_scope(|| tracing::debug!(headers = "None"));
                    }

                    let hash = match buckets.get(&endpoint) {
                        Some(bucket) => hasher.bucket(bucket, &endpoint),
                        None => hasher.endpoint(&endpoint),
                    };
                    let (_, queue) = queues.find_mut(hash, |&(key, _)| key == hash).unwrap();
                    queue.in_flight = false;
                    try_pop!(queue);
                }
            }
            Some((msg, predicate)) = rx.recv() => {
                if msg.notifier.is_closed() {
                    continue;
                }

                if !msg.endpoint.is_valid() {
                    tracing::warn!(path = msg.endpoint.path, "improperly formatted path");
                }

                let hash = match buckets.get(&msg.endpoint) {
                    Some(bucket) => hasher.bucket(bucket, &msg.endpoint),
                    None => hasher.endpoint(&msg.endpoint),
                };
                let (_, queue) = queues
                    .entry(hash, |&(key, _)| key == hash, |&(key, _)| key)
                    .or_insert_with(|| (hash, Queue::default()))
                    .into_mut();

                if let Some(predicate) = predicate {
                    let bucket = queue.to_bucket(|key| resets.deadline(&key));
                    if !predicate(bucket) {
                        continue;
                    }
                }

                let globally_exhausted = global_remaining == 0 && !msg.endpoint.is_interaction();
                if globally_exhausted || queue.in_flight || queue.is_exhausted() {
                    queue.pending.push_back(msg);
                } else {
                    let (tx, rx) = oneshot::channel();
                    if msg.notifier.send(tx).is_err() {
                        continue;
                    }

                    let span = tracing::info_span!("token", endpoint = %msg.endpoint);
                    span.in_scope(|| tracing::debug!("permitted"));
                    if !msg.endpoint.is_interaction() {
                        on_global!();
                    }
                    in_flight.spawn(async move { (msg.endpoint, rx.await.map_err(|_| ()), span) });
                    queue.in_flight = true;
                }
            }
            else => break,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unchecked_time_subtraction)]

    use std::time::{Duration, Instant};
    use tokio::time;

    use crate::{Endpoint, Method, RateLimitHeaders, RateLimiter, actor::GC_INTERVAL};

    const RESET_AFTER: Duration = Duration::from_secs(5);
    const ENDPOINT: fn() -> Endpoint = || Endpoint {
        method: Method::Get,
        path: String::from("applications/@me"),
    };
    const ENDPOINT2: fn() -> Endpoint = || Endpoint {
        method: Method::Get,
        path: String::from("channels/1"),
    };

    #[tokio::test(start_paused = true)]
    async fn gc() {
        let rate_limiter = RateLimiter::default();

        rate_limiter
            .acquire(ENDPOINT())
            .await
            .complete(Some(RateLimitHeaders {
                bucket: vec![1, 2, 3],
                limit: 5,
                remaining: 4,
                reset_at: Instant::now() + RESET_AFTER,
            }));

        time::advance(GC_INTERVAL - RESET_AFTER).await;

        rate_limiter
            .acquire(ENDPOINT2())
            .await
            .complete(Some(RateLimitHeaders {
                bucket: vec![2, 3, 4],
                limit: 5,
                remaining: 4,
                reset_at: Instant::now() + RESET_AFTER,
            }));

        time::advance(RESET_AFTER).await;

        rate_limiter.acquire(ENDPOINT()).await.complete(None);
        rate_limiter.acquire(ENDPOINT2()).await.complete(None);
    }
}
