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

/// Number of commands allowed in a given [`RESET_DURATION`].
const COMMANDS_PER_RESET: u8 = 120;

/// Duration until the ratelimit bucket resets.
const RESET_DURATION: Duration = Duration::from_secs(60);

/// Ratelimiter for sending commands over the gateway to Discord.
#[derive(Debug)]
pub struct CommandRatelimiter {
    /// Future that completes the next time the `CommandRatelimiter` allows a
    /// permit.
    delay: Pin<Box<Sleep>>,
    /// Queue of instants started when a command was sent.
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

    /// Current number of commands that are still available within the interval.
    pub fn available(&self) -> u8 {
        // filter out elapsed instants
        #[allow(clippy::cast_possible_truncation)]
        let used_permits = self
            .instants
            .iter()
            .filter(|instant| instant.elapsed() < RESET_DURATION)
            .count() as u8;

        self.max() - used_permits
    }

    /// Maximum number of commands that may be made per interval.
    #[allow(clippy::cast_possible_truncation)]
    pub fn max(&self) -> u8 {
        self.instants.capacity() as u8
    }

    /// When the next command is available.
    pub fn next_available(&self) -> Duration {
        self.instants.first().map_or(Duration::ZERO, |instant| {
            RESET_DURATION.saturating_sub(instant.elapsed())
        })
    }

    /// Completes when a ratelimit permit is available.
    pub(crate) async fn acquire(&mut self) {
        poll_fn(|cx| self.poll_available(cx)).await;
        self.clean();

        self.instants.push(Instant::now());
    }

    /// Polls for the next time a permit is available.
    pub(crate) fn poll_available(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        if self.available() == 0 {
            let new_deadline = Instant::now() + self.next_available();
            if self.delay.deadline() < new_deadline {
                self.delay.as_mut().reset(new_deadline);
            }
            ready!(self.delay.as_mut().poll(cx));
        }

        Poll::Ready(())
    }

    /// Cleans up elapsed instants.
    fn clean(&mut self) {
        self.instants
            .retain(|instant| instant.elapsed() < RESET_DURATION);
    }
}

/// Calculate the number of non reserved commands for heartbeating (which skips
/// the ratelimiter) in a given [`RESET_DURATION`].
///
/// Reserves capacity for the amount of heartbeats + 1, to account for Discord
/// absurdly sending [`OpCode::Heartbeat`]s when the gateway is ratelimited
/// (which requires the gateway to immediately send a heartbeat back).
///
/// [`OpCode::Heartbeat`]: twilight_model::gateway::OpCode::Heartbeat
fn nonreserved_commands_per_reset(heartbeat_interval: Duration) -> u8 {
    /// Guard against faulty gateway implementations sending absurdly low
    /// heartbeat intervals by maximally reserving some number of heartbeats per
    /// [`RESET_DURATION`].
    const MAX_NONRESERVED_COMMANDS_PER_RESET: u8 = COMMANDS_PER_RESET - 10;

    // Round up to be on the safe side.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let heartbeats_per_reset =
        (RESET_DURATION.as_secs_f32() / heartbeat_interval.as_secs_f32()).ceil() as u8;

    COMMANDS_PER_RESET
        .saturating_sub(heartbeats_per_reset + 1)
        .max(MAX_NONRESERVED_COMMANDS_PER_RESET)
}

#[cfg(test)]
mod tests {
    use super::{nonreserved_commands_per_reset, CommandRatelimiter, RESET_DURATION};
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

        // Should not refill until RESET_PERIOD has passed.
        time::advance(RESET_DURATION - Duration::from_millis(100)).await;
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

        time::advance(RESET_DURATION / 2).await;

        assert_eq!(ratelimiter.available(), ratelimiter.max() / 2);
        for _ in 0..ratelimiter.max() / 2 {
            ratelimiter.acquire().await;
        }
        assert_eq!(ratelimiter.available(), 0);

        // Half should be refilled.
        time::advance(RESET_DURATION / 2).await;
        assert_eq!(ratelimiter.available(), ratelimiter.max() / 2);

        // All should be refilled.
        time::advance(RESET_DURATION / 2).await;
        assert_eq!(ratelimiter.available(), ratelimiter.max());
    }
}
