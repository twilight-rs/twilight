//! Ratelimiting functionality for HTTP requests.
//!
//! Discord ratelimits requests to the HTTP API both globally and per-route.
//! For more information on the specifics, please take a look at
//! [Discord's documentation].
//!
//! This crate provides a common [`Ratelimiter`] trait that all ratelimiter
//! implementations need to implement.
//!
//! It also ships a default implementation, [`InMemoryRatelimiter`], that manages
//! the bucket states in memory.
//!
//! [Discord's documentation]: https://discord.com/developers/docs/topics/rate-limits

#![deny(unsafe_code)]

pub mod headers;
pub mod in_memory;
pub mod request;
pub mod ticket;

pub use self::headers::RatelimitHeaders;
pub use self::in_memory::InMemoryRatelimiter;
pub use self::request::{Method, Path};

use self::ticket::TicketReceiver;
use std::{
    error::Error,
    fmt::Debug,
    future::Future,
    pin::Pin,
    time::{Duration, Instant},
};

pub struct Bucket {
    limit: u64,
    remaining: u64,
    reset_after: Duration,
    started_at: Option<Instant>,
}

impl Bucket {
    /// Total number of tickets allotted in a cycle.
    pub fn limit(&self) -> u64 {
        self.limit
    }

    /// Number of tickets remaining.
    pub fn remaining(&self) -> u64 {
        self.remaining
    }

    /// Duration after the [`Self::started_at`] time the bucket will
    /// refresh.
    pub fn reset_after(&self) -> Duration {
        self.reset_after
    }

    /// When the bucket's ratelimit refresh countdown started.
    pub fn started_at(&self) -> Option<Instant> {
        self.started_at
    }

    /// How long until the bucket will refresh.
    ///
    /// May return `None` if the refresh timer has not been started yet or
    /// the bucket has already refreshed.
    pub fn time_remaining(&self) -> Option<Duration> {
        let started_at = self.started_at?;
        let now = Instant::now();
        let reset_at = started_at + self.reset_after;

        if now >= reset_at {
            return None;
        }

        Some(reset_at.duration_since(now))
    }
}

type GenericError = Box<dyn Error + Send + Sync>;
pub type GetBucketFuture =
    Pin<Box<dyn Future<Output = Result<Option<Bucket>, GenericError>> + Send + 'static>>;
pub type IsGloballyLockedFuture =
    Pin<Box<dyn Future<Output = Result<bool, GenericError>> + Send + 'static>>;
pub type HasBucketFuture =
    Pin<Box<dyn Future<Output = Result<bool, GenericError>> + Send + 'static>>;
pub type GetTicketFuture =
    Pin<Box<dyn Future<Output = Result<TicketReceiver, GenericError>> + Send + 'static>>;

pub trait Ratelimiter: Debug + Send + Sync {
    /// Retrieve the basic information of the bucket for a given path.
    fn bucket(&self, path: &Path) -> GetBucketFuture;

    /// Whether the ratelimiter is currently globally locked.
    fn globally_locked(&self) -> IsGloballyLockedFuture;

    /// Determine if the ratelimiter has a bucket for the given path.
    fn has(&self, path: &Path) -> HasBucketFuture;

    /// Retrieve a ticket to know when to send a request.
    /// The provided future will be ready when a ticket in the bucket is
    /// available. Tickets are ready in order of retrieval.
    fn ticket(&self, path: Path) -> GetTicketFuture;
}
