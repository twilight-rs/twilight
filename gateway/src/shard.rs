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
//! serialized into a raw websocket message and [`Shard::send`] is called;
//! 2. The user calls [`Shard::send`] and the sending of the message goes
//! through ratelimiting via [`CommandRatelimiter`] if ratelimiting
//! [is enabled];
//! 3. The [websocket message] is sent over the [websocket connection].
//!
//! Receiving a message is a little bit more complicated, but follows as:
//!
//! 1. The user calls [`Shard::next_message`] to receive the next message from
//! the Websocket;
//! 2. If the shard is disconnected then the connection is reconnected;
//! 3. One of three things wait to happen:
//!   a. the interval for the shard to send the next heartbeat occurs, in which
//!   case [`Shard::heartbeat`] is called; or
//!   b. the shard receives a [raw websocket message] from the user over the
//!   [user channel], which is then forwarded via [`Shard::send`]; or
//!   c. the shard receives a message from Discord via the websocket connection.
//! 4. In the case of 3(a) and 3(b), 3 is repeated; otherwise...
//! 5. If the message is a close it's returned to the user; otherwise, the
//! message is [processed] by the shard;
//! 6. The raw Websocket message is returned to the user
//!
//! If the user called [`Shard::next_event`] instead of [`Shard::next_message`],
//! then the previous steps are taken and the resultant message is deserialized
//! into a [`GatewayEvent`].
//!
//! [command]: crate::Command
//! [is enabled]: Config::ratelimit_messages
//! [processed]: Shard::process
//! [websocket message]: crate::message::Message
//! [websocket connection]: Shard::connection

use crate::{
    channel::{MessageChannel, ShardMessageSender},
    command::Command,
    compression::{self, Compression},
    config::{Config, ShardId},
    error::{
        ProcessError, ProcessErrorType, ReceiveMessageError, ReceiveMessageErrorType, SendError,
        SendErrorType, ShardInitializeError, ShardInitializeErrorType,
    },
    future::{NextMessageFuture, NextMessageFutureReturn, TickHeartbeatFuture},
    json,
    latency::Latency,
    message::{CloseFrame, Message},
    ratelimiter::CommandRatelimiter,
    session::Session,
    Connection, GATEWAY_URL,
};
use crate::{command, API_VERSION};
use futures_util::{future::FutureExt, stream::Next, SinkExt, StreamExt};
use serde::Deserialize;
use std::{
    env::consts::OS,
    future::Future,
    pin::Pin,
    str,
    task::{Context, Poll},
    time::Duration,
};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_tungstenite::tungstenite::{
    protocol::WebSocketConfig, Error as TungsteniteError, Message as TungsteniteMessage,
};
use twilight_model::gateway::{
    event::{DispatchEvent, Event, GatewayEvent, GatewayEventDeserializer},
    payload::outgoing::{
        identify::{IdentifyInfo, IdentifyProperties},
        Heartbeat, Identify,
    },
    Intents, OpCode,
};
use url::Url;

/// List of opcodes internally handled by the shard via [`Shard::process`].
///
/// Used to determine what opcodes' payloads should be deserialized.
const PROCESSED_GATEWAY_OPCODES: &[u8] = &[
    OpCode::Heartbeat as u8,
    OpCode::Reconnect as u8,
    OpCode::InvalidSession as u8,
    OpCode::Hello as u8,
    OpCode::HeartbeatAck as u8,
];

/// Configuration used for Websocket connections.
///
/// `max_frame_size` and `max_message_queue` limits are disabled because
/// Discord is not a malicious actor.
///
/// `accept_unmasked_frames` and `max_send_queue` are set to their
/// defaults.
const WEBSOCKET_CONFIG: WebSocketConfig = WebSocketConfig {
    accept_unmasked_frames: false,
    max_frame_size: None,
    max_message_size: None,
    max_send_queue: None,
};

/// Disconnect a shard, optionally invalidating the session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Disconnect {
    /// Disconnect a shard and invalidate its session, requiring a new session
    /// to be identified after initializing a new connection.
    InvalidateSession,
    /// Disconnect a shard but don't invalidate its session, re-using a session
    /// with a new connection.
    Resume,
}

impl Disconnect {
    /// Create a disconnect action based on whether a session should be re-used.
    const fn from_resumable(resumable: bool) -> Self {
        if resumable {
            Self::Resume
        } else {
            Self::InvalidateSession
        }
    }
}

/// Current status of a shard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectionStatus {
    /// Shard is connected.
    ///
    /// Note that this does not mean the shard has an active gateway session.
    Connected,
    /// Shard is disconnected but may reconnect in the future.
    Disconnected,
    /// Shard has fatally closed, such as due to an invalid token.
    FatallyClosed,
}

impl ConnectionStatus {
    /// Whether the shard is connected.
    pub fn is_connected(self) -> bool {
        self == Self::Connected
    }

    /// Whether the shard has disconnected but may reconnect in the future.
    pub fn is_disconnected(self) -> bool {
        self == Self::Disconnected
    }

    /// Whether the shard has fatally closed, such as due to an invalid token.
    pub fn is_fatally_closed(self) -> bool {
        self == Self::FatallyClosed
    }
}

#[derive(Deserialize)]
struct GatewayMessage<T> {
    #[serde(rename = "d")]
    data: T,
}

// todo put hello definition in model crate, since it's not already?

#[derive(Debug, Deserialize)]
struct Hello {
    heartbeat_interval: u64,
}

#[derive(Deserialize)]
struct Ready {
    session_id: String,
}

/// Shard to run and manage a session with the gateway.
///
/// Shards are responsible for handling incoming events, process events relevant
/// to the operation of shards - such as requests from the gateway to re-connect
/// or invalidate a session - and then pass the events on to the user via an
/// event stream.
///
/// Shards will [go through a queue][`queue`] to initialize new ratelimited
/// sessions with the ratelimit. Refer to Discord's [documentation][docs:shards]
/// on shards to have a better understanding of what they are.
///
/// # Using a shard in multiple tasks
///
/// To use a shard instance in multiple tasks, consider wrapping it in an
/// [`std::sync::Arc`] or [`std::rc::Rc`].
///
/// # Examples
///
/// Create and start a shard and print new and deleted messages:
///
/// ```no_run
/// use futures::stream::StreamExt;
/// use std::env;
/// use twilight_gateway::{
///     config::{Config, ShardId},
///     EventTypeFlags,
///     Event,
///     Intents,
///     Shard,
/// };
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
/// let mut shard = Shard::with_config(ShardId::ONE, config).await?;
///
/// // Create a loop of only new messages and deleted messages.
///
/// loop {
///     let event = match shard.next_event().await {
///         Ok(event) => event,
///         Err(source) => {
///             tracing::warn!(?source, "error receiving event");
///
///             continue;
///         }
///     };
///
///     match event {
///         Event::MessageCreate(message) => {
///             println!("message received with content: {}", message.content);
///         },
///         Event::MessageDelete(message) => {
///             println!("message with ID {} deleted", message.id);
///         },
///         _ => {},
///     }
/// }
/// # Ok(()) }
/// ```
///
/// [`queue`]: crate::queue
/// [docs:shards]: https://discord.com/developers/docs/topics/gateway#sharding
#[derive(Debug)]
pub struct Shard {
    compression: Compression,
    /// User provided configuration.
    ///
    /// Configurations are provided or created in shard initializing via
    /// [`Shard::new`] or [`Shard::with_config`].
    config: Config,
    connection: Connection,
    /// Interval of how often the gateway would like the shard to
    /// [send heartbeats][`Self::heartbeat`].
    ///
    /// The interval is received in the [`GatewayEvent::Hello`] event when
    /// first opening a new [connection][`Self::connection`].
    heartbeat_interval: Option<Duration>,
    /// ID of the shard.
    id: ShardId,
    /// Recent heartbeat latency statistics.
    latency: Latency,
    /// Command ratelimiter, if it was enabled via
    /// [`Config::ratelimit_messages`].
    ratelimiter: Option<CommandRatelimiter>,
    /// Active session of the shard.
    ///
    /// The shard may not have an active session if it hasn't completed a full
    /// identify, done via [`identify`] in response to receiving a
    /// [`GatewayEvent::Hello`].
    ///
    /// [`identify`]: Self::identify
    session: Option<Session>,
    /// Current connection status of the Websocket connection, not necessarily
    /// correlating to an [active session][`Self::session`].
    status: ConnectionStatus,
    user_channel: MessageChannel,
}

impl Shard {
    /// Create a new shard with the default configuration.
    ///
    /// # Examples
    ///
    /// Create a new shard and start it, wait a second, and then print its
    /// current connection status:
    ///
    /// ```no_run
    /// use twilight_gateway::{config::ShardId, Intents, Shard};
    /// use std::{env, time::Duration};
    /// use tokio::time as tokio_time;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let token = env::var("DISCORD_TOKEN")?;
    /// let intents = Intents::GUILD_MESSAGES | Intents::GUILD_MESSAGE_TYPING;
    /// let mut shard = Shard::new(ShardId::ONE, token, intents).await?;
    ///
    /// println!("Shard connection status: {:?}", shard.status());
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Refer to [`Shard::with_config`] for possible errors.
    pub async fn new(
        id: ShardId,
        token: String,
        intents: Intents,
    ) -> Result<Self, ShardInitializeError> {
        let config = Config::builder(token, intents).build();

        Self::with_config(id, config).await
    }

    /// Create a new shard with the provided configuration.
    ///
    /// # Errors
    ///
    /// Returns a [`ShardInitializeErrorType::Establishing`] error type if the
    /// connection with the Discord gateway could not be established, such as
    /// due to network or TLS errors.
    pub async fn with_config(id: ShardId, config: Config) -> Result<Self, ShardInitializeError> {
        let session = config.session().cloned();

        // Determine whether we need to go through the queue; if the user has
        // configured an existing gateway session then we can skip it
        if session.is_none() {
            tracing::debug!(%id, "queued for identify");
            config.queue().request([id.current(), id.total()]).await;
            tracing::debug!(%id, "passed queue");
        }

        let connection = connect(id, config.gateway_url()).await?;

        Ok(Self {
            compression: Compression::new(id),
            config,
            connection,
            heartbeat_interval: None,
            id,
            latency: Latency::new(),
            ratelimiter: None,
            session,
            status: ConnectionStatus::Connected,
            user_channel: MessageChannel::new(),
        })
    }

    /// Immutable reference to the configuration used to instantiate this shard.
    pub const fn config(&self) -> &Config {
        &self.config
    }

    /// ID of the shard.
    pub const fn id(&self) -> ShardId {
        self.id
    }

    /// Whether the shard is currently connected to the gateway.
    ///
    /// The shard may not be connected if the gateway session was recently
    /// invalidated and has not yet reconnected, or if the shard was explicitly
    /// [closed] by the user.
    ///
    /// [closed]: Self::close
    pub const fn status(&self) -> ConnectionStatus {
        self.status
    }

    /// Shard latency statistics, including average latency and recent heartbeat
    /// latency times.
    pub const fn latency(&self) -> &Latency {
        &self.latency
    }

    /// Statistics about the number of available commands and when the command
    /// ratelimiter will refresh.
    ///
    /// This won't be present if ratelimiting was disabled via
    /// [`ConfigBuilder::ratelimit_messages`].
    ///
    /// [`ConfigBuilder::ratelimit_messages`]: crate::config::ConfigBuilder::ratelimit_messages
    pub const fn ratelimiter(&self) -> Option<&CommandRatelimiter> {
        self.ratelimiter.as_ref()
    }

    /// Immutable reference to the active gateway session.
    ///
    /// An active session may not be present if the shard has recently
    /// disconnected or had its session invalidated and has not yet performed a
    /// reconnect.
    pub const fn session(&self) -> Option<&Session> {
        self.session.as_ref()
    }

    /// Wait for the next Discord event from the gateway.
    ///
    /// # Errors
    ///
    /// Returns a [`ReceiveMessageErrorType::Deserializing`] error type if the
    /// message payload failed to deserialize.
    pub async fn next_event(&mut self) -> Result<Event, ReceiveMessageError> {
        let mut bytes = loop {
            match self.next_message().await? {
                Message::Binary(binary) => break binary,
                Message::Text(text) => break text.into_bytes(),
                _ => continue,
            }
        };

        json::parse(&mut bytes)
            .map(Event::from)
            .map_err(ReceiveMessageError::from_json)
    }

    /// Wait for the next raw message from the websocket connection.
    ///
    /// # Errors
    ///
    /// Returns a [`ReceiveMessageErrorType::Process`] error type if the shard
    /// failed to internally process a received event.
    ///
    /// Returns a [`ReceiveMessageErrorType::SendingMessage`] error type if the
    /// shard failed to send a message to the gateway, such as a heartbeat.
    ///
    /// [`ReceiveMessageErrorType::Process`]: crate::error::ReceiveMessageErrorType::Process
    /// [`ReceiveMessageErrorType::SendingMessage`]: crate::error::ReceiveMessageErrorType::SendingMessage
    pub async fn next_message(&mut self) -> Result<Message, ReceiveMessageError> {
        self.compression.clear();

        if self.status.is_disconnected() {
            self.connection = connect(self.id(), self.config.gateway_url())
                .await
                .map_err(ReceiveMessageError::from_reconnect)?;
            self.status = ConnectionStatus::Connected;
        }

        let message = loop {
            let future = NextMessageFuture::new(
                self.user_channel.rx_mut(),
                self.connection.next(),
                self.heartbeat_interval,
                self.latency.sent(),
            );

            // todo cleanup the match, it's not very clean
            let tungstenite_message = match future.await {
                NextMessageFutureReturn::GatewayMessage(Some(Ok(message))) => message,
                NextMessageFutureReturn::GatewayMessage(Some(Err(_source))) => {
                    self.disconnect(Disconnect::Resume);

                    TungsteniteMessage::Close(None)
                }
                NextMessageFutureReturn::GatewayMessage(None) => {
                    self.disconnect(Disconnect::Resume);

                    TungsteniteMessage::Close(None)
                }
                NextMessageFutureReturn::SendHeartbeat => {
                    self.heartbeat(None)
                        .await
                        .map_err(ReceiveMessageError::from_send)?;

                    continue;
                }
                NextMessageFutureReturn::UserChannelMessage(Some(message)) => {
                    self.send(message)
                        .await
                        .map_err(ReceiveMessageError::from_send)?;

                    continue;
                }
                NextMessageFutureReturn::UserChannelMessage(None) => {
                    unreachable!("a copy of the channel tx is owned by the shard")
                }
            };

            if let Some(message) = Message::from_tungstenite(tungstenite_message) {
                break message;
            }
        };

        match message {
            Message::Binary(ref bytes) => self.compression.extend(bytes),
            Message::Text(ref text) => self.compression.extend(text.as_bytes()),
            other => return Ok(other),
        }

        if let Err(source) = self.process().await {
            return Err(ReceiveMessageError {
                kind: ReceiveMessageErrorType::Process,
                source: Some(Box::new(source)),
            });
        }

        Ok(match message {
            Message::Binary(_) => Message::Binary(self.compression.take()),
            other => other,
        })
    }

    /// Send a command over the gateway.
    ///
    /// # Examples
    ///
    /// Request members whose names start with "tw" in a guild:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::env;
    /// use twilight_gateway::{config::ShardId, Intents, Shard};
    /// use twilight_model::{
    ///     gateway::payload::outgoing::RequestGuildMembers,
    ///     id::Id,
    /// };
    ///
    /// let intents = Intents::GUILD_VOICE_STATES;
    /// let token = env::var("DISCORD_TOKEN")?;
    ///
    /// let mut shard = Shard::new(ShardId::ONE, token, intents).await?;
    ///
    /// // Query members whose names start with "tw" and limit the results to
    /// // 10 members.
    /// let request =
    ///     RequestGuildMembers::builder(Id::new(1))
    ///         .query("tw", Some(10));
    ///
    /// // Send the request over the shard.
    /// shard.command(&request).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`SendErrorType::Sending`] error type if the message could
    /// not be sent over the websocket. This indicates the shard is either
    /// currently restarting or closed and will restart.
    ///
    /// Returns a [`SendErrorType::Serializing`] error type if the provided
    /// command failed to serialize.
    pub async fn command(&mut self, command: &impl Command) -> Result<(), SendError> {
        let message = command::prepare(command)?;

        self.send(message).await
    }

    /// Send a raw websocket message.
    ///
    /// # Examples
    ///
    /// Send a normal close, which [`Shard::close`] is shorthand for:
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::{borrow::Cow, env};
    /// use twilight_gateway::{
    ///     config::ShardId,
    ///     message::{CloseFrame, Message},
    ///     Intents,
    ///     Shard,
    /// };
    ///
    /// let token = env::var("DISCORD_TOKEN")?;
    /// let mut shard = Shard::new(ShardId::ONE, token, Intents::GUILDS).await?;
    ///
    /// let message = Message::Close(Some(CloseFrame::NORMAL));
    /// shard.send(message).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`SendErrorType::Sending`] error type if the message could
    /// not be sent over the websocket. This indicates the shard is either
    /// currently restarting or closed and will restart.
    pub async fn send(&mut self, message: Message) -> Result<(), SendError> {
        if let Some(ref ratelimiter) = self.ratelimiter {
            ratelimiter.acquire_one().await;
        }

        self.connection
            .send(message.into_tungstenite())
            .await
            .map_err(|source| {
                self.disconnect(Disconnect::Resume);

                SendError {
                    kind: SendErrorType::Sending,
                    source: Some(Box::new(source)),
                }
            })
    }

    /// Retrieve a channel to send messages over the shard to the gateway.
    ///
    /// This is primarily useful for sending to other tasks and threads where
    /// the shard won't be available.
    pub fn sender(&mut self) -> ShardMessageSender {
        self.user_channel.sender()
    }

    /// Close the shard's connection, providing a close frame indicating whether
    /// a resume is intended.
    ///
    /// Returns the gateway session of the shard, which may be provided via
    /// [`ConfigBuilder::session`] to create a new shard that will resume the
    /// gateway session.
    ///
    /// If sending a close frame such as [`CloseFrame::NORMAL`] then Discord
    /// will invalidate the shard's session, showing the application's bot as
    /// offline. If sending a close frame such as [`CloseFrame::RESUME`] then
    /// Discord will not invalidate the shard's session and will continue to
    /// show the application's bot as online until its presence times out.
    ///
    /// # Errors
    ///
    /// Returns a [`SendErrorType::Sending`] error type if the message could
    /// not be sent over the websocket. This indicates the shard is either
    /// currently restarting or closed and will restart.
    ///
    /// [`ConfigBuilder::session`]: crate::config::ConfigBuilder::session
    pub async fn close(
        &mut self,
        maybe_close_frame: Option<CloseFrame<'static>>,
    ) -> Result<Option<Session>, SendError> {
        let message = Message::Close(maybe_close_frame);

        self.send(message).await?;

        Ok(self.session.take())
    }

    /// Disconnect the shard's Websocket connection, optionally invalidating the
    /// session.
    fn disconnect(&mut self, disconnect: Disconnect) {
        tracing::debug!(id = %self.id(), "disconnected");
        self.status = ConnectionStatus::Disconnected;

        if disconnect == Disconnect::InvalidateSession {
            tracing::debug!(id = %self.id(), "session invalidated");
            self.session = None;
        }
    }

    /// Send a heartbeat, optionally overriding the session's sequence.
    async fn heartbeat(&mut self, override_sequence: Option<u64>) -> Result<(), SendError> {
        let session_sequence = self.session.as_ref().map(Session::sequence);

        if let Some(sequence) = override_sequence.or(session_sequence) {
            let command = Heartbeat::new(sequence);
            self.command(&command).await?;

            self.latency.track_sent();
        }

        Ok(())
    }

    /// Identify a new session with the Discord gateway.
    ///
    /// # Errors
    ///
    /// Refer to [`command`][`Self::command`] for possible errors.
    async fn identify(&mut self) -> Result<(), SendError> {
        let properties = self
            .config()
            .identify_properties()
            .cloned()
            .unwrap_or_else(default_identify_properties);

        let identify = Identify::new(IdentifyInfo {
            compress: false,
            large_threshold: self.config.large_threshold(),
            intents: self.config.intents(),
            properties,
            shard: Some([self.id().current(), self.id().total()]),
            presence: self.config.presence().cloned(),
            token: self.config.token().to_owned(),
        });

        self.command(&identify).await
    }

    async fn process(&mut self) -> Result<(), ProcessError> {
        let buffer = match self.compression.message_mut() {
            Ok(Some(buffer)) => buffer,
            Ok(None) => return Ok(()),
            Err(source) => return Err(ProcessError::from_compression(source)),
        };

        // Instead of returning the event type, we return whether the event type
        // is the one we handle in order to get around having both an immutable
        // and mutable lifetime to the buffer.
        let (opcode, maybe_sequence, is_ready) = {
            let json = String::from_utf8_lossy(buffer);
            let deserializer = GatewayEventDeserializer::from_json(&json).ok_or(ProcessError {
                kind: ProcessErrorType::ParsingPayload,
                source: None,
            })?;

            (
                deserializer.op(),
                deserializer.sequence(),
                deserializer.event_type_ref() == Some("READY"),
            )
        };

        if let Some(sequence) = maybe_sequence {
            if let Some(session) = self.session.as_mut() {
                let last_sequence = session.set_sequence(sequence);

                if sequence > last_sequence + 1 {
                    self.disconnect(Disconnect::Resume);

                    // todo document why early return
                    return Ok(());
                }
            }
        }

        // We can do a few little optimization tricks here. For the
        // "heartbeat ack" and "reconnect" opcodes we can construct
        // the gateway events without needing to go through a serde
        // context.
        //
        // Additionally, the processor cares about the "resumed"
        // dispatch event type, which has no payload and can be constructed.
        //
        // This might not be shaving off entire milliseconds for these few
        // events each time, but it certainly adds up.
        let is_gateway_event = PROCESSED_GATEWAY_OPCODES.contains(&opcode);

        if !is_gateway_event && !is_ready {
            return Ok(());
        }

        let event =
            json::parse_gateway_event(opcode, maybe_sequence, is_ready.then(|| "READY"), buffer)
                .map_err(ProcessError::from_json)?;

        match event {
            GatewayEvent::Dispatch(sequence, dispatch) => {
                debug_assert_eq!(sequence, 1, "ready should be the first sequence");

                if let DispatchEvent::Ready(ready) = *dispatch {
                    self.status = ConnectionStatus::Connected;
                    self.session = Some(Session::new(sequence, ready.session_id));
                } else {
                    unreachable!("only ready dispatches are handled")
                }
            }
            GatewayEvent::Heartbeat(sequence) => {
                if let Err(source) = self.heartbeat(Some(sequence)).await {
                    self.disconnect(Disconnect::Resume);

                    return Err(ProcessError::from_send(source));
                }
            }
            GatewayEvent::HeartbeatAck => {
                self.latency.track_received();
            }
            GatewayEvent::Hello(heartbeat_interval) => {
                let heartbeat_duration = Duration::from_millis(heartbeat_interval);
                self.heartbeat_interval = Some(heartbeat_duration);

                if self.config().ratelimit_messages() {
                    self.ratelimiter = Some(CommandRatelimiter::new(heartbeat_interval));
                }

                self.identify().await.map_err(ProcessError::from_send)?;
            }
            GatewayEvent::InvalidateSession(can_resume) => {
                self.disconnect(Disconnect::from_resumable(can_resume));
            }
            GatewayEvent::Reconnect => {
                self.disconnect(Disconnect::Resume);
            }
        }

        Ok(())
    }
}

/// Configure a URL with the requested Gateway version and encoding.
fn configure_url(url: &mut String) {
    // Discord's documentation states:
    //
    // "Generally, it is a good idea to explicitly pass the gateway version
    // and encoding".
    //
    // <https://discord.com/developers/docs/topics/gateway#connecting-gateway-url-query-string-params>
    url.push_str("?v=");
    url.push_str(&API_VERSION.to_string());

    url.push_str("&encoding=json");

    compression::add_url_feature(url);
}

/// Connect to the gateway for a given URL, defaulting if not present.
///
/// If a URL isn't provided then [`GATEWAY_URL`] is used. The Shard ID is used
/// only for tracing logs.
///
/// # Errors
///
/// Returns a [`ShardInitializeErrorType::Establishing`] error type if the
/// connection with the Discord gateway could not be established, such as
/// due to network or TLS errors.
async fn connect(id: ShardId, maybe_url: Option<&str>) -> Result<Connection, ShardInitializeError> {
    let mut raw_url = maybe_url.unwrap_or(GATEWAY_URL).to_owned();
    configure_url(&mut raw_url);

    let url = Url::parse(&raw_url).expect("gateway url is valid");

    tracing::debug!(%id, ?url, "shaking hands with remote");
    let (stream, _) =
        tokio_tungstenite::connect_async_tls_with_config(url, Some(WEBSOCKET_CONFIG), None)
            .await
            .map_err(|source| ShardInitializeError {
                kind: ShardInitializeErrorType::Establishing,
                source: Some(Box::new(source)),
            })?;
    tracing::debug!(%id, "shook hands with remote");

    Ok(stream)
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
    use super::{SendError, SendErrorType, Shard, ShardInitializeError, ShardInitializeErrorType};
    use crate::API_VERSION;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{error::Error, fmt::Debug};

    assert_impl_all!(SendErrorType: Debug, Send, Sync);
    assert_impl_all!(SendError: Error, Send, Sync);
    assert_impl_all!(ShardInitializeErrorType: Debug, Send, Sync);
    assert_impl_all!(ShardInitializeError: Error, Send, Sync);
    assert_impl_all!(Shard: Debug, Send, Sync);

    /// Test that [`super::configure_url`] formats URLs as expected.
    ///
    /// There's a little byte trickery to avoid an allocation in it, so we just
    /// want to make sure it formats right.
    #[test]
    fn test_configure_url() {
        let mut buf = String::new();
        super::configure_url(&mut buf);

        assert_eq!(
            format!("?v={}&encoding=json&compress=zlib-stream", API_VERSION),
            buf
        );
    }
}
