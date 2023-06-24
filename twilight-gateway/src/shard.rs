//! Primary logic and implementation details of Discord gateway websocket
//! connections.
//!
//! Shards are, at their heart, a websocket connection with some state for
//! maintaining an identified session with the Discord gateway. For more
//! information about what a shard is in the context of Discord's gateway API,
//! refer to the documentation for [`Shard`].
//!
//! # Implementation Flow
//!
//! Other than informative methods and getters, shards have two functions:
//! sending websocket messages and receiving websocket messages, called
//! "events" in the context of the Discord gateway.
//!
//! Sending a message is simple and the flow of it looks like:
//!
//! 1. If the user sends a [command] via [`Shard::command`] the command is
//! serialized into a raw websocket message and then...
//! 2. [`Shard::send`] is called and the sending of the message goes through
//! ratelimiting via [`CommandRatelimiter`] if ratelimiting [is enabled];
//! 3. The [websocket message] is sent over the [websocket connection].
//!
//! Receiving a message is a little bit more complicated, but follows as:
//!
//! 1. The user calls [`Shard::next_message`] to receive the next message from
//! the Websocket;
//! 2. If the shard is disconnected then the connection is reconnected;
//! 3. One of five things wait to happen:
//!   a. the shard receives a close frame from the user over the [user channel],
//!   which is then forwarded via [`Shard::close`]; or
//!   b. the interval for the shard to send the next heartbeat occurs, in which
//!   case [`Shard::heartbeat`] is called; or
//!   c. the background identify queue task finishes, in which case
//!   [`Shard::send`] is called with the identify payload; or
//!   d. the shard receives a command from the user over the [user channel],
//!   which is then forwarded via [`Shard::send`]; or
//!   e. the shard receives a message from Discord via the websocket connection.
//! 4. In the case of 3(a) through 3(d), 3 is repeated; otherwise...
//! 5. If the message is not a close it's [processed] by the shard;
//! 6. The raw Websocket message is returned to the user
//!
//! If the user called [`Shard::next_event`] instead of [`Shard::next_message`],
//! then the previous steps are taken and the resultant message is deserialized
//! into a [`GatewayEvent`] if it matches the user's [`EventTypeFlags`].
//!
//! [`GatewayEvent`]: twilight_model::gateway::event::GatewayEvent
//! [command]: crate::Command
//! [close message]: Message::Close
//! [`EventTypeFlags`]: crate::EventTypeFlags
//! [is enabled]: Config::ratelimit_messages
//! [processed]: Shard::process
//! [user channel]: crate::MessageSender
//! [websocket connection]: Shard::connection
//! [websocket message]: Message

#[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
use crate::inflater::Inflater;
use crate::{
    channel::{MessageChannel, MessageSender},
    command::{self, Command},
    connection::{self, Connection},
    error::{
        ProcessError, ProcessErrorType, ReceiveMessageError, ReceiveMessageErrorType, SendError,
        SendErrorType,
    },
    json::{self, UnknownEventError},
    latency::Latency,
    ratelimiter::CommandRatelimiter,
    session::Session,
    Config, Message, ShardId,
};
use futures_util::{SinkExt, StreamExt};
use serde::{de::DeserializeOwned, Deserialize};
#[cfg(any(
    feature = "native",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
))]
use std::io::ErrorKind as IoErrorKind;
use std::{
    env::consts::OS,
    error::Error,
    future::{poll_fn, Future},
    pin::Pin,
    str,
    task::{Context, Poll},
};
use tokio::{
    task::JoinHandle,
    time::{self, Duration, Instant, Interval, MissedTickBehavior},
};
use tokio_tungstenite::tungstenite::{Error as TungsteniteError, Message as TungsteniteMessage};
use twilight_model::gateway::{
    event::{Event, GatewayEventDeserializer},
    payload::{
        incoming::Hello,
        outgoing::{
            identify::{IdentifyInfo, IdentifyProperties},
            Heartbeat, Identify, Resume,
        },
    },
    CloseCode, CloseFrame, Intents, OpCode,
};

/// Who initiated the closing of the websocket connection.
#[derive(Clone, Copy, Debug)]
enum CloseInitiator {
    /// The gateway initiated the close.
    ///
    /// Contains an optional close code.
    Gateway(Option<u16>),
    /// The shard initiated the close.
    ///
    /// Contains a close code.
    Shard(u16),
    /// Nobody initiated the close (underlying connection errored).
    None,
}

impl CloseInitiator {
    /// The inner close code.
    const fn close_code(self) -> Option<u16> {
        match self {
            CloseInitiator::Gateway(close_code) => close_code,
            CloseInitiator::Shard(close_code) => Some(close_code),
            CloseInitiator::None => None,
        }
    }
}

/// Current status of a shard.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConnectionStatus {
    /// Shard is connected to the gateway with an active session.
    Connected,
    /// Shard is disconnected from the gateway but may reconnect in the future.
    ///
    /// The underlying connection may still be open.
    Disconnected {
        /// Close code, if available.
        close_code: Option<u16>,
        /// Number of reconnection attempts that have been made.
        reconnect_attempts: u8,
    },
    /// Shard has fatally closed.
    ///
    /// Possible reasons may be due to [failed authentication],
    /// [invalid intents], or other reasons. Refer to the documentation for
    /// [`CloseCode`] for possible reasons.
    ///
    /// [failed authentication]: CloseCode::AuthenticationFailed
    /// [invalid intents]: CloseCode::InvalidIntents
    FatallyClosed {
        /// Close code of the close message.
        close_code: CloseCode,
    },
    /// Shard is waiting to establish an active session.
    Identifying,
    /// Shard is replaying missed dispatch events.
    ///
    /// The shard is considered identified whilst resuming.
    Resuming,
}

impl ConnectionStatus {
    /// Determine the connection status from the close code.
    ///
    /// Defers to [`CloseCode::can_reconnect`] to determine whether the
    /// connection can be reconnected, defaulting to [`Self::Disconnected`] if
    /// the close code is unknown.
    fn from_close_code(close_code: Option<u16>) -> Self {
        match close_code.map(CloseCode::try_from) {
            Some(Ok(close_code)) if !close_code.can_reconnect() => {
                Self::FatallyClosed { close_code }
            }
            _ => Self::Disconnected {
                close_code,
                reconnect_attempts: 0,
            },
        }
    }

    /// Whether the shard is connected with an active session.
    pub const fn is_connected(&self) -> bool {
        matches!(self, Self::Connected)
    }

    /// Whether the shard has disconnected but may reconnect in the future.
    pub const fn is_disconnected(&self) -> bool {
        matches!(self, Self::Disconnected { .. })
    }

    /// Whether the shard has fatally closed, such as due to an invalid token.
    pub const fn is_fatally_closed(&self) -> bool {
        matches!(self, Self::FatallyClosed { .. })
    }

    /// Whether the shard is identified with an active session.
    ///
    /// `true` if the status is [`Connected`] or [`Resuming`].
    ///
    /// [`Connected`]: Self::Connected
    /// [`Resuming`]: Self::Resuming
    pub const fn is_identified(&self) -> bool {
        self.is_connected() || self.is_resuming()
    }

    /// Whether the shard is waiting to establish an active session.
    pub const fn is_identifying(&self) -> bool {
        matches!(self, Self::Identifying)
    }

    /// Whether the shard is replaying missed dispatch events.
    ///
    /// The shard is considered identified whilst resuming.
    pub const fn is_resuming(&self) -> bool {
        matches!(self, Self::Resuming)
    }
}

/// Gateway event with only minimal required data.
#[derive(Deserialize)]
struct MinimalEvent<T> {
    /// Attached data of the gateway event.
    #[serde(rename = "d")]
    data: T,
}

/// Minimal [`Ready`] for light deserialization.
///
/// [`Ready`]: twilight_model::gateway::payload::incoming::Ready
#[derive(Deserialize)]
struct MinimalReady {
    /// Used for resuming connections.
    resume_gateway_url: Box<str>,
    /// ID of the new identified session.
    session_id: String,
}

/// Gateway API client responsible for up to 2500 guilds.
///
/// Shards are responsible for maintaining the gateway connection by processing
/// events relevant to the operation of shards---such as requests from the
/// gateway to re-connect or invalidate a session---and then to pass them on to
/// the user.
///
/// Shards start out disconnected, but will on the first call to
/// [`next_message`] try to reconnect to the gateway. [`next_message`] must then
/// be repeatedly called in order for the shard to maintain its connection and
/// update its internal state. Note that the [`next_event`] method internally
/// calls [`next_message`].
///
/// Shards go through an [identify queue][`queue`] that ratelimits the amount of
/// concurrent identifies (across all shards) per 5 seconds. Exceeding this
/// limit invalidates the shard's session and it is therefore very important to
/// reuse the same queue when running multiple shards. Note that shards must be
/// identified before they start receiving dispatch events and are able to send
/// [`Command`]s.
///
/// # Sharding
///
/// A shard may not be connected to more than 2500 guilds, so large bots must
/// split themselves across multiple shards. See the
/// [Discord Docs/Sharding][docs:sharding], [`ShardId`], and [`stream`]
/// documentation for more info.
///
/// # Sending shard commands in different tasks
///
/// Because shards should not be used across multiple tasks it's not always easy
/// to directly send [gateway commands] over a shard. As a convenience method,
/// [`Shard::sender`] can be used to receive an MPSC channel sender which, in
/// addition to being cheaply cloned, also only sends queued up commands when
/// the shard is identified and not ratelimited. Multiple shards' senders can,
/// for example, be collected into an `Arc<Vec<MessageSender>>` and be shared
/// across all event handler tasks.
///
/// # Examples
///
/// Create and start a shard and print new and deleted messages:
///
/// ```no_run
/// use std::env;
/// use twilight_gateway::{Config, Event, EventTypeFlags, Intents, Shard, ShardId};
///
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Use the value of the "DISCORD_TOKEN" environment variable as the bot's
/// // token. Of course, this value may be passed into the program however is
/// // preferred.
/// let token = env::var("DISCORD_TOKEN")?;
/// let event_types = EventTypeFlags::MESSAGE_CREATE | EventTypeFlags::MESSAGE_DELETE;
///
/// let config = Config::builder(token, Intents::GUILD_MESSAGES)
///     .event_types(event_types)
///     .build();
/// let mut shard = Shard::with_config(ShardId::ONE, config);
///
/// // Create a loop of only new messages and deleted messages.
///
/// loop {
///     let event = match shard.next_event().await {
///         Ok(event) => event,
///         Err(source) => {
///             tracing::warn!(?source, "error receiving event");
///
///             if source.is_fatal() {
///                 break;
///             }
///
///             continue;
///         }
///     };
///
///     match event {
///         Event::MessageCreate(message) => {
///             println!("message received with content: {}", message.content);
///         }
///         Event::MessageDelete(message) => {
///             println!("message with ID {} deleted", message.id);
///         }
///         _ => {}
///     }
/// }
/// # Ok(()) }
/// ```
///
/// [docs:sharding]: https://discord.com/developers/docs/topics/gateway#sharding
/// [gateway commands]: Shard::command
/// [`next_event`]: Shard::next_event
/// [`next_message`]: Shard::next_message
/// [`stream`]: crate::stream
/// [`queue`]: crate::queue
#[derive(Debug)]
pub struct Shard {
    /// User provided configuration.
    ///
    /// Configurations are provided or created in shard initializing via
    /// [`Shard::new`] or [`Shard::with_config`].
    config: Config,
    /// Websocket connection, which may be connected to Discord's gateway.
    ///
    /// The connection should only be dropped after it has returned `Ok(None)`
    /// to comply with the WebSocket protocol.
    connection: Option<Connection>,
    /// Interval of how often the gateway would like the shard to
    /// [send heartbeats][`Self::heartbeat`].
    ///
    /// The interval is received in the [`GatewayEvent::Hello`] event when
    /// first opening a new [connection].
    ///
    /// [`GatewayEvent::Hello`]: twilight_model::gateway::event::GatewayEvent::Hello
    /// [connection]: Self::connection
    heartbeat_interval: Option<Interval>,
    /// Whether an event has been received in the current heartbeat interval.
    heartbeat_interval_event: bool,
    /// ID of the shard.
    id: ShardId,
    /// Identify queue background task handle.
    identify_handle: Option<JoinHandle<()>>,
    /// Zlib decompressor.
    #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
    inflater: Inflater,
    /// Recent heartbeat latency statistics.
    ///
    /// The latency is reset on receiving [`GatewayEvent::Hello`] as the host
    /// may have changed, invalidating previous latency statistic.
    ///
    /// [`GatewayEvent::Hello`]: twilight_model::gateway::event::GatewayEvent::Hello
    latency: Latency,
    /// Command ratelimiter, if it was enabled via
    /// [`Config::ratelimit_messages`].
    ratelimiter: Option<CommandRatelimiter>,
    /// Used for resuming connections.
    resume_gateway_url: Option<Box<str>>,
    /// Active session of the shard.
    ///
    /// The shard may not have an active session if it hasn't yet identified and
    /// received a `READY` dispatch event response.
    session: Option<Session>,
    /// Current connection status of the Websocket connection.
    status: ConnectionStatus,
    /// Messages from the user to be relayed and sent over the Websocket
    /// connection.
    user_channel: MessageChannel,
}

impl Shard {
    /// Create a new shard with the default configuration.
    pub fn new(id: ShardId, token: String, intents: Intents) -> Self {
        let config = Config::builder(token, intents).build();

        Self::with_config(id, config)
    }

    /// Create a new shard with the provided configuration.
    pub fn with_config(shard_id: ShardId, mut config: Config) -> Self {
        let session = config.take_session();

        Self {
            config,
            connection: None,
            heartbeat_interval: None,
            heartbeat_interval_event: false,
            id: shard_id,
            identify_handle: None,
            #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
            inflater: Inflater::new(),
            latency: Latency::new(),
            ratelimiter: None,
            resume_gateway_url: None,
            session,
            status: ConnectionStatus::Disconnected {
                close_code: None,
                reconnect_attempts: 0,
            },
            user_channel: MessageChannel::new(),
        }
    }

    /// Immutable reference to the configuration used to instantiate this shard.
    pub const fn config(&self) -> &Config {
        &self.config
    }

    /// ID of the shard.
    pub const fn id(&self) -> ShardId {
        self.id
    }

    /// Zlib decompressor statistics.
    ///
    /// Reset when reconnecting to the gateway.
    #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
    pub const fn inflater(&self) -> &Inflater {
        &self.inflater
    }

    /// Connection status of the shard.
    pub const fn status(&self) -> &ConnectionStatus {
        &self.status
    }

    /// Shard latency statistics, including average latency and recent heartbeat
    /// latency times.
    ///
    /// Reset when reconnecting to the gateway.
    pub const fn latency(&self) -> &Latency {
        &self.latency
    }

    /// Statistics about the number of available commands and when the command
    /// ratelimiter will refresh.
    ///
    /// This won't be present if ratelimiting was disabled via
    /// [`ConfigBuilder::ratelimit_messages`] or if the shard is disconnected.
    ///
    /// [`ConfigBuilder::ratelimit_messages`]: crate::ConfigBuilder::ratelimit_messages
    pub const fn ratelimiter(&self) -> Option<&CommandRatelimiter> {
        self.ratelimiter.as_ref()
    }

    /// Immutable reference to the active gateway session.
    ///
    /// An active session may not be present if the shard had its session
    /// invalidated and has not yet reconnected.
    pub const fn session(&self) -> Option<&Session> {
        self.session.as_ref()
    }

    /// Wait for the next Discord event from the gateway.
    ///
    /// This is a convenience method that internally calls [`next_message`] and
    /// only returns wanted [`Event`]s, configured via
    /// [`ConfigBuilder::event_types`]. Close messages are always considered
    /// wanted and map onto the [`Event::GatewayClose`] variant.
    ///
    /// Events not registered in Twilight are skipped. If you need to receive
    /// events Twilight doesn't support, use [`next_message`] to receive raw
    /// payloads.
    ///
    /// # Errors
    ///
    /// Returns a [`ReceiveMessageErrorType::Compression`] error type if the
    /// message payload failed to decompress.
    ///
    /// Returns a [`ReceiveMessageErrorType::Deserializing`] error type if the
    /// message payload failed to deserialize.
    ///
    /// Returns a [`ReceiveMessageErrorType::FatallyClosed`] error type if the
    /// shard was closed due to a fatal error, such as invalid authorization.
    ///
    /// Returns a [`ReceiveMessageErrorType::Process`] error type if the shard
    /// failed to internally process a received event.
    ///
    /// Returns a [`ReceiveMessageErrorType::Reconnect`] error type if the shard
    /// failed to reconnect to the gateway. This isn't a fatal error and can be
    /// retried.
    ///
    /// Returns a [`ReceiveMessageErrorType::SendingMessage`] error type if the
    /// shard failed to send a message to the gateway, such as a heartbeat.
    ///
    /// [`ConfigBuilder::event_types`]: crate::ConfigBuilder::event_types
    /// [`next_message`]: Self::next_message
    pub async fn next_event(&mut self) -> Result<Event, ReceiveMessageError> {
        loop {
            match self.next_message().await? {
                Message::Close(frame) => return Ok(Event::GatewayClose(frame)),
                Message::Text(text) => match crate::parse(text, self.config.event_types()) {
                    Ok(Some(event)) => return Ok(event.into()),
                    Ok(None) => {}
                    Err(source) => {
                        // Discord has many events that aren't documented, so we
                        // need to skip over errors caused by unknown events or
                        // opcodes.
                        //
                        // clippy: the recommendation is to reference the method
                        // by name with a turbofish, which is invalid syntax
                        #[allow(clippy::redundant_closure_for_method_calls)]
                        let maybe_unknown_event = source
                            .source()
                            .and_then(|source| source.downcast_ref::<UnknownEventError>());

                        if let Some(unknown_event) = maybe_unknown_event {
                            tracing::debug!(
                                id=%self.id,
                                event_type=?unknown_event.event_type,
                                opcode=?unknown_event.opcode,
                                "skipped deserializing unknown event",
                            );

                            continue;
                        }

                        return Err(source);
                    }
                },
            }
        }
    }

    /// Wait for the next raw message from the websocket connection.
    ///
    /// # Errors
    ///
    /// Returns a [`ReceiveMessageErrorType::Compression`] error type if the
    /// message payload failed to decompress.
    ///
    /// Returns a [`ReceiveMessageErrorType::FatallyClosed`] error type if the
    /// shard was closed due to a fatal error, such as invalid authorization.
    ///
    /// Returns a [`ReceiveMessageErrorType::Process`] error type if the shard
    /// failed to internally process a received event.
    ///
    /// Returns a [`ReceiveMessageErrorType::Reconnect`] error type if the shard
    /// failed to reconnect to the gateway. This isn't a fatal error and can be
    /// retried.
    ///
    /// Returns a [`ReceiveMessageErrorType::SendingMessage`] error type if the
    /// shard failed to send a message to the gateway, such as a heartbeat.
    #[tracing::instrument(fields(id = %self.id()), name = "shard", skip(self))]
    pub async fn next_message(&mut self) -> Result<Message, ReceiveMessageError> {
        /// Actions the shard might take.
        enum Action {
            /// Close the gateway connection with this close frame.
            Close(CloseFrame<'static>),
            /// Send this command to the gateway.
            Command(String),
            /// Send a heartbeat command to the gateway.
            Heartbeat,
            /// Identify with the gateway.
            Identify,
            /// Handle this incoming gateway message.
            Message(Option<Result<TungsteniteMessage, TungsteniteError>>),
        }

        match self.status {
            ConnectionStatus::Disconnected {
                close_code,
                reconnect_attempts,
            } => {
                // The shard is considered disconnected after having received a
                // close frame or encountering a websocket error, but it should
                // only reconnect after the underlying TCP connection is closed
                // by the server (having returned `Ok(None)`).
                if self.connection.is_none() {
                    self.reconnect(close_code, reconnect_attempts).await?;
                }
            }
            ConnectionStatus::FatallyClosed { close_code } if self.connection.is_none() => {
                return Err(ReceiveMessageError::from_fatally_closed(close_code));
            }
            _ => {}
        }

        let message = loop {
            let next_action = |cx: &mut Context<'_>| {
                if !(self.status.is_disconnected() || self.status.is_fatally_closed()) {
                    if let Poll::Ready(frame) = self.user_channel.close_rx.poll_recv(cx) {
                        return Poll::Ready(Action::Close(frame.expect("shard owns channel")));
                    }
                }

                if self
                    .heartbeat_interval
                    .as_mut()
                    .map_or(false, |heartbeater| heartbeater.poll_tick(cx).is_ready())
                {
                    return Poll::Ready(Action::Heartbeat);
                }

                let ratelimited = self.ratelimiter.as_mut().map_or(false, |ratelimiter| {
                    ratelimiter.poll_available(cx).is_pending()
                });

                if !ratelimited
                    && self
                        .identify_handle
                        .as_mut()
                        .map_or(false, |handle| Pin::new(handle).poll(cx).is_ready())
                {
                    return Poll::Ready(Action::Identify);
                }

                if !ratelimited && self.status.is_identified() {
                    if let Poll::Ready(command) = self.user_channel.command_rx.poll_recv(cx) {
                        return Poll::Ready(Action::Command(command.expect("shard owns channel")));
                    }
                }

                if let Poll::Ready(message) =
                    Pin::new(&mut self.connection.as_mut().expect("connected").next()).poll(cx)
                {
                    return Poll::Ready(Action::Message(message));
                }

                Poll::Pending
            };

            match poll_fn(next_action).await {
                Action::Message(Some(Ok(message))) => {
                    #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
                    if let TungsteniteMessage::Binary(bytes) = &message {
                        if let Some(decompressed) = self
                            .inflater
                            .inflate(bytes)
                            .map_err(ReceiveMessageError::from_compression)?
                        {
                            tracing::trace!(%decompressed);
                            break Message::Text(decompressed);
                        };
                    }
                    if let Some(message) = Message::from_tungstenite(message) {
                        break message;
                    }
                }
                // Discord, against recommendations from the WebSocket spec,
                // does not send a close_notify prior to shutting down the TCP
                // stream. This arm tries to gracefully handle this. The
                // connection is considered unusable after encountering an io
                // error, returning `None`.
                #[cfg(any(
                    feature = "native",
                    feature = "rustls-native-roots",
                    feature = "rustls-webpki-roots"
                ))]
                Action::Message(Some(Err(TungsteniteError::Io(e))))
                    if e.kind() == IoErrorKind::UnexpectedEof
                        // Assert we're directly connected to Discord's gateway.
                        && self.config.proxy_url().is_none()
                        && (self.status.is_disconnected() || self.status.is_fatally_closed()) =>
                {
                    continue
                }
                Action::Message(Some(Err(source))) => {
                    self.disconnect(CloseInitiator::None);

                    return Err(ReceiveMessageError {
                        kind: ReceiveMessageErrorType::Io,
                        source: Some(Box::new(source)),
                    });
                }
                Action::Message(None) => {
                    tracing::debug!("gateway connection closed");
                    self.connection = None;

                    // This match statement should be similar the initial one in
                    // this method.
                    match self.status {
                        ConnectionStatus::Disconnected {
                            close_code,
                            reconnect_attempts,
                        } => self.reconnect(close_code, reconnect_attempts).await?,
                        ConnectionStatus::FatallyClosed { close_code } => {
                            return Err(ReceiveMessageError::from_fatally_closed(close_code))
                        }
                        _ => unreachable!(
                            "stream ended because websocket is closed (received close frame sets \
                            status to disconnected or fatally closed) or because it errored (which \
                            also sets status to disconnected)"
                        ),
                    };

                    continue;
                }
                Action::Heartbeat => {
                    let is_first_heartbeat =
                        self.heartbeat_interval.is_some() && self.latency.sent().is_none();

                    // Discord never responded after the last heartbeat,
                    // connection is failed or "zombied", see
                    // https://discord.com/developers/docs/topics/gateway#heartbeat-interval-example-heartbeat-ack
                    // Note that unlike documented *any* event is okay; it does
                    // not have to be a heartbeat ACK.
                    if !is_first_heartbeat && !self.heartbeat_interval_event {
                        tracing::info!("connection is failed or \"zombied\"");
                        self.session = self
                            .close(CloseFrame::RESUME)
                            .await
                            .map_err(ReceiveMessageError::from_send)?;
                    } else {
                        self.heartbeat()
                            .await
                            .map_err(ReceiveMessageError::from_send)?;
                        self.heartbeat_interval_event = false;
                    }

                    continue;
                }
                Action::Identify => {
                    self.identify_handle = None;

                    tracing::debug!("sending identify");
                    let identify = Identify::new(IdentifyInfo {
                        compress: false,
                        intents: self.config.intents(),
                        large_threshold: self.config.large_threshold(),
                        presence: self.config.presence().cloned(),
                        properties: self
                            .config
                            .identify_properties()
                            .cloned()
                            .unwrap_or_else(default_identify_properties),
                        shard: Some(self.id()),
                        token: self.config.token().to_owned(),
                    });
                    let json =
                        command::prepare(&identify).map_err(ReceiveMessageError::from_send)?;
                    self.send(json)
                        .await
                        .map_err(ReceiveMessageError::from_send)?;

                    continue;
                }
                Action::Close(frame) => {
                    tracing::debug!("sending close frame from user channel");
                    self.session = self
                        .close(frame)
                        .await
                        .map_err(ReceiveMessageError::from_send)?;

                    continue;
                }
                Action::Command(json) => {
                    tracing::debug!("sending command from user channel");
                    self.send(json)
                        .await
                        .map_err(ReceiveMessageError::from_send)?;

                    continue;
                }
            }
        };

        match &message {
            Message::Close(frame) => {
                // Tungstenite automatically replies to the close message.
                tracing::debug!(?frame, "received websocket close message");
                // Don't run `disconnect` if we initiated the close.
                if !self.status.is_disconnected() {
                    self.disconnect(CloseInitiator::Gateway(
                        frame.as_ref().map(|frame| frame.code),
                    ));
                }
            }
            Message::Text(event) => {
                self.process(event)
                    .await
                    .map_err(|source| ReceiveMessageError {
                        kind: ReceiveMessageErrorType::Process,
                        source: Some(Box::new(source)),
                    })?;
            }
        }

        Ok(message)
    }

    /// Send a command over the gateway.
    ///
    /// Serializes the command and then calls [`send`].
    ///
    /// # Examples
    ///
    /// Request members whose names start with "tw" in a guild:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::env;
    /// use twilight_gateway::{ConnectionStatus, Intents, Shard, ShardId};
    /// use twilight_model::{gateway::payload::outgoing::RequestGuildMembers, id::Id};
    ///
    /// let intents = Intents::GUILD_VOICE_STATES;
    /// let token = env::var("DISCORD_TOKEN")?;
    ///
    /// let mut shard = Shard::new(ShardId::ONE, token, intents);
    ///
    /// // Discord only allows sending the `RequestGuildMembers` command after
    /// // the shard is identified.
    /// while !shard.status().is_identified() {
    ///     // Ignore these messages.
    ///     shard.next_message().await?;
    /// }
    ///
    /// // Query members whose names start with "tw" and limit the results to 10
    /// // members.
    /// let request = RequestGuildMembers::builder(Id::new(1)).query("tw", Some(10));
    ///
    /// // Send the request over the shard.
    /// shard.command(&request).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`SendErrorType::Sending`] error type if the command could
    /// not be sent over the websocket. This indicates the shard is either
    /// currently restarting or closed and will restart.
    ///
    /// Returns a [`SendErrorType::Serializing`] error type if the provided
    /// command failed to serialize.
    ///
    /// [`send`]: Self::send
    pub async fn command(&mut self, command: &impl Command) -> Result<(), SendError> {
        // Types implementing `Command` may only be sent when identified.
        if !self.status.is_identified() {
            return Err(SendError {
                kind: SendErrorType::Sending,
                source: None,
            });
        }

        let json = command::prepare(command)?;

        self.send(json).await
    }

    /// Send a JSON encoded gateway event.
    ///
    /// A permit from the shard's [ratelimiter] is first awaited (if
    /// ratelimiting is [enabled]) before sending the event.
    ///
    /// # Errors
    ///
    /// Returns a [`SendErrorType::Sending`] error type if the event could not
    /// be sent over the websocket. This indicates the shard is either currently
    /// restarting or closed and will restart.
    ///
    /// [enabled]: crate::ConfigBuilder::ratelimit_messages
    /// [ratelimiter]: CommandRatelimiter
    pub async fn send(&mut self, json: String) -> Result<(), SendError> {
        if let Some(ratelimiter) = &mut self.ratelimiter {
            ratelimiter.acquire().await;
        }

        self.send_unratelimited(Message::Text(json)).await
    }

    /// Send a raw websocket message without first passing the ratelimiter.
    async fn send_unratelimited(&mut self, message: Message) -> Result<(), SendError> {
        self.connection
            .as_mut()
            .ok_or(SendError {
                kind: SendErrorType::Sending,
                source: None,
            })?
            .send(message.into_tungstenite())
            .await
            .map_err(|source| SendError {
                kind: SendErrorType::Sending,
                source: Some(Box::new(source)),
            })
    }

    /// Retrieve a channel to send outgoing gateway events over the shard to the
    /// gateway.
    ///
    /// This is primarily useful for sending to other tasks and threads where
    /// the shard won't be available.
    pub fn sender(&self) -> MessageSender {
        self.user_channel.sender()
    }

    /// Send a Websocket close frame indicating whether to also invalidate the
    /// shard's session.
    ///
    /// Returns the shard's session if the close frame code is not `1000` or
    /// `1001`, which invalidates the session and shows the application's bot as
    /// offline. Otherwise Discord will not invalidate the shard's session and
    /// will continue to show the application's bot as online until its presence
    /// times out.
    ///
    /// Sets status to [`ConnectionStatus::Disconnected`] with the `close_code`
    /// from the `close_frame`.
    ///
    /// To read all remaining events, continue calling [`Shard::next_message`]
    /// until it returns the response close message or a
    /// [`ReceiveMessageErrorType::Io`] error type.
    ///
    /// You do not need to call this method upon receiving a close message,
    /// Twilight automatically responds for you.
    ///
    /// # Example
    ///
    /// Close the gateway connection but process already received messages:
    ///
    /// ```no_run
    /// # use twilight_gateway::{Intents, Shard, ShardId};
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut shard = Shard::new(ShardId::ONE, String::new(), Intents::empty());
    /// use twilight_gateway::{error::ReceiveMessageErrorType, CloseFrame, Message};
    ///
    /// shard.close(CloseFrame::NORMAL).await?;
    ///
    /// loop {
    ///     match shard.next_message().await {
    ///         Ok(Message::Close(_)) => {
    ///             // We've now received a close message response from the
    ///             // Gateway.
    ///             // Further calls to `next_message` would cause a reconnect.
    ///             break;
    ///         }
    ///         Ok(Message::Text(_)) => unimplemented!("handle message"),
    ///         Err(source) if matches!(source.kind(), ReceiveMessageErrorType::Io) => break,
    ///         Err(source) => tracing::warn!(?source, "error receiving message"),
    ///     }
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`SendErrorType::Sending`] error type if the close frame could
    /// not be sent over the websocket. This indicates the shard is either
    /// currently restarting or closed and will restart.
    ///
    /// [`ConfigBuilder::session`]: crate::ConfigBuilder::session
    pub async fn close(
        &mut self,
        close_frame: CloseFrame<'static>,
    ) -> Result<Option<Session>, SendError> {
        let close_code = close_frame.code;

        tracing::debug!(frame = ?close_frame, "sending websocket close message");
        let message = Message::Close(Some(close_frame));

        self.send_unratelimited(message).await?;

        self.disconnect(CloseInitiator::Shard(close_code));

        Ok(self.session.take())
    }

    /// Update internal state from gateway disconnect.
    fn disconnect(&mut self, initiator: CloseInitiator) {
        // May not send any additional WebSocket messages.
        self.heartbeat_interval = None;
        self.ratelimiter = None;
        // Not resuming, drop session and resume URL.
        // https://discord.com/developers/docs/topics/gateway#initiating-a-disconnect
        if matches!(initiator, CloseInitiator::Shard(1000 | 1001)) {
            self.resume_gateway_url = None;
            self.session = None;
        }
        // Avoid setting the status to FatallyClosed should it match for Shard initiated disconnect.
        self.status = match initiator {
            CloseInitiator::Gateway(close_code) => ConnectionStatus::from_close_code(close_code),
            _ => ConnectionStatus::Disconnected {
                close_code: initiator.close_code(),
                reconnect_attempts: 0,
            },
        };
    }

    /// Send a heartbeat.
    async fn heartbeat(&mut self) -> Result<(), SendError> {
        // Sequence should be null if no dispatch event has been received.
        let sequence = self.session().map(Session::sequence);
        tracing::debug!(?sequence, "sending heartbeat");
        let message = Message::Text(command::prepare(&Heartbeat::new(sequence))?);
        // The ratelimiter reserves capacity for heartbeat messages.
        self.send_unratelimited(message).await?;

        self.latency.record_sent();

        Ok(())
    }

    /// Updates the shard's internal state from a gateway event by recording
    /// and/or responding to certain Discord events.
    ///
    /// # Errors
    ///
    /// Returns a [`ProcessErrorType::Deserializing`] error type if the gateway
    /// event isn't a recognized structure, which may be the case for new or
    /// undocumented events.
    ///
    /// Returns a [`ProcessErrorType::SendingMessage`] error type if a Websocket
    /// message couldn't be sent over the connection, which may be the case if
    /// the connection isn't connected.
    ///
    /// [`GatewayEvent`]: twilight_model::gateway::event::GatewayEvent
    #[allow(clippy::too_many_lines)]
    async fn process(&mut self, event: &str) -> Result<(), ProcessError> {
        let (raw_opcode, maybe_sequence, maybe_event_type) =
            GatewayEventDeserializer::from_json(event)
                .ok_or(ProcessError {
                    kind: ProcessErrorType::Deserializing {
                        event: event.to_owned(),
                    },
                    source: Some("missing opcode".into()),
                })?
                .into_parts();

        if self.latency.sent().is_some() {
            self.heartbeat_interval_event = true;
        }

        match OpCode::from(raw_opcode) {
            Some(OpCode::Dispatch) => {
                let event_type = maybe_event_type.ok_or(ProcessError {
                    kind: ProcessErrorType::Deserializing {
                        event: event.to_owned(),
                    },
                    source: Some("missing dispatch event type".into()),
                })?;
                let sequence = maybe_sequence.ok_or(ProcessError {
                    kind: ProcessErrorType::Deserializing {
                        event: event.to_owned(),
                    },
                    source: Some("missing sequence".into()),
                })?;
                tracing::debug!(%event_type, %sequence, "received dispatch");

                match event_type.as_ref() {
                    "READY" => {
                        let event = Self::parse_event::<MinimalReady>(event)?;

                        self.resume_gateway_url = Some(event.data.resume_gateway_url);
                        self.session = Some(Session::new(sequence, event.data.session_id));
                        self.status = ConnectionStatus::Connected;
                    }
                    "RESUMED" => self.status = ConnectionStatus::Connected,
                    _ => {}
                }

                // READY *should* be the first received dispatch event (which
                // initializes `self.session`), but it shouldn't matter that
                // much if it's not.
                if let Some(session) = self.session.as_mut() {
                    let last_sequence = session.set_sequence(sequence);

                    // If a sequence has been skipped then we may have missed a
                    // message and should cause a reconnect so we can attempt to get
                    // that message again.
                    if sequence > last_sequence + 1 {
                        tracing::info!(
                            missed_events = sequence - (last_sequence + 1),
                            "dispatch events have been missed",
                        );
                        self.session = self
                            .close(CloseFrame::RESUME)
                            .await
                            .map_err(ProcessError::from_send)?;
                    }
                } else {
                    tracing::info!("unable to store sequence");
                }
            }
            Some(OpCode::Heartbeat) => {
                tracing::debug!("received heartbeat");
                self.heartbeat().await.map_err(ProcessError::from_send)?;
            }
            Some(OpCode::HeartbeatAck) => {
                let requested = self.latency.received().is_none() && self.latency.sent().is_some();
                if requested {
                    tracing::debug!("received heartbeat ack");
                    self.latency.record_received();
                } else {
                    tracing::info!("received unrequested heartbeat ack");
                }
            }
            Some(OpCode::Hello) => {
                let event = Self::parse_event::<Hello>(event)?;
                let heartbeat_interval = Duration::from_millis(event.data.heartbeat_interval);
                // First heartbeat should have some jitter, see
                // https://discord.com/developers/docs/topics/gateway#heartbeat-interval
                let jitter = heartbeat_interval.mul_f64(rand::random());
                tracing::debug!(?heartbeat_interval, ?jitter, "received hello");

                if self.config().ratelimit_messages() {
                    self.ratelimiter = Some(CommandRatelimiter::new(heartbeat_interval).await);
                }

                let mut interval = time::interval_at(Instant::now() + jitter, heartbeat_interval);
                interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
                self.heartbeat_interval = Some(interval);

                // Reset `Latency` since the shard might have connected to a new
                // remote which invalidates the recorded latencies.
                self.latency = Latency::new();

                match self.session() {
                    Some(session) => {
                        tracing::debug!(sequence = session.sequence(), "sending resume");
                        let resume =
                            Resume::new(session.sequence(), session.id(), self.config().token());
                        let json = command::prepare(&resume).map_err(ProcessError::from_send)?;
                        self.send(json).await.map_err(ProcessError::from_send)?;
                    }
                    None => {
                        // Can not use `MessageSender` since it is only polled
                        // after the shard is identified.
                        self.identify_handle = Some(tokio::spawn({
                            let shard_id = self.id();
                            let queue = self.config().queue().clone();

                            async move {
                                queue.request([shard_id.number(), shard_id.total()]).await;
                            }
                        }));
                    }
                }
            }
            Some(OpCode::InvalidSession) => {
                let resumable = Self::parse_event(event)?.data;
                tracing::debug!(resumable, "received invalid session");
                if resumable {
                    self.session = self
                        .close(CloseFrame::RESUME)
                        .await
                        .map_err(ProcessError::from_send)?;
                } else {
                    self.close(CloseFrame::NORMAL)
                        .await
                        .map_err(ProcessError::from_send)?;
                }
            }
            Some(OpCode::Reconnect) => {
                tracing::debug!("received reconnect");
                self.session = self
                    .close(CloseFrame::RESUME)
                    .await
                    .map_err(ProcessError::from_send)?;
            }
            _ => tracing::info!("received an unknown opcode: {raw_opcode}"),
        }

        Ok(())
    }

    /// Establishes a Websocket connection, sets the [status] to [`Resuming`] or
    /// [`Identifying`] if holding an active [`Session`] or not, and resets the
    /// [inflater].
    ///
    /// Drops [`Connection`], see [`Self::connection`] when this is okay.
    ///
    /// [`Identifying`]: ConnectionStatus::Identifying
    /// [inflater]: Self::inflater
    /// [`Resuming`]: ConnectionStatus::Resuming
    /// [status]: Self::status
    async fn reconnect(
        &mut self,
        close_code: Option<u16>,
        reconnect_attempts: u8,
    ) -> Result<(), ReceiveMessageError> {
        if reconnect_attempts != 0 {
            let secs = 2u8.saturating_pow(reconnect_attempts.into());
            time::sleep(Duration::from_secs(secs.into())).await;
        }

        let maybe_gateway_url = self
            .resume_gateway_url
            .as_deref()
            .or_else(|| self.config.proxy_url());

        self.connection = Some(
            connection::connect(maybe_gateway_url, self.config.tls())
                .await
                .map_err(|source| {
                    self.resume_gateway_url = None;
                    self.status = ConnectionStatus::Disconnected {
                        close_code,
                        reconnect_attempts: reconnect_attempts + 1,
                    };

                    source
                })?,
        );

        if self.session().is_some() {
            // Defer sending a Resume event until Hello has been received to
            // guard against the first message being a websocket close message
            // (causing us to miss replayed dispatch events).
            // We also set/reset the ratelimiter upon receiving Hello, which
            // means sending anything before then will not be recorded by the
            // ratelimiter.
            self.status = ConnectionStatus::Resuming;
        } else {
            self.status = ConnectionStatus::Identifying;
        }

        #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
        self.inflater.reset();

        Ok(())
    }

    /// Parse a JSON message into an event with minimal data for [processing].
    ///
    /// # Errors
    ///
    /// Returns a [`ProcessErrorType::Deserializing`] error type if the gateway
    /// event isn't a recognized structure, which may be the case for new or
    /// undocumented events.
    ///
    /// [processing]: Self::process
    fn parse_event<T: DeserializeOwned>(json: &str) -> Result<MinimalEvent<T>, ProcessError> {
        json::from_str::<MinimalEvent<T>>(json).map_err(|source| ProcessError {
            kind: ProcessErrorType::Deserializing {
                event: json.to_owned(),
            },
            source: Some(Box::new(source)),
        })
    }
}

/// Default identify properties to use when the user hasn't customized it in
/// [`Config::identify_properties`].
///
/// [`Config::identify_properties`]: Config::identify_properties
fn default_identify_properties() -> IdentifyProperties {
    IdentifyProperties::new("twilight.rs", "twilight.rs", OS)
}

#[cfg(test)]
mod tests {
    use super::{ConnectionStatus, Shard};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;
    use twilight_model::gateway::CloseCode;

    assert_fields!(
        ConnectionStatus::Disconnected: close_code,
        reconnect_attempts
    );
    assert_fields!(ConnectionStatus::FatallyClosed: close_code);
    assert_impl_all!(ConnectionStatus: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Shard: Debug, Send, Sync);

    #[test]
    fn connection_status_from_close_frame() {
        let empty = ConnectionStatus::from_close_code(None);
        assert_eq!(
            empty,
            ConnectionStatus::Disconnected {
                close_code: None,
                reconnect_attempts: 0
            }
        );

        let non_fatal_code = CloseCode::SessionTimedOut as u16;
        let non_fatal_status = ConnectionStatus::from_close_code(Some(non_fatal_code));

        assert_eq!(
            non_fatal_status,
            ConnectionStatus::Disconnected {
                close_code: Some(non_fatal_code),
                reconnect_attempts: 0
            }
        );

        let fatal_code = CloseCode::AuthenticationFailed;
        let fatal_status = ConnectionStatus::from_close_code(Some(fatal_code as u16));

        assert_eq!(
            fatal_status,
            ConnectionStatus::FatallyClosed {
                close_code: fatal_code
            }
        );

        let unknown_code = u16::MAX;
        let non_fatal_status = ConnectionStatus::from_close_code(Some(unknown_code));

        assert_eq!(
            non_fatal_status,
            ConnectionStatus::Disconnected {
                close_code: Some(unknown_code),
                reconnect_attempts: 0
            }
        );
    }
}
