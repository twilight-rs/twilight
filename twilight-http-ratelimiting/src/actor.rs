//! Rate limiting state manager.

use crate::{Bucket, Path, Predicate, RateLimitHeaders, Request, GLOBAL_LIMIT_PERIOD};
use hashbrown::hash_table;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    future::poll_fn,
    hash::{BuildHasher, Hash, Hasher, RandomState},
    mem,
    pin::pin,
};
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinSet,
    time::{sleep, Duration, Instant},
};
use tokio_util::time::delay_queue::{DelayQueue, Key};

/// Grouped pending permits holder.
///
/// Grouping may be done by path or bucket, based on previous permits' response
/// headers.
///
/// Queue may not be rate limited, in which case the values of [`limit`][Self::limit],
/// [`reset`][Self::reset], and [`remaining`][Self::remaining] are unused.
#[derive(Debug)]
struct Queue {
    /// Whether the queue is handling outstanding permits.
    ///
    /// Note that this is `true` when globally exhausted and `false` when
    /// the queue is exhausted.
    idle: bool,
    /// List of pending permit requests.
    inner: VecDeque<Request>,
    /// Total number of permits until the queue becomes exhausted.
    limit: u16,
    /// Key mapping to an [`Instant`] when the queue resets, if rate limited.
    reset: Option<Key>,
    /// Number of remaining permits until the queue becomes exhausted.
    remaining: u16,
}

impl Default for Queue {
    fn default() -> Self {
        Self {
            idle: true,
            inner: VecDeque::new(),
            limit: 0,
            reset: None,
            remaining: 0,
        }
    }
}

impl Queue {
    /// Completes and returns the first queued permit, unless the queue is
    /// globally exhausted.
    fn pop(
        &mut self,
        globally_exhausted: bool,
    ) -> Option<(Path, oneshot::Receiver<Option<RateLimitHeaders>>)> {
        let (mut tx, rx) = oneshot::channel();
        while self
            .inner
            .front()
            .is_some_and(|req| req.path.is_interaction() || !globally_exhausted)
        {
            let req = self.inner.pop_front().unwrap();
            match req.notifier.send(tx) {
                Ok(()) => {
                    self.idle = false;
                    return Some((req.path, rx));
                }
                Err(recover) => tx = recover,
            }
        }
        self.idle = true;

        None
    }
}

/// Interval at which to prune stale queues.
const GC_INTERVAL: Duration = Duration::from_secs(60 * 60 * 6);

/// Rate limiter actor runner.
#[allow(clippy::too_many_lines)]
pub async fn runner(
    global_limit: u16,
    mut rx: mpsc::UnboundedReceiver<(Request, Option<Predicate>)>,
) {
    let mut global_remaining = global_limit;
    let mut global_timer = pin!(sleep(Duration::ZERO));

    let mut buckets = HashMap::<Path, Vec<u8>>::new();
    // Invariants: may never contain more than one task per path at once.
    let mut in_flight = JoinSet::<(
        Path,
        Result<Option<RateLimitHeaders>, oneshot::error::RecvError>,
    )>::new();

    let mut gc_interval = pin!(sleep(GC_INTERVAL));
    let mut reset = DelayQueue::<u64>::new();
    let mut queues = hashbrown::HashTable::<(u64, Queue)>::new();
    let hasher = RandomState::new();

    macro_rules! on_global {
        () => {
            // Global must be decremented before sending the message as, unlike the bucket,
            // it is not blocked until this request receives response headers.
            global_remaining -= 1;
            if global_remaining == global_limit - 1 {
                global_timer
                    .as_mut()
                    .reset(Instant::now() + GLOBAL_LIMIT_PERIOD);
            } else if global_remaining == 0 {
                let now = Instant::now();
                let reset_after = global_timer.deadline().saturating_duration_since(now);
                if reset_after.is_zero() {
                    global_remaining = global_limit - 1;
                    global_timer.as_mut().reset(now + GLOBAL_LIMIT_PERIOD);
                } else {
                    tracing::info!(?reset_after, "globally exhausted");
                }
            }
        };
    }

    macro_rules! on_pop {
        ($queue:ident) => {
            if let Some((path, rx)) = $queue.pop(global_remaining == 0) {
                tracing::debug!(?path, "permitted");
                if !path.is_interaction() {
                    on_global!();
                }
                in_flight.spawn(async move { (path, rx.await) });
            }
        };
    }

    loop {
        tokio::select! {
            biased;
            () = &mut gc_interval => {
                let _span = tracing::debug_span!("garbage collection").entered();
                let keep = buckets
                .iter()
                .map(|(path, bucket)| {
                    let mut builder = hasher.build_hasher();
                    path.hash_components(&mut builder);
                    bucket.hash(&mut builder);
                    builder.finish()
                })
                .filter(|&hash| {
                    let entry = queues.find_entry(hash, |a| a.0 == hash).unwrap();
                    let queue = &entry.get().1;

                    !queue.idle || !queue.inner.is_empty() || queue.reset.is_some()
                })
                .collect::<HashSet<_>>();
                queues.retain(|(hash, _)| {
                    let contained = keep.contains(hash);
                    if !contained {
                        tracing::debug!(hash, "removed");
                    }

                    contained
                });
                gc_interval.as_mut().reset(Instant::now() + GC_INTERVAL);
            }
            () = &mut global_timer, if global_remaining == 0 => {
                global_remaining = global_limit;
                for (_, queue) in queues.iter_mut().filter(|(_, queue)| queue.idle) {
                    on_pop!(queue);
                }
            }
            Some(hash) = poll_fn(|cx| reset.poll_expired(cx)) => {
                let hash = hash.into_inner();
                let (_, queue) = queues.find_mut(hash, |val| val.0 == hash).expect("hash is unchanged");

                queue.reset = None;

                // An active permit may be in flight if the rate limit expired
                // with the queue not exhausted.
                if queue.remaining == 0 {
                    on_pop!(queue);
                }
            }
            Some(response) = in_flight.join_next() => {
                let (path, headers) = response.expect("task should not fail");

                let mut builder = hasher.build_hasher();
                path.hash_components(&mut builder);

                let queue = match headers {
                    Ok(Some(headers)) => {
                        let _span = tracing::info_span!("headers", ?path).entered();
                        tracing::trace!(?headers);
                        let bucket = headers.bucket;

                        bucket.hash(&mut builder);
                        let hash = builder.finish();
                        let queue = match buckets.entry(path) {
                            Entry::Occupied(mut entry) if *entry.get() != bucket => {
                                let mut old_builder = hasher.build_hasher();
                                entry.key().hash_components(&mut old_builder);
                                entry.get().hash(&mut old_builder);
                                let old_hash = old_builder.finish();

                                tracing::debug!(new = hash, previous = old_hash, "bucket changed");

                                *entry.get_mut() = bucket;
                                let path = entry.key();

                                let mut entry = queues.find_entry(old_hash, |a| a.0 == old_hash).expect("hash is unchanged");

                                // Retain the queue as another path may map to it.
                                let mut inner = VecDeque::new();
                                for req in mem::take(&mut entry.get_mut().1.inner) {
                                    if req.path == *path {
                                        inner.push_back(req);
                                    } else {
                                        entry.get_mut().1.inner.push_back(req);
                                    }
                                }

                                let old_queue = &mut entry.get_mut().1;
                                on_pop!(old_queue);

                                let queue = Queue {
                                    idle: false,
                                    inner,
                                    limit: 0,
                                    reset: None,
                                    remaining: 0,
                                };

                                match queues.entry(hash, |a| a.0 == hash, |a| a.0) {
                                    hash_table::Entry::Occupied(entry) => {
                                        let incoming = &mut entry.into_mut().1;
                                        incoming.inner.extend(queue.inner);

                                        // Avoid spawning another task if the incoming queue already has one.
                                        if !incoming.idle {
                                            continue;
                                        }

                                        incoming
                                    }
                                    hash_table::Entry::Vacant(entry) => &mut entry.insert((hash, queue)).into_mut().1,
                                }
                            }
                            Entry::Occupied(_) => &mut queues.find_mut(hash, |a| a.0 == hash).unwrap().1,
                            Entry::Vacant(entry) => {
                                let mut old_builder = hasher.build_hasher();
                                entry.key().hash_components(&mut old_builder);
                                let old_hash = old_builder.finish();

                                tracing::debug!(hash, "bucket assigned");
                                entry.insert(bucket);

                                let ((_, queue), _) = queues.find_entry(old_hash, |a| a.0 == old_hash).expect("hash is unchanged").remove();
                                &mut queues.entry(hash, |a| a.0 == hash, |a| a.0).or_insert((hash, queue)).into_mut().1
                            },
                        };

                        queue.limit = headers.limit;
                        queue.remaining = headers.remaining;
                        if let Some(key) = &queue.reset {
                            reset.reset_at(key, headers.reset_at);
                        } else {
                            queue.reset = Some(reset.insert_at(hash, headers.reset_at));
                        }
                        if queue.remaining == 0 {
                            let reset_after = Instant::now().saturating_duration_since(headers.reset_at);
                            tracing::info!(?reset_after, "exhausted");
                            continue;
                        }

                        queue
                    }
                    Ok(None) => {
                        if let Some(bucket) = buckets.get(&path) {
                            bucket.hash(&mut builder);
                        }
                        let hash = builder.finish();

                        &mut queues.find_mut(hash, |a| a.0 == hash).expect("hash is unchanged").1
                    }
                    Err(_) => {
                        tracing::debug!(?path, "cancelled");
                        if global_remaining != global_limit {
                            global_remaining += 1;
                        }

                        if let Some(bucket) = buckets.get(&path) {
                            bucket.hash(&mut builder);
                        }
                        let hash = builder.finish();

                        &mut queues.find_mut(hash, |a| a.0 == hash).expect("hash is unchanged").1
                    }
                };
                on_pop!(queue);
            }
            Some((msg, predicate)) = rx.recv() => {
                let mut builder = hasher.build_hasher();
                msg.path.hash_components(&mut builder);

                let (_, queue) = if let Some(bucket) = buckets.get(&msg.path) {
                    bucket.hash(&mut builder);
                    let hash = builder.finish();
                    queues.find_mut(hash, |a| a.0 == hash).unwrap()
                } else {
                    let hash = builder.finish();
                    queues.entry(hash, |a| a.0 == hash, |a| a.0).or_insert((hash, Queue::default())).into_mut()
                };

                let bucket = queue.reset.map(|key| Bucket {
                    limit: queue.limit,
                    remaining: queue.remaining,
                    reset_at: reset.deadline(&key),
                });

                if predicate.is_some_and(|p| !p(bucket)) {
                    drop(msg);
                } else if !queue.idle || (!msg.path.is_interaction() && global_remaining == 0) {
                    queue.inner.push_back(msg);
                } else {
                    let (tx, rx) = oneshot::channel();
                    if msg.notifier.send(tx).is_ok() {
                        queue.idle = false;
                        tracing::debug!(path = ?msg.path, "permitted");
                        if !msg.path.is_interaction() {
                            on_global!();
                        }
                        in_flight.spawn(async move { (msg.path, rx.await) });
                    }
                }
            }
            else => break,
        }
    }
}
