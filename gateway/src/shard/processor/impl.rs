use super::{
    super::{
        config::Config,
        json::{self, GatewayEventParsingError},
        stage::Stage,
        ShardStream,
    },
    emitter::{EmitJsonError, Emitter},
    inflater::Inflater,
    session::{Session, SessionSendError},
    socket_forwarder::SocketForwarder,
};
use crate::{event::EventTypeFlags, listener::Listeners};
use async_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Error as TungsteniteError, Message,
};
use flate2::DecompressError;
use futures_channel::mpsc::{TrySendError, UnboundedReceiver};
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    env::consts::OS,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    str::{self, Utf8Error},
    sync::{atomic::Ordering, Arc},
    time::Duration,
};
use tokio::sync::watch::{
    channel as watch_channel, Receiver as WatchReceiver, Sender as WatchSender,
};
use twilight_model::gateway::{
    event::{
        shard::{Connected, Connecting, Disconnected, Identifying, Reconnecting, Resuming},
        DispatchEvent, Event, GatewayEvent, GatewayEventDeserializer,
    },
    payload::{
        identify::{Identify, IdentifyInfo, IdentifyProperties},
        resume::Resume,
        Ready,
    },
    Intents, OpCode,
};
use url::{ParseError as UrlParseError, Url};

/// Connecting to the gateway failed.
#[derive(Debug)]
#[non_exhaustive]
pub enum ConnectingError {
    Establishing { source: TungsteniteError },
    ParsingUrl { source: UrlParseError, url: String },
}

impl Display for ConnectingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Establishing { source } => Display::fmt(source, f),
            Self::ParsingUrl { source, url } => f.write_fmt(format_args!(
                "the gateway url `{}` is invalid: {}",
                url, source,
            )),
        }
    }
}

impl Error for ConnectingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Establishing { source } => Some(source),
            Self::ParsingUrl { source, .. } => Some(source),
        }
    }
}

#[derive(Debug)]
enum ProcessError {
    /// Provided event type and/or opcode combination doesn't match a known
    /// event type flag.
    EventTypeUnknown {
        /// Received dispatch event type.
        event_type: Option<String>,
        /// Received opcode.
        op: u8,
    },
    /// There was an error parsing a GatewayEvent payload.
    ParsingPayload {
        /// Reason for the error.
        source: GatewayEventParsingError,
    },
    /// The binary payload received from Discord wasn't validly encoded as
    /// UTF-8.
    PayloadNotUtf8 {
        /// Source error when converting to a UTF-8 valid string.
        source: Utf8Error,
    },
    /// A close message tried to be sent but the receiving half was dropped.
    /// This typically means that the shard is shutdown.
    SendingClose {
        /// Reason for the error.
        source: TrySendError<Message>,
    },
    /// The sequence was missing from the payload.
    SequenceMissing,
    /// Sending a message over the session was unsuccessful.
    SessionSend {
        /// Reason for the error.
        source: SessionSendError,
    },
}

impl ProcessError {
    fn fatal(&self) -> bool {
        matches!(self, Self::SendingClose { .. } | Self::SessionSend { .. })
    }
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::EventTypeUnknown { event_type, op } => f.write_fmt(format_args!(
                "provided event type ({:?})/op ({}) pair is unknown",
                event_type, op,
            )),
            Self::ParsingPayload { source } => Display::fmt(source, f),
            Self::PayloadNotUtf8 { .. } => {
                f.write_str("the payload from Discord wasn't UTF-8 valid")
            }
            Self::SendingClose { source } => Display::fmt(source, f),
            Self::SequenceMissing => f.write_str("sequence missing from payload"),
            Self::SessionSend { source } => Display::fmt(source, f),
        }
    }
}

impl Error for ProcessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParsingPayload { source } => Some(source),
            Self::PayloadNotUtf8 { source } => Some(source),
            Self::SendingClose { source } => Some(source),
            Self::SessionSend { source } => Some(source),
            Self::EventTypeUnknown { .. } | Self::SequenceMissing => None,
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
enum ReceivingEventError {
    /// Provided authorization token is invalid.
    AuthorizationInvalid { shard_id: u64, token: String },
    /// Decompressing a frame from Discord failed.
    Decompressing {
        /// Reason for the error.
        source: DecompressError,
    },
    /// The event stream has ended, this is recoverable by resuming.
    EventStreamEnded,
    /// Current user isn't allowed to use at least one of the configured
    /// intents.
    ///
    /// The intents are provided.
    IntentsDisallowed {
        /// The configured intents for the shard.
        intents: Intents,
        /// The ID of the shard.
        shard_id: u64,
    },
    /// Configured intents aren't supported by Discord's gateway.
    ///
    /// The intents are provided.
    IntentsInvalid {
        /// Configured intents for the shard.
        intents: Intents,
        /// ID of the shard.
        shard_id: u64,
    },
}

impl ReceivingEventError {
    fn fatal(&self) -> bool {
        matches!(
            self,
            ReceivingEventError::AuthorizationInvalid { .. }
            | ReceivingEventError::IntentsDisallowed { .. }
            | ReceivingEventError::IntentsInvalid { .. }
        )
    }

    fn reconnectable(&self) -> bool {
        matches!(self, ReceivingEventError::Decompressing { .. })
    }

    fn resumable(&self) -> bool {
        matches!(self, ReceivingEventError::EventStreamEnded)
    }
}

impl Display for ReceivingEventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::AuthorizationInvalid { shard_id, .. } => f.write_fmt(format_args!(
                "the authorization token for shard {} is invalid",
                shard_id
            )),
            Self::Decompressing { .. } => f.write_str("a frame could not be decompressed"),
            Self::IntentsDisallowed { intents, shard_id } => f.write_fmt(format_args!(
                "at least one of the intents ({:?}) for shard {} are disallowed",
                intents, shard_id
            )),
            Self::IntentsInvalid { intents, shard_id } => f.write_fmt(format_args!(
                "at least one of the intents ({:?}) for shard {} are invalid",
                intents, shard_id
            )),
            Self::EventStreamEnded => f.write_str("event stream from gateway ended"),
        }
    }
}

impl Error for ReceivingEventError {}

#[derive(Deserialize)]
struct ReadyMinimal {
    d: Ready,
}

/// Runs in the background and processes incoming events, and then broadcasts
/// to all listeners.
#[derive(Debug)]
pub struct ShardProcessor {
    pub config: Arc<Config>,
    pub emitter: Emitter,
    pub properties: IdentifyProperties,
    pub rx: UnboundedReceiver<Message>,
    pub session: Arc<Session>,
    inflater: Inflater,
    url: Box<str>,
    resume: Option<(u64, Box<str>)>,
    wtx: WatchSender<Arc<Session>>,
}

impl ShardProcessor {
    /// Gateway version to use in the URL to connect to the gateway.
    const GATEWAY_VERSION: u64 = 6;

    pub async fn new(
        config: Arc<Config>,
        mut url: String,
        listeners: Listeners<Event>,
    ) -> Result<(Self, WatchReceiver<Arc<Session>>), ConnectingError> {
        //if we got resume info we don't need to wait
        let shard_id = config.shard();
        let resumable = config.sequence.is_some() && config.session_id.is_some();
        if !resumable {
            tracing::debug!("shard {:?} is not resumable", shard_id);
            tracing::debug!("shard {:?} queued", shard_id);
            config.queue.request(shard_id).await;
            tracing::debug!("shard {:?} finished queue", config.shard());
        }

        let properties = IdentifyProperties::new("twilight.rs", "twilight.rs", OS, "", "");

        url.push_str("?v=8&compress=zlib-stream");

        let emitter = Emitter::new(listeners);
        emitter.event(Event::ShardConnecting(Connecting {
            gateway: url.clone(),
            shard_id: config.shard()[0],
        }));
        let stream = Self::connect(&url).await?;
        let (forwarder, rx, tx) = SocketForwarder::new(stream);
        tokio::spawn(async move {
            forwarder.run().await;
        });

        let session = Arc::new(Session::new(tx));
        if resumable {
            session.set_id(config.session_id.clone().unwrap());
            session
                .seq
                .store(config.sequence.unwrap(), Ordering::Relaxed)
        }

        let (wtx, wrx) = watch_channel(Arc::clone(&session));

        let mut processor = Self {
            config,
            emitter,
            properties,
            rx,
            session,
            inflater: Inflater::new(shard_id),
            url: url.into_boxed_str(),
            resume: None,
            wtx,
        };

        if resumable {
            tracing::debug!("resuming shard {:?}", shard_id);
            processor.resume().await;
        }

        Ok((processor, wrx))
    }

    pub async fn run(mut self) {
        loop {
            match self.next_payload().await {
                Ok(v) => v,
                Err(source) => {
                    tracing::warn!("{}", source);

                    if source.fatal() {
                        break;
                    }

                    if source.reconnectable() {
                        self.reconnect().await;
                    }

                    if source.resumable() {
                        self.resume().await;
                    }

                    continue;
                }
            };

            if let Err(source) = self.process().await {
                tracing::warn!(
                    shard_id = self.config.shard()[0],
                    shard_total = self.config.shard()[1],
                    "processing incoming event failed: {:?}",
                    source,
                );

                if source.fatal() {
                    tracing::debug!("error processing event; reconnecting");

                    self.reconnect().await;
                }
            }
        }

        self.emitter.into_listeners().remove_all();
    }

    async fn process(&mut self) -> Result<(), ProcessError> {
        let (op, seq, event_type) = {
            let json = str::from_utf8_mut(self.inflater.buffer_mut())
                .map_err(|source| ProcessError::PayloadNotUtf8 { source })?;

            tracing::trace!(%json, "Received JSON");
            let emitter = self.emitter.clone();

            let (op, seq, event_type) =
                if let Some(deserializer) = GatewayEventDeserializer::from_json(json) {
                    let (op, seq, event_type) = deserializer.into_parts();

                    // Unfortunately lifetimes and mutability requirements
                    // conflict here if we return an immutable reference to the
                    // event type, so we're going to have to take ownership of
                    // this if we don't want to do anything too dangerous. It
                    // should be a good trade-off either way.
                    (op, seq, event_type.map(ToOwned::to_owned))
                } else {
                    tracing::warn!(
                        json = ?self.inflater.buffer_ref(),
                        shard_id = self.config.shard()[0],
                        shard_total = self.config.shard()[1],
                        seq = self.session.seq(),
                        stage = ?self.session.stage(),
                        "received payload without opcode",
                    );

                    return Err(ProcessError::ParsingPayload {
                        source: GatewayEventParsingError::PayloadInvalid,
                    });
                };

            // We can do a few little optimisation tricks here. For the
            // "heartbeat ack" and "reconnect" opcodes we can construct
            // the gateway events without needing to go through a serde
            // context.
            //
            // Additionally, the processor cares about the "resumed"
            // dispatch event type, which has no payload and can be constructed.
            //
            // This might not be shaving off entire milliseconds for these few
            // events each time, but it certainly adds up.
            if matches!(op, 1 | 7 | 9 | 10 | 11) {
                // Have to use an if statement here if we want to use the OpCode
                // enum, since matching with repr values isn't allowed.
                let gateway_event = if op == OpCode::HeartbeatAck as u8 {
                    GatewayEvent::HeartbeatAck
                } else if op == OpCode::Reconnect as u8 {
                    GatewayEvent::Reconnect
                } else {
                    json::parse_gateway_event(op, seq, event_type.as_deref(), json)
                        .map_err(|source| ProcessError::ParsingPayload { source })?
                };

                self.process_gateway_event(&gateway_event).await?;
                emitter.event(Event::from(gateway_event));

                if let Some(seq) = seq {
                    self.session.set_seq(seq);
                }

                return Ok(());
            }

            let seq = seq.ok_or(ProcessError::SequenceMissing)?;

            if event_type.as_deref() == Some("RESUMED") {
                self.process_resumed(seq);

                if emitter.wants(EventTypeFlags::RESUMED) {
                    let gateway_event =
                        GatewayEvent::Dispatch(seq, Box::new(DispatchEvent::Resumed));

                    emitter.event(Event::from(gateway_event));
                }

                return Ok(());
            } else if event_type.as_deref() == Some("READY") {
                let ready = json::from_slice::<ReadyMinimal>(self.inflater.buffer_mut()).map_err(
                    |source| ProcessError::ParsingPayload {
                        source: GatewayEventParsingError::Deserializing { source },
                    },
                )?;
                self.process_ready(&ready.d);
                emitter.event(Event::Ready(Box::new(ready.d)));

                return Ok(());
            }

            self.session.set_seq(seq);

            (op, seq, event_type)
        };

        // We already know from earlier that the payload is valid UTF-8, so we
        // can skip having to re-validate here since it hasn't been mutated.
        let json = unsafe { str::from_utf8_unchecked_mut(self.inflater.buffer_mut()) };

        self.emitter
            .json(op, Some(seq), event_type.as_deref(), json)
            .map_err(|source| match source {
                EmitJsonError::Parsing { source } => ProcessError::ParsingPayload { source },
                EmitJsonError::EventTypeUnknown { event_type, op } => {
                    ProcessError::EventTypeUnknown { event_type, op }
                }
            })
    }

    fn process_ready(&mut self, ready: &Ready) {
        #[cfg(feature = "metrics")]
        metrics::counter!("GatewayEvent", 1, "GatewayEvent" => "Dispatch");

        self.session.set_stage(Stage::Connected);
        self.session
            .set_id(ready.session_id.clone().into_boxed_str());

        self.emitter.event(Event::ShardConnected(Connected {
            heartbeat_interval: self.session.heartbeat_interval(),
            shard_id: self.config.shard()[0],
        }));
    }

    fn process_resumed(&self, seq: u64) {
        #[cfg(feature = "metrics")]
        metrics::counter!("GatewayEvent", 1, "GatewayEvent" => "Dispatch");

        self.session.set_seq(seq);
        self.session.set_stage(Stage::Connected);
        self.emitter.event(Event::ShardConnected(Connected {
            heartbeat_interval: self.session.heartbeat_interval(),
            shard_id: self.config.shard()[0],
        }));
        self.session.heartbeats.receive();
    }

    async fn process_gateway_event(&mut self, event: &GatewayEvent) -> Result<(), ProcessError> {
        match event {
            GatewayEvent::Dispatch(_, _) => unreachable!("dispatch events separately handled"),
            GatewayEvent::Heartbeat(seq) => self.process_heartbeat(*seq).await,
            GatewayEvent::Hello(interval) => self.process_hello(*interval).await?,
            GatewayEvent::HeartbeatAck => self.process_heartbeat_ack(),
            GatewayEvent::InvalidateSession(resumable) => {
                self.process_invalidate_session(*resumable).await
            }
            GatewayEvent::Reconnect => self.process_reconnect().await?,
        }

        Ok(())
    }

    fn process_heartbeat_ack(&self) {
        #[cfg(feature = "metrics")]
        metrics::counter!("GatewayEvent", 1, "GatewayEvent" => "HeartbeatAck");

        self.session.heartbeats.receive();
    }

    async fn process_heartbeat(&mut self, seq: u64) {
        #[cfg(feature = "metrics")]
        metrics::counter!("GatewayEvent", 1, "GatewayEvent" => "Heartbeat");

        if seq > self.session.seq() + 1 {
            self.resume().await;
        }

        if let Err(err) = self.session.heartbeat() {
            tracing::warn!("error sending heartbeat; reconnecting: {}", err);

            self.reconnect().await;
        }
    }

    async fn process_hello(&mut self, interval: u64) -> Result<(), ProcessError> {
        #[cfg(feature = "metrics")]
        metrics::counter!("GatewayEvent", 1, "GatewayEvent" => "Hello");

        tracing::debug!("got hello with interval {}", interval);

        if self.session.stage() == Stage::Resuming && self.resume.is_some() {
            // Safe to unwrap so here as we have just checked that
            // it is some.
            let (seq, id) = self.resume.take().unwrap();
            tracing::debug!("resuming with sequence {}, session id {}", seq, id);
            let payload = Resume::new(seq, id.clone().into_string(), self.config.token());

            // Set id so it is correct for next resume.
            self.session.set_id(id);

            if interval > 0 {
                self.session.set_heartbeat_interval(interval);
                self.session.start_heartbeater();
            }

            self.send(payload)
                .await
                .map_err(|source| ProcessError::SessionSend { source })?;
        } else {
            self.session.set_stage(Stage::Identifying);

            if interval > 0 {
                self.session.set_heartbeat_interval(interval);
                self.session.start_heartbeater();
            }

            self.identify()
                .await
                .map_err(|source| ProcessError::SessionSend { source })?;
        }

        Ok(())
    }

    async fn process_invalidate_session(&mut self, resumable: bool) {
        if resumable {
            #[cfg(feature = "metrics")]
            metrics::counter!("GatewayEvent", 1, "GatewayEvent" => "InvalidateSessionTrue");

            tracing::debug!("got request to resume the session");
            self.resume().await;
        } else {
            #[cfg(feature = "metrics")]
            metrics::counter!("GatewayEvent", 1, "GatewayEvent" => "InvalidateSessionFalse");

            tracing::debug!("got request to invalidate the session and reconnect");
            self.reconnect().await;
        }
    }

    async fn process_reconnect(&mut self) -> Result<(), ProcessError> {
        #[cfg(feature = "metrics")]
        metrics::counter!("GatewayEvent", 1, "GatewayEvent" => "Reconnect");

        tracing::debug!("got request to reconnect");
        let frame = CloseFrame {
            code: CloseCode::Restart,
            reason: Cow::Borrowed("Reconnecting"),
        };
        self.session
            .close(Some(frame))
            .map_err(|source| ProcessError::SendingClose { source })?;
        self.resume().await;

        Ok(())
    }

    pub async fn send(&mut self, payload: impl Serialize) -> Result<(), SessionSendError> {
        if let Err(source) = self.session.send(payload) {
            tracing::warn!("sending message failed: {:?}", source);

            if matches!(source, SessionSendError::Sending { .. }) {
                self.reconnect().await;
            }

            return Err(source);
        }

        Ok(())
    }

    /// Wait for the next available complete event.
    ///
    /// When this resolves, the event buffer will be available in the inflater.
    /// Calling this again will clear the inflater's buffer.
    ///
    /// # Errors
    ///
    /// Returns [`ReceivingEventError::AuthorizationInvalid`] if the provided authorization
    /// is invalid.
    async fn next_payload(&mut self) -> Result<(), ReceivingEventError> {
        self.inflater.clear();

        loop {
            // Returns None when the socket forwarder has ended, meaning the
            // connection was dropped.
            let mut msg = self
                .rx
                .next()
                .await
                .ok_or(ReceivingEventError::EventStreamEnded)?;

            if self.handle_message(&mut msg).await? {
                return Ok(());
            }
        }
    }

    /// Handle a received websocket message, returning whether a decompressed
    /// message buffer is available in the inflater.
    ///
    /// If the message is a binary payload, then the bytes are added to the
    /// inflater buffer. If the inflater determines that a message is ready,
    /// then `true` is returned. The buffer can then be accessed via
    /// `self.inflater.buffer_ref()` or `buffer_mut()`.
    ///
    /// If a close message is received then an error may be returned if fatal,
    /// or the connection may be resumed.
    ///
    /// If a ping or pong are received, then they are ignored.
    ///
    /// Text messages aren't sent by Discord, so they are left unhandled.
    async fn handle_message<'a>(
        &'a mut self,
        msg: &'a mut Message,
    ) -> Result<bool, ReceivingEventError> {
        match msg {
            Message::Binary(bin) => {
                self.inflater.extend(&bin[..]);

                let bytes = match self.inflater.msg() {
                    Ok(Some(bytes)) => bytes,
                    Ok(None) => return Ok(false),
                    Err(source) => return Err(ReceivingEventError::Decompressing { source }),
                };

                self.emitter.bytes(bytes);

                Ok(true)
            }
            Message::Close(close_frame) => {
                self.handle_close(close_frame.as_ref()).await?;

                Ok(false)
            }
            // Discord doesn't appear to send Text messages, so we can ignore
            // these.
            Message::Ping(_) | Message::Pong(_) | Message::Text(_) => Ok(false),
        }
    }

    async fn handle_close(
        &mut self,
        close_frame: Option<&CloseFrame<'_>>,
    ) -> Result<(), ReceivingEventError> {
        tracing::info!("got close code: {:?}", close_frame);

        self.emitter.event(Event::ShardDisconnected(Disconnected {
            code: close_frame.as_ref().map(|frame| frame.code.into()),
            reason: close_frame
                .as_ref()
                .map(|frame| frame.reason.clone().into()),
            shard_id: self.config.shard()[0],
        }));

        if let Some(close_frame) = close_frame {
            match close_frame.code {
                CloseCode::Library(4004) => {
                    return Err(ReceivingEventError::AuthorizationInvalid {
                        shard_id: self.config.shard()[0],
                        token: self.config.token().to_owned(),
                    });
                }
                CloseCode::Library(4013) => {
                    return Err(ReceivingEventError::IntentsInvalid {
                        intents: self.config.intents(),
                        shard_id: self.config.shard()[0],
                    });
                }
                CloseCode::Library(4014) => {
                    return Err(ReceivingEventError::IntentsDisallowed {
                        intents: self.config.intents(),
                        shard_id: self.config.shard()[0],
                    });
                }
                _ => {}
            }
        }

        self.resume().await;

        Ok(())
    }

    async fn connect(url: &str) -> Result<ShardStream, ConnectingError> {
        let url = Url::parse(url).map_err(|source| ConnectingError::ParsingUrl {
            source,
            url: url.to_owned(),
        })?;

        let (stream, _) = async_tungstenite::tokio::connect_async(url)
            .await
            .map_err(|source| ConnectingError::Establishing { source })?;

        tracing::debug!("Shook hands with remote");

        Ok(stream)
    }

    /// Identifies with the gateway to create a new session.
    async fn identify(&mut self) -> Result<(), SessionSendError> {
        self.session.set_stage(Stage::Identifying);

        let identify = Identify::new(IdentifyInfo {
            compress: false,
            large_threshold: self.config.large_threshold(),
            intents: self.config.intents(),
            properties: self.properties.clone(),
            shard: Some(self.config.shard()),
            presence: self.config.presence().cloned(),
            token: self.config.token().to_owned(),
            v: Self::GATEWAY_VERSION,
        });
        self.emitter.event(Event::ShardIdentifying(Identifying {
            shard_id: self.config.shard()[0],
            shard_total: self.config.shard()[1],
        }));

        self.send(identify).await
    }

    /// Perform a full reconnect to the gateway, instantiating a new session.
    async fn reconnect(&mut self) {
        tracing::info!("reconnection started");

        let mut wait = Duration::from_secs(1);

        loop {
            tracing::debug!(
                shard_id = self.config.shard()[0],
                shard_total = self.config.shard()[1],
                wait_in_seconds = wait.as_secs(),
                "waiting before attempting a reconnect",
            );
            tokio::time::sleep(wait).await;

            // Await allowance when doing a full reconnect.
            self.config.queue.request(self.config.shard()).await;

            self.emitter.event(Event::ShardReconnecting(Reconnecting {
                shard_id: self.config.shard()[0],
            }));

            let stream = match Self::connect(&self.url).await {
                Ok(s) => s,
                Err(why) => {
                    tracing::warn!("reconnecting failed: {:?}", why);

                    if wait < Duration::from_secs(128) {
                        wait *= 2;
                    }

                    continue;
                }
            };

            self.set_session(stream, Stage::Connected);

            break;
        }

        self.emitter.event(Event::ShardConnecting(Connecting {
            gateway: self.url.clone().into_string(),
            shard_id: self.config.shard()[0],
        }));
    }

    /// Resume a session if possible, defaulting to instantiating a new
    /// connection.
    async fn resume(&mut self) {
        tracing::info!("resuming shard {:?}", self.config.shard());
        self.session.set_stage(Stage::Resuming);
        self.session.stop_heartbeater();

        let seq = self.session.seq();

        let id = if let Some(id) = self.session.id() {
            id
        } else {
            tracing::info!("session id unavailable, reconnecting");
            self.reconnect().await;
            return;
        };

        self.resume = Some((seq, id));

        if let Err(why) = self.try_resume().await {
            tracing::warn!(
                seq = seq,
                session_id = ?self.session.id(),
                shard_id = self.config.shard()[0],
                "failed to resume session: {:?}",
                why,
            );

            self.reconnect().await;
        }
    }

    /// Attempt to resume a session.
    async fn try_resume(&mut self) -> Result<(), ConnectingError> {
        self.emitter.event(Event::ShardResuming(Resuming {
            seq: self.session.seq(),
            shard_id: self.config.shard()[0],
        }));

        let stream = Self::connect(&self.url).await?;

        self.set_session(stream, Stage::Resuming);

        Ok(())
    }

    /// Set the session with a new connection.
    ///
    /// Set the session details and create and run a new socket forwarder for a
    /// new websocket connection.
    fn set_session(&mut self, stream: ShardStream, stage: Stage) {
        let (forwarder, rx, tx) = SocketForwarder::new(stream);

        tokio::spawn(forwarder.run());

        self.rx = rx;
        self.session = Arc::new(Session::new(tx));

        if let Err(why) = self.wtx.send(Arc::clone(&self.session)) {
            tracing::error!("failed to broadcast new session: {:?}", why);
        }

        self.session.set_stage(stage);
        self.inflater.reset();
    }
}
