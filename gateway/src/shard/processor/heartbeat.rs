use super::{
    super::json,
    session::{SessionSendError, SessionSendErrorType},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    convert::TryInto,
    sync::{
        atomic::{AtomicU32, AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
use twilight_model::gateway::payload::outgoing::Heartbeat;

/// Information about the latency of a [`Shard`]'s websocket connection.
///
/// This is obtained through [`Shard::info`].
///
/// [`Shard`]: crate::shard::Shard
/// [`Shard::info`]: crate::shard::Shard::info
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Latency {
    average: Option<Duration>,
    heartbeats: u32,
    recent: VecDeque<Duration>,
    #[serde(skip)]
    received: Option<Instant>,
    #[serde(skip)]
    sent: Option<Instant>,
}

impl Latency {
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
        self.average
    }

    /// The total number of heartbeats that have been sent during this session.
    pub const fn heartbeats(&self) -> u32 {
        self.heartbeats
    }

    /// The 5 most recent latency times.
    ///
    /// Index 0 is the oldest, 4 is the most recent.
    pub const fn recent(&self) -> &VecDeque<Duration> {
        &self.recent
    }

    /// When the last heartbeat acknowledgement was received.
    pub const fn received(&self) -> Option<Instant> {
        self.received
    }

    /// When the last heartbeat was sent.
    pub const fn sent(&self) -> Option<Instant> {
        self.sent
    }
}

#[derive(Debug)]
pub struct Heartbeats {
    received: Mutex<Option<Instant>>,
    recent: Mutex<VecDeque<u64>>,
    sent: Mutex<Option<Instant>>,
    total_iterations: AtomicU32,
    total_time: AtomicU64,
}

impl Heartbeats {
    pub fn latency(&self) -> Latency {
        let iterations = self.total_iterations();
        let recent = self
            .recent
            .lock()
            .expect("recent poisoned")
            .iter()
            .copied()
            .map(Duration::from_millis)
            .collect();

        Latency {
            average: self.total_time().checked_div(iterations),
            heartbeats: iterations,
            recent,
            received: self.received(),
            sent: self.sent(),
        }
    }

    pub fn last_acked(&self) -> bool {
        self.received().is_some()
    }

    pub fn receive(&self) {
        self.set_received(Instant::now());

        self.total_iterations.fetch_add(1, Ordering::SeqCst);

        if let Some(dur) = self.sent().map(|s| s.elapsed()) {
            let millis = if let Ok(millis) = dur.as_millis().try_into() {
                millis
            } else {
                #[cfg(feature = "tracing")]
                tracing::error!("duration millis is more than u64: {:?}", dur);

                return;
            };

            self.total_time.fetch_add(millis, Ordering::SeqCst);

            let mut recent = self.recent.lock().expect("recent poisoned");

            if recent.len() == 5 {
                recent.pop_front();
            }

            recent.push_back(millis);
        }
    }

    pub fn send(&self) {
        self.received.lock().expect("received poisoned").take();
        self.sent
            .lock()
            .expect("sent poisoned")
            .replace(Instant::now());
    }

    fn received(&self) -> Option<Instant> {
        *self.received.lock().expect("received poisoned")
    }

    fn set_received(&self, received: Instant) {
        self.received
            .lock()
            .expect("received poisoned")
            .replace(received);
    }

    fn sent(&self) -> Option<Instant> {
        *self.sent.lock().expect("sent poisoned")
    }

    fn total_iterations(&self) -> u32 {
        self.total_iterations.load(Ordering::Relaxed)
    }

    fn total_time(&self) -> Duration {
        Duration::from_millis(self.total_time.load(Ordering::Relaxed))
    }
}

impl Default for Heartbeats {
    fn default() -> Self {
        Self {
            received: Mutex::new(None),
            recent: Mutex::new(VecDeque::with_capacity(5)),
            sent: Mutex::new(None),
            total_iterations: AtomicU32::new(0),
            total_time: AtomicU64::new(0),
        }
    }
}

pub struct Heartbeater {
    heartbeats: Arc<Heartbeats>,
    interval: u64,
    seq: Arc<AtomicU64>,
    tx: UnboundedSender<TungsteniteMessage>,
}

impl Heartbeater {
    pub fn new(
        heartbeats: Arc<Heartbeats>,
        interval: u64,
        seq: Arc<AtomicU64>,
        tx: UnboundedSender<TungsteniteMessage>,
    ) -> Self {
        Self {
            heartbeats,
            interval,
            seq,
            tx,
        }
    }

    pub async fn run(self) {
        if let Err(_source) = self.try_run().await {
            #[cfg(feature = "tracing")]
            tracing::warn!("Error sending heartbeat: {:?}", _source);
        }
    }

    // If there's an issue sending over the channel, then odds are it
    // got disconnected due to the session ending. This task should have
    // *also* become aborted. Log if that's the case, because that's a
    // programmatic error.
    async fn try_run(self) -> Result<(), SessionSendError> {
        let duration = Duration::from_millis(self.interval);

        let mut last = true;

        loop {
            tokio::time::sleep(duration).await;

            // Check if a heartbeat acknowledgement was received.
            //
            // If so, then check if one was received last time.
            //
            // - if so, then mark that we didn't get one this time
            // - if not, then end the heartbeater because something is off
            // (connecting closed?)
            if self.heartbeats.last_acked() {
                last = true;
            } else if last {
                last = false;
            } else {
                return Ok(());
            }

            let seq = self.seq.load(Ordering::Acquire);
            let heartbeat = Heartbeat::new(seq);
            let bytes = json::to_vec(&heartbeat).map_err(|source| SessionSendError {
                kind: SessionSendErrorType::Serializing,
                source: Some(Box::new(source)),
            })?;

            #[cfg(feature = "tracing")]
            tracing::debug!(seq, "sending heartbeat");

            self.tx
                .send(TungsteniteMessage::Binary(bytes))
                .map_err(|source| SessionSendError {
                    kind: SessionSendErrorType::Sending,
                    source: Some(Box::new(source)),
                })?;

            #[cfg(feature = "tracing")]
            tracing::debug!(seq, "sent heartbeat");

            self.heartbeats.send();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Latency;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(Latency: Clone, Debug, Send, Sync);
}
