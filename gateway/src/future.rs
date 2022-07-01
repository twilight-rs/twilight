//! Various utility futures used by the [`Shard`].
//!
//! These tend to be used to get around lifetime and borrow requirements, but
//! are also sometimes used to simplify logic.
//!
//! [`Shard`]: super::Shard

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use tokio::time::{self, Sleep};

#[must_use]
pub struct TickHeartbeatFuture {
    inner: Option<Pin<Box<Sleep>>>,
}

impl TickHeartbeatFuture {
    pub fn new(
        maybe_last_sent: Option<Instant>,
        maybe_heartbeat_interval: Option<Duration>,
    ) -> Self {
        let heartbeat_interval = if let Some(heartbeat_interval) = maybe_heartbeat_interval {
            heartbeat_interval
        } else {
            return Self { inner: None };
        };

        let remaining = if let Some(last_sent) = maybe_last_sent {
            let time_since = last_sent.elapsed();

            heartbeat_interval.saturating_sub(time_since)
        } else {
            Duration::ZERO
        };

        Self {
            inner: Some(Box::pin(time::sleep(remaining))),
        }
    }
}

impl Future for TickHeartbeatFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(inner) = self.inner.as_mut() {
            return inner.as_mut().poll(cx);
        }

        Poll::Pending
    }
}
