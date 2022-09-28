//! Ratelimiter on the user's ability to [send messages].
//!
//! See <https://discord.com/developers/docs/topics/gateway#rate-limiting>
//!
//! [send messages]: crate::Shard::send

use leaky_bucket_lite::LeakyBucket;
use std::time::{Duration, Instant};

/// Number of commands allowed in a given [`RESET_PERIOD`].
const COMMANDS_PER_RESET: u32 = 120;

/// Duration until the ratelimit bucket resets.
const RESET_PERIOD: Duration = Duration::from_secs(60);

/// Ratelimiter for sending commands over the gateway to Discord.
#[derive(Debug)]
pub struct CommandRatelimiter {
    /// Bucket used for limiting actions.
    bucket: LeakyBucket,
}

impl CommandRatelimiter {
    /// Create a new ratelimiter.
    pub(crate) fn new() -> Self {
        Self {
            bucket: LeakyBucket::builder()
                .max(COMMANDS_PER_RESET)
                .tokens(COMMANDS_PER_RESET)
                .refill_interval(RESET_PERIOD)
                .refill_amount(COMMANDS_PER_RESET)
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

#[cfg(test)]
mod tests {
    use super::{CommandRatelimiter, COMMANDS_PER_RESET, RESET_PERIOD};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, time::Duration};
    use tokio::time;

    assert_impl_all!(CommandRatelimiter: Debug, Send, Sync);

    #[tokio::test]
    async fn ratelimiter_refills() {
        let ratelimiter = CommandRatelimiter::new();

        assert!(ratelimiter.available() == COMMANDS_PER_RESET);
        for _ in 0..COMMANDS_PER_RESET {
            time::timeout(Duration::from_micros(1), ratelimiter.acquire_one())
                .await
                .unwrap();
        }
        assert!(ratelimiter.available() == 0);

        time::pause();
        // Should not refill until RESET_PERIOD has passed.
        time::advance(RESET_PERIOD - Duration::from_secs(1)).await;
        assert!(ratelimiter.available() == 0);

        // Should now be refilled.
        time::advance(Duration::from_secs(1)).await;
        assert!(ratelimiter.available() == COMMANDS_PER_RESET);
    }
}
