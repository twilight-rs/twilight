//! Statistics about the latency of a shard, useful for debugging.

use std::time::{Duration, Instant};

/// [`Shard`]'s gateway connection latency.
///
/// Calculated by measuring the difference between when sending a heartbeat and
/// receiving an acknowledgement.
///
/// May be obtained via [`Shard::latency`].
///
/// [`Shard`]: crate::Shard
/// [`Shard::latency`]: crate::Shard::latency
#[derive(Clone, Debug)]
pub struct Latency {
    /// Total number of heartbeat acknowledgements that have been received.
    heartbeats: u32,
    /// When the last heartbeat received an acknowledgement.
    received: Option<Instant>,
    /// List of most recent latencies.
    recent: [Duration; Self::RECENT_LEN],
    /// When the last heartbeat was sent.
    sent: Option<Instant>,
    /// Combined latencies of all heartbeats, used in conjunction with
    /// [`heartbeats`] to determine the average latency.
    ///
    /// [`heartbeats`]: Self::heartbeats
    total_duration: Duration,
}

impl Latency {
    /// Maximum number of recent latencies to store.
    const RECENT_LEN: usize = 5;

    /// Create a new instance for tracking shard latency.
    pub(crate) const fn new() -> Self {
        Self {
            heartbeats: 0,
            received: None,
            recent: [Duration::ZERO; Self::RECENT_LEN],
            sent: None,
            total_duration: Duration::ZERO,
        }
    }

    /// The average latency over all recorded heartbeats.
    ///
    /// For example, a reasonable value for this may be between 10 to 100
    /// milliseconds depending on the network connection and physical location.
    ///
    /// # Note
    ///
    /// If this is None, the shard has not received a heartbeat yet.
    pub const fn average(&self) -> Option<Duration> {
        self.total_duration.checked_div(self.heartbeats)
    }

    /// The total number of heartbeats that have been received.
    pub const fn heartbeats(&self) -> u32 {
        self.heartbeats
    }

    /// The most recent latencies from newest to oldest.
    pub fn recent(&self) -> &[Duration] {
        let maybe_zero_idx = self.recent.iter().position(Duration::is_zero);

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

    /// Track that a heartbeat acknowledgement was received.
    ///
    /// The current time will be used to calculate against when the last
    /// heartbeat [was sent][`track_sent`] to determine latency for the period.
    ///
    /// [`track_sent`]: Self::track_sent
    pub(crate) fn track_received(&mut self) {
        self.received = Some(Instant::now());
        self.heartbeats += 1;

        let duration = if let Some(sent) = self.sent {
            sent.elapsed()
        } else {
            return;
        };

        self.total_duration += duration;
        self.recent.rotate_right(1);
        self.recent[0] = duration;
    }

    /// Track that a heartbeat acknowledgement was sent.
    ///
    /// The current time will be stored to be used in [`track_received`].
    ///
    /// [`track_received`]: Self::track_received
    pub(crate) fn track_sent(&mut self) {
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
            heartbeats: 17,
            received: None,
            recent: [
                Duration::from_millis(20),
                Duration::from_millis(25),
                Duration::from_millis(30),
                Duration::from_millis(35),
                Duration::from_millis(40),
            ],
            sent: None,
            total_duration: Duration::from_millis(510),
        }
    }

    #[test]
    fn latency() {
        let latency = default_latency();
        assert_eq!(latency.average(), Some(Duration::from_millis(30)));
        assert_eq!(latency.heartbeats(), 17);
        assert!(latency.received().is_none());
        assert!(latency.sent().is_none());
    }

    #[test]
    fn recent_latency_iter() {
        let latency = default_latency();
        let recent = latency.recent();
        assert_eq!(recent.len(), Latency::RECENT_LEN);
        let mut iter = recent.iter();
        assert_eq!(iter.next(), Some(&Duration::from_millis(20)));
        assert_eq!(iter.next_back(), Some(&Duration::from_millis(40)));
        assert_eq!(iter.next(), Some(&Duration::from_millis(25)));
        assert_eq!(iter.next(), Some(&Duration::from_millis(30)));
        assert_eq!(iter.next_back(), Some(&Duration::from_millis(35)));
        assert!(iter.next().is_none());
        assert!(iter.next_back().is_none());
    }

    #[test]
    fn latency_track() {
        let mut latency = Latency::new();
        assert!(latency.received().is_none());
        assert!(latency.sent().is_none());
        assert!(latency.recent().is_empty());

        latency.track_sent();
        assert_eq!(latency.heartbeats(), 0);
        assert!(latency.received().is_none());
        assert!(latency.sent().is_some());

        latency.track_received();
        assert_eq!(latency.heartbeats(), 1);
        assert!(latency.received().is_some());
        assert!(latency.sent().is_some());
        assert_eq!(latency.recent().len(), 1);
    }
}
