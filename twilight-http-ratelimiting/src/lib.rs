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
    time::Duration,
};
use tokio::{
    sync::{mpsc, oneshot},
    time::Instant,
};

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

/// Pending permit state.
#[derive(Debug)]
struct Request {
    /// Completion handle for the associated [`PermitFuture`].
    notifier: oneshot::Sender<oneshot::Sender<Option<RateLimitHeaders>>>,
    /// Path the permit is for, mapping to a [`Queue`].
    path: Path,
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
    tx: mpsc::UnboundedSender<(Request, Option<Predicate>)>,
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
            .send((Request { path, notifier: tx }, None))
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
    /// # #[tokio::main] async fn main() {
    /// # let rate_limiter = twilight_http_ratelimiting::RateLimiter::default();
    /// use twilight_http_ratelimiting::Path;
    ///
    /// if let Some(permit) = rate_limiter
    ///     .acquire_if(Path::ApplicationsMe, |b| b.is_none_or(|b| b.remaining > 10))
    ///     .await
    /// {
    ///     let headers = unimplemented!("send /applications/@me request");
    ///     permit.complete(headers);
    /// }
    /// # }
    /// ```
    #[allow(clippy::missing_panics_doc)]
    pub fn acquire_if<P>(&self, path: Path, predicate: P) -> MaybePermitFuture
    where
        P: FnOnce(Option<Bucket>) -> bool + Send + 'static,
    {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send((Request { path, notifier: tx }, Some(Box::new(predicate))))
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
