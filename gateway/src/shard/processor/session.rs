use super::{
    super::{json, stage::Stage},
    heartbeat::{Heartbeater, Heartbeats},
    throttle::Throttle,
};
use async_tungstenite::tungstenite::{protocol::CloseFrame, Message as TungsteniteMessage};
use futures_channel::mpsc::{TrySendError, UnboundedSender};
use futures_util::{
    future::{self, AbortHandle},
    lock::Mutex,
};
use serde::ser::Serialize;
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    sync::{
        atomic::{AtomicU64, AtomicU8, Ordering},
        Arc, Mutex as MutexSync,
    },
    time::Duration,
};
use twilight_model::gateway::payload::Heartbeat;

#[derive(Debug)]
pub struct SessionSendError {
    pub(super) source: Option<Box<dyn Error + Send + Sync>>,
    pub(super) kind: SessionSendErrorType,
}

impl SessionSendError {
    /// Immutable reference to the type of error that occurred.
    pub fn kind(&self) -> &SessionSendErrorType {
        &self.kind
    }
}

impl Display for SessionSendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            SessionSendErrorType::Serializing => f.write_str("failed to serialize payload as json"),
            SessionSendErrorType::Sending => f.write_str("failed to send message over websocket"),
        }
    }
}

impl Error for SessionSendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`SessionSendError`] that occurred.
#[derive(Debug)]
pub enum SessionSendErrorType {
    Sending,
    Serializing,
}

#[derive(Debug)]
pub struct Session {
    // Needs to be Arc so it can be cloned in the `Drop` impl when spawned on
    // the runtime.
    pub heartbeater_handle: Arc<MutexSync<Option<AbortHandle>>>,
    pub heartbeats: Arc<Heartbeats>,
    pub heartbeat_interval: AtomicU64,
    pub id: MutexSync<Option<Box<str>>>,
    pub seq: Arc<AtomicU64>,
    pub stage: AtomicU8,
    pub tx: UnboundedSender<TungsteniteMessage>,
    pub ratelimit: Mutex<Throttle>,
}

impl Session {
    pub fn new(tx: UnboundedSender<TungsteniteMessage>) -> Self {
        Self {
            heartbeater_handle: Arc::new(MutexSync::new(None)),
            heartbeats: Arc::new(Heartbeats::default()),
            heartbeat_interval: AtomicU64::new(0),
            id: MutexSync::new(None),
            seq: Arc::new(AtomicU64::new(0)),
            stage: AtomicU8::new(Stage::default() as u8),
            tx,
            // 520 instead of 500 to make sure that it can heartbeat.
            ratelimit: Mutex::new(Throttle::new(Duration::from_millis(520))),
        }
    }

    /// Sends a payload as a message over the socket.
    ///
    /// # Errors
    ///
    /// Returns a [`SessionSendErrorType::Serializing`] error type when there is
    /// an error serializing the payload into an acceptable format.
    ///
    /// Returns a [`SessionSendErrorType::Sending`] error type when the
    /// receiving channel has hung up. This will only happen when the shard has
    /// either not started or has already shutdown.
    pub fn send(&self, payload: impl Serialize) -> Result<(), SessionSendError> {
        let bytes = json::to_vec(&payload).map_err(|source| SessionSendError {
            kind: SessionSendErrorType::Serializing,
            source: Some(Box::new(source)),
        })?;

        self.tx
            .unbounded_send(TungsteniteMessage::Binary(bytes))
            .map_err(|source| SessionSendError {
                kind: SessionSendErrorType::Sending,
                source: Some(Box::new(source)),
            })?;

        Ok(())
    }

    pub fn close(
        &self,
        close_frame: Option<CloseFrame<'static>>,
    ) -> Result<(), TrySendError<TungsteniteMessage>> {
        self.tx
            .unbounded_send(TungsteniteMessage::Close(close_frame))
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

    pub fn heartbeat(&self) -> Result<(), SessionSendError> {
        self.send(Heartbeat::new(self.seq()))
    }

    pub fn id(&self) -> Option<Box<str>> {
        self.id.lock().expect("id poisoned").clone()
    }

    pub fn set_id(&self, new_id: Box<str>) {
        self.id.lock().expect("id poisoned").replace(new_id);
    }

    pub fn stop_heartbeater(&self) {
        if let Some(handle) = self
            .heartbeater_handle
            .lock()
            .expect("heartbeater poisoned")
            .take()
        {
            handle.abort();
        }
    }

    pub fn start_heartbeater(&self) {
        let interval = self.heartbeat_interval();
        let seq = Arc::clone(&self.seq);
        let heartbeats = Arc::clone(&self.heartbeats);

        let heartbeater = Heartbeater::new(heartbeats, interval, seq, self.tx.clone()).run();
        let (fut, handle) = future::abortable(heartbeater);

        tokio::spawn(fut);

        if let Some(old) = self
            .heartbeater_handle
            .lock()
            .expect("heartbeater poisoned")
            .replace(handle)
        {
            old.abort();
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.stop_heartbeater();
    }
}
