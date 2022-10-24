//! Ratelimiter on the user's ability to [send messages].
//!
//! See <https://discord.com/developers/docs/topics/gateway#rate-limiting>
//!
//! [send messages]: crate::Shard::send

use std::{
    future::{poll_fn, Future},
    pin::Pin,
    task::{ready, Context, Poll},
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
    /// Queue of instants started when a permit was acquired.
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
    pub fn available(&self) -> u8 {
        // Filter out elapsed instants.
        #[allow(clippy::cast_possible_truncation)]
        let used_permits = self
            .instants
            .iter()
            .filter(|instant| instant.elapsed() < PERIOD)
            .count() as u8;

        self.max() - used_permits
    }

    /// Maximum number of available permits.
    #[allow(clippy::cast_possible_truncation)]
    pub fn max(&self) -> u8 {
        self.instants.capacity() as u8
    }

    /// Duration until the next permit is available.
    pub fn next_available(&self) -> Duration {
        self.instants.first().map_or(Duration::ZERO, |instant| {
            PERIOD.saturating_sub(instant.elapsed())
        })
    }

    /// Returns when a ratelimit permit becomes available.
    pub(crate) async fn acquire(&mut self) {
        poll_fn(|cx| self.poll_available(cx)).await;
        self.clean();

        self.instants.push(Instant::now());
    }

    /// Polls for the next time a permit is available.
    pub(crate) fn poll_available(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        if self.available() == 0 {
            let new_deadline = Instant::now() + self.next_available();
            // Conditionally reset `Sleep` as it is expensive to do so.
            if self.delay.deadline() != new_deadline {
                tracing::trace!(?new_deadline, old_deadline = ?self.delay.deadline());
                self.delay.as_mut().reset(new_deadline);
            }
            ready!(self.delay.as_mut().poll(cx));
        }

        Poll::Ready(())
    }

    /// Cleans up elapsed instants.
    fn clean(&mut self) {
        self.instants.retain(|instant| instant.elapsed() < PERIOD);
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
    use std::{fmt::Debug, time::Duration};
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
        assert_eq!(110, nonreserved_commands_per_reset(Duration::ZERO))
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
}
