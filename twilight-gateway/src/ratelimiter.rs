//! Rate limit events sent to the Gateway.
//!
//! See <https://discord.com/developers/docs/topics/gateway#rate-limiting>
//!
//! # Algorithm
//!
//! [`CommandRatelimiter`] is implemented as a sliding window log. This is the
//! only ratelimit algorithm that supports burst requests and guarantees that
//! the (t - [`PERIOD`], t] window is never exceeded. See
//! <https://hechao.li/2018/06/25/Rate-Limiter-Part1> for an overview of it and
//! other alternative algorithms.

use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};
use tokio::time::{Duration, Instant, Sleep};

/// Number of commands allowed in a [`PERIOD`].
const COMMANDS_PER_PERIOD: u8 = 120;

/// Duration until an acquired permit is released.
const PERIOD: Duration = Duration::from_secs(60);

/// Ratelimiter for sending commands over the gateway to Discord.
#[derive(Debug)]
pub struct CommandRatelimiter {
    /// Future that completes when a permit is released.
    delay: Pin<Box<Sleep>>,
    /// Milliseconds after delay elapses.
    /// Ordered queue of instants when permits release.
    pending: Vec<u16>,
}

impl CommandRatelimiter {
    /// Create a new ratelimiter with some capacity reserved for heartbeating.
    pub(crate) fn new(heartbeat_interval: Duration) -> Self {
        let allotted = nonreserved_commands_per_reset(heartbeat_interval);

        Self {
            // Delay must be < now for algorithm correctness (relevant for tests).
            delay: Box::pin(tokio::time::sleep_until(
                Instant::now() - Duration::from_secs(1),
            )),
            pending: Vec::with_capacity(usize::from(allotted) - 1),
        }
    }

    /// Number of available permits.
    #[allow(clippy::cast_possible_truncation)]
    pub fn available(&self) -> u8 {
        let now = Instant::now();
        let then = self.delay.deadline();
        if now >= then {
            let released = self
                .pending
                .iter()
                .map(|&milli| then + Duration::from_millis(milli.into()))
                .position(|deadline| deadline > now)
                .unwrap_or_else(|| self.pending.len());
            let acquired = self.pending.len() - released;

            self.max() - acquired as u8
        } else {
            (self.pending.capacity() - self.pending.len()) as u8
        }
    }

    /// Maximum number of available permits.
    #[allow(clippy::cast_possible_truncation)]
    pub fn max(&self) -> u8 {
        self.pending.capacity() as u8 + 1
    }

    /// Duration until the next permit is available.
    pub fn next_available(&self) -> Duration {
        self.delay
            .deadline()
            .saturating_duration_since(Instant::now())
    }

    /// Attempts to acquire a permit.
    ///
    /// # Return value
    ///
    /// The function returns:
    ///
    /// * `Poll::Pending` if no permit is available
    /// * `Poll::Ready` if a permit was acquired.
    pub(crate) fn poll_acquire(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        ready!(self.poll_available(cx));

        let now = Instant::now();
        let then = self.delay.deadline();
        if now >= then {
            // Use linear search due to:
            // 1. Easier implementation, especially since binary_search's returned index is unstable for equal elements
            // 2. Searched for element is up front if called frequently, so "optimize" for that.
            if let Some(released) = self
                .pending
                .iter()
                .map(|&milli| then + Duration::from_millis(milli.into()))
                .position(|deadline| deadline > now)
            {
                let new_delay = self.pending[released];
                let acquired = self.pending.len() - released;
                let new_delay = then + Duration::from_millis(new_delay.into());

                // drop new_delay too.
                self.pending.rotate_left(released + 1);
                self.pending.truncate(acquired - 1);
                self.delay.as_mut().reset(new_delay);
            } else {
                // All pending values are less than `now`.
                self.pending.clear();

                self.delay.as_mut().reset(now + PERIOD);

                return Poll::Ready(());
            }
        }

        let pending = PERIOD - self.delay.deadline().saturating_duration_since(now);

        // pending.as_millis() <= 60_000 < u16::MAX
        self.pending.push(pending.as_millis() as u16);

        if self.pending.len() == self.pending.capacity() {
            tracing::debug!(duration = ?(self.delay.deadline() - now), "ratelimited");
        }

        Poll::Ready(())
    }

    /// Checks whether a permit is available.
    ///
    /// # Returns
    ///
    /// * `Poll::Pending` if no permit is available
    /// * `Poll::Ready` if a permit is available.
    pub(crate) fn poll_available(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        if self.pending.len() < self.pending.capacity() {
            return Poll::Ready(());
        }

        self.delay.as_mut().poll(cx)
    }
}

/// Calculates the number of non reserved commands for heartbeating (which
/// bypasses the ratelimiter) in a [`PERIOD`].
///
/// Reserves capacity for an additional gateway event to guard against Discord
/// sending [`OpCode::Heartbeat`]s (which requires sending a heartbeat back
/// immediately).
///
/// [`OpCode::Heartbeat`]: twilight_model::gateway::OpCode::Heartbeat
fn nonreserved_commands_per_reset(heartbeat_interval: Duration) -> u8 {
    /// Guard against faulty gateways specifying low heartbeat intervals by
    /// maximally reserving this many heartbeats per [`PERIOD`].
    const MAX_NONRESERVED_COMMANDS_PER_PERIOD: u8 = COMMANDS_PER_PERIOD - 10;

    // Calculate the amount of heartbeats per heartbeat interval.
    let heartbeats_per_reset = PERIOD.as_secs_f32() / heartbeat_interval.as_secs_f32();

    // Round up to be on the safe side.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let heartbeats_per_reset = heartbeats_per_reset.ceil() as u8;

    // Reserve an extra heartbeat just in case.
    let heartbeats_per_reset = heartbeats_per_reset.saturating_add(1);

    // Subtract the reserved heartbeats from the total available events.
    let nonreserved_commands_per_reset = COMMANDS_PER_PERIOD.saturating_sub(heartbeats_per_reset);

    // Take the larger value between this and the guard value.
    nonreserved_commands_per_reset.max(MAX_NONRESERVED_COMMANDS_PER_PERIOD)
}

#[cfg(test)]
mod tests {
    use super::{nonreserved_commands_per_reset, CommandRatelimiter, PERIOD};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, future::poll_fn, task::Poll, time::Duration};
    use tokio::time;

    assert_impl_all!(CommandRatelimiter: Debug, Send, Sync);

    #[test]
    fn nonreserved_commands() {
        assert_eq!(
            118,
            nonreserved_commands_per_reset(Duration::from_secs(u64::MAX))
        );
        assert_eq!(118, nonreserved_commands_per_reset(Duration::from_secs(60)));
        assert_eq!(
            117,
            nonreserved_commands_per_reset(Duration::from_millis(42_500))
        );
        assert_eq!(117, nonreserved_commands_per_reset(Duration::from_secs(30)));
        assert_eq!(
            116,
            nonreserved_commands_per_reset(Duration::from_millis(29_999))
        );
        assert_eq!(110, nonreserved_commands_per_reset(Duration::ZERO));
    }

    const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(60);

    #[tokio::test(start_paused = true)]
    async fn full_reset() {
        let mut ratelimiter = CommandRatelimiter::new(HEARTBEAT_INTERVAL);

        assert_eq!(ratelimiter.available(), ratelimiter.max());
        for _ in 0..ratelimiter.max() {
            poll_fn(|cx| ratelimiter.poll_acquire(cx)).await;
        }
        assert_eq!(ratelimiter.available(), 0);

        // Should not refill until PERIOD has passed.
        time::advance(PERIOD - Duration::from_millis(100)).await;
        assert_eq!(ratelimiter.available(), 0);

        // All should be refilled.
        time::advance(Duration::from_millis(100)).await;
        assert_eq!(ratelimiter.available(), ratelimiter.max());
    }

    #[tokio::test(start_paused = true)]
    async fn half_reset() {
        let mut ratelimiter = CommandRatelimiter::new(HEARTBEAT_INTERVAL);

        assert_eq!(ratelimiter.available(), ratelimiter.max());
        for _ in 0..ratelimiter.max() / 2 {
            poll_fn(|cx| ratelimiter.poll_acquire(cx)).await;
        }
        assert_eq!(ratelimiter.available(), ratelimiter.max() / 2);

        time::advance(PERIOD / 2).await;

        assert_eq!(ratelimiter.available(), ratelimiter.max() / 2);
        for _ in 0..ratelimiter.max() / 2 {
            poll_fn(|cx| ratelimiter.poll_acquire(cx)).await;
        }
        assert_eq!(ratelimiter.available(), 0);

        // Half should be refilled.
        time::advance(PERIOD / 2).await;
        assert_eq!(ratelimiter.available(), ratelimiter.max() / 2);

        // All should be refilled.
        time::advance(PERIOD / 2).await;
        assert_eq!(ratelimiter.available(), ratelimiter.max());
    }

    #[tokio::test(start_paused = true)]
    async fn constant_capacity() {
        let mut ratelimiter = CommandRatelimiter::new(HEARTBEAT_INTERVAL);
        let max = ratelimiter.max();

        for _ in 0..max {
            poll_fn(|cx| ratelimiter.poll_acquire(cx)).await;
        }
        assert_eq!(ratelimiter.available(), 0);

        poll_fn(|cx| ratelimiter.poll_acquire(cx)).await;
        assert_eq!(max, ratelimiter.max());
    }

    #[tokio::test(start_paused = true)]
    async fn spurious_poll() {
        let mut ratelimiter = CommandRatelimiter::new(HEARTBEAT_INTERVAL);

        for _ in 0..ratelimiter.max() {
            poll_fn(|cx| ratelimiter.poll_acquire(cx)).await;
        }
        assert_eq!(ratelimiter.available(), 0);

        // Spuriously poll after registering the waker but before the timer has
        // fired.
        poll_fn(|cx| {
            if ratelimiter.poll_available(cx).is_ready() {
                return Poll::Ready(());
            };
            let deadline = ratelimiter.delay.deadline();
            assert!(ratelimiter.poll_available(cx).is_pending());
            assert_eq!(deadline, ratelimiter.delay.deadline(), "deadline was reset");
            Poll::Pending
        })
        .await;
    }
}
