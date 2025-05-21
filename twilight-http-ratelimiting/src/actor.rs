//! Rate limiting state manager.

use crate::{Path, RateLimitHeaders, GLOBAL_LIMIT_PERIOD};
use hashbrown::{hash_table::Entry as TableEntry, HashTable};
use std::{
    collections::{hash_map::Entry as MapEntry, HashMap, VecDeque},
    future::poll_fn,
    hash::{BuildHasher, Hash, Hasher as _, RandomState},
    mem,
    pin::pin,
};
use tokio::{
    sync::{
        mpsc,
        oneshot::{self, error::RecvError},
    },
    task::JoinSet,
    time::{sleep, Duration, Instant},
};
use tokio_util::time::delay_queue::{DelayQueue, Key};

/// Rate limiter hasher.
#[derive(Debug)]
struct Hasher(RandomState);

impl Hasher {
    /// Hashes a bucket and a path.
    fn bucket(&self, bucket: &[u8], path: &Path) -> u64 {
        let mut hasher = self.0.build_hasher();
        path.hash_components(&mut hasher);
        bucket.hash(&mut hasher);
        hasher.finish()
    }

    /// Hashes a path.
    fn path(&self, path: &Path) -> u64 {
        let mut hasher = self.0.build_hasher();
        path.hash_components(&mut hasher);
        hasher.finish()
    }
}

/// Pending permit request state.
#[derive(Debug)]
pub struct Message {
    /// Completion handle.
    pub notifier: oneshot::Sender<oneshot::Sender<Option<RateLimitHeaders>>>,
    /// Path the permit is for, mapping to a [`Queue`].
    pub path: Path,
}

/// Grouped pending permits holder.
///
/// Grouping may be done by path or bucket, based on previous permits' response
/// headers.
///
/// Queue may not be rate limited, in which case the values of [`limit`][Self::limit],
/// [`reset`][Self::reset], and [`remaining`][Self::remaining] are unused.
#[derive(Debug)]
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
    const fn is_exhasted(&self) -> bool {
        self.remaining == 0
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self {
            in_flight: false,
            pending: VecDeque::new(),
            limit: 0,
            reset: None,
            remaining: 0,
        }
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
    let mut global_remaining = global_limit;
    let mut global_timer = pin!(sleep(Duration::ZERO));

    let mut buckets = HashMap::<Path, Vec<u8>>::new();
    // Invariants: may never contain more than one task per path at once.
    let mut in_flight = JoinSet::<(Path, Result<Option<RateLimitHeaders>, RecvError>)>::new();

    let mut gc_interval = pin!(sleep(GC_INTERVAL));
    let mut reset = DelayQueue::<u64>::new();
    let mut queues = HashTable::<(u64, Queue)>::new();
    let hasher = Hasher(RandomState::new());

    /// Updates global rate limit state.
    macro_rules! on_global {
        () => {
            debug_assert_ne!(global_remaining, 0);
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
        };
    }

    /// Tries to pop a pending request off a queue.
    macro_rules! try_pop {
        ($queue:ident) => {
            let (mut tx, rx) = oneshot::channel();
            while let Some(req) = $queue
                .pending
                .front()
                .is_some_and(|req| global_remaining != 0 || req.path.is_interaction())
                .then(|| $queue.pending.pop_front())
                .flatten()
            {
                match req.notifier.send(tx) {
                    Ok(()) => {
                        tracing::debug!(path = ?req.path, "permitted");
                        if !req.path.is_interaction() {
                            on_global!();
                        }
                        in_flight.spawn(async move { (req.path, rx.await) });
                        $queue.in_flight = true;
                        break;
                    }
                    Err(recover) => tx = recover,
                }
            }
        };
    }

    loop {
        tokio::select! {
            biased;
            () = &mut gc_interval => {
                let _span = tracing::debug_span!("gc").entered();
                buckets.retain(|path, bucket| {
                    let hash = hasher.bucket(bucket, path);
                    let entry = queues.find_entry(hash, |&(key, _)| key == hash).unwrap();
                    let (_, queue) = entry.get();

                    let retain = queue.in_flight || !queue.pending.is_empty() || queue.reset.is_some();
                    if !retain {
                        entry.remove();
                        tracing::debug!(hash, "removed");
                    }

                    retain
                });
                gc_interval.as_mut().reset(Instant::now() + GC_INTERVAL);
            }
            () = &mut global_timer, if global_remaining != global_limit => {
                global_remaining = global_limit;
                // Try resume all stopped queues.
                for (_, queue) in queues.iter_mut().filter(|(_, queue)| {
                    !queue.in_flight && (!queue.is_exhasted() || queue.reset.is_none())
                }) {
                    try_pop!(queue);
                }
            }
            Some(hash) = poll_fn(|cx| reset.poll_expired(cx)) => {
                let hash = hash.into_inner();
                let (_, queue) = queues.find_mut(hash, |&(key, _)| key == hash).unwrap();

                debug_assert!(!queue.in_flight);
                queue.reset = None;
                // Note that non-exhausted queues are not stopped.
                if queue.is_exhasted() {
                    try_pop!(queue);
                }
            }
            Some(Ok((path, headers))) = in_flight.join_next() => {
                let _span = tracing::info_span!("resp", ?path).entered();
                if let Ok(Some(headers)) = headers {
                    tracing::trace!(?headers);

                    let hash = hasher.bucket(&headers.bucket, &path);
                    let queue = match buckets.entry(path.clone()) {
                        MapEntry::Occupied(occupied) if *occupied.get() == headers.bucket => {
                            &mut queues.find_mut(hash, |&(key, _)| key == hash).unwrap().1
                        }
                        old_entry => {
                            let old_hash = match &old_entry {
                                MapEntry::Occupied(occupied) => hasher.bucket(occupied.get(), occupied.key()),
                                MapEntry::Vacant(vacant) => hasher.path(vacant.key()),
                            };
                            tracing::debug!(new = hash, previous = old_hash, "updated bucket");

                            // Retrieve this path's requests.
                            let (_, old_queue) = queues.find_mut(old_hash, |&(key, _)| key == old_hash).unwrap();
                            old_queue.in_flight = false;
                            let (pending, old_pending) = mem::take(&mut old_queue.pending)
                                .into_iter()
                                .partition::<VecDeque<_>, _>(|req| req.path == *old_entry.key());
                            old_queue.pending = old_pending;
                            try_pop!(old_queue);

                            old_entry.insert_entry(headers.bucket);
                            // And move them into the new queue.
                            match queues.entry(hash, |&(key, _)| key == hash, |&(key, _)| key) {
                                TableEntry::Occupied(occupied) => {
                                    let (_, incoming_queue) = occupied.into_mut();
                                    incoming_queue.pending.extend(pending);

                                    if incoming_queue.in_flight {
                                        continue;
                                    }

                                    incoming_queue
                                }
                                TableEntry::Vacant(vacant) => &mut vacant.insert((hash, Queue::from(pending))).into_mut().1,
                            }
                        }
                    };

                    queue.in_flight = false;
                    queue.limit = headers.limit;
                    queue.remaining = headers.remaining;
                    if let Some(key) = &queue.reset {
                        reset.reset_at(key, headers.reset_at);
                    } else {
                        queue.reset = Some(reset.insert_at(hash, headers.reset_at));
                    }
                    if queue.is_exhasted() {
                        tracing::info!(
                            reset_after = ?headers.reset_at.saturating_duration_since(Instant::now()),
                            "exhausted"
                        );
                        continue;
                    }

                    try_pop!(queue);
                } else {
                    if headers.is_err() {
                        tracing::debug!("cancelled");
                        if global_remaining != global_limit {
                            global_remaining += 1;
                        }
                    } else {
                        tracing::debug!(headers = "None");
                    }

                    let hash = buckets.get(&path).map_or_else(|| hasher.path(&path), |bucket| hasher.bucket(bucket, &path));
                    let (_, queue) = queues.find_mut(hash, |&(key, _)| key == hash).unwrap();
                    queue.in_flight = false;
                    try_pop!(queue);
                }
            }
            Some((msg, pred)) = rx.recv() => {
                // Group bucketless requests until they are assigned a bucket.
                let (_, queue) = if let Some(bucket) = buckets.get(&msg.path) {
                    let hash = hasher.bucket(bucket, &msg.path);
                    queues.find_mut(hash, |&(key, _)| key == hash).unwrap()
                } else {
                    let hash = hasher.path(&msg.path);
                    match queues.entry(hash, |&(key, _)| key == hash, |&(key, _)| key) {
                        TableEntry::Occupied(occupied) => occupied.into_mut(),
                        TableEntry::Vacant(vacant) => {
                            tracing::debug!(path = ?msg.path, "new queue");
                            vacant.insert((hash, Queue::default())).into_mut()
                        }
                    }
                };

                let is_cancelled = pred.is_some_and(|p| !p(queue.reset.map(|key| crate::Bucket {
                    limit: queue.limit,
                    remaining: queue.remaining,
                    reset_at: reset.deadline(&key),
                })));

                let queue_active = queue.in_flight || (queue.is_exhasted() && queue.reset.is_some());
                if is_cancelled {
                    drop(msg);
                } else if queue_active || (global_remaining == 0 && !msg.path.is_interaction()) {
                    queue.pending.push_back(msg);
                } else if !msg.notifier.is_closed() {
                    let (tx, rx) = oneshot::channel();
                    if msg.notifier.send(tx).is_ok() {
                        tracing::debug!(path = ?msg.path, "permitted");
                        if !msg.path.is_interaction() {
                            on_global!();
                        }
                        in_flight.spawn(async move { (msg.path, rx.await) });
                        queue.in_flight = true;
                    }
                }
            }
            else => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::{advance, Duration, Instant};

    use crate::{actor::GC_INTERVAL, Path, RateLimitHeaders, RateLimiter};

    const RESET_AFTER: Duration = Duration::from_secs(5);
    const PATH: Path = Path::ApplicationsMe;
    const PATH2: Path = Path::ChannelsId(1);

    #[tokio::test(start_paused = true)]
    async fn gc() {
        let rate_limiter = RateLimiter::default();

        rate_limiter
            .acquire(PATH)
            .await
            .complete(Some(RateLimitHeaders {
                bucket: vec![1, 2, 3],
                limit: 5,
                remaining: 4,
                reset_at: Instant::now() + RESET_AFTER,
            }));

        advance(GC_INTERVAL - RESET_AFTER).await;

        rate_limiter
            .acquire(PATH2)
            .await
            .complete(Some(RateLimitHeaders {
                bucket: vec![2, 3, 4],
                limit: 5,
                remaining: 4,
                reset_at: Instant::now() + RESET_AFTER,
            }));

        advance(RESET_AFTER).await;

        rate_limiter.acquire(PATH).await.complete(None);
        rate_limiter.acquire(PATH2).await.complete(None);
    }
}
