#![doc = include_str!("../README.md")]
#![warn(
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::pedantic,
    missing_docs,
    unsafe_code
)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

mod actor;
mod request;

pub use crate::request::{Method, Path, PathParseError, PathParseErrorType};

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use tokio::sync::{mpsc, oneshot};

/// Duration from the first globally limited request until the remaining count
/// resets to the global limit count.
pub const GLOBAL_LIMIT_PERIOD: Duration = Duration::from_secs(1);

/// Parsed user response rate limit headers.
///
/// A `limit` of zero marks the [`Bucket`] as exhausted until `reset_at` elapses.
///
/// # Global limits
///
/// Please open an issue if the [`RateLimiter`] exceeded the global limit.
///
/// # Shared limits
///
/// You may preemptively exhaust the bucket until `Reset-After` by completing
/// the [`Permit`] with [`RateLimitHeaders::shared`], but are not required to
/// since these limits do not count towards the invalid request limit.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RateLimitHeaders {
    /// Bucket identifier.
    pub bucket: Vec<u8>,
    /// Total number of requests until the bucket becomes exhausted.
    pub limit: u16,
    /// Number of remaining requests until the bucket becomes exhausted.
    pub remaining: u16,
    /// Time at which the bucket resets.
    pub reset_at: Instant,
}

impl RateLimitHeaders {
    /// Lowercased name for the bucket header.
    pub const BUCKET: &'static str = "x-ratelimit-bucket";

    /// Lowercased name for the limit header.
    pub const LIMIT: &'static str = "x-ratelimit-limit";

    /// Lowercased name for the remaining header.
    pub const REMAINING: &'static str = "x-ratelimit-remaining";

    /// Lowercased name for the reset-after header.
    pub const RESET_AFTER: &'static str = "x-ratelimit-reset-after";

    /// Lowercased name for the scope header.
    pub const SCOPE: &'static str = "x-ratelimit-scope";

    /// Emulates a shared resource limit as a user limit by setting `limit` and
    /// `remaining` to zero.
    pub fn shared(bucket: Vec<u8>, retry_after: u16) -> Self {
        Self {
            bucket,
            limit: 0,
            remaining: 0,
            reset_at: Instant::now() + Duration::from_secs(retry_after.into()),
        }
    }
}

/// Permit to send a Discord HTTP API request to the acquired path.
#[derive(Debug)]
#[must_use = "dropping the permit immediately cancels itself"]
pub struct Permit(oneshot::Sender<Option<RateLimitHeaders>>);

impl Permit {
    /// Update the [`RateLimiter`] based on the response headers.
    ///
    /// Non-completed permits are regarded as cancelled, so only call this
    /// on receiving a response.
    #[allow(clippy::missing_panics_doc)]
    pub fn complete(self, headers: Option<RateLimitHeaders>) {
        self.0.send(headers).expect("actor is alive");
    }
}

/// Future that completes when a permit is ready.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct PermitFuture(oneshot::Receiver<oneshot::Sender<Option<RateLimitHeaders>>>);

impl Future for PermitFuture {
    type Output = Permit;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0)
            .poll(cx)
            .map(|r| Permit(r.expect("actor is alive")))
    }
}

/// Future that completes when a permit is ready or cancelled.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct MaybePermitFuture(oneshot::Receiver<oneshot::Sender<Option<RateLimitHeaders>>>);

impl Future for MaybePermitFuture {
    type Output = Option<Permit>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx).map(|r| r.ok().map(Permit))
    }
}

/// Rate limit information for one or more paths from previous
/// [`RateLimitHeaders`].
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Bucket {
    /// Total number of permits until the bucket becomes exhausted.
    pub limit: u16,
    /// Number of remaining permits until the bucket becomes exhausted.
    pub remaining: u16,
    /// Time at which the bucket resets.
    pub reset_at: Instant,
}

/// Actor run closure pre-enqueue for early [`MaybePermitFuture`] cancellation.
type Predicate = Box<dyn FnOnce(Option<Bucket>) -> bool + Send>;

/// Discord HTTP client API rate limiter.
///
/// The [`RateLimiter`] runs an associated actor task to concurrently handle permit
/// requests and responses.
///
/// Cloning a [`RateLimiter`] increments just the amount of senders for the actor.
/// The actor completes when there are no senders and non-completed permits left.
#[derive(Clone, Debug)]
pub struct RateLimiter {
    /// Actor message sender.
    tx: mpsc::UnboundedSender<(actor::Message, Option<Predicate>)>,
}

impl RateLimiter {
    /// Create a new [`RateLimiter`] with a custom global limit.
    pub fn new(global_limit: u16) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(actor::runner(global_limit, rx));

        Self { tx }
    }

    /// Await a single permit for this path.
    ///
    /// Permits are queued per path in the order they were requested.
    #[allow(clippy::missing_panics_doc)]
    pub fn acquire(&self, path: Path) -> PermitFuture {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send((actor::Message { path, notifier: tx }, None))
            .expect("actor is alive");

        PermitFuture(rx)
    }

    /// Await a single permit for this path, but only if the predicate evaluates
    /// to `true`.
    ///
    /// Permits are queued per path in the order they were requested.
    ///
    /// Note that the predicate is asynchronously called in the actor task.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # let rt = tokio::runtime::Builder::new_current_thread()
    /// #     .enable_time()
    /// #     .build()
    /// #     .unwrap();
    /// # rt.block_on(async {
    /// # let rate_limiter = twilight_http_ratelimiting::RateLimiter::default();
    /// use twilight_http_ratelimiting::Path;
    ///
    /// if let Some(permit) = rate_limiter
    ///     .acquire_if(Path::ApplicationsMe, |b| b.is_none_or(|b| b.remaining > 10))
    ///     .await
    /// {
    ///     let headers = unimplemented!("GET /applications/@me");
    ///     permit.complete(headers);
    /// }
    /// # });
    /// ```
    #[allow(clippy::missing_panics_doc)]
    pub fn acquire_if<P>(&self, path: Path, predicate: P) -> MaybePermitFuture
    where
        P: FnOnce(Option<Bucket>) -> bool + Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send((
                actor::Message { path, notifier: tx },
                Some(Box::new(predicate)),
            ))
            .expect("actor is alive");

        MaybePermitFuture(rx)
    }

    /// Retrieve the [`Bucket`] for this path.
    ///
    /// The bucket is internally retrieved via [`acquire_if`][Self::acquire_if].
    #[allow(clippy::missing_panics_doc)]
    pub async fn bucket(&self, path: Path) -> Option<Bucket> {
        let (tx, rx) = oneshot::channel();
        self.acquire_if(path, |bucket| {
            _ = tx.send(bucket);
            false
        })
        .await;

        rx.await.expect("actor is alive")
    }
}

impl Default for RateLimiter {
    /// Create a new [`RateLimiter`] with Discord's default global limit.
    ///
    /// Currently this is `50`.
    fn default() -> Self {
        Self::new(50)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Bucket, MaybePermitFuture, Path, Permit, PermitFuture, RateLimitHeaders, RateLimiter,
    };
    use static_assertions::assert_impl_all;
    use std::{
        fmt::Debug,
        future::Future,
        hash::Hash,
        time::{Duration, Instant},
    };
    use tokio::task;

    assert_impl_all!(Bucket: Clone, Copy, Debug, Eq, Hash, PartialEq, Send, Sync);
    assert_impl_all!(MaybePermitFuture: Debug, Future<Output = Option<Permit>>);
    assert_impl_all!(Permit: Debug, Send, Sync);
    assert_impl_all!(PermitFuture: Debug, Future<Output = Permit>);
    assert_impl_all!(RateLimitHeaders: Clone, Debug, Eq, Hash, PartialEq, Send, Sync);
    assert_impl_all!(RateLimiter: Clone, Debug, Default, Send, Sync);

    const PATH: Path = Path::ApplicationsMe;

    #[tokio::test]
    async fn acquire_if() {
        let rate_limiter = RateLimiter::default();

        assert!(rate_limiter.acquire_if(PATH, |_| false).await.is_none());
        assert!(rate_limiter.acquire_if(PATH, |_| true).await.is_some());
    }

    #[tokio::test]
    async fn bucket() {
        let rate_limiter = RateLimiter::default();

        let limit = 2;
        let remaining = 1;
        let reset_at = Instant::now() + Duration::from_secs(1);
        let headers = RateLimitHeaders {
            bucket: vec![1, 2, 3],
            limit,
            remaining,
            reset_at,
        };

        rate_limiter.acquire(PATH).await.complete(Some(headers));
        task::yield_now().await;

        let bucket = rate_limiter.bucket(PATH).await.unwrap();
        assert_eq!(bucket.limit, limit);
        assert_eq!(bucket.remaining, remaining);
        assert!(
            bucket.reset_at.saturating_duration_since(reset_at) < Duration::from_millis(1)
                && reset_at.saturating_duration_since(bucket.reset_at) < Duration::from_millis(1)
        );
    }
}
