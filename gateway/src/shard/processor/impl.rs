use super::{
    super::{config::Config, stage::Stage, ShardStream},
    emitter::Emitter,
    inflater::Inflater,
    session::{Session, SessionSendError},
    socket_forwarder::SocketForwarder,
};
use crate::listener::Listeners;
use async_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Error as TungsteniteError, Message,
};
use flate2::DecompressError;
use futures_channel::mpsc::{TrySendError, UnboundedReceiver};
use futures_util::stream::StreamExt;
use serde::Serialize;
use std::{
    borrow::Cow,
    env::consts::OS,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    str::{self, Utf8Error},
    sync::{atomic::Ordering, Arc},
};
use tokio::sync::watch::{
    channel as watch_channel, Receiver as WatchReceiver, Sender as WatchSender,
};
use twilight_model::gateway::{
    event::{
        shard::{Connected, Connecting, Disconnected, Identifying, Reconnecting, Resuming},
        DispatchEvent, Event, GatewayEvent,
    },
    payload::{
        identify::{Identify, IdentifyInfo, IdentifyProperties},
        resume::Resume,
    },
    GatewayIntents,
};
use url::{ParseError as UrlParseError, Url};

#[cfg(not(feature = "simd-json"))]
use serde_json::Error as JsonError;
#[cfg(feature = "simd-json")]
use simd_json::Error as JsonError;

/// Connecting to the gateway failed.
#[derive(Debug)]
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
enum GatewayEventParsingError {
    /// Deserializing the GatewayEvent payload from JSON failed.
    Deserializing {
        /// Reason for the error.
        source: JsonError,
    },
    /// The payload received from Discord was an unrecognized or invalid
    /// structure.
    ///
    /// The payload was either invalid JSON or did not contain the necessary
    /// "op" key in the object.
    PayloadInvalid,
}

impl Display for GatewayEventParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Deserializing { source } => Display::fmt(source, f),
            Self::PayloadInvalid => f.write_str("payload is an invalid json structure"),
        }
    }
}

impl Error for GatewayEventParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Deserializing { source } => Some(source),
            Self::PayloadInvalid => None,
        }
    }
}

#[derive(Debug)]
enum ProcessError {
    /// A close message tried to be sent but the receiving half was dropped.
    /// This typically means that the shard is shutdown.
    SendingClose {
        /// Reason for the error.
        source: TrySendError<Message>,
    },
    /// Sending a message over the session was unsuccessful.
    SessionSend {
        /// Reason for the error.
        source: SessionSendError,
    },
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::SendingClose { source } => Display::fmt(source, f),
            Self::SessionSend { source } => Display::fmt(source, f),
        }
    }
}

impl Error for ProcessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SendingClose { source } => Some(source),
            Self::SessionSend { source } => Some(source),
        }
    }
}

#[derive(Debug)]
enum ReceivingEventError {
    /// Provided authorization token is invalid.
    AuthorizationInvalid { shard_id: u64, token: String },
    /// Decompressing a frame from Discord failed.
    Decompressing {
        /// Reason for the error.
        source: DecompressError,
    },
    /// Current user isn't allowed to use at least one of the configured
    /// intents.
    ///
    /// The intents are provided.
    IntentsDisallowed {
        /// The configured intents for the shard.
        intents: Option<GatewayIntents>,
        /// The ID of the shard.
        shard_id: u64,
    },
    /// Configured intents aren't supported by Discord's gateway.
    ///
    /// The intents are provided.
    IntentsInvalid {
        /// Configured intents for the shard.
        intents: Option<GatewayIntents>,
        /// ID of the shard.
        shard_id: u64,
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
    /// The event stream has ended, this is recoverable by resuming.
    EventStreamEnded,
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
            Self::ParsingPayload { source } => Display::fmt(source, f),
            Self::PayloadNotUtf8 { .. } => {
                f.write_str("the payload from Discord wasn't UTF-8 valid")
            }
            Self::EventStreamEnded => f.write_str("event stream from gateway ended"),
        }
    }
}

impl Error for ReceivingEventError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParsingPayload { source } => Some(source),
            Self::PayloadNotUtf8 { source } => Some(source),
            Self::AuthorizationInvalid { .. }
            | Self::Decompressing { .. }
            | Self::IntentsDisallowed { .. }
            | Self::IntentsInvalid { .. }
            | Self::EventStreamEnded => None,
        }
    }
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
    url: String,
    resume: Option<(u64, String)>,
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

        url.push_str("?v=6&compress=zlib-stream");

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
            url,
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
            let gateway_event = match self.next_event().await {
                Ok(ev) => ev,
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

            // The only reason for an error is if the sender couldn't send a
            // message or if the session didn't exist when it should, so do a
            // reconnect if this fails.
            if self.process(&gateway_event).await.is_err() {
                tracing::debug!("error processing event; reconnecting");

                self.reconnect().await;

                continue;
            }

            self.emitter.event(Event::from(gateway_event));
        }

        self.emitter.into_listeners().remove_all();
    }

    /// Identifies with the gateway to create a new session.
    async fn identify(&mut self) -> Result<(), SessionSendError> {
        self.session.set_stage(Stage::Identifying);

        let intents = self.config.intents().copied();

        let identify = Identify::new(IdentifyInfo {
            compression: false,
            intents,
            large_threshold: self.config.large_threshold(),
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

    async fn process(&mut self, event: &GatewayEvent) -> Result<(), ProcessError> {
        use GatewayEvent::{
            Dispatch, Heartbeat, HeartbeatAck, Hello, InvalidateSession, Reconnect,
        };

        match event {
            Dispatch(seq, dispatch) => self.process_dispatch(*seq, dispatch.as_ref()),
            Heartbeat(seq) => self.process_heartbeat(*seq).await,
            Hello(interval) => self.process_hello(*interval).await?,
            HeartbeatAck => self.process_heartbeat_ack(),
            InvalidateSession(resumable) => self.process_invalidate_session(*resumable).await,
            Reconnect => self.process_reconnect().await?,
        }

        Ok(())
    }

    fn process_dispatch(&self, seq: u64, dispatch: &DispatchEvent) {
        #[cfg(feature = "metrics")]
        metrics::counter!("GatewayEvent", 1, "GatewayEvent" => "Dispatch");
        self.session.set_seq(seq);

        match dispatch {
            DispatchEvent::Ready(ready) => {
                self.session.set_stage(Stage::Connected);
                self.session.set_id(&ready.session_id);

                self.emitter.event(Event::ShardConnected(Connected {
                    heartbeat_interval: self.session.heartbeat_interval(),
                    shard_id: self.config.shard()[0],
                }));
            }
            DispatchEvent::Resumed => {
                self.session.set_stage(Stage::Connected);
                self.emitter.event(Event::ShardConnected(Connected {
                    heartbeat_interval: self.session.heartbeat_interval(),
                    shard_id: self.config.shard()[0],
                }));
                self.session.heartbeats.receive();
            }
            _ => {}
        }
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
            tracing::warn!("resuming with sequence {}, session id {}", seq, id);
            let payload = Resume::new(seq, &id, self.config.token());

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

    /// # Errors
    ///
    /// Returns [`Error::AuthorizationInvalid`] if the provided authorization
    /// is invalid.
    ///
    /// [`Error::AuthorizationInvalid`]: ../../error/enum.Error.html#variant.AuthorizationInvalid
    #[allow(unsafe_code)]
    async fn next_event(&mut self) -> Result<GatewayEvent, ReceivingEventError> {
        loop {
            // Returns None when the socket forwarder has ended, meaning the
            // connection was dropped.
            let msg = self
                .rx
                .next()
                .await
                .ok_or(ReceivingEventError::EventStreamEnded)?;

            match msg {
                Message::Binary(bin) => {
                    self.inflater.extend(&bin[..]);
                    let decompressed_msg = self
                        .inflater
                        .msg()
                        .map_err(|source| ReceivingEventError::Decompressing { source })?;
                    let msg_or_error = match decompressed_msg {
                        Some(json) => {
                            self.emitter.bytes(json);

                            let mut text = str::from_utf8_mut(json)
                                .map_err(|source| ReceivingEventError::PayloadNotUtf8 { source })?;

                            Self::parse_gateway_event(&mut text)
                        }
                        None => continue,
                    };
                    self.inflater.clear();
                    break msg_or_error
                        .map_err(|source| ReceivingEventError::ParsingPayload { source });
                }
                Message::Close(close_frame) => {
                    tracing::warn!("got close code: {:?}", close_frame);
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
                                    intents: self.config.intents().copied(),
                                    shard_id: self.config.shard()[0],
                                });
                            }
                            CloseCode::Library(4014) => {
                                return Err(ReceivingEventError::IntentsDisallowed {
                                    intents: self.config.intents().copied(),
                                    shard_id: self.config.shard()[0],
                                });
                            }
                            _ => {}
                        }
                    }

                    self.resume().await;
                }
                Message::Ping(_) | Message::Pong(_) => {}
                Message::Text(mut text) => {
                    tracing::trace!("text payload: {}", text);
                    self.emitter.bytes(text.as_bytes());

                    break Self::parse_gateway_event(&mut text)
                        .map_err(|source| ReceivingEventError::ParsingPayload { source });
                }
            }
        }
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

    /// Perform a full reconnect to the gateway, instantiating a new session.
    async fn reconnect(&mut self) {
        tracing::info!("reconnection started");

        loop {
            // Await allowance when doing a full reconnect.
            self.config.queue.request(self.config.shard()).await;

            self.emitter.event(Event::ShardReconnecting(Reconnecting {
                shard_id: self.config.shard()[0],
            }));

            let stream = match Self::connect(&self.url).await {
                Ok(s) => s,
                Err(why) => {
                    tracing::warn!("reconnecting failed: {:?}", why);

                    continue;
                }
            };

            self.set_session(stream, Stage::Connected);

            break;
        }

        self.emitter.event(Event::ShardConnecting(Connecting {
            gateway: self.url.clone(),
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
            tracing::warn!("session id unavailable, reconnecting");
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

        if let Err(why) = self.wtx.broadcast(Arc::clone(&self.session)) {
            tracing::error!("failed to broadcast new session: {:?}", why);
        }

        self.session.set_stage(stage);
        self.inflater.reset();
    }

    /// Parse a gateway event from a string using `serde_json`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::PayloadInvalid`] if the payload wasn't a valid
    /// `GatewayEvent` data structure.
    ///
    /// Returns [`Error::PayloadSerialization`] if the payload failed to
    /// deserialize.
    ///
    /// [`Error::PayloadInvalid`]: ../enum.Error.html#variant.PayloadInvalid
    /// [`Error::PayloadSerialization`]: ../enum.Error.html#variant.PayloadSerialization
    #[cfg(not(feature = "simd-json"))]
    fn parse_gateway_event(json: &mut str) -> Result<GatewayEvent, GatewayEventParsingError> {
        use serde::de::DeserializeSeed;
        use serde_json::Deserializer;
        use twilight_model::gateway::event::GatewayEventDeserializer;

        let gateway_deserializer = GatewayEventDeserializer::from_json(json)
            .ok_or_else(|| GatewayEventParsingError::PayloadInvalid)?;
        let mut json_deserializer = Deserializer::from_str(json);

        gateway_deserializer
            .deserialize(&mut json_deserializer)
            .map_err(|source| {
                tracing::debug!("invalid JSON: {}", json);

                GatewayEventParsingError::Deserializing { source }
            })
    }

    /// Parse a gateway event from a string using `simd-json`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::PayloadInvalid`] if the payload wasn't a valid
    /// `GatewayEvent` data structure.
    ///
    /// Returns [`Error::PayloadSerialization`] if the payload failed to
    /// deserialize.
    ///
    /// [`Error::PayloadInvalid`]: ../enum.Error.html#variant.PayloadInvalid
    /// [`Error::PayloadSerialization`]: ../enum.Error.html#variant.PayloadSerialization
    #[allow(unsafe_code)]
    #[cfg(feature = "simd-json")]
    fn parse_gateway_event(json: &mut str) -> Result<GatewayEvent, GatewayEventParsingError> {
        use serde::de::DeserializeSeed;
        use simd_json::Deserializer;
        use twilight_model::gateway::event::gateway::GatewayEventDeserializerOwned;

        let gateway_deserializer = GatewayEventDeserializerOwned::from_json(json)
            .ok_or_else(|| GatewayEventParsingError::PayloadInvalid)?;

        /// This is unsafe because it calls `std::str::as_bytes_mut`, which may
        /// change the string in ways that aren't UTF-8 valid. The string won't
        /// be used again.
        let json_bytes = unsafe { json.as_bytes_mut() };

        let mut json_deserializer = Deserializer::from_slice(json_bytes)
            .map_err(|_| GatewayEventParsingError::PayloadInvalid)?;

        gateway_deserializer
            .deserialize(&mut json_deserializer)
            .map_err(|source| {
                tracing::debug!("invalid JSON: {}", json);

                GatewayEventParsingError::Deserializing { source }
            })
    }
}
