//! Rate limit events sent to the Gateway.
//!
//! See <https://discord.com/developers/docs/topics/gateway#rate-limiting>
//!
//! # Algorithm
//!
//! [`CommandRatelimiter`] is implemented as a sliding window log. This is the
//! only ratelimit algorithm that supports burst requests and guarantees that
//! the (t - [`PERIOD`], t] window is never exceeded. See
//! <https://hechao.li/posts/Rate-Limiter-Part1/> for an overview of it and
//! other alternative algorithms.

#![allow(clippy::cast_possible_truncation)]

use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};
use tokio::time::{Duration, Instant, Sleep};

/// Duration until an acquired permit is released.
const PERIOD: Duration = Duration::from_secs(60);

/// Number of permits per [`PERIOD`].
const PERMITS: u8 = 120;

/// Ratelimiter for sending commands over the gateway to Discord.
#[derive(Debug)]
pub struct CommandRatelimiter {
    /// Future that completes when the next permit is released.
    ///
    /// Counts as an acquired permit if pending.
    delay: Pin<Box<Sleep>>,
    /// Ordered queue of timestamps relative to [`Self::delay`] in milliseconds
    /// when permits release.
    queue: VecDeque<u16>,
}

impl CommandRatelimiter {
    /// Create a new ratelimiter with some capacity reserved for heartbeating.
    pub(crate) fn new(heartbeat_interval: Duration) -> Self {
        let capacity = usize::from(nonreserved_commands_per_reset(heartbeat_interval)) - 1;

        let mut queue = VecDeque::with_capacity(capacity);
        if queue.capacity() != capacity {
            queue.resize(capacity, 0);
            // `into_boxed_slice().into_vec()` guarantees len == capacity.
            let vec = Vec::from(queue).into_boxed_slice().into_vec();
            // This is guaranteed to not allocate.
            queue = VecDeque::from(vec);
            queue.clear();
        }

        Self {
            delay: Box::pin(tokio::time::sleep_until(Instant::now())),
            queue,
        }
    }

    /// Number of available permits.
    pub fn available(&self) -> u8 {
        let now = Instant::now();
        let acquired = if now >= self.delay.deadline() {
            self.next_acquired_position(now)
                .map_or(0, |released_count| self.queue.len() - released_count)
        } else {
            self.queue.len() + 1
        };

        self.max() - acquired as u8
    }

    /// Maximum number of available permits.
    pub fn max(&self) -> u8 {
        self.queue.capacity() as u8 + 1
    }

    /// Duration until the next permit is available.
    pub fn next_available(&self) -> Duration {
        self.delay
            .deadline()
            .saturating_duration_since(Instant::now())
    }

    /// Attempts to acquire a permit.
    ///
    /// # Returns
    ///
    /// * `Poll::Pending` if no permit is available
    /// * `Poll::Ready` if a permit is acquired.
    pub(crate) fn poll_acquire(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        ready!(self.poll_available(cx));

        let now = Instant::now();
        if now >= self.delay.deadline() {
            if let Some(new_deadline_idx) = self.next_acquired_position(now) {
                self.rebase(new_deadline_idx);
            } else {
                self.queue.clear();
                self.delay.as_mut().reset(now + PERIOD);

                return Poll::Ready(());
            }
        }

        let releases = (now + PERIOD) - self.delay.deadline();
        debug_assert_ne!(self.queue.capacity(), self.queue.len());
        self.queue.push_back(releases.as_millis() as u16);

        if self.queue.len() == self.queue.capacity() {
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
        if self.queue.len() < self.queue.capacity() {
            return Poll::Ready(());
        }

        self.delay.as_mut().poll(cx)
    }

    /// Searches for the first acquired timestamp, returning its index.
    ///
    /// If every timestamp is released, it returns `None`.
    fn next_acquired_position(&self, now: Instant) -> Option<usize> {
        self.queue
            .iter()
            .map(|&m| self.delay.deadline() + Duration::from_millis(m.into()))
            .position(|deadline| deadline > now)
    }

    /// Resets to a new deadline and updates acquired permits' relative timestamp.
    fn rebase(&mut self, new_deadline_idx: usize) {
        let duration = Duration::from_millis(self.queue[new_deadline_idx].into());
        let new_deadline = self.delay.deadline() + duration;

        self.queue.drain(..=new_deadline_idx);

        for timestamp in &mut self.queue {
            let deadline = self.delay.deadline() + Duration::from_millis((*timestamp).into());
            *timestamp = (deadline - new_deadline).as_millis() as u16;
        }

        self.delay.as_mut().reset(new_deadline);
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
    const MAX_NONRESERVED_COMMANDS_PER_PERIOD: u8 = PERMITS - 10;

    // Calculate the amount of heartbeats per heartbeat interval.
    let heartbeats_per_reset = PERIOD.as_secs_f32() / heartbeat_interval.as_secs_f32();

    // Round up to be on the safe side.
    #[allow(clippy::cast_sign_loss)]
    let heartbeats_per_reset = heartbeats_per_reset.ceil() as u8;

    // Reserve an extra heartbeat just in case.
    let heartbeats_per_reset = heartbeats_per_reset.saturating_add(1);

    // Subtract the reserved heartbeats from the total available events.
    let nonreserved_commands_per_reset = PERMITS.saturating_sub(heartbeats_per_reset);

    // Take the larger value between this and the guard value.
    nonreserved_commands_per_reset.max(MAX_NONRESERVED_COMMANDS_PER_PERIOD)
}

#[cfg(test)]
mod tests {
    use super::{nonreserved_commands_per_reset, CommandRatelimiter, PERIOD};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, future::poll_fn, time::Duration};
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
    async fn rebase() {
        let mut ratelimiter = CommandRatelimiter::new(HEARTBEAT_INTERVAL);

        for _ in 0..5 {
            time::advance(Duration::from_millis(20)).await;
            poll_fn(|cx| ratelimiter.poll_acquire(cx)).await;
        }
        assert_eq!(ratelimiter.available(), ratelimiter.max() - 5);

        time::advance(PERIOD - Duration::from_millis(80)).await;
        assert_eq!(ratelimiter.available(), ratelimiter.max() - 4);

        for _ in 0..4 {
            poll_fn(|cx| ratelimiter.poll_acquire(cx)).await;
            time::advance(Duration::from_millis(20)).await;
            assert_eq!(ratelimiter.available(), ratelimiter.max() - 4);
        }
    }
}
