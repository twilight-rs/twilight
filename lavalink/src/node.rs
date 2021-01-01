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
use async_tungstenite::{
    tokio::ConnectStream,
    tungstenite::{Error as TungsteniteError, Message},
    WebSocketStream,
};
use futures_channel::mpsc::{self, TrySendError, UnboundedReceiver, UnboundedSender};
use futures_util::{
    future::{self, Either},
    lock::BiLock,
    sink::SinkExt,
    stream::StreamExt,
};
use http::{header::HeaderName, Error as HttpError, Request, Response, StatusCode};
use serde_json::Error as JsonError;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tokio::time as tokio_time;
use twilight_model::id::UserId;

/// An error occurred while either initializing a connection or while running
/// its event loop.
#[derive(Debug)]
#[non_exhaustive]
pub enum NodeError {
    /// Building the HTTP request to initialize a connection failed.
    BuildingConnectionRequest {
        /// The source of the error from the `http` crate.
        source: HttpError,
    },
    /// Connecting to the Lavalink server failed after several backoff attempts.
    Connecting {
        /// The source of the error from the `tungstenite` crate.
        source: TungsteniteError,
    },
    /// Serializing a JSON message to be sent to a Lavalink node failed.
    SerializingMessage {
        /// The message that couldn't be serialized.
        message: OutgoingEvent,
        /// The source of the error from the `serde_json` crate.
        source: JsonError,
    },
    /// The given authorization for the node is incorrect.
    Unauthorized {
        /// The address of the node that failed to authorize.
        address: SocketAddr,
        /// The authorization used to connect to the node.
        authorization: String,
    },
}

impl Display for NodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BuildingConnectionRequest { .. } => {
                f.write_str("failed to build connection request")
            }
            Self::Connecting { .. } => f.write_str("Failed to connect to the node"),
            Self::SerializingMessage { .. } => {
                f.write_str("failed to serialize outgoing message as json")
            }
            Self::Unauthorized { address, .. } => write!(
                f,
                "the authorization used to connect to node {} is invalid",
                address
            ),
        }
    }
}

impl Error for NodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::BuildingConnectionRequest { source } => Some(source),
            Self::Connecting { source } => Some(source),
            Self::SerializingMessage { source, .. } => Some(source),
            Self::Unauthorized { .. } => None,
        }
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
    pub fn new(seconds: u64) -> Self {
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

    fn _new(
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

#[derive(Debug)]
struct NodeRef {
    config: NodeConfig,
    lavalink_tx: UnboundedSender<OutgoingEvent>,
    players: PlayerManager,
    stats: BiLock<Stats>,
}

/// A connection to a single Lavalink server. It receives events and forwards
/// events from players to the server.
///
/// Please refer to the [module] documentation.
///
/// [module]: crate
#[derive(Clone, Debug)]
pub struct Node(Arc<NodeRef>);

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
    ) -> Result<(Self, UnboundedReceiver<IncomingEvent>), NodeError> {
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
            Self(Arc::new(NodeRef {
                config,
                lavalink_tx,
                players,
                stats: bilock_left,
            })),
            lavalink_rx,
        ))
    }

    /// Retrieve an immutable reference to the node's configuration.
    pub fn config(&self) -> &NodeConfig {
        &self.0.config
    }

    /// Retrieve an immutable reference to the player manager used by the node.
    pub async fn players(&self) -> &PlayerManager {
        &self.0.players
    }

    /// Retrieve an immutable reference to the node's configuration.
    ///
    /// Note that sending player events through the node's sender won't update
    /// player states, such as whether it's paused.
    pub fn send(&self, event: OutgoingEvent) -> Result<(), TrySendError<OutgoingEvent>> {
        self.sender().unbounded_send(event)
    }

    /// Retrieve a unique sender to send events to the Lavalink server.
    ///
    /// Note that sending player events through the node's sender won't update
    /// player states, such as whether it's paused.
    pub fn sender(&self) -> UnboundedSender<OutgoingEvent> {
        self.0.lavalink_tx.clone()
    }

    /// Retrieve a copy of the node's stats.
    pub async fn stats(&self) -> Stats {
        (*self.0.stats.lock().await).clone()
    }

    /// Retrieve the calculated penalty score of the node.
    ///
    /// This score can be used to calculate how loaded the server is. A higher
    /// number means it is more heavily loaded.
    pub async fn penalty(&self) -> i32 {
        let stats = self.0.stats.lock().await;
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
    connection: WebSocketStream<ConnectStream>,
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

        let (to_node, from_lavalink) = mpsc::unbounded();
        let (to_lavalink, from_node) = mpsc::unbounded();

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
            let from_lavalink = self.connection.next();
            let to_lavalink = self.node_from.next();

            match future::select(from_lavalink, to_lavalink).await {
                Either::Left((Some(Ok(incoming)), _)) => {
                    self.incoming(incoming).await?;
                }
                Either::Left((_, _)) => {
                    tracing::debug!("connection to {} closed, reconnecting", self.config.address);
                    self.connection = reconnect(&self.config).await?;
                }
                Either::Right((Some(outgoing), _)) => {
                    tracing::debug!(
                        "forwarding event to {}: {:?}",
                        self.config.address,
                        outgoing
                    );

                    let payload = serde_json::to_string(&outgoing).map_err(|source| {
                        NodeError::SerializingMessage {
                            message: outgoing,
                            source,
                        }
                    })?;
                    let msg = Message::Text(payload);
                    self.connection.send(msg).await.unwrap();
                }
                Either::Right((_, _)) => {
                    tracing::debug!("node {} closed, ending connection", self.config.address);

                    break;
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
            let _ = self.node_to.unbounded_send(event);
        }

        Ok(true)
    }

    async fn player_update(&self, update: &PlayerUpdate) -> Result<(), NodeError> {
        let mut player = match self.players.get_mut(&update.guild_id) {
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

        *player.value_mut().position_mut() = update.state.position;
        *player.value_mut().time_mut() = update.state.time;

        Ok(())
    }

    async fn stats(&self, stats: &Stats) -> Result<(), NodeError> {
        *self.stats.lock().await = stats.clone();

        Ok(())
    }
}

fn connect_request(state: &NodeConfig) -> Result<Request<()>, NodeError> {
    let mut builder = Request::get(format!("ws://{}", state.address));
    builder = builder.header("Authorization", &state.authorization);
    builder = builder.header("Num-Shards", state.shard_count);
    builder = builder.header("User-Id", state.user_id.0);

    if state.resume.is_some() {
        builder = builder.header("Resume-Key", state.address.to_string());
    }

    builder
        .body(())
        .map_err(|source| NodeError::BuildingConnectionRequest { source })
}

async fn reconnect(config: &NodeConfig) -> Result<WebSocketStream<ConnectStream>, NodeError> {
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
) -> Result<(WebSocketStream<ConnectStream>, Response<()>), NodeError> {
    let mut seconds = 1;

    loop {
        let req = connect_request(config)?;

        match async_tungstenite::tokio::connect_async(req).await {
            Ok((stream, res)) => return Ok((stream, res)),
            Err(source) => {
                tracing::warn!("failed to connect to node {}: {:?}", source, config.address);

                if matches!(source, TungsteniteError::Http(status) if status == StatusCode::UNAUTHORIZED)
                {
                    return Err(NodeError::Unauthorized {
                        address: config.address,
                        authorization: config.authorization.to_owned(),
                    });
                }

                if seconds > 64 {
                    tracing::debug!("no longer trying to connect to node {}", config.address);

                    return Err(NodeError::Connecting { source });
                }

                tracing::debug!(
                    "waiting {} sceonds before attempting to connect to node {} again",
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
    use super::{Node, NodeConfig, NodeError, Resume};
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
    assert_fields!(NodeError::BuildingConnectionRequest: source);
    assert_fields!(NodeError::Connecting: source);
    assert_fields!(NodeError::SerializingMessage: message, source);
    assert_fields!(NodeError::Unauthorized: address, authorization);
    assert_impl_all!(NodeError: Debug, Error, Send, Sync);
    assert_impl_all!(Node: Clone, Debug, Send, Sync);
    assert_fields!(Resume: timeout);
    assert_impl_all!(Resume: Clone, Debug, Default, Eq, PartialEq, Send, Sync);
}
