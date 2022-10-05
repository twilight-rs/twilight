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
//! # Reconnecting
//!
//! If a custom gateway URL is used when reconnecting, the shard will always
//! prefer it over the [`resume_gateway_url`]. Proper reconnection is left to
//! the proxy.
//!
//! [`GatewayEvent`]: twilight_model::gateway::event::GatewayEvent
//! [`resume_gateway_url`]: twilight_model::gateway::payload::incoming::Ready::resume_gateway_url
//! [command]: crate::Command
//! [is enabled]: Config::ratelimit_messages
//! [processed]: Shard::process
//! [websocket connection]: Shard::connection
//! [websocket message]: crate::message::Message

use crate::{
    channel::{MessageChannel, MessageSender},
    command::{self, Command},
    compression::Compression,
    connection::{self, Connection},
    error::{
        ProcessError, ProcessErrorType, ReceiveMessageError, ReceiveMessageErrorType, SendError,
        SendErrorType, ShardInitializeError,
    },
    future::{self, NextMessageFuture, NextMessageFutureOutput},
    json,
    latency::Latency,
    message::{CloseFrame, Message},
    ratelimiter::CommandRatelimiter,
    session::Session,
    Config, ShardId,
};
use futures_util::{SinkExt, StreamExt};
use serde::{de::DeserializeOwned, Deserialize};
use std::{env::consts::OS, str, time::Duration};
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
use twilight_model::gateway::{
    event::{Event, GatewayEventDeserializer},
    payload::{
        incoming::Hello,
        outgoing::{
            identify::{IdentifyInfo, IdentifyProperties},
            Heartbeat, Identify, Resume,
        },
    },
    CloseCode, Intents, OpCode,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConnectionStatus {
    /// Shard is connected.
    ///
    /// Note that this does not mean the shard has an active gateway session.
    Connected,
    /// Shard is disconnected but may reconnect in the future.
    Disconnected {
        /// Close code, if available.
        ///
        /// May not be available if the shard was closed via an event, such as
        /// [`GatewayEvent::InvalidateSession`].
        ///
        ///
        /// [`GatewayEvent::InvalidateSession`]: twilight_model::gateway::event::GatewayEvent::InvalidateSession
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
        ///
        /// The close code may be able to parse into [`CloseCode`] if it's a
        /// known close code. Unknown close codes aren't considered fatal.
        close_code: u16,
    },
}

impl ConnectionStatus {
    /// Determine the connection status from the close code.
    ///
    /// Defers to [`CloseCode::can_reconnect`] to determine whether the
    /// connection can be reconnected, defaulting to [`Self::FatallyClosed`] if
    /// the close code is unknown.
    fn from_close_frame(maybe_frame: Option<&CloseFrame<'static>>) -> Self {
        let raw_code = if let Some(frame) = maybe_frame {
            frame.code()
        } else {
            return Self::Disconnected {
                close_code: None,
                reconnect_attempts: 0,
            };
        };

        let can_reconnect = CloseCode::try_from(raw_code)
            .map(CloseCode::can_reconnect)
            .unwrap_or(true);

        if can_reconnect {
            Self::Disconnected {
                close_code: Some(raw_code),
                reconnect_attempts: 0,
            }
        } else {
            Self::FatallyClosed {
                close_code: raw_code,
            }
        }
    }

    /// Whether the shard is connected.
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
    resume_gateway_url: String,
    /// ID of the new identified session.
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
/// # Sending shard commands in different tasks
///
/// Because a shard itself can't be used in multiple tasks it's not possible to
/// directly send [gateway commands] over a shard. To solve this
/// [`Shard::sender`] can be used to receive an MPSC channel to send commands.
///
/// # Examples
///
/// Create and start a shard and print new and deleted messages:
///
/// ```no_run
/// use futures::stream::StreamExt;
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
/// [`queue`]: crate::queue
/// [docs:shards]: https://discord.com/developers/docs/topics/gateway#sharding
/// [gateway commands]: Shard::command
#[derive(Debug)]
pub struct Shard {
    /// Abstraction to decompress Websocket messages, if compression is enabled.
    compression: Compression,
    /// User provided configuration.
    ///
    /// Configurations are provided or created in shard initializing via
    /// [`Shard::new`] or [`Shard::with_config`].
    config: Config,
    /// Websocket connection, which may be connected to Discord's gateway.
    connection: Connection,
    /// Interval of how often the gateway would like the shard to
    /// [send heartbeats][`Self::heartbeat`].
    ///
    /// The interval is received in the [`GatewayEvent::Hello`] event when
    /// first opening a new [connection].
    ///
    /// [`GatewayEvent::Hello`]: twilight_model::gateway::event::GatewayEvent::Hello
    /// [connection]: Self::connection
    heartbeat_interval: Option<Duration>,
    /// ID of the shard.
    id: ShardId,
    /// Recent heartbeat latency statistics.
    latency: Latency,
    /// Command ratelimiter, if it was enabled via
    /// [`Config::ratelimit_messages`].
    ratelimiter: Option<CommandRatelimiter>,
    /// Used for resuming connections.
    resume_gateway_url: Option<String>,
    /// Active session of the shard.
    ///
    /// The shard may not have an active session if it hasn't completed a full
    /// identify, done via [`identify`] in response to receiving a
    /// [`GatewayEvent::Hello`].
    ///
    /// [`GatewayEvent::Hello`]: twilight_model::gateway::event::GatewayEvent::Hello
    /// [`identify`]: Self::identify
    session: Option<Session>,
    /// Current connection status of the Websocket connection, not necessarily
    /// correlating to an [active session][`Self::session`].
    status: ConnectionStatus,
    /// Messages from the user to be relayed and sent over the Websocket
    /// connection.
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
    /// use std::{env, time::Duration};
    /// use tokio::time as tokio_time;
    /// use twilight_gateway::{Intents, Shard, ShardId};
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
    ///
    /// [`ShardInitializeErrorType::Establishing`]: crate::error::ShardInitializeErrorType::Establishing
    pub async fn with_config(
        shard_id: ShardId,
        mut config: Config,
    ) -> Result<Self, ShardInitializeError> {
        let session = config.take_session();

        let connection = connection::connect(shard_id, config.gateway_url(), config.tls()).await?;

        Ok(Self {
            compression: Compression::new(shard_id),
            config,
            connection,
            heartbeat_interval: None,
            id: shard_id,
            latency: Latency::new(),
            ratelimiter: None,
            resume_gateway_url: None,
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
    pub const fn status(&self) -> &ConnectionStatus {
        &self.status
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
    /// [`ConfigBuilder::ratelimit_messages`]: crate::ConfigBuilder::ratelimit_messages
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
    pub async fn next_event(&mut self) -> Result<Event, ReceiveMessageError> {
        loop {
            let mut bytes = loop {
                match self.next_message().await? {
                    Message::Binary(bytes) => break bytes,
                    Message::Text(text) => break text.into_bytes(),
                    _ => continue,
                }
            };

            // loop if event is unwanted
            if let Some(event) = json::parse(self.config.event_types(), &mut bytes)
                .map_err(ReceiveMessageError::from_json)?
            {
                return Ok(event.into());
            }
        }
    }

    /// Wait for the next raw message from the websocket connection.
    ///
    /// # Errors
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
    pub async fn next_message(&mut self) -> Result<Message, ReceiveMessageError> {
        self.compression.clear();

        match self.status {
            ConnectionStatus::Connected => {}
            ConnectionStatus::Disconnected {
                close_code,
                reconnect_attempts,
                ..
            } => {
                self.reconnect(close_code, reconnect_attempts).await?;
            }
            ConnectionStatus::FatallyClosed { close_code } => {
                return Err(ReceiveMessageError::from_fatally_closed(close_code));
            }
        }

        let message = loop {
            let future = NextMessageFuture::new(
                self.user_channel.rx_mut(),
                self.connection.next(),
                self.heartbeat_interval,
                self.latency.sent(),
            );

            let tungstenite_message = match future.await {
                NextMessageFutureOutput::Message(Some(message)) => message,
                NextMessageFutureOutput::Message(None) => {
                    self.disconnect(Disconnect::Resume);

                    TungsteniteMessage::Close(None)
                }
                NextMessageFutureOutput::SendHeartbeat => {
                    self.heartbeat(None)
                        .await
                        .map_err(ReceiveMessageError::from_send)?;

                    continue;
                }
                NextMessageFutureOutput::UserChannelMessage(message) => {
                    self.send(message)
                        .await
                        .map_err(ReceiveMessageError::from_send)?;

                    continue;
                }
            };

            if let Some(message) = Message::from_tungstenite(tungstenite_message) {
                break message;
            }
        };

        match message {
            Message::Close(maybe_frame) => {
                self.status = ConnectionStatus::from_close_frame(maybe_frame.as_ref());

                return Ok(Message::Close(maybe_frame));
            }
            Message::Binary(ref bytes) => self.compression.extend(bytes),
            Message::Text(ref text) => self.compression.extend(text.as_bytes()),
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
    /// use twilight_gateway::{Intents, Shard, ShardId};
    /// use twilight_model::{gateway::payload::outgoing::RequestGuildMembers, id::Id};
    ///
    /// let intents = Intents::GUILD_VOICE_STATES;
    /// let token = env::var("DISCORD_TOKEN")?;
    ///
    /// let mut shard = Shard::new(ShardId::ONE, token, intents).await?;
    ///
    /// // Query members whose names start with "tw" and limit the results to
    /// // 10 members.
    /// let request = RequestGuildMembers::builder(Id::new(1)).query("tw", Some(10));
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
    ///
    /// [`send`]: Self::send
    pub async fn command(&mut self, command: &impl Command) -> Result<(), SendError> {
        let message = command::prepare(command)?;

        self.send(message).await
    }

    /// Send a raw websocket message.
    ///
    /// First calls in to the shard's [ratelimiter] if one [was enabled] in the
    /// shard's configuration.
    ///
    /// # Examples
    ///
    /// Send a normal close, which [`Shard::close`] is shorthand for:
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::{borrow::Cow, env};
    /// use twilight_gateway::{
    ///     message::{CloseFrame, Message},
    ///     Intents, Shard, ShardId,
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
    ///
    /// [ratelimiter]: CommandRatelimiter
    /// [was enabled]: crate::ConfigBuilder::ratelimit_messages
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
    pub fn sender(&self) -> MessageSender {
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
    /// [`ConfigBuilder::session`]: crate::ConfigBuilder::session
    pub async fn close(
        &mut self,
        close_frame: CloseFrame<'static>,
    ) -> Result<Option<Session>, SendError> {
        let message = Message::Close(Some(close_frame));

        self.send(message).await?;

        Ok(self.session.take())
    }

    /// Disconnect the shard's Websocket connection, optionally invalidating the
    /// session.
    fn disconnect(&mut self, disconnect: Disconnect) {
        tracing::debug!(shard_id = %self.id(), "disconnected");
        self.status = ConnectionStatus::Disconnected {
            close_code: None,
            reconnect_attempts: 0,
        };

        if disconnect == Disconnect::InvalidateSession {
            tracing::debug!(shard_id = %self.id(), "session invalidated");
            self.session = None;
        }
    }

    /// Send a heartbeat, optionally overriding the session's sequence.
    ///
    /// Closes the connection and resumes if previous sent heartbeat never got
    /// a reply.
    ///
    /// # Panics
    ///
    /// Panics if called without an `override_sequence` and without having
    /// received an [`OpCode::Hello`] event.
    #[track_caller]
    async fn heartbeat(&mut self, override_sequence: Option<u64>) -> Result<(), SendError> {
        let is_first_heartbeat = self.heartbeat_interval.is_some() && self.latency.sent().is_none();

        // Discord never replied to the last heartbeat, connection is failed or
        // "zombied", see
        // https://discord.com/developers/docs/topics/gateway#heartbeat-interval-example-heartbeat-ack
        if !is_first_heartbeat && self.latency().received().is_none() {
            tracing::warn!("connection failed or \"zombied\"");
            self.session = self.close(CloseFrame::RESUME).await?;
            self.disconnect(Disconnect::Resume);
        } else {
            let sequence = override_sequence
                .or_else(|| self.session.as_ref().map(Session::sequence))
                .unwrap();

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

        tracing::debug!(shard_id = %self.id(), "queued for identify");
        self.config
            .queue()
            .request([self.id.number(), self.id.total()])
            .await;
        tracing::debug!(shard_id = %self.id(), "passed queue");

        let identify = Identify::new(IdentifyInfo {
            compress: false,
            large_threshold: self.config.large_threshold(),
            intents: self.config.intents(),
            properties,
            shard: Some([self.id().number(), self.id().total()]),
            presence: self.config.presence().cloned(),
            token: self.config.token().to_owned(),
        });

        self.command(&identify).await
    }

    /// Process the buffer of the current websocket message to update the shard's
    /// state.
    ///
    /// # Errors
    ///
    /// Returns a [`ProcessErrorType::Compression`] error type if the buffer
    /// could not be read. This may be because the message was invalid or not
    /// all frames have been received.
    ///
    /// Returns a [`ProcessErrorType::Deserializing`] error type if the gateway
    /// event isn't a recognized structure, which may be the case for new or
    /// undocumented events.
    ///
    /// Returns a [`ProcessErrorType::ParsingPayload`] error type if the buffer
    /// isn't a valid [`GatewayEvent`]. This may happen if the opcode isn't
    /// present.
    ///
    /// Returns a [`ProcessErrorType::SendingMessage`] error type if a Websocket
    /// message couldn't be sent over the connection, which may be the case if
    /// the connection isn't connected.
    ///
    /// [`GatewayEvent`]: twilight_model::gateway::event::GatewayEvent
    async fn process(&mut self) -> Result<(), ProcessError> {
        let buffer = match self.compression.message_mut() {
            Ok(Some(buffer)) => buffer,
            Ok(None) => return Ok(()),
            Err(source) => return Err(ProcessError::from_compression(source)),
        };

        // Instead of returning the event type, we return whether the event type
        // is a Ready event, which is the only one we handle. This gets around
        // having both an immutable and mutable lifetime to the buffer.
        let (raw_opcode, maybe_sequence, is_ready) = {
            let json = str::from_utf8(buffer).map_err(|source| ProcessError {
                kind: ProcessErrorType::ParsingPayload,
                source: Some(Box::new(source)),
            })?;
            let deserializer = GatewayEventDeserializer::from_json(json).ok_or(ProcessError {
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

                // If a sequence has been skipped then we may have missed a
                // message and should cause a reconnect so we can attempt to get
                // that message again.
                if sequence > last_sequence + 1 {
                    self.disconnect(Disconnect::Resume);

                    return Ok(());
                }
            }
        }

        match OpCode::try_from(raw_opcode) {
            Ok(OpCode::Event) if is_ready => {
                let event = Self::parse_event::<MinimalReady>(buffer)?;
                let sequence = maybe_sequence.ok_or(ProcessError {
                    kind: ProcessErrorType::ParsingPayload,
                    source: None,
                })?;

                self.resume_gateway_url = Some(event.data.resume_gateway_url);
                self.status = ConnectionStatus::Connected;
                self.session = Some(Session::new(sequence, event.data.session_id));
            }
            Ok(OpCode::Heartbeat) => {
                let event = Self::parse_event(buffer)?;

                if let Err(source) = self.heartbeat(Some(event.data)).await {
                    self.disconnect(Disconnect::Resume);

                    return Err(ProcessError::from_send(source));
                }
            }
            Ok(OpCode::HeartbeatAck) => {
                self.latency.track_received();
            }
            Ok(OpCode::Hello) => {
                let event = Self::parse_event::<Hello>(buffer)?;
                let interval = event.data.heartbeat_interval;
                let heartbeat_duration = Duration::from_millis(interval);
                self.heartbeat_interval = Some(heartbeat_duration);

                if self.config().ratelimit_messages() {
                    self.ratelimiter = Some(CommandRatelimiter::new(interval));
                }

                if self.session.is_none() {
                    self.identify().await.map_err(ProcessError::from_send)?;
                }
            }
            Ok(OpCode::InvalidSession) => {
                let event = Self::parse_event(buffer)?;
                self.disconnect(Disconnect::from_resumable(event.data));
            }
            Ok(OpCode::Reconnect) => {
                self.disconnect(Disconnect::Resume);
            }
            _ => {}
        }

        Ok(())
    }

    /// Reconnect to the gateway with a new Websocket connection.
    ///
    /// Resumes the connection if a session is available, clears the
    /// [compression] buffer, and sets the [status] to
    /// [`ConnectionStatus::Connected`].
    ///
    /// [compression]: Self::compression
    /// [status]: Self::status
    async fn reconnect(
        &mut self,
        close_code: Option<u16>,
        reconnect_attempts: u8,
    ) -> Result<(), ReceiveMessageError> {
        future::reconnect_delay(reconnect_attempts).await;

        let maybe_gateway_url = self
            .config
            .gateway_url()
            .or(self.resume_gateway_url.as_deref());

        self.connection = connection::connect(self.id(), maybe_gateway_url, self.config.tls())
            .await
            .map_err(|source| {
                self.status = ConnectionStatus::Disconnected {
                    close_code,
                    reconnect_attempts: reconnect_attempts + 1,
                };

                ReceiveMessageError::from_reconnect(source)
            })?;

        if let Some(session) = self.session() {
            let resume = Resume::new(session.sequence(), session.id(), self.config().token());
            self.command(&resume)
                .await
                .map_err(ReceiveMessageError::from_send)?;
        }

        self.compression.reset();
        self.status = ConnectionStatus::Connected;

        Ok(())
    }

    /// Parse a JSON buffer into an event with minimal data for [processing].
    ///
    /// # Errors
    ///
    /// Returns a [`ProcessErrorType::Deserializing`] error type if the gateway
    /// event isn't a recognized structure, which may be the case for new or
    /// undocumented events.
    ///
    /// [processing]: Self::process
    fn parse_event<T: DeserializeOwned>(
        buffer: &mut [u8],
    ) -> Result<MinimalEvent<T>, ProcessError> {
        json::from_slice::<MinimalEvent<T>>(buffer).map_err(ProcessError::from_json)
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
    use super::{ConnectionStatus, Disconnect, Shard};
    use crate::message::CloseFrame;
    use static_assertions::{assert_fields, assert_impl_all, const_assert};
    use std::fmt::Debug;
    use twilight_model::gateway::CloseCode;

    assert_fields!(
        ConnectionStatus::Disconnected: close_code,
        reconnect_attempts
    );
    assert_fields!(ConnectionStatus::FatallyClosed: close_code);
    assert_impl_all!(ConnectionStatus: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Shard: Debug, Send, Sync);
    const_assert!(matches!(
        Disconnect::from_resumable(true),
        Disconnect::Resume
    ));
    const_assert!(matches!(
        Disconnect::from_resumable(false),
        Disconnect::InvalidateSession
    ));

    #[test]
    fn connection_status_from_close_frame() {
        let empty = ConnectionStatus::from_close_frame(None);
        assert_eq!(
            empty,
            ConnectionStatus::Disconnected {
                close_code: None,
                reconnect_attempts: 0
            }
        );

        let non_fatal_code = CloseCode::SessionTimedOut as u16;
        let non_fatal_frame = CloseFrame::new(non_fatal_code, "");
        let non_fatal_status = ConnectionStatus::from_close_frame(Some(&non_fatal_frame));

        assert_eq!(
            non_fatal_status,
            ConnectionStatus::Disconnected {
                close_code: Some(non_fatal_code),
                reconnect_attempts: 0
            }
        );

        let fatal_code = CloseCode::AuthenticationFailed as u16;
        let fatal_frame = CloseFrame::new(fatal_code, "");
        let fatal_status = ConnectionStatus::from_close_frame(Some(&fatal_frame));

        assert_eq!(
            fatal_status,
            ConnectionStatus::FatallyClosed {
                close_code: fatal_code
            }
        );
    }
}
