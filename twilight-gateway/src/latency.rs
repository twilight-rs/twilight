//! Statistics about the latency of a shard, useful for debugging.

use std::time::{Duration, Instant};

/// [`Shard`]'s gateway connection latency.
///
/// Measures the difference between sending a heartbeat and receiving an
/// acknowledgement, also known as a heartbeat period. Spurious heartbeat
/// acknowledgements are ignored.
///
/// May be obtained via [`Shard::latency`].
///
/// [`Shard`]: crate::Shard
/// [`Shard::latency`]: crate::Shard::latency
#[derive(Clone, Debug)]
pub struct Latency {
    /// Sum of recorded latencies.
    latency_sum: Duration,
    /// Number of recorded heartbeat periods.
    periods: u32,
    /// When the last heartbeat received an acknowledgement.
    received: Option<Instant>,
    /// List of most recent latencies.
    recent: [Duration; Self::RECENT_LEN],
    /// When the last heartbeat was sent.
    sent: Option<Instant>,
}

impl Latency {
    /// Number of recent latencies to store.
    const RECENT_LEN: usize = 5;

    /// Create a new instance for tracking shard latency.
    pub(crate) const fn new() -> Self {
        Self {
            latency_sum: Duration::ZERO,
            periods: 0,
            received: None,
            recent: [Duration::MAX; Self::RECENT_LEN],
            sent: None,
        }
    }

    /// Average latency.
    ///
    /// For example, a reasonable value for this may be between 10 to 100
    /// milliseconds depending on the network connection and physical location.
    ///
    /// Returns [`None`] if no heartbeat periods have been recorded.
    pub const fn average(&self) -> Option<Duration> {
        self.latency_sum.checked_div(self.periods)
    }

    /// Number of recorded heartbeat periods.
    pub const fn periods(&self) -> u32 {
        self.periods
    }

    /// Most recent latencies from newest to oldest.
    pub fn recent(&self) -> &[Duration] {
        // We use the sentinel value of Duration::MAX since using
        // `Duration::ZERO` would cause tests depending on elapsed time on fast
        // CPUs to flake. See issue #2114.
        let maybe_zero_idx = self
            .recent
            .iter()
            .position(|duration| *duration == Duration::MAX);

        &self.recent[0..maybe_zero_idx.unwrap_or(Self::RECENT_LEN)]
    }

    /// When the last heartbeat received an acknowledgement.
    pub const fn received(&self) -> Option<Instant> {
        self.received
    }

    /// When the last heartbeat was sent.
    pub const fn sent(&self) -> Option<Instant> {
        self.sent
    }

    /// Record that a heartbeat acknowledgement was received, completing the
    /// period.
    ///
    /// The current time is subtracted against when the last heartbeat
    /// [was sent] to calculate the heartbeat period's latency.
    ///
    /// # Panics
    ///
    /// Panics if the period is already complete or has not begun.
    ///
    /// [was sent]: Self::record_sent
    #[track_caller]
    pub(crate) fn record_received(&mut self) {
        debug_assert!(self.received.is_none(), "period completed multiple times");

        let now = Instant::now();
        let period_latency = now - self.sent.expect("period has not begun");
        self.received = Some(now);
        self.periods += 1;

        self.latency_sum += period_latency;
        self.recent.copy_within(..Self::RECENT_LEN - 1, 1);
        self.recent[0] = period_latency;
    }

    /// Record that a heartbeat was sent, beginning a new period.
    ///
    /// The current time is stored to be used in [`record_received`].
    ///
    /// [`record_received`]: Self::record_received
    pub(crate) fn record_sent(&mut self) {
        self.received = None;
        self.sent = Some(Instant::now());
    }
}

#[cfg(test)]
mod tests {
    use super::Latency;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, time::Duration};

    assert_impl_all!(Latency: Clone, Debug, Send, Sync);

    const fn default_latency() -> Latency {
        Latency {
            latency_sum: Duration::from_millis(510),
            periods: 17,
            received: None,
            recent: [
                Duration::from_millis(20),
                Duration::from_millis(25),
                Duration::from_millis(30),
                Duration::from_millis(35),
                Duration::from_millis(40),
            ],
            sent: None,
        }
    }

    #[test]
    fn public_api() {
        let latency = default_latency();
        assert_eq!(latency.average(), Some(Duration::from_millis(30)));
        assert_eq!(latency.periods(), 17);
        assert!(latency.received().is_none());
        assert!(latency.sent().is_none());

        assert_eq!(latency.recent.len(), Latency::RECENT_LEN);
        let mut iter = latency.recent().iter();
        assert_eq!(iter.next(), Some(&Duration::from_millis(20)));
        assert_eq!(iter.next_back(), Some(&Duration::from_millis(40)));
        assert_eq!(iter.next(), Some(&Duration::from_millis(25)));
        assert_eq!(iter.next(), Some(&Duration::from_millis(30)));
        assert_eq!(iter.next_back(), Some(&Duration::from_millis(35)));
        assert!(iter.next().is_none());
        assert!(iter.next_back().is_none());
    }

    /// Test that only recent values up to and not including the sentinel value
    /// are returned.
    #[test]
    fn recent() {
        // Assert that when all recent latencies are the sentinel value then an
        // empty slice is returned.
        let no_recents = Latency {
            latency_sum: Duration::ZERO,
            periods: 0,
            received: None,
            recent: [Duration::MAX; Latency::RECENT_LEN],
            sent: None,
        };
        assert!(no_recents.recent().is_empty());

        // Assert that when only some recent latencies aren't the sentinel value
        // then a partial slice is returned.
        let partial = Latency {
            recent: [
                Duration::from_millis(40),
                Duration::from_millis(50),
                Duration::MAX,
                Duration::MAX,
                Duration::MAX,
            ],
            ..no_recents
        };
        assert_eq!(
            [Duration::from_millis(40), Duration::from_millis(50)],
            partial.recent()
        );

        // Assert that when all recent latencies aren't the sentinel value then
        // the full slice is returned.
        let full = Latency {
            recent: [
                Duration::from_millis(40),
                Duration::from_millis(50),
                Duration::from_millis(60),
                Duration::from_millis(70),
                Duration::from_millis(60),
            ],
            ..no_recents
        };
        assert_eq!(
            [
                Duration::from_millis(40),
                Duration::from_millis(50),
                Duration::from_millis(60),
                Duration::from_millis(70),
                Duration::from_millis(60),
            ],
            full.recent()
        );
    }

    #[test]
    fn record_period() {
        let mut latency = Latency::new();
        assert_eq!(latency.periods(), 0);
        assert!(latency.received().is_none());
        assert!(latency.sent().is_none());
        assert!(latency.recent().is_empty());

        latency.record_sent();
        assert_eq!(latency.periods(), 0);
        assert!(latency.received().is_none());
        assert!(latency.sent().is_some());

        latency.record_received();
        assert_eq!(latency.periods(), 1);
        assert!(latency.received().is_some());
        assert!(latency.sent().is_some());
        assert_eq!(latency.recent().len(), 1);

        latency.record_sent();
        assert_eq!(latency.periods(), 1);
        assert!(latency.received().is_none());
        assert!(latency.sent().is_some());
        assert_eq!(latency.recent().len(), 1);
    }

    #[test]
    #[should_panic(expected = "period completed multiple times")]
    fn record_completed_period() {
        let mut latency = Latency::new();
        latency.record_sent();
        latency.record_received();
        latency.record_received();
    }

    #[test]
    #[should_panic(expected = "period has not begun")]
    fn record_not_begun_period() {
        let mut latency = Latency::new();
        latency.record_received();
    }
}
