use super::{
    error::{PayloadSerialization, Result, SendingMessage},
    heartbeat::{Heartbeater, Heartbeats},
    stage::Stage,
};
use dawn_model::gateway::payload::Heartbeat;
use futures_channel::mpsc::UnboundedSender;
use futures_util::{
    future::{self, AbortHandle},
    lock::Mutex,
};
use serde::ser::Serialize;
use snafu::ResultExt;
use std::{
    convert::TryFrom,
    sync::{
        atomic::{AtomicU64, AtomicU8, Ordering},
        Arc,
    },
};
use tokio_executor::{DefaultExecutor, Executor};
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

#[derive(Debug)]
pub struct Session {
    // Needs to be Arc so it can be cloned in the `Drop` impl when spawned on
    // the runtime.
    pub heartbeater_handle: Arc<Mutex<Option<AbortHandle>>>,
    pub heartbeats: Arc<Heartbeats>,
    pub heartbeat_interval: AtomicU64,
    pub id: Mutex<Option<String>>,
    pub seq: Arc<AtomicU64>,
    pub stage: AtomicU8,
    pub tx: UnboundedSender<TungsteniteMessage>,
}

impl Session {
    pub fn new(tx: UnboundedSender<TungsteniteMessage>) -> Self {
        Self {
            heartbeater_handle: Arc::new(Mutex::new(None)),
            heartbeats: Arc::new(Heartbeats::default()),
            heartbeat_interval: AtomicU64::new(0),
            id: Mutex::new(None),
            seq: Arc::new(AtomicU64::new(0)),
            stage: AtomicU8::new(Stage::default() as u8),
            tx,
        }
    }

    /// Sends a payload as a message over the socket.
    ///
    /// # Errors
    ///
    /// Returns [`Error::PayloadSerialization`] when there is an error
    /// serializing the payload into an acceptable format.
    ///
    /// Returns [`Error::SendingMessage`] when the receiving channel has hung
    /// up. This will only happen when the shard has either not started or has
    /// already shutdown.
    ///
    /// [`Error::PayloadSerialization`]: ../enum.Error.html#variant.PayloadSerialization
    /// [`Error::SendingMessage`]: ../enum.Error.html#variant.SendingMessage
    pub fn send(&self, payload: impl Serialize) -> Result<()> {
        let bytes = serde_json::to_vec(&payload).context(PayloadSerialization)?;

        self.tx
            .unbounded_send(TungsteniteMessage::Binary(bytes))
            .context(SendingMessage)?;

        Ok(())
    }

    pub fn heartbeat_interval(&self) -> u64 {
        self.heartbeat_interval.load(Ordering::Relaxed)
    }

    pub fn set_heartbeat_interval(&self, new_heartbeat_interval: u64) {
        self.heartbeat_interval
            .store(new_heartbeat_interval, Ordering::Release);
    }

    /// Returns the current sequence.
    pub fn seq(&self) -> u64 {
        self.seq.load(Ordering::Relaxed)
    }

    /// Sets the sequence.
    pub fn set_seq(&self, seq: u64) {
        self.seq.store(seq, Ordering::Release);
    }

    /// Returns the current shard stage.
    pub fn stage(&self) -> Stage {
        Stage::try_from(self.stage.load(Ordering::Relaxed)).unwrap_or_default()
    }

    /// Sets the stage.
    pub fn set_stage(&self, stage: Stage) {
        self.stage.store(stage as u8, Ordering::Release);
    }

    pub fn heartbeat(&self) -> Result<()> {
        self.send(Heartbeat::new(self.seq()))
    }

    pub async fn id(&self) -> Option<String> {
        self.id.lock().await.clone()
    }

    pub async fn set_id(&self, new_id: impl Into<String>) {
        self.id.lock().await.replace(new_id.into());
    }

    pub async fn stop_heartbeater(&self) {
        if let Some(handle) = self.heartbeater_handle.lock().await.take() {
            handle.abort();
        }
    }

    pub async fn start_heartbeater(&self) {
        let interval = self.heartbeat_interval();
        let seq = Arc::clone(&self.seq);
        let heartbeats = Arc::clone(&self.heartbeats);

        let heartbeater = Heartbeater::new(heartbeats, interval, seq, self.tx.clone()).run();
        let (fut, handle) = future::abortable(heartbeater);

        tokio_executor::spawn(async {
            let _ = fut.await;
        });

        if let Some(old) = self.heartbeater_handle.lock().await.replace(handle) {
            old.abort();
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let mut executor = DefaultExecutor::current();

        if executor.status().is_ok() {
            let handle = Arc::clone(&self.heartbeater_handle);

            let _ = executor.spawn(Box::pin(async move {
                if let Some(handle) = handle.lock().await.take() {
                    handle.abort();
                }
            }));
        }
    }
}
