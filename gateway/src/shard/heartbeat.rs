use super::error::{PayloadSerialization, Result, SendingMessage};
use dawn_model::gateway::payload::Heartbeat;
use futures_channel::mpsc::UnboundedSender;
use log::{debug, warn};
use snafu::ResultExt;
use std::{
    collections::VecDeque,
    convert::TryInto,
    sync::{
        atomic::{AtomicU32, AtomicU64, Ordering},
        Arc,
        Mutex,
    },
    time::{Duration, Instant},
};
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

/// Information about the latency of a [`Shard`]'s websocket connection.
///
/// This is obtained through [`Shard::info`].
///
/// [`Shard`]: struct.Shard.html
/// [`Shard::info`]: struct.Shard.html#method.info
#[derive(Clone, Debug)]
pub struct Latency {
    average: Duration,
    heartbeats: u32,
    recent: VecDeque<Duration>,
    received: Option<Instant>,
    sent: Option<Instant>,
}

impl Latency {
    /// The average time it took to receive an acknowledgement for every
    /// heartbeat sent over the duration of the session.
    ///
    /// For example, a reasonable value for this may be between 10 to 100
    /// milliseconds depending on the network connection and physical location.
    pub fn average(&self) -> Duration {
        self.average
    }

    /// The total number of heartbeats that have been sent during this session.
    pub fn heartbeats(&self) -> u32 {
        self.heartbeats
    }

    /// The 5 most recent latency times.
    ///
    /// Index 0 is the oldest, 4 is the most recent.
    pub fn recent(&self) -> &VecDeque<Duration> {
        &self.recent
    }

    /// When the last heartbeat acknowledgement was received.
    pub fn received(&self) -> Option<Instant> {
        self.received
    }

    /// When the last heartbeat was sent.
    pub fn sent(&self) -> Option<Instant> {
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
            .expect("failed to open mutex")
            .iter()
            .map(|x| Duration::from_millis(*x))
            .collect();

        Latency {
            average: self.total_time() / iterations,
            heartbeats: iterations,
            recent,
            received: *self.received.lock().expect("failed to open mutex"),
            sent: *self.sent.lock().expect("failed to open mutex"),
        }
    }

    pub fn last_acked(&self) -> bool {
        self.received
            .lock()
            .expect("failed to open mutex")
            .is_some()
    }

    pub fn receive(&self) {
        let now = Instant::now();
        self.received
            .lock()
            .expect("failed to open mutex")
            .replace(now);

        self.total_iterations.fetch_add(1, Ordering::SeqCst);

        if let Some(dur) = self
            .sent
            .lock()
            .expect("failed to open mutex")
            .map(|s| s.elapsed())
        {
            let millis = dur
                .as_millis()
                .try_into()
                .expect("failed to convert millis to u64");

            self.total_time.fetch_add(millis as u64, Ordering::SeqCst);

            let mut recent = self.recent.lock().unwrap();

            if recent.len() == 5 {
                recent.pop_front();
            }

            recent.push_back(millis);
        }
    }

    pub fn send(&self) {
        self.received.lock().unwrap().take();
        self.sent.lock().unwrap().replace(Instant::now());
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
        if let Err(why) = self.run_inner().await {
            warn!("Error sending heartbeat: {:?}", why);
        }
    }

    // If there's an issue sending over the channel, then odds are it
    // got disconnected due to the session ending. This task should have
    // *also* become aborted. Log if that's the case, because that's a
    // programmatic error.
    async fn run_inner(self) -> Result<()> {
        let duration = Duration::from_millis(self.interval);

        let mut last = true;

        loop {
            tokio_timer::delay_for(duration).await;

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
            let bytes = serde_json::to_vec(&heartbeat).context(PayloadSerialization)?;

            debug!("[Heartbeat] Sending heartbeat with seq: {}", seq);
            self.tx
                .unbounded_send(TungsteniteMessage::Binary(bytes))
                .context(SendingMessage)?;
            debug!("[Heartbeat] Sent heartbeat with seq: {}", seq);
            self.heartbeats.send();
        }
    }
}
