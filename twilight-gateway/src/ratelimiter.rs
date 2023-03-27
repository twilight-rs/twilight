//! Ratelimiter on the user's ability to [send messages].
//!
//! See <https://discord.com/developers/docs/topics/gateway#rate-limiting>
//!
//! # Algorithm
//!
//! [`CommandRatelimiter`] is implemented as a sliding window log. This is the
//! only ratelimit algorithm that supports burst requests and guarantees that
//! the (t - [`PERIOD`], t] window is never exceeded. See
//! <https://hechao.li/2018/06/25/Rate-Limiter-Part1> for an overview of it and
//! other alternative ratelimit algorithms.
//!
//! [`Instant::now`]: std::time::Instant::now
//! [send messages]: crate::Shard::send

use std::{
    future::{poll_fn, Future},
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::{sleep, Duration, Instant, Sleep};

/// Number of commands allowed in a [`PERIOD`].
const COMMANDS_PER_PERIOD: u8 = 120;

/// Gateway ratelimiter period duration.
const PERIOD: Duration = Duration::from_secs(60);

/// Ratelimiter for sending commands over the gateway to Discord.
#[derive(Debug)]
pub struct CommandRatelimiter {
    /// Future that completes the next time the ratelimiter allows a permit.
    delay: Pin<Box<Sleep>>,
    /// Ordered queue of instants when a permit elapses.
    instants: Vec<Instant>,
}

impl CommandRatelimiter {
    /// Create a new ratelimiter with some capacity reserved for heartbeating.
    pub(crate) fn new(heartbeat_interval: Duration) -> Self {
        let allotted = nonreserved_commands_per_reset(heartbeat_interval);

        Self {
            delay: Box::pin(sleep(Duration::ZERO)),
            instants: Vec::with_capacity(allotted.into()),
        }
    }

    /// Number of available permits.
    #[allow(clippy::cast_possible_truncation)]
    pub fn available(&self) -> u8 {
        let now = Instant::now();
        let elapsed_permits = self.instants.partition_point(|&elapsed| elapsed <= now);
        let used_permits = self.instants.len() - elapsed_permits;

        self.max() - used_permits as u8
    }

    /// Maximum number of available permits.
    #[allow(clippy::cast_possible_truncation)]
    pub fn max(&self) -> u8 {
        self.instants.capacity() as u8
    }

    /// Duration until the next permit is available.
    pub fn next_available(&self) -> Duration {
        self.instants.first().map_or(Duration::ZERO, |elapsed| {
            elapsed.saturating_duration_since(Instant::now())
        })
    }

    /// Returns when a ratelimit permit becomes available.
    pub(crate) async fn acquire(&mut self) {
        poll_fn(|cx| self.poll_available(cx)).await;

        self.instants.push(Instant::now() + PERIOD);
    }

    /// Polls for the next time a permit is available.
    pub(crate) fn poll_available(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        if self.instants.len() != self.instants.capacity() {
            return Poll::Ready(());
        }

        let new_deadline = self.instants[0];
        let is_deadline_different = new_deadline != self.delay.deadline();
        let is_deadline_in_future = new_deadline > Instant::now();
        match (is_deadline_different, is_deadline_in_future) {
            (true, true) => {
                tracing::trace!(?new_deadline, old_deadline = ?self.delay.deadline());
                self.delay.as_mut().reset(new_deadline);

                // Register waker.
                _ = self.delay.as_mut().poll(cx);

                Poll::Pending
            }
            (false, true) => Poll::Pending,
            _ => {
                let used_permits = (self.max() - self.available()).into();
                self.instants.rotate_right(used_permits);
                self.instants.truncate(used_permits);

                Poll::Ready(())
            }
        }
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
            ratelimiter.acquire().await;
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
            ratelimiter.acquire().await;
        }
        assert_eq!(ratelimiter.available(), ratelimiter.max() / 2);

        time::advance(PERIOD / 2).await;

        assert_eq!(ratelimiter.available(), ratelimiter.max() / 2);
        for _ in 0..ratelimiter.max() / 2 {
            ratelimiter.acquire().await;
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
            ratelimiter.acquire().await;
        }
        assert_eq!(ratelimiter.available(), 0);

        ratelimiter.acquire().await;
        assert_eq!(max, ratelimiter.max());
    }

    #[tokio::test(start_paused = true)]
    async fn spurious_poll() {
        let mut ratelimiter = CommandRatelimiter::new(HEARTBEAT_INTERVAL);

        for _ in 0..ratelimiter.max() {
            ratelimiter.acquire().await;
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
