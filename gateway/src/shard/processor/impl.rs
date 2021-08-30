use super::{
    super::{
        config::Config,
        emitter::{EmitJsonErrorType, Emitter},
        json::{self, GatewayEventParsingError, GatewayEventParsingErrorType},
        stage::Stage,
        ShardStream,
    },
    compression::{self, Compression},
    session::{Session, SessionSendError, SessionSendErrorType},
    socket_forwarder::SocketForwarder,
};
use crate::event::EventTypeFlags;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    env::consts::OS,
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str,
    sync::{atomic::Ordering, Arc},
    time::Duration,
};
use tokio::sync::{
    mpsc::UnboundedReceiver,
    watch::{channel as watch_channel, Receiver as WatchReceiver, Sender as WatchSender},
};
use tokio_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame, WebSocketConfig},
    Message,
};
use twilight_model::gateway::{
    event::{
        shard::{Connected, Connecting, Disconnected, Identifying, Reconnecting, Resuming},
        DispatchEvent, Event, GatewayEvent, GatewayEventDeserializer,
    },
    payload::{
        incoming::Ready,
        outgoing::{
            identify::{Identify, IdentifyInfo, IdentifyProperties},
            resume::Resume,
        },
    },
    Intents, OpCode,
};
use url::Url;

/// Connecting to the gateway failed.
#[derive(Debug)]
pub struct ConnectingError {
    kind: ConnectingErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl ConnectingError {
    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (ConnectingErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for ConnectingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ConnectingErrorType::Establishing => f.write_str("failed to establish the connection"),
            ConnectingErrorType::ParsingUrl { url } => {
                f.write_str("the gateway url `")?;
                f.write_str(url)?;

                f.write_str("` is invalid")
            }
        }
    }
}

impl Error for ConnectingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`ConnectingError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum ConnectingErrorType {
    Establishing,
    ParsingUrl { url: String },
}

#[derive(Debug)]
struct ProcessError {
    kind: ProcessErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl ProcessError {
    const fn fatal(&self) -> bool {
        matches!(
            self.kind,
            ProcessErrorType::SendingClose { .. } | ProcessErrorType::SessionSend { .. }
        )
    }
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ProcessErrorType::EventTypeUnknown { event_type, op } => {
                f.write_str("provided event type (")?;
                Debug::fmt(event_type, f)?;
                f.write_str(")/op (")?;
                Display::fmt(op, f)?;

                f.write_str(") pair is unknown")
            }
            ProcessErrorType::ParsingPayload => f.write_str("payload could not be parsed as json"),
            ProcessErrorType::PayloadNotUtf8 { .. } => {
                f.write_str("the payload from Discord wasn't UTF-8 valid")
            }
            ProcessErrorType::SendingClose => f.write_str("couldn't send close message"),
            ProcessErrorType::SequenceMissing => f.write_str("sequence missing from payload"),
            ProcessErrorType::SessionSend => f.write_str("shard hasn't been started"),
        }
    }
}

impl Error for ProcessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`ProcessError`] that occurred.
#[derive(Debug)]
enum ProcessErrorType {
    /// Provided event type and/or opcode combination doesn't match a known
    /// event type flag.
    EventTypeUnknown {
        /// Received dispatch event type.
        event_type: Option<String>,
        /// Received opcode.
        op: u8,
    },
    /// There was an error parsing a GatewayEvent payload.
    ParsingPayload,
    /// The binary payload received from Discord wasn't validly encoded as
    /// UTF-8.
    PayloadNotUtf8,
    /// A close message tried to be sent but the receiving half was dropped.
    /// This typically means that the shard is shutdown.
    SendingClose,
    /// The sequence was missing from the payload.
    SequenceMissing,
    /// Sending a message over the session was unsuccessful.
    SessionSend,
}

#[derive(Debug)]
pub struct ReceivingEventError {
    pub kind: ReceivingEventErrorType,
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

impl ReceivingEventError {
    const fn fatal(&self) -> bool {
        matches!(
            self.kind,
            ReceivingEventErrorType::AuthorizationInvalid { .. }
                | ReceivingEventErrorType::IntentsDisallowed { .. }
                | ReceivingEventErrorType::IntentsInvalid { .. }
        )
    }

    const fn reconnectable(&self) -> bool {
        matches!(self.kind, ReceivingEventErrorType::Decompressing)
    }

    const fn resumable(&self) -> bool {
        matches!(self.kind, ReceivingEventErrorType::EventStreamEnded)
    }
}

impl Display for ReceivingEventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ReceivingEventErrorType::AuthorizationInvalid { shard_id, .. } => {
                f.write_str("the authorization token for shard ")?;
                Display::fmt(shard_id, f)?;

                f.write_str(" is invalid")
            }
            ReceivingEventErrorType::Decompressing => {
                f.write_str("a frame could not be decompressed")
            }
            ReceivingEventErrorType::IntentsDisallowed { intents, shard_id } => {
                f.write_str("at least one of the intents (")?;
                Debug::fmt(intents, f)?;
                f.write_str(") for shard ")?;
                Display::fmt(shard_id, f)?;

                f.write_str(" are disallowed")
            }
            ReceivingEventErrorType::IntentsInvalid { intents, shard_id } => {
                f.write_str("at least one of the intents (")?;
                Debug::fmt(intents, f)?;
                f.write_str(") for shard ")?;
                Display::fmt(shard_id, f)?;

                f.write_str(" are invalid")
            }
            ReceivingEventErrorType::EventStreamEnded => {
                f.write_str("event stream from gateway ended")
            }
        }
    }
}

impl Error for ReceivingEventError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`ReceivingEventError`] that occurred.
#[derive(Debug)]
pub enum ReceivingEventErrorType {
    /// Provided authorization token is invalid.
    AuthorizationInvalid { shard_id: u64, token: String },
    /// Decompressing a frame from Discord failed.
    Decompressing,
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
    pub rx: UnboundedReceiver<Message>,
    pub session: Arc<Session>,
    compression: Compression,
    url: Box<str>,
    resume: Option<(u64, Box<str>)>,
    wtx: WatchSender<Arc<Session>>,
}

impl ShardProcessor {
    pub async fn new(
        config: Arc<Config>,
        mut url: String,
        emitter: Emitter,
    ) -> Result<(Self, WatchReceiver<Arc<Session>>), ConnectingError> {
        //if we got resume info we don't need to wait
        let shard_id = config.shard();
        let resumable = config.sequence.is_some() && config.session_id.is_some();
        if !resumable {
            #[cfg(feature = "tracing")]
            tracing::debug!("shard {:?} is not resumable", shard_id);
            #[cfg(feature = "tracing")]
            tracing::debug!("shard {:?} queued", shard_id);

            config.queue.request(shard_id).await;

            #[cfg(feature = "tracing")]
            tracing::debug!("shard {:?} finished queue", config.shard());
        }

        url.push_str("?v=9");

        // Discord's documentation states:
        //
        // "Generally, it is a good idea to explicitly pass the gateway version
        // and encoding".
        //
        // <https://discord.com/developers/docs/topics/gateway#connecting-gateway-url-query-string-params>
        url.push_str("&encoding=json");

        compression::add_url_feature(&mut url);

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
            compression: Compression::new(shard_id),
            config,
            emitter,
            rx,
            session,
            url: url.into_boxed_str(),
            resume: None,
            wtx,
        };

        if resumable {
            #[cfg(feature = "tracing")]
            tracing::debug!("resuming shard {:?}", shard_id);

            processor.resume().await;
        }

        Ok((processor, wrx))
    }

    pub async fn run(mut self) {
        loop {
            if let Err(source) = self.next_payload().await {
                #[cfg(feature = "tracing")]
                tracing::warn!("{}", source);

                self.emit_disconnected(None, None).await;

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

            if let Err(source) = self.process().await {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    shard_id = self.config.shard()[0],
                    shard_total = self.config.shard()[1],
                    "processing incoming event failed: {:?}",
                    source,
                );

                if source.fatal() {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("error processing event; reconnecting");
                    self.emit_disconnected(None, None).await;

                    self.reconnect().await;
                }
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn process(&mut self) -> Result<(), ProcessError> {
        let (op, seq, event_type) = {
            let buffer = self.compression.buffer_slice_mut();
            let json = str::from_utf8_mut(buffer).map_err(|source| ProcessError {
                kind: ProcessErrorType::PayloadNotUtf8,
                source: Some(Box::new(source)),
            })?;

            #[cfg(feature = "tracing")]
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
                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        json = ?self.compression.buffer_slice_mut(),
                        shard_id = self.config.shard()[0],
                        shard_total = self.config.shard()[1],
                        seq = self.session.seq(),
                        stage = ?self.session.stage(),
                        "received payload without opcode",
                    );

                    return Err(ProcessError {
                        kind: ProcessErrorType::ParsingPayload,
                        source: Some(Box::new(GatewayEventParsingError {
                            kind: GatewayEventParsingErrorType::PayloadInvalid,
                            source: None,
                        })),
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
                    json::parse_gateway_event(op, seq, event_type.as_deref(), buffer).map_err(
                        |source| ProcessError {
                            kind: ProcessErrorType::ParsingPayload,
                            source: Some(Box::new(source)),
                        },
                    )?
                };

                self.process_gateway_event(&gateway_event).await?;
                emitter.event(Event::from(gateway_event));

                if let Some(seq) = seq {
                    self.session.set_seq(seq);
                }

                return Ok(());
            }

            let seq = seq.ok_or(ProcessError {
                kind: ProcessErrorType::SequenceMissing,
                source: None,
            })?;

            if event_type.as_deref() == Some("RESUMED") {
                self.process_resumed(seq);

                if emitter.wants(EventTypeFlags::RESUMED) {
                    let gateway_event =
                        GatewayEvent::Dispatch(seq, Box::new(DispatchEvent::Resumed));

                    emitter.event(Event::from(gateway_event));
                }

                return Ok(());
            } else if event_type.as_deref() == Some("READY") {
                let ready = json::from_slice::<ReadyMinimal>(self.compression.buffer_slice_mut())
                    .map_err(|source| ProcessError {
                    kind: ProcessErrorType::ParsingPayload,
                    source: Some(Box::new(GatewayEventParsingError {
                        kind: GatewayEventParsingErrorType::Deserializing,
                        source: Some(Box::new(source)),
                    })),
                })?;

                self.process_ready(&ready.d);
                emitter.event(Event::Ready(Box::new(ready.d)));

                return Ok(());
            }

            self.session.set_seq(seq);

            (op, seq, event_type)
        };

        let buffer = self.compression.buffer_slice_mut();

        self.emitter
            .json(op, Some(seq), event_type.as_deref(), buffer)
            .map_err(|source| {
                let (kind, source) = source.into_parts();

                let new_kind = match kind {
                    EmitJsonErrorType::Parsing => ProcessErrorType::ParsingPayload,
                    EmitJsonErrorType::EventTypeUnknown { event_type, op } => {
                        ProcessErrorType::EventTypeUnknown { event_type, op }
                    }
                };

                ProcessError {
                    kind: new_kind,
                    source,
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

        if let Err(_source) = self.session.heartbeat() {
            #[cfg(feature = "tracing")]
            tracing::warn!("error sending heartbeat; reconnecting: {}", _source);

            self.emit_disconnected(None, None).await;

            self.reconnect().await;
        }
    }

    async fn process_hello(&mut self, interval: u64) -> Result<(), ProcessError> {
        #[cfg(feature = "metrics")]
        metrics::counter!("GatewayEvent", 1, "GatewayEvent" => "Hello");

        #[cfg(feature = "tracing")]
        tracing::debug!("got hello with interval {}", interval);

        if self.session.stage() == Stage::Resuming && self.resume.is_some() {
            // Safe to unwrap so here as we have just checked that
            // it is some.
            let (seq, id) = self.resume.take().unwrap();

            #[cfg(feature = "tracing")]
            tracing::debug!("resuming with sequence {}, session id {}", seq, id);

            let payload = Resume::new(seq, id.clone().into_string(), self.config.token());

            // Set id so it is correct for next resume.
            self.session.set_id(id);

            if interval > 0 {
                self.session.set_heartbeat_interval(interval);
                self.session.start_heartbeater();
            }

            self.send(payload).await.map_err(|source| ProcessError {
                kind: ProcessErrorType::SessionSend,
                source: Some(Box::new(source)),
            })?;
        } else {
            self.session.set_stage(Stage::Identifying);

            if interval > 0 {
                self.session.set_heartbeat_interval(interval);
                self.session.start_heartbeater();
            }

            self.identify().await.map_err(|source| ProcessError {
                source: Some(Box::new(source)),
                kind: ProcessErrorType::SessionSend,
            })?;
        }

        Ok(())
    }

    async fn process_invalidate_session(&mut self, resumable: bool) {
        self.emit_disconnected(None, None).await;

        if resumable {
            #[cfg(feature = "metrics")]
            metrics::counter!("GatewayEvent", 1, "GatewayEvent" => "InvalidateSessionTrue");

            #[cfg(feature = "tracing")]
            tracing::debug!("got request to resume the session");

            self.resume().await;
        } else {
            #[cfg(feature = "metrics")]
            metrics::counter!("GatewayEvent", 1, "GatewayEvent" => "InvalidateSessionFalse");

            #[cfg(feature = "tracing")]
            tracing::debug!("got request to invalidate the session and reconnect");

            self.reconnect().await;
        }
    }

    async fn process_reconnect(&mut self) -> Result<(), ProcessError> {
        #[cfg(feature = "metrics")]
        metrics::counter!("GatewayEvent", 1, "GatewayEvent" => "Reconnect");

        #[cfg(feature = "tracing")]
        tracing::debug!("got request to reconnect");

        let frame = CloseFrame {
            code: CloseCode::Restart,
            reason: Cow::Borrowed("Reconnecting"),
        };
        self.session
            .close(Some(frame.clone()))
            .map_err(|source| ProcessError {
                source: Some(Box::new(source)),
                kind: ProcessErrorType::SendingClose,
            })?;
        self.emit_disconnected(Some(frame.code.into()), Some(frame.reason.to_string()))
            .await;
        self.resume().await;

        Ok(())
    }

    pub async fn send(&mut self, payload: impl Serialize) -> Result<(), SessionSendError> {
        if let Err(source) = self.session.send(payload) {
            #[cfg(feature = "tracing")]
            tracing::warn!("sending message failed: {:?}", source);

            if matches!(source.kind(), SessionSendErrorType::Sending { .. }) {
                self.emit_disconnected(None, None).await;

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
    /// Returns a [`ReceivingEventErrorType::AuthorizationInvalid`] error type
    /// if the provided authorization is invalid.
    async fn next_payload(&mut self) -> Result<(), ReceivingEventError> {
        self.compression.clear();

        loop {
            // Returns None when the socket forwarder has ended, meaning the
            // connection was dropped.
            let mut msg = self.rx.recv().await.ok_or(ReceivingEventError {
                kind: ReceivingEventErrorType::EventStreamEnded,
                source: None,
            })?;

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
            Message::Binary(json) => {
                let extended = self.compression.extend_binary(json.as_slice());

                if extended {
                    match self.compression.message_mut() {
                        Ok(Some(bytes)) => self.emitter.bytes(bytes),
                        Ok(None) => return Ok(false),
                        Err(source) => {
                            return Err(ReceivingEventError {
                                kind: ReceivingEventErrorType::Decompressing,
                                source: Some(Box::new(source)),
                            })
                        }
                    };
                }

                Ok(extended)
            }
            Message::Close(close_frame) => {
                self.handle_close(close_frame.as_ref()).await?;

                Ok(false)
            }
            Message::Text(json) => {
                let extended = self.compression.extend_text(json.as_bytes());

                if extended {
                    self.emitter.bytes(json.as_bytes());
                }

                Ok(extended)
            }
            // Discord doesn't appear to send Text messages, so we can ignore
            // these.
            Message::Ping(_) | Message::Pong(_) => Ok(false),
        }
    }

    async fn handle_close(
        &mut self,
        close_frame: Option<&CloseFrame<'_>>,
    ) -> Result<(), ReceivingEventError> {
        #[cfg(feature = "tracing")]
        tracing::info!("got close code: {:?}", close_frame);

        self.emit_disconnected(
            close_frame.map(|c| c.code.into()),
            close_frame.map(|c| c.reason.to_string()),
        )
        .await;

        if let Some(close_frame) = close_frame {
            match close_frame.code {
                CloseCode::Library(4004) => {
                    return Err(ReceivingEventError {
                        kind: ReceivingEventErrorType::AuthorizationInvalid {
                            shard_id: self.config.shard()[0],
                            token: self.config.token().to_owned(),
                        },
                        source: None,
                    });
                }
                CloseCode::Library(4013) => {
                    return Err(ReceivingEventError {
                        kind: ReceivingEventErrorType::IntentsInvalid {
                            intents: self.config.intents(),
                            shard_id: self.config.shard()[0],
                        },
                        source: None,
                    });
                }
                CloseCode::Library(4014) => {
                    return Err(ReceivingEventError {
                        kind: ReceivingEventErrorType::IntentsDisallowed {
                            intents: self.config.intents(),
                            shard_id: self.config.shard()[0],
                        },
                        source: None,
                    });
                }
                _ => {}
            }
        }

        self.resume().await;

        Ok(())
    }

    async fn connect(url: &str) -> Result<ShardStream, ConnectingError> {
        #[allow(rust_2021_incompatible_closure_captures)]
        let url = Url::parse(url).map_err(|source| ConnectingError {
            kind: ConnectingErrorType::ParsingUrl {
                url: url.to_owned(),
            },
            source: Some(Box::new(source)),
        })?;

        // `max_frame_size` and `max_message_queue` limits are disabled because
        // Discord is not a malicious actor.
        //
        // `accept_unmasked_frames` and `max_send_queue` are set to their
        // defaults.
        let config = WebSocketConfig {
            accept_unmasked_frames: false,
            max_frame_size: None,
            max_message_size: None,
            max_send_queue: None,
        };

        let (stream, _) = tokio_tungstenite::connect_async_with_config(url, Some(config))
            .await
            .map_err(|source| ConnectingError {
                kind: ConnectingErrorType::Establishing,
                source: Some(Box::new(source)),
            })?;

        #[cfg(feature = "tracing")]
        tracing::debug!("Shook hands with remote");

        Ok(stream)
    }

    /// Identifies with the gateway to create a new session.
    async fn identify(&mut self) -> Result<(), SessionSendError> {
        self.session.set_stage(Stage::Identifying);

        let properties = self
            .config
            .identify_properties()
            .cloned()
            .unwrap_or_else(default_identify_properties);

        let identify = Identify::new(IdentifyInfo {
            compress: false,
            large_threshold: self.config.large_threshold(),
            intents: self.config.intents(),
            properties,
            shard: Some(self.config.shard()),
            presence: self.config.presence().cloned(),
            token: self.config.token().to_owned(),
        });
        self.emitter.event(Event::ShardIdentifying(Identifying {
            shard_id: self.config.shard()[0],
            shard_total: self.config.shard()[1],
        }));

        self.send(identify).await
    }

    /// Perform a full reconnect to the gateway, instantiating a new session.
    async fn reconnect(&mut self) {
        #[cfg(feature = "tracing")]
        tracing::info!("reconnection started");

        let mut wait = Duration::from_secs(1);

        loop {
            #[cfg(feature = "tracing")]
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
                Err(_source) => {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("reconnecting failed: {:?}", _source);

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
        #[cfg(feature = "tracing")]
        tracing::debug!("resuming shard {:?}", self.config.shard());

        self.session.set_stage(Stage::Resuming);
        self.session.stop_heartbeater();

        let seq = self.session.seq();

        let id = if let Some(id) = self.session.id() {
            id
        } else {
            #[cfg(feature = "tracing")]
            tracing::info!("session id unavailable, reconnecting");

            self.reconnect().await;
            return;
        };

        self.resume = Some((seq, id));

        if let Err(_source) = self.try_resume().await {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                seq = seq,
                session_id = ?self.session.id(),
                shard_id = self.config.shard()[0],
                "failed to resume session: {:?}",
                _source,
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

        if let Err(_source) = self.wtx.send(Arc::clone(&self.session)) {
            #[cfg(feature = "tracing")]
            tracing::error!("failed to broadcast new session: {:?}", _source);
        }

        self.session.set_stage(stage);
        self.compression.reset();
    }

    async fn emit_disconnected(&self, code: Option<u16>, reason: Option<String>) {
        self.emitter.event(Event::ShardDisconnected(Disconnected {
            code,
            reason,
            shard_id: self.config.shard()[0],
        }));
    }
}

/// Default identify properties to use when the user has not customized it via
/// [`ShardBuilder::identify_properties`].
///
/// [`ShardBuilder::identify_properties`]: super::super::ShardBuilder::identify_properties
fn default_identify_properties() -> IdentifyProperties {
    IdentifyProperties::new("twilight.rs", "twilight.rs", OS, "", "")
}
