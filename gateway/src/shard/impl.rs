use super::{
    builder::ShardBuilder,
    config::Config,
    event::Events,
    processor::{ConnectingError, Latency, Session, ShardProcessor},
    sink::ShardSink,
    stage::Stage,
};
use crate::{listener::Listeners, EventTypeFlags};
use async_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Error as TungsteniteError, Message,
};
use futures_channel::mpsc::TrySendError;
use futures_util::{
    future::{self, AbortHandle},
    stream::StreamExt,
};
use once_cell::sync::OnceCell;
use std::{
    borrow::Cow,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    sync::{atomic::Ordering, Arc},
};
use tokio::sync::watch::Receiver as WatchReceiver;
use twilight_http::Error as HttpError;
use twilight_model::gateway::event::Event;
use url::ParseError as UrlParseError;

#[cfg(not(feature = "simd-json"))]
use serde_json::Error as JsonError;
#[cfg(feature = "simd-json")]
use simd_json::Error as JsonError;

/// Sending a command failed.
#[derive(Debug)]
pub enum CommandError {
    /// Sending the payload over the WebSocket failed. This is indicative of a
    /// shutdown shard.
    Sending {
        /// Reason for the error.
        source: TrySendError<Message>,
    },
    /// Serializing the payload as JSON failed.
    Serializing {
        /// Reason for the error.
        source: JsonError,
    },
    /// Shard's session is inactive because the shard hasn't been started.
    SessionInactive {
        /// Reason for the error.
        source: SessionInactiveError,
    },
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("the shard session is inactive and has not been started")
    }
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Sending { source } => Some(source),
            Self::Serializing { source } => Some(source),
            Self::SessionInactive { source } => Some(source),
        }
    }
}

/// Shard's session is inactive.
///
/// This means that the shard has not yet been started.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionInactiveError;

impl Display for SessionInactiveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("the shard session is inactive and was not started")
    }
}

impl Error for SessionInactiveError {}

/// Starting a shard and connecting to the gateway failed.
#[derive(Debug)]
pub enum ShardStartError {
    /// Establishing a connection to the gateway failed.
    Establishing {
        /// Reason for the error.
        source: TungsteniteError,
    },
    /// Parsing the gateway URL provided by Discord to connect to the gateway
    /// failed due to an invalid URL.
    ParsingGatewayUrl {
        /// Reason for the error.
        source: UrlParseError,
        /// URL that couldn't be parsed.
        url: String,
    },
    /// Retrieving the gateway URL via the Twilight HTTP client failed.
    RetrievingGatewayUrl {
        /// The reason for the error.
        source: HttpError,
    },
}

impl Display for ShardStartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Establishing { source } => Display::fmt(source, f),
            Self::ParsingGatewayUrl { source, url } => f.write_fmt(format_args!(
                "the gateway url `{}` is invalid: {}",
                url, source,
            )),
            Self::RetrievingGatewayUrl { .. } => {
                f.write_str("retrieving the gateway URL via HTTP failed")
            }
        }
    }
}

impl Error for ShardStartError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Establishing { source } => Some(source),
            Self::ParsingGatewayUrl { source, .. } => Some(source),
            Self::RetrievingGatewayUrl { source } => Some(source),
        }
    }
}

impl From<ConnectingError> for ShardStartError {
    fn from(error: ConnectingError) -> Self {
        match error {
            ConnectingError::Establishing { source } => Self::Establishing { source },
            ConnectingError::ParsingUrl { source, url } => Self::ParsingGatewayUrl { source, url },
        }
    }
}

/// Information about a shard, including its latency, current session sequence,
/// and connection stage.
#[derive(Clone, Debug)]
pub struct Information {
    id: u64,
    latency: Latency,
    seq: u64,
    stage: Stage,
}

impl Information {
    /// Return the ID of the shard.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Return an immutable reference to the latency information for the shard.
    ///
    /// This includes the average latency over all time, and the latency
    /// information for the 5 most recent heartbeats.
    pub fn latency(&self) -> &Latency {
        &self.latency
    }

    /// Current sequence of the connection.
    ///
    /// This is the number of the event that was received this session (without
    /// reconnecting). A larger number typically correlates that the shard has
    /// been connected for a longer time, while a smaller number typically
    /// correlates to meaning that it's been connected for a less amount of
    /// time.
    pub fn seq(&self) -> u64 {
        self.seq
    }

    /// Current stage of the shard.
    ///
    /// For example, once a shard is fully booted then it will be [`Connected`].
    ///
    /// [`Connected`]: stage/enum.Stage.html#variant.Connected
    pub fn stage(&self) -> Stage {
        self.stage
    }
}
/// Details to resume a gateway session.
#[derive(Clone, Debug)]
pub struct ResumeSession {
    /// ID of the session being resumed.
    pub session_id: String,
    /// Last received event sequence number.
    pub sequence: u64,
}

#[derive(Debug)]
struct ShardRef {
    config: Arc<Config>,
    listeners: Listeners<Event>,
    processor_handle: OnceCell<AbortHandle>,
    session: OnceCell<WatchReceiver<Arc<Session>>>,
}

/// Shard to run and manage a session with the gateway.
///
/// Shards are responsible for handling incoming events, process events relevant
/// to the operation of shards - such as requests from the gateway to re-connect
/// or invalidate a session - and then pass the events on to the user via an
/// [event stream][`events`].
///
/// Shards will [go through a queue][`queue`] to initialize new ratelimited
/// sessions with the ratelimit. Refer to Discord's [documentation][docs:shards]
/// on shards to have a better understanding of what they are.
///
/// # Examples
///
/// Create and start a shard and print new and deleted messages:
///
/// ```no_run
/// use futures::stream::StreamExt;
/// use std::env;
/// use twilight_gateway::{EventTypeFlags, Event, Shard};
///
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Use the value of the "DISCORD_TOKEN" environment variable as the bot's
/// // token. Of course, you may pass this into your program however you want.
/// let token = env::var("DISCORD_TOKEN")?;
/// let mut shard = Shard::new(token);
///
/// // Start the shard.
/// shard.start().await?;
///
/// // Create a loop of only new message and deleted message events.
/// let event_types = EventTypeFlags::MESSAGE_CREATE | EventTypeFlags::MESSAGE_DELETE;
/// let mut events = shard.some_events(event_types);
///
/// while let Some(event) = events.next().await {
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
/// [`events`]: #method.events
/// [`queue`]: ../queue/index.html
/// [docs:shards]: https://discord.com/developers/docs/topics/gateway#sharding
#[derive(Clone, Debug)]
pub struct Shard(Arc<ShardRef>);

impl Shard {
    /// Create a new unconfingured shard.
    ///
    /// Use [`start`] to initiate the gateway session.
    ///
    /// # Examples
    ///
    /// Create a new shard and start it, wait a second, and then print its
    /// current connection stage:
    ///
    /// ```no_run
    /// use twilight_gateway::Shard;
    /// use std::{env, time::Duration};
    /// use tokio::time as tokio_time;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let token = env::var("DISCORD_TOKEN")?;
    ///
    /// let mut shard = Shard::new(token);
    /// shard.start().await?;
    ///
    /// tokio_time::delay_for(Duration::from_secs(1)).await;
    ///
    /// let info = shard.info()?;
    /// println!("Shard stage: {}", info.stage());
    /// # Ok(()) }
    /// ```
    ///
    /// [`start`]: #method.start
    pub fn new(token: impl Into<String>) -> Self {
        Self::builder(token).build()
    }

    pub(crate) fn new_with_config(config: Config) -> Self {
        let config = Arc::new(config);

        Self(Arc::new(ShardRef {
            config,
            listeners: Listeners::default(),
            processor_handle: OnceCell::new(),
            session: OnceCell::new(),
        }))
    }

    /// Create a builder to configure and construct a shard.
    ///
    /// Refer to the builder for more information.
    pub fn builder(token: impl Into<String>) -> ShardBuilder {
        ShardBuilder::new(token)
    }

    /// Return an immutable reference to the configuration used for this client.
    pub fn config(&self) -> &Config {
        &self.0.config
    }

    /// Start the shard, connecting it to the gateway and starting the process
    /// of receiving and processing events.
    ///
    /// # Errors
    ///
    /// Returns [`ShardStartError::Establishing`] if establishing a connection
    /// to the gateway failed.
    ///
    /// Returns [`ShardStartError::ParsingGatewayUrl`] if the gateway URL
    /// couldn't be parsed.
    ///
    /// Returns [`ShardStartError::RetrievingGatewayUrl`] if the gateway URL
    /// couldn't be retrieved from the HTTP API.
    ///
    /// [`ShardStartError::Establishing`]: enum.ShardStartError.html#variant.Establishing
    /// [`ShardStartError::ParsingGatewayUrl`]: enum.ShardStartError.html#variant.ParsingGatewayUrl
    /// [`ShardStartError::RetrievingGatewayUrl`]: enum.ShardStartError.html#variant.RetrievingGatewayUrl
    pub async fn start(&mut self) -> Result<(), ShardStartError> {
        let url = self
            .0
            .config
            .http_client()
            .gateway()
            .authed()
            .await
            .map_err(|source| ShardStartError::RetrievingGatewayUrl { source })?
            .url;

        let config = Arc::clone(&self.0.config);
        let listeners = self.0.listeners.clone();
        let (processor, wrx) = ShardProcessor::new(config, url, listeners)
            .await
            .map_err(ShardStartError::from)?;
        let (fut, handle) = future::abortable(processor.run());

        tokio::spawn(async move {
            let _ = fut.await;

            tracing::debug!("shard processor future ended");
        });

        // We know that these haven't been set, so we can ignore the result.
        let _ = self.0.processor_handle.set(handle);
        let _ = self.0.session.set(wrx);

        Ok(())
    }

    /// Create a new stream of events from the shard.
    ///
    /// There can be multiple streams of events. All events will be broadcast to
    /// all streams of events.
    ///
    /// The returned event stream implements [`futures::stream::Stream`].
    ///
    /// All event types except for [`EventType::ShardPayload`] are enabled. If
    /// you need to enable it, consider calling [`some_events`] instead.
    ///
    /// [`EventType::ShardPayload`]: ../../twilight_model/gateway/event/enum.EventType.html#variant.ShardPayload
    /// [`futures::stream::Stream`]: https://docs.rs/futures/*/futures/stream/trait.Stream.html
    /// [`some_events`]: #method.some_events
    pub fn events(&self) -> Events {
        self.some_events(EventTypeFlags::default())
    }

    /// Create a new filtered stream of events from the shard.
    ///
    /// Only the events specified in the bitflags will be sent over the stream.
    ///
    /// The returned event stream implements [`futures::stream::Stream`].
    ///
    /// # Examples
    ///
    /// Filter the events so that you only receive the [`Event::ShardConnected`]
    /// and [`Event::ShardDisconnected`] events:
    ///
    /// ```no_run
    /// use twilight_gateway::{EventTypeFlags, Event, Shard};
    /// use futures::StreamExt;
    /// use std::env;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let mut shard = Shard::new(env::var("DISCORD_TOKEN")?);
    /// shard.start().await?;
    ///
    /// let event_types = EventTypeFlags::SHARD_CONNECTED | EventTypeFlags::SHARD_DISCONNECTED;
    /// let mut events = shard.some_events(event_types);
    ///
    /// while let Some(event) = events.next().await {
    ///     match event {
    ///         Event::ShardConnected(_) => println!("Shard is now connected"),
    ///         Event::ShardDisconnected(_) => println!("Shard is now disconnected"),
    ///         // No other event will come in through the stream.
    ///         _ => {},
    ///     }
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// [`Event::ShardConnected`]: ../../twilight_model/gateway/event/enum.Event.html#variant.ShardConnected
    /// [`Event::ShardDisconnected`]: ../../twilight_model/gateway/event/enum.Event.html#variant.ShardDisconnected
    /// [`futures::stream::Stream`]: https://docs.rs/futures/*/futures/stream/trait.Stream.html
    pub fn some_events(&self, event_types: EventTypeFlags) -> Events {
        let rx = self.0.listeners.add(event_types);

        Events::new(event_types, rx)
    }

    /// Retrieve information about the running of the shard, such as the current
    /// connection stage.
    ///
    /// # Errors
    ///
    /// Returns a [`SessionInactiveError`] if the shard's session is inactive.
    ///
    /// [`SessionInactiveError`]: struct.SessionInactiveError.html
    pub fn info(&self) -> Result<Information, SessionInactiveError> {
        let session = self.session()?;

        Ok(Information {
            id: self.config().shard()[0],
            latency: session.heartbeats.latency(),
            seq: session.seq(),
            stage: session.stage(),
        })
    }

    /// Returns a handle to the current session
    ///
    /// # Note
    ///
    /// This session can be invalidated if it is kept around
    /// under a reconnect or resume. In consequence this call
    /// should not be cached.
    ///
    /// # Errors
    ///
    /// Returns a [`SessionInactiveError`] if the shard's session is inactive.
    ///
    /// [`SessionInactiveError`]: struct.SessionInactiveError.html
    pub fn session(&self) -> Result<Arc<Session>, SessionInactiveError> {
        let session = self.0.session.get().ok_or(SessionInactiveError)?;

        Ok(Arc::clone(&session.borrow()))
    }

    /// Retrieve an interface implementing the `Sink` trait which can be used to
    /// send messages.
    ///
    /// This sink is only valid for the current websocket connection. If the
    /// shard's session is invalidated, or network connectivity is lost, or
    /// anything else happens that causes a need to create a new connection,
    /// then the sink will be invalidated.
    ///
    /// # Errors
    ///
    /// Returns a [`SessionInactiveError`] if the shard's session is inactive.
    ///
    /// [`SessionInactiveError`]: struct.SessionInactiveError.html
    pub fn sink(&self) -> Result<ShardSink, SessionInactiveError> {
        let session = self.session()?;

        Ok(ShardSink(session.tx.clone()))
    }

    /// Send a command over the gateway.
    ///
    /// # Errors
    ///
    /// Returns [`CommandError::Sending`] if the message could not be sent
    /// over the websocket. This indicates the shard is currently restarting.
    ///
    /// Returns [`CommandError::Serializing`] if the provided value failed to
    /// serialize into JSON.
    ///
    /// Returns [`CommandError::SessionInactive`] if the shard has not been
    /// started.
    ///
    /// [`CommandError::Sending`]: enum.CommandError.html#variant.Sending
    /// [`CommandError::Serializing`]: enum.CommandError.html#variant.Serializing
    /// [`CommandError::SessionInactive`]: enum.CommandError.html#variant.SessionInactive
    pub async fn command(&self, value: &impl serde::Serialize) -> Result<(), CommandError> {
        let session = self
            .session()
            .map_err(|source| CommandError::SessionInactive { source })?;
        let json =
            crate::json_to_string(value).map_err(|source| CommandError::Serializing { source })?;
        let message = Message::Text(json);

        // Tick ratelimiter.
        session.ratelimit.lock().await.next().await;

        session
            .tx
            .unbounded_send(message)
            .map_err(|source| CommandError::Sending { source })
    }

    /// Shut down the shard.
    ///
    /// The shard will cleanly close the connection by sending a normal close
    /// code, causing Discord to show the bot as being offline. The session will
    /// not be resumable.
    pub fn shutdown(&self) {
        self.0.listeners.remove_all();

        if let Some(processor_handle) = self.0.processor_handle.get() {
            processor_handle.abort();
        }

        if let Ok(session) = self.session() {
            // Since we're shutting down now, we don't care if it sends or not.
            let _ = session.tx.unbounded_send(Message::Close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "".into(),
            })));
            session.stop_heartbeater();
        }
    }

    /// Shut down the shard in a resumable fashion.
    ///
    /// The shard will cleanly close the connection by sending a restart close
    /// code, causing Discord to keep the bot as showing online. The connection
    /// will be resumable by using the provided session resume information
    /// to [`ClusterBuilder::resume_sessions`].
    ///
    /// [`ClusterBuilder::resume_sessions`]: ../cluster/struct.ClusterBuilder.html#method.resume_sessions
    pub fn shutdown_resumable(&self) -> (u64, Option<ResumeSession>) {
        self.0.listeners.remove_all();

        if let Some(processor_handle) = self.0.processor_handle.get() {
            processor_handle.abort();
        }

        let shard_id = self.config().shard()[0];

        let session = match self.session() {
            Ok(session) => session,
            Err(_) => return (shard_id, None),
        };

        let _ = session.tx.unbounded_send(Message::Close(Some(CloseFrame {
            code: CloseCode::Restart,
            reason: Cow::from("Closing in a resumable way"),
        })));

        let session_id = session.id();
        let sequence = session.seq.load(Ordering::Relaxed);

        session.stop_heartbeater();

        let data = session_id.map(|id| ResumeSession {
            session_id: id,
            sequence,
        });

        (shard_id, data)
    }
}
