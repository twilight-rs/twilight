//! Nodes for communicating with a Lavalink server.
//!
//! Using nodes, you can send events to a server and receive events.
//!
//! This is a bit more low level than using the [`Lavalink`] client because you
//! will need to provide your own `VoiceUpdate` events when your bot joins
//! channels, meaning you will have to accumulate and combine voice state update
//! and voice server update events from the Discord gateway to send them to
//! a node.
//!
//! Additionally, you will have to create and manage your own [`PlayerManager`]
//! and make your own players for guilds when your bot joins voice channels.
//!
//! This can be a lot of work, and there's not really much reason to do it
//! yourself. For that reason, you should almost always use the `Lavalink`
//! client which does all of this for you.
//!
//! [`Lavalink`]: crate::client::Lavalink

use crate::{
    model::{IncomingEvent, Opcode, OutgoingEvent, PlayerUpdate, Stats, StatsCpu, StatsMemory},
    player::PlayerManager,
};
use futures_util::{
    lock::BiLock,
    sink::SinkExt,
    stream::{Stream, StreamExt},
};
use http::{header::HeaderName, Request, Response, StatusCode};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    net::TcpStream,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    time as tokio_time,
};
use tokio_tungstenite::{
    tungstenite::{Error as TungsteniteError, Message},
    MaybeTlsStream, WebSocketStream,
};
use twilight_model::id::UserId;

/// An error occurred while either initializing a connection or while running
/// its event loop.
#[derive(Debug)]
pub struct NodeError {
    kind: NodeErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl NodeError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &NodeErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (NodeErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for NodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            NodeErrorType::BuildingConnectionRequest { .. } => {
                f.write_str("failed to build connection request")
            }
            NodeErrorType::Connecting { .. } => f.write_str("Failed to connect to the node"),
            NodeErrorType::SerializingMessage { .. } => {
                f.write_str("failed to serialize outgoing message as json")
            }
            NodeErrorType::Unauthorized { address, .. } => {
                f.write_str("the authorization used to connect to node ")?;
                Display::fmt(address, f)?;

                f.write_str(" is invalid")
            }
        }
    }
}

impl Error for NodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`NodeError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum NodeErrorType {
    /// Building the HTTP request to initialize a connection failed.
    BuildingConnectionRequest,
    /// Connecting to the Lavalink server failed after several backoff attempts.
    Connecting,
    /// Serializing a JSON message to be sent to a Lavalink node failed.
    SerializingMessage {
        /// The message that couldn't be serialized.
        message: OutgoingEvent,
    },
    /// The given authorization for the node is incorrect.
    Unauthorized {
        /// The address of the node that failed to authorize.
        address: SocketAddr,
        /// The authorization used to connect to the node.
        authorization: String,
    },
}

/// An error that can occur while sending an event over a node.
#[derive(Debug)]
pub struct NodeSenderError {
    kind: NodeSenderErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl NodeSenderError {
    /// Immutable reference to the type of error that occurred.
    pub const fn kind(&self) -> &NodeSenderErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (NodeSenderErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for NodeSenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            NodeSenderErrorType::Sending => f.write_str("failed to send over channel"),
        }
    }
}

impl Error for NodeSenderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`NodeSenderError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum NodeSenderErrorType {
    /// Error occurred while sending over the channel.
    Sending,
}

/// Stream of incoming events from a node.
pub struct IncomingEvents {
    inner: UnboundedReceiver<IncomingEvent>,
}

impl IncomingEvents {
    /// Closes the receiving half of a channel without dropping it.
    pub fn close(&mut self) {
        self.inner.close();
    }
}

impl Stream for IncomingEvents {
    type Item = IncomingEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.poll_recv(cx)
    }
}

/// Send outgoing events to the associated node.
pub struct NodeSender {
    inner: UnboundedSender<OutgoingEvent>,
}

impl NodeSender {
    /// Returns whether this channel is closed without needing a context.
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    /// Sends a message along this channel.
    ///
    /// This is an unbounded sender, so this function differs from `Sink::send`
    /// by ensuring the return type reflects that the channel is always ready to
    /// receive messages.
    ///
    /// # Errors
    ///
    /// Returns a [`NodeSenderErrorType::Sending`] error type if node is no
    /// longer connected.
    pub fn send(&self, msg: OutgoingEvent) -> Result<(), NodeSenderError> {
        self.inner.send(msg).map_err(|source| NodeSenderError {
            kind: NodeSenderErrorType::Sending,
            source: Some(Box::new(source)),
        })
    }
}

/// The configuration that a [`Node`] uses to connect to a Lavalink server.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct NodeConfig {
    /// The address of the node.
    pub address: SocketAddr,
    /// The password to use when authenticating.
    pub authorization: String,
    /// The details for resuming a Lavalink session, if any.
    ///
    /// Set this to `None` to disable resume capability.
    pub resume: Option<Resume>,
    /// The number of shards in use by the bot.
    pub shard_count: u64,
    /// The user ID of the bot.
    pub user_id: UserId,
}

/// Configuration for a session which can be resumed.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct Resume {
    /// The number of seconds that the Lavalink server will allow the session to
    /// be resumed for after a disconnect.
    ///
    /// The default is 60.
    pub timeout: u64,
}

impl Resume {
    /// Configure resume capability, providing the number of seconds that the
    /// Lavalink server should queue events for when the connection is resumed.
    pub const fn new(seconds: u64) -> Self {
        Self { timeout: seconds }
    }
}

impl Default for Resume {
    fn default() -> Self {
        Self { timeout: 60 }
    }
}

impl NodeConfig {
    /// Create a new configuration for connecting to a node via
    /// [`Node::connect`].
    ///
    /// If adding a node through the [`Lavalink`] client then you don't need to
    /// do this yourself.
    ///
    /// [`Lavalink`]: crate::client::Lavalink
    pub fn new(
        user_id: UserId,
        shard_count: u64,
        address: impl Into<SocketAddr>,
        authorization: impl Into<String>,
        resume: impl Into<Option<Resume>>,
    ) -> Self {
        Self::_new(
            user_id,
            shard_count,
            address.into(),
            authorization.into(),
            resume.into(),
        )
    }

    const fn _new(
        user_id: UserId,
        shard_count: u64,
        address: SocketAddr,
        authorization: String,
        resume: Option<Resume>,
    ) -> Self {
        Self {
            address,
            authorization,
            resume,
            shard_count,
            user_id,
        }
    }
}

/// A connection to a single Lavalink server. It receives events and forwards
/// events from players to the server.
///
/// Please refer to the [module] documentation.
///
/// [module]: crate
#[derive(Debug)]
pub struct Node {
    config: NodeConfig,
    lavalink_tx: UnboundedSender<OutgoingEvent>,
    players: PlayerManager,
    stats: BiLock<Stats>,
}

impl Node {
    /// Connect to a node, providing a player manager so that the node can
    /// update player details.
    ///
    /// Please refer to the [module] documentation for some additional
    /// information about directly creating and using nodes. You are encouraged
    /// to use the [`Lavalink`] client instead.
    ///
    /// [`Lavalink`]: crate::client::Lavalink
    /// [module]: crate
    pub async fn connect(
        config: NodeConfig,
        players: PlayerManager,
    ) -> Result<(Self, IncomingEvents), NodeError> {
        let (bilock_left, bilock_right) = BiLock::new(Stats {
            cpu: StatsCpu {
                cores: 0,
                lavalink_load: 0f64,
                system_load: 0f64,
            },
            frames: None,
            memory: StatsMemory {
                allocated: 0,
                free: 0,
                used: 0,
                reservable: 0,
            },
            players: 0,
            playing_players: 0,
            op: Opcode::Stats,
            uptime: 0,
        });
        tracing::debug!("starting connection to {}", config.address);
        let (conn_loop, lavalink_tx, lavalink_rx) =
            Connection::connect(config.clone(), players.clone(), bilock_right).await?;
        tracing::debug!("started connection to {}", config.address);

        tokio::spawn(conn_loop.run());

        Ok((
            Self {
                config,
                lavalink_tx,
                players,
                stats: bilock_left,
            },
            IncomingEvents { inner: lavalink_rx },
        ))
    }

    /// Retrieve an immutable reference to the node's configuration.
    pub const fn config(&self) -> &NodeConfig {
        &self.config
    }

    /// Retrieve an immutable reference to the player manager used by the node.
    pub const fn players(&self) -> &PlayerManager {
        &self.players
    }

    /// Retrieve an immutable reference to the node's configuration.
    ///
    /// Note that sending player events through the node's sender won't update
    /// player states, such as whether it's paused.
    ///
    /// # Errors
    ///
    /// Returns a [`NodeSenderErrorType::Sending`] error type if node is no
    /// longer connected.
    pub fn send(&self, event: OutgoingEvent) -> Result<(), NodeSenderError> {
        self.sender().send(event)
    }

    /// Retrieve a unique sender to send events to the Lavalink server.
    ///
    /// Note that sending player events through the node's sender won't update
    /// player states, such as whether it's paused.
    pub fn sender(&self) -> NodeSender {
        NodeSender {
            inner: self.lavalink_tx.clone(),
        }
    }

    /// Retrieve a copy of the node's stats.
    pub async fn stats(&self) -> Stats {
        (*self.stats.lock().await).clone()
    }

    /// Retrieve the calculated penalty score of the node.
    ///
    /// This score can be used to calculate how loaded the server is. A higher
    /// number means it is more heavily loaded.
    pub async fn penalty(&self) -> i32 {
        let stats = self.stats.lock().await;
        let cpu = 1.05f64.powf(100f64 * stats.cpu.system_load) * 10f64 - 10f64;

        let (deficit_frame, null_frame) = (
            1.03f64
                .powf(500f64 * (stats.frames.as_ref().map_or(0, |f| f.deficit) as f64 / 3000f64))
                * 300f64
                - 300f64,
            (1.03f64
                .powf(500f64 * (stats.frames.as_ref().map_or(0, |f| f.nulled) as f64 / 3000f64))
                * 300f64
                - 300f64)
                * 2f64,
        );

        stats.playing_players as i32 + cpu as i32 + deficit_frame as i32 + null_frame as i32
    }
}

struct Connection {
    config: NodeConfig,
    connection: WebSocketStream<MaybeTlsStream<TcpStream>>,
    node_from: UnboundedReceiver<OutgoingEvent>,
    node_to: UnboundedSender<IncomingEvent>,
    players: PlayerManager,
    stats: BiLock<Stats>,
}

impl Connection {
    async fn connect(
        config: NodeConfig,
        players: PlayerManager,
        stats: BiLock<Stats>,
    ) -> Result<
        (
            Self,
            UnboundedSender<OutgoingEvent>,
            UnboundedReceiver<IncomingEvent>,
        ),
        NodeError,
    > {
        let connection = reconnect(&config).await?;

        let (to_node, from_lavalink) = mpsc::unbounded_channel();
        let (to_lavalink, from_node) = mpsc::unbounded_channel();

        Ok((
            Self {
                config,
                connection,
                node_from: from_node,
                node_to: to_node,
                players,
                stats,
            },
            to_lavalink,
            from_lavalink,
        ))
    }

    async fn run(mut self) -> Result<(), NodeError> {
        loop {
            tokio::select! {
                incoming = self.connection.next() => {
                    if let Some(Ok(incoming)) = incoming {
                        self.incoming(incoming).await?;
                    } else {
                        tracing::debug!("connection to {} closed, reconnecting", self.config.address);
                        self.connection = reconnect(&self.config).await?;
                    }
                }
                outgoing = self.node_from.recv() => {
                    if let Some(outgoing) = outgoing {
                        tracing::debug!(
                            "forwarding event to {}: {:?}",
                            self.config.address,
                            outgoing
                        );

                        let payload = serde_json::to_string(&outgoing).map_err(|source| NodeError {
                            kind: NodeErrorType::SerializingMessage { message: outgoing },
                            source: Some(Box::new(source)),
                        })?;
                        let msg = Message::Text(payload);
                        self.connection.send(msg).await.unwrap();
                    } else {
                        tracing::debug!("node {} closed, ending connection", self.config.address);

                        break;
                    }
                }
            }
        }

        Ok(())
    }

    async fn incoming(&mut self, incoming: Message) -> Result<bool, NodeError> {
        tracing::debug!(
            "received message from {}: {:?}",
            self.config.address,
            incoming
        );

        let text = match incoming {
            Message::Close(_) => {
                tracing::debug!("got close, closing connection");
                let _ = self.connection.send(Message::Close(None)).await;

                return Ok(false);
            }
            Message::Ping(data) => {
                tracing::debug!("got ping, sending pong");
                let msg = Message::Pong(data);

                // We don't need to immediately care if a pong fails.
                let _ = self.connection.send(msg).await;

                return Ok(true);
            }
            Message::Text(text) => text,
            other => {
                tracing::debug!("got pong or bytes payload: {:?}", other);

                return Ok(true);
            }
        };

        let event = match serde_json::from_str(&text) {
            Ok(event) => event,
            Err(_) => {
                tracing::warn!("unknown message from lavalink node: {}", text);

                return Ok(true);
            }
        };

        match event {
            IncomingEvent::PlayerUpdate(ref update) => self.player_update(update).await?,
            IncomingEvent::Stats(ref stats) => self.stats(stats).await?,
            _ => {}
        }

        // It's fine if the rx end dropped, often users don't need to care about
        // these events.
        if !self.node_to.is_closed() {
            let _ = self.node_to.send(event);
        }

        Ok(true)
    }

    async fn player_update(&self, update: &PlayerUpdate) -> Result<(), NodeError> {
        let player = match self.players.get(&update.guild_id) {
            Some(player) => player,
            None => {
                tracing::warn!(
                    "invalid player update for guild {}: {:?}",
                    update.guild_id,
                    update,
                );

                return Ok(());
            }
        };

        player.set_position(update.state.position);
        player.set_time(update.state.time);

        Ok(())
    }

    async fn stats(&self, stats: &Stats) -> Result<(), NodeError> {
        *self.stats.lock().await = stats.clone();

        Ok(())
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // Cleanup local players associated with the node
        self.players
            .players
            .retain(|_, v| v.node().config().address != self.config.address);
    }
}

fn connect_request(state: &NodeConfig) -> Result<Request<()>, NodeError> {
    let mut builder = Request::get(format!("ws://{}", state.address));
    builder = builder.header("Authorization", &state.authorization);
    builder = builder.header("Num-Shards", state.shard_count);
    builder = builder.header("User-Id", state.user_id.get());

    if state.resume.is_some() {
        builder = builder.header("Resume-Key", state.address.to_string());
    }

    builder.body(()).map_err(|source| NodeError {
        kind: NodeErrorType::BuildingConnectionRequest,
        source: Some(Box::new(source)),
    })
}

async fn reconnect(
    config: &NodeConfig,
) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, NodeError> {
    let (mut stream, res) = backoff(config).await?;

    let headers = res.headers();

    if let Some(resume) = config.resume.as_ref() {
        let header = HeaderName::from_static("session-resumed");

        if let Some(value) = headers.get(header) {
            if value.as_bytes() == b"false" {
                tracing::debug!("session to node {} didn't resume", config.address);

                let payload = serde_json::json!({
                    "op": "configureResuming",
                    "key": config.address,
                    "timeout": resume.timeout,
                });
                let msg = Message::Text(serde_json::to_string(&payload).unwrap());

                stream.send(msg).await.unwrap();
            } else {
                tracing::debug!("session to {} resumed", config.address);
            }
        }
    }

    Ok(stream)
}

async fn backoff(
    config: &NodeConfig,
) -> Result<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response<()>), NodeError> {
    let mut seconds = 1;

    loop {
        let req = connect_request(config)?;

        match tokio_tungstenite::connect_async(req).await {
            Ok((stream, res)) => return Ok((stream, res)),
            Err(source) => {
                tracing::warn!("failed to connect to node {}: {:?}", source, config.address);

                if matches!(source, TungsteniteError::Http(ref resp) if resp.status() == StatusCode::UNAUTHORIZED)
                {
                    return Err(NodeError {
                        kind: NodeErrorType::Unauthorized {
                            address: config.address,
                            authorization: config.authorization.to_owned(),
                        },
                        source: None,
                    });
                }

                if seconds > 64 {
                    tracing::debug!("no longer trying to connect to node {}", config.address);

                    return Err(NodeError {
                        kind: NodeErrorType::Connecting,
                        source: Some(Box::new(source)),
                    });
                }

                tracing::debug!(
                    "waiting {} seconds before attempting to connect to node {} again",
                    seconds,
                    config.address,
                );
                tokio_time::sleep(Duration::from_secs(seconds)).await;

                seconds *= 2;

                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, NodeConfig, NodeError, NodeErrorType, Resume};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{error::Error, fmt::Debug};

    assert_fields!(
        NodeConfig: address,
        authorization,
        resume,
        shard_count,
        user_id
    );
    assert_impl_all!(NodeConfig: Clone, Debug, Send, Sync);
    assert_fields!(NodeErrorType::SerializingMessage: message);
    assert_fields!(NodeErrorType::Unauthorized: address, authorization);
    assert_impl_all!(NodeErrorType: Debug, Send, Sync);
    assert_impl_all!(NodeError: Error, Send, Sync);
    assert_impl_all!(Node: Debug, Send, Sync);
    assert_fields!(Resume: timeout);
    assert_impl_all!(Resume: Clone, Debug, Default, Eq, PartialEq, Send, Sync);
}
