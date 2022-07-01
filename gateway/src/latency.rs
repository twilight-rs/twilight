use std::{
    iter::FusedIterator,
    slice::Iter,
    time::{Duration, Instant},
};

/// Information about the latency of a [`Shard`]'s websocket connection.
///
/// May be obtained via [`Shard::latency`].
///
/// [`Shard`]: super::Shard
/// [`Shard::latency`]: super::Shard::latency
#[derive(Clone, Debug)]
pub struct Latency {
    /// Total number of heartbeat acknowledgements that have been received
    /// during the session.
    heartbeats: u32,
    /// When the last heartbeat acknowledgement was received.
    received: Option<Instant>,
    /// Most recent latencies between sending a heartbeat and receiving an
    /// acknowledgement.
    recent: [u64; Self::RECENT_LEN],
    /// When the last heartbeat was sent.
    sent: Option<Instant>,
    /// Combined latencies of all heartbeats, used in conjunction with
    /// [`heartbeats`] to determine the average latency.
    ///
    /// [`heartbeats`]: Self::heartbeats
    total_time: u64,
}

impl Latency {
    /// Maximum number of recent latencies to store.
    const RECENT_LEN: usize = 5;

    /// Create a new instance for tracking shard latency.
    pub(crate) const fn new() -> Self {
        Self {
            heartbeats: 0,
            received: None,
            recent: [0; Self::RECENT_LEN],
            sent: None,
            total_time: 0,
        }
    }

    /// The average time it took to receive an acknowledgement for every
    /// heartbeat sent over the duration of the session.
    ///
    /// For example, a reasonable value for this may be between 10 to 100
    /// milliseconds depending on the network connection and physical location.
    ///
    /// # Note
    ///
    /// If this is None, the shard has not received a heartbeat yet.
    pub const fn average(&self) -> Option<Duration> {
        Duration::from_millis(self.total_time).checked_div(self.heartbeats)
    }

    /// The total number of heartbeats that have been sent during this session.
    pub const fn heartbeats(&self) -> u32 {
        self.heartbeats
    }

    /// The 5 most recent latency times.
    ///
    /// Index 0 is the oldest, 4 is the most recent.
    pub fn recent(&self) -> RecentLatencyIter<'_> {
        RecentLatencyIter::new(&self.recent)
    }

    /// When the last heartbeat acknowledgement was received.
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

        if let Some(duration) = self.sent.map(|instant| instant.elapsed()) {
            let millis = if let Ok(millis) = duration.as_millis().try_into() {
                millis
            } else {
                tracing::error!(duration = ?duration, "milliseconds is more than u64");

                return;
            };

            self.total_time += millis;
            self.recent.rotate_right(1);
            self.recent[0] = millis;
        }
    }

    /// Track that a heartbeat acknowledgement was received.
    ///
    /// The current time will be stored to be used in [`track_received`].
    pub(crate) fn track_sent(&mut self) {
        self.received = None;
        self.sent = Some(Instant::now());
    }
}

/// Iterator over the most recent latencies.
#[derive(Debug)]
pub struct RecentLatencyIter<'a> {
    /// Inner protected iterator over the raw latency numbers.
    inner: Iter<'a, u64>,
}

impl<'a> RecentLatencyIter<'a> {
    /// Create a new iterator over the recent latencies.
    fn new(recent: &'a [u64; Latency::RECENT_LEN]) -> Self {
        Self {
            inner: recent.iter(),
        }
    }
}

impl DoubleEndedIterator for RecentLatencyIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().copied().map(Duration::from_millis)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth_back(n).copied().map(Duration::from_millis)
    }
}

impl ExactSizeIterator for RecentLatencyIter<'_> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl FusedIterator for RecentLatencyIter<'_> {}

impl Iterator for RecentLatencyIter<'_> {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().copied().map(Duration::from_millis)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth(n).copied().map(Duration::from_millis)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use super::{Latency, RecentLatencyIter};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, iter::FusedIterator, time::Duration};

    assert_impl_all!(Latency: Clone, Debug, Send, Sync);
    assert_impl_all!(
        RecentLatencyIter<'_>: Debug,
        DoubleEndedIterator,
        ExactSizeIterator,
        FusedIterator,
        Iterator,
        Send,
        Sync
    );

    const fn latency() -> Latency {
        Latency {
            heartbeats: 17,
            received: None,
            recent: [20, 25, 30, 35, 40],
            sent: None,
            total_time: 510,
        }
    }

    #[test]
    fn test_latency() {
        let latency = latency();
        assert_eq!(latency.average(), Some(Duration::from_millis(30)));
        assert_eq!(latency.heartbeats(), 17);
        assert!(latency.received().is_none());
        assert!(latency.sent().is_none());
    }

    #[test]
    fn test_recent_latency_iter() {
        let latency = latency();
        let mut iter = latency.recent();
        assert_eq!(iter.len(), Latency::RECENT_LEN);
        assert_eq!(
            iter.size_hint(),
            (Latency::RECENT_LEN, Some(Latency::RECENT_LEN))
        );
        assert_eq!(iter.next(), Some(Duration::from_millis(20)));
        assert_eq!(iter.next_back(), Some(Duration::from_millis(40)));
        assert_eq!(iter.next(), Some(Duration::from_millis(25)));
        assert_eq!(iter.next(), Some(Duration::from_millis(30)));
        assert_eq!(iter.next_back(), Some(Duration::from_millis(35)));
        assert!(iter.next().is_none());
        assert!(iter.next_back().is_none());
    }
}
