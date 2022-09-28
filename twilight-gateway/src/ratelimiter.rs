//! Ratelimiter on the user's ability to [send messages].
//!
//! See <https://discord.com/developers/docs/topics/gateway#rate-limiting>
//!
//! [send messages]: crate::Shard::send

use leaky_bucket_lite::LeakyBucket;
use std::time::{Duration, Instant};

/// Number of commands allowed in a given [`RESET_DURATION`].
const COMMANDS_PER_RESET: u32 = 120;

/// Duration until the ratelimit bucket resets.
const RESET_DURATION: Duration = Duration::from_secs(60);

/// Ratelimiter for sending commands over the gateway to Discord.
#[derive(Debug)]
pub struct CommandRatelimiter {
    /// Bucket used for limiting actions.
    bucket: LeakyBucket,
}

impl CommandRatelimiter {
    /// Create a new ratelimiter.
    pub(crate) fn new(heartbeat_interval: u64) -> Self {
        let allotted = nonreserved_commands_per_reset(Duration::from_millis(heartbeat_interval));

        Self {
            bucket: LeakyBucket::builder()
                .max(allotted)
                .tokens(allotted)
                .refill_interval(RESET_DURATION)
                .refill_amount(allotted)
                .build(),
        }
    }

    /// Current number of commands that are still available within the interval.
    pub fn available(&self) -> u32 {
        self.bucket.tokens()
    }

    /// Maximum number of commands that may be made per interval.
    pub fn max(&self) -> u32 {
        self.bucket.max()
    }

    /// When the bucket will refresh the available number of commands again.
    pub fn next_refill(&self) -> Instant {
        self.bucket.next_refill().into_std()
    }

    /// Acquire a token from the bucket, waiting until one is available.
    pub(crate) async fn acquire_one(&self) {
        self.bucket.acquire_one().await;
    }
}

/// Calculate the number of non reserved commands for heartbeating (which skips
/// the ratelimiter) in a given [`RESET_DURATION`].
///
/// Reserves capacity for twice the amount of heartbeats to account for Discord
/// absurdly sending [`OpCode::Heartbeat`]s when the gateway is ratelimited
/// (which requires the gateway to immediately send a heartbeat back).
///
/// [`OpCode::Heartbeat`]: twilight_model::gateway::OpCode::Heartbeat
fn nonreserved_commands_per_reset(heartbeat_interval: Duration) -> u32 {
    /// Guard against faulty gateway implementations sending absurdly low
    /// heartbeat intervals by maximally reserving 10 heartbeats per
    /// [`RESET_DURATION`].
    const MAX_NONRESERVED_COMMANDS_PER_RESET: u32 = COMMANDS_PER_RESET - 10;

    // Round up to be on the safe side.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let reserved_heartbeats =
        (RESET_DURATION.as_secs_f32() / heartbeat_interval.as_secs_f32()).ceil() as u32;

    COMMANDS_PER_RESET
        .saturating_sub(reserved_heartbeats * 2)
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
    fn commands_per_interval() {
        assert_eq!(118, nonreserved_commands_per_reset(Duration::from_secs(60)));
        assert_eq!(
            116,
            nonreserved_commands_per_reset(Duration::from_millis(42_500))
        );
        assert_eq!(116, nonreserved_commands_per_reset(Duration::from_secs(30)));
        assert_eq!(
            114,
            nonreserved_commands_per_reset(Duration::from_millis(29_999))
        );
    }

    #[tokio::test(start_paused = true)]
    async fn ratelimiter_specification() {
        let ratelimiter = CommandRatelimiter::new(RESET_DURATION.as_millis().try_into().unwrap());

        let commands_per_reset: u32 = nonreserved_commands_per_reset(RESET_DURATION);

        assert!(ratelimiter.available() == commands_per_reset);
        for _ in 0..commands_per_reset {
            ratelimiter.acquire_one().await;
        }
        assert!(ratelimiter.available() == 0);

        // Should not refill until RESET_PERIOD has passed.
        time::advance(RESET_DURATION - Duration::from_secs(1)).await;
        assert!(ratelimiter.available() == 0);

        // All should be refilled.
        time::advance(Duration::from_secs(1)).await;
        assert!(ratelimiter.available() == commands_per_reset);

        for _ in 0..commands_per_reset / 2 {
            ratelimiter.acquire_one().await;
        }
        assert!(ratelimiter.available() == commands_per_reset / 2);

        time::advance(RESET_DURATION / 2).await;

        for _ in 0..commands_per_reset / 2 {
            ratelimiter.acquire_one().await;
        }
        assert!(ratelimiter.available() == 0);

        // Half should be refilled.
        time::advance(RESET_DURATION / 2).await;
        assert!(ratelimiter.available() == commands_per_reset / 2);

        // All should be refilled.
        time::advance(RESET_DURATION / 2).await;
        assert!(ratelimiter.available() == commands_per_reset);
    }
}
