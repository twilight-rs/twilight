//! Ratelimiter on the user's ability to [send messages].
//!
//! See <https://discord.com/developers/docs/topics/gateway#rate-limiting>
//!
//! [send messages]: crate::Shard::send

use std::{sync::Arc, time::Duration};
use tokio::sync::Semaphore;

/// Number of commands allowed in a given [`RESET_DURATION`].
const COMMANDS_PER_RESET: u8 = 120;

/// Duration until the ratelimit bucket resets.
const RESET_DURATION: Duration = Duration::from_secs(60);

/// Ratelimiter for sending commands over the gateway to Discord.
#[derive(Debug)]
pub struct CommandRatelimiter {
    /// Semaphore to limit actions.
    semaphore: Arc<Semaphore>,
    /// Capacity of the semaphore.
    ///
    /// Needs to be stored for [`max`] ([`Semaphore`] does not expose the
    /// value).
    ///
    /// [`max`]: Self::max
    semaphore_capacity: u8,
}

impl CommandRatelimiter {
    /// Create a new ratelimiter.
    pub(crate) fn new(heartbeat_interval: u64) -> Self {
        let allotted = nonreserved_commands_per_reset(Duration::from_millis(heartbeat_interval));

        Self {
            semaphore: Arc::new(Semaphore::new(usize::from(allotted))),
            semaphore_capacity: allotted,
        }
    }

    /// Current number of commands that are still available within the interval.
    pub fn available(&self) -> u8 {
        self.semaphore
            .available_permits()
            .try_into()
            .expect("constructed from u8")
    }

    /// Maximum number of commands that may be made per interval.
    // Don't stabilize const for the public API yet as the implementation may
    // change in a way it cannot be const.
    #[allow(clippy::missing_const_for_fn)]
    pub fn max(&self) -> u8 {
        self.semaphore_capacity
    }

    /// Acquire a token from the bucket, waiting until one is available.
    pub(crate) async fn acquire(&self) -> RatelimiterGuard {
        // Is reinserted inside of RatelimiterGuard.
        self.semaphore
            .acquire()
            .await
            .expect("never closed")
            .forget();

        RatelimiterGuard {
            semaphore: (Arc::clone(&self.semaphore)),
        }
    }
}

/// Guard around the ratelimit permit. As long as this is held one command is
/// guaranteed to not be ratelimited.
///
/// Because this hold a shared reference to [`Semaphore`],
/// [`CommandRatelimiter`] will only be dropped after the last guard has been
/// dropped. This can cause delays when shutting down a [`Shard`].
///
/// [`Shard`]: super::Shard
pub(crate) struct RatelimiterGuard {
    /// Shared reference to the semaphore in [`CommandRatelimiter`].
    ///
    /// Needed to reinsert the permit in `Drop`.
    semaphore: Arc<Semaphore>,
}

/// Reinsert the ratelimit permit after [`RESET_DURATION`].
///
/// This is a workaround for async drop not being available.
impl Drop for RatelimiterGuard {
    fn drop(&mut self) {
        let semaphore = Arc::clone(&self.semaphore);
        tokio::spawn(async move {
            // Reinsert after RESET_DURATION.
            tokio::time::sleep(RESET_DURATION).await;
            semaphore.add_permits(1);
        });
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
fn nonreserved_commands_per_reset(heartbeat_interval: Duration) -> u8 {
    /// Guard against faulty gateway implementations sending absurdly low
    /// heartbeat intervals by maximally reserving some number of heartbeats per
    /// [`RESET_DURATION`].
    const MAX_NONRESERVED_COMMANDS_PER_RESET: u8 = COMMANDS_PER_RESET - 10;

    // Round up to be on the safe side.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let reserved_heartbeats =
        (RESET_DURATION.as_secs_f32() / heartbeat_interval.as_secs_f32()).ceil() as u8;

    COMMANDS_PER_RESET
        .saturating_sub(reserved_heartbeats * 2)
        .max(MAX_NONRESERVED_COMMANDS_PER_RESET)
}

#[cfg(test)]
mod tests {
    use super::{nonreserved_commands_per_reset, CommandRatelimiter, RESET_DURATION};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, time::Duration};
    use tokio::{task, time};

    assert_impl_all!(CommandRatelimiter: Debug, Send, Sync);

    #[test]
    fn nonreserved_commands() {
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
    async fn ratelimiter_specification_1() {
        let ratelimiter = CommandRatelimiter::new(RESET_DURATION.as_millis().try_into().unwrap());

        let commands_per_reset = nonreserved_commands_per_reset(RESET_DURATION);

        assert!(ratelimiter.available() == commands_per_reset);
        for _ in 0..commands_per_reset {
            ratelimiter.acquire().await;
        }
        assert!(ratelimiter.available() == 0);

        // Should not refill until RESET_PERIOD has passed.
        time::sleep(RESET_DURATION - Duration::from_secs(1)).await;
        assert!(ratelimiter.available() == 0);

        // All should be refilled.
        time::sleep(Duration::from_secs(1)).await;
        for _ in 0..10 {
            task::yield_now().await;
        }
        assert!(ratelimiter.available() == commands_per_reset);
    }

    #[tokio::test(start_paused = true)]
    async fn ratelimiter_specification_2() {
        let ratelimiter = CommandRatelimiter::new(RESET_DURATION.as_millis().try_into().unwrap());

        let commands_per_reset = nonreserved_commands_per_reset(RESET_DURATION);

        assert!(ratelimiter.available() == commands_per_reset);

        for _ in 0..commands_per_reset / 2 {
            ratelimiter.acquire().await;
        }
        assert!(ratelimiter.available() == commands_per_reset / 2);

        time::sleep(RESET_DURATION / 2).await;

        for _ in 0..commands_per_reset / 2 {
            ratelimiter.acquire().await;
        }
        assert!(ratelimiter.available() == 0);

        // Half should be refilled.
        time::sleep(RESET_DURATION / 2).await;
        for _ in 0..10 {
            task::yield_now().await;
        }
        assert!(ratelimiter.available() == commands_per_reset / 2);

        // All should be refilled.
        time::sleep(RESET_DURATION / 2).await;
        for _ in 0..10 {
            task::yield_now().await;
        }
        assert!(ratelimiter.available() == commands_per_reset);
    }
}
