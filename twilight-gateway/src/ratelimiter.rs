//! Ratelimiter on the user's ability to [send messages].
//!
//! [send messages]: crate::Shard::send

use leaky_bucket_lite::LeakyBucket;
use std::time::{Duration, Instant};

/// Interval of how often the ratelimit bucket resets, in milliseconds.
const RESET_DURATION_MILLISECONDS: u64 = 60_000;

/// Ratelimiter for sending commands over the gateway to Discord.
#[derive(Debug)]
pub struct CommandRatelimiter {
    /// Bucket used for limiting actions.
    bucket: LeakyBucket,
}

impl CommandRatelimiter {
    /// Create a new ratelimiter.
    pub(crate) fn new(heartbeat_interval: u64) -> Self {
        /// Interval of how often to refill the bucket.
        const REFILL_INTERVAL: Duration = Duration::from_millis(RESET_DURATION_MILLISECONDS);

        // Number of commands allotted to the user per reset period.
        let commands_allotted = u32::from(available_commands_per_interval(heartbeat_interval));

        let bucket = LeakyBucket::builder()
            .max(commands_allotted)
            .tokens(commands_allotted)
            .refill_interval(REFILL_INTERVAL)
            .refill_amount(commands_allotted)
            .build();

        Self { bucket }
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

/// Calculate the number of commands to allot in a given reset period while
/// taking the heartbeat interval into account.
///
/// This is reserving twice as much as needed for heartbeats, to account for
/// Discord sending us a heartbeat and expecting a heartbeat in response.
///
/// For example, when the heartbeat interval is 42500 milliseconds then 116
/// commands will be allotted per reset period.
fn available_commands_per_interval(heartbeat_interval: u64) -> u8 {
    /// Number of commands to reserve per reset. This number is a bit
    /// high because the heartbeat interval may be anything, so we're
    /// just being cautious here.
    const ALLOT_ON_FAIL: u8 = COMMANDS_PER_RESET - 10;

    /// Number of commands allowed in a given reset period.
    ///
    /// API documentation with details:
    /// <https://discord.com/developers/docs/topics/gateway#rate-limiting>
    const COMMANDS_PER_RESET: u8 = 120;

    // Guard against the interval being 0, in which case we can default.
    if heartbeat_interval == 0 {
        return ALLOT_ON_FAIL;
    }

    let mut heartbeats = RESET_DURATION_MILLISECONDS / heartbeat_interval;
    let remainder = RESET_DURATION_MILLISECONDS % heartbeat_interval;

    // If we have a remainder then we reserve an additional heartbeat.
    //
    // If there is a remainder per reset then in theory we could allot one less
    // command for heartbeating variably every number of resets, but it's best
    // to be cautious and keep it simple.
    if remainder > 0 {
        heartbeats = heartbeats.saturating_add(1);
    }

    // Convert the heartbeats to a u8. The number of heartbeats **should** never
    // be above `u8::MAX`, so the error pattern branch should never be reached.
    let heartbeats_converted = if let Ok(value) = heartbeats.try_into() {
        value
    } else {
        tracing::warn!(
            %heartbeats,
            "heartbeats > u8 max; defaulting to allotting {}",
            ALLOT_ON_FAIL,
        );

        ALLOT_ON_FAIL
    };

    COMMANDS_PER_RESET.saturating_sub(heartbeats_converted * 2)
}

#[cfg(test)]
mod tests {
    use super::CommandRatelimiter;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(CommandRatelimiter: Debug, Send, Sync);

    #[test]
    fn available_commands_per_interval() {
        assert_eq!(118, super::available_commands_per_interval(60_000));
        assert_eq!(116, super::available_commands_per_interval(42_500));
        assert_eq!(116, super::available_commands_per_interval(30_000));
        assert_eq!(114, super::available_commands_per_interval(29_999));
    }
}
