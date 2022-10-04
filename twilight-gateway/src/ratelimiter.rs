//! Ratelimiter on the user's ability to [send messages].
//!
//! See <https://discord.com/developers/docs/topics/gateway#rate-limiting>
//!
//! [send messages]: crate::Shard::send

use std::time::Instant as StdInstant;
use tokio::time::{self, Duration, Instant};

/// Number of commands allowed in a given [`RESET_DURATION`].
const COMMANDS_PER_RESET: u8 = 120;

/// Duration until the ratelimit bucket resets.
const RESET_DURATION: Duration = Duration::from_secs(60);

/// Ratelimiter for sending commands over the gateway to Discord.
#[derive(Debug)]
pub struct CommandRatelimiter {
    /// Queue of instants started when a command was sent.
    ///
    /// The instants are considered elapsed when they've been running for
    /// [`RESET_DURATION`].
    instants: Vec<Instant>,
}

impl CommandRatelimiter {
    /// Create a new ratelimiter with capacity reserved for heartbeating, see
    /// [`nonreserved_commands_per_reset`] for why.
    pub(crate) fn new(heartbeat_interval: Duration) -> Self {
        let allotted = nonreserved_commands_per_reset(heartbeat_interval);

        Self {
            instants: Vec::with_capacity(allotted.into()),
        }
    }

    /// Current number of commands that are still available within the interval.
    #[allow(clippy::cast_possible_truncation)]
    pub fn available(&self) -> u8 {
        self.max()
            - self
                .instants
                .iter()
                .filter(|instant| instant.elapsed() < RESET_DURATION)
                .count() as u8
    }

    /// Maximum number of commands that may be made per interval.
    #[allow(clippy::cast_possible_truncation)]
    pub fn max(&self) -> u8 {
        self.instants.capacity() as u8
    }

    /// When the next command is available.
    pub fn next_refill(&self) -> StdInstant {
        self.instants.first().map_or(StdInstant::now(), |instant| {
            instant.into_std() + (RESET_DURATION - instant.elapsed())
        })
    }

    /// Acquire a token from the ratelimiter, waiting until one is available.
    pub(crate) async fn acquire(&mut self) {
        if self.available() == 0 {
            time::sleep_until(Instant::from_std(self.next_refill())).await;
        }
        self.clean();
        assert!(self.available() > 0);
        self.instants.push(Instant::now());
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

    const DURATION: Duration = Duration::from_secs(60);

    #[tokio::test(start_paused = true)]
    async fn full_reset() {
        let mut ratelimiter = CommandRatelimiter::new(DURATION);

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
        let mut ratelimiter = CommandRatelimiter::new(DURATION);

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
