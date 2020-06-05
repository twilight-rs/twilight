use crate::{
    model::{IncomingEvent, Opcode, OutgoingEvent, PlayerUpdate, Stats, StatsCpu, StatsMemory},
    player::PlayerManager,
};
use async_tungstenite::{tokio::TokioAdapter, tungstenite::Message, WebSocketStream};
use futures_channel::mpsc::{self, TrySendError, UnboundedReceiver, UnboundedSender};
use futures_util::{
    future::{self, Either},
    lock::BiLock,
    sink::SinkExt,
    stream::StreamExt,
};
use http::{header::HeaderName, Error as HttpError, Request};
use serde_json::Error as JsonError;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    net::SocketAddr,
    sync::Arc,
};
use tokio::net::TcpStream;
use twilight_model::id::UserId;

#[derive(Debug)]
#[non_exhaustive]
pub enum NodeError {
    BuildingConnectionRequest {
        source: HttpError,
    },
    IncomingMessageInvalid {
        source: JsonError,
        text: String,
    },
    SerializingMessage {
        message: OutgoingEvent,
        source: JsonError,
    },
}

impl Display for NodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::BuildingConnectionRequest { .. } => {
                f.write_str("failed to build connection request")
            }
            Self::IncomingMessageInvalid { .. } => {
                f.write_str("the incoming message is invalid json or unsupported")
            }
            Self::SerializingMessage { .. } => {
                f.write_str("failed to serialize outgoing message as json")
            }
        }
    }
}

impl Error for NodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::BuildingConnectionRequest { source } => Some(source),
            Self::IncomingMessageInvalid { source, .. } => Some(source),
            Self::SerializingMessage { source, .. } => Some(source),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct NodeConfig {
    pub address: SocketAddr,
    pub authorization: String,
    pub resume: Option<Resume>,
    pub shard_count: u64,
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

#[derive(Clone, Debug)]
pub struct Node(Arc<NodeRef>);

impl Node {
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
        log::debug!("Starting connection to {}", config.address);
        let (conn_loop, lavalink_tx, lavalink_rx) =
            Connection::connect(config.clone(), players.clone(), bilock_right).await?;
        log::debug!("Started connection to {}", config.address);

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

    pub fn config(&self) -> &NodeConfig {
        &self.0.config
    }

    pub async fn players(&self) -> &PlayerManager {
        &self.0.players
    }

    pub fn send(&self, event: OutgoingEvent) -> Result<(), TrySendError<OutgoingEvent>> {
        self.sender().unbounded_send(event)
    }

    pub fn sender(&self) -> UnboundedSender<OutgoingEvent> {
        self.0.lavalink_tx.clone()
    }

    pub async fn stats(&self) -> Stats {
        (*self.0.stats.lock().await).clone()
    }

    pub async fn penalty(&self) -> i32 {
        let stats = self.0.stats.lock().await;
        let cpu = 1.05f64.powf(100f64 * stats.cpu.system_load) * 10f64 - 10f64;

        stats.playing_players as i32 + cpu as i32
    }
}

struct Connection {
    config: NodeConfig,
    connection: WebSocketStream<TokioAdapter<TcpStream>>,
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
            if self.node_to.is_closed() {
                break;
            }

            let from_lavalink = self.connection.next();
            let to_lavalink = self.node_from.next();

            match future::select(from_lavalink, to_lavalink).await {
                Either::Left((Some(Ok(incoming)), _)) => {
                    self.incoming(incoming).await?;
                }
                Either::Left((_, _)) => {
                    log::debug!("Connection to {} closed, reconnecting", self.config.address);
                    self.connection = reconnect(&self.config).await?;
                }
                Either::Right((Some(outgoing), _)) => {
                    log::debug!(
                        "Forwarding event to {}: {:?}",
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
                    log::debug!("Node {} closed, ending connection", self.config.address);

                    break;
                }
            }
        }

        Ok(())
    }

    async fn incoming(&mut self, incoming: Message) -> Result<bool, NodeError> {
        log::debug!(
            "Received message from {}: {:?}",
            self.config.address,
            incoming
        );

        let text = match incoming {
            Message::Close(_) => {
                let _ = self.connection.send(Message::Close(None)).await;

                return Ok(false);
            }
            Message::Ping(data) => {
                let msg = Message::Pong(data);

                // We don't need to immediately care if a pong fails.
                let _ = self.connection.send(msg).await;

                return Ok(true);
            }
            Message::Text(text) => text,
            other => {
                log::info!("Got a pong or bytes payload: {:?}", other);

                return Ok(true);
            }
        };

        let event = serde_json::from_str(&text)
            .map_err(|source| NodeError::IncomingMessageInvalid { source, text })?;

        match event {
            IncomingEvent::PlayerUpdate(ref update) => self.player_update(update).await?,
            IncomingEvent::Stats(ref stats) => self.stats(stats).await?,
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
                log::warn!(
                    "Got invalid player update for guild {}: {:?}",
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

async fn reconnect(
    state: &NodeConfig,
) -> Result<WebSocketStream<TokioAdapter<TcpStream>>, NodeError> {
    let mut builder = Request::get(format!("ws://{}", state.address));
    builder = builder.header("Authorization", &state.authorization);
    builder = builder.header("Num-Shards", state.shard_count);
    builder = builder.header("User-Id", state.user_id.0);

    if state.resume.is_some() {
        builder = builder.header("Resume-Key", state.address.to_string());
    }

    let req = builder
        .body(())
        .map_err(|source| NodeError::BuildingConnectionRequest { source })?;

    let (mut stream, res) = async_tungstenite::tokio::connect_async(req).await.unwrap();
    let headers = res.headers();

    if let Some(resume) = state.resume.as_ref() {
        let header = HeaderName::from_static("session-resumed");

        if let Some(value) = headers.get(header) {
            if value.as_bytes() == b"false" {
                let payload = serde_json::json!({
                    "op": "configureResuming",
                    "key": state.address,
                    "timeout": resume.timeout,
                });
                let msg = Message::Text(serde_json::to_string(&payload).unwrap());

                stream.send(msg).await.unwrap();
            }
        }
    }

    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::{Node, NodeConfig, NodeError, NodeRef};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(NodeConfig: Clone, Debug, Send, Sync);
    assert_impl_all!(NodeError: Debug, Send, Sync);
    assert_impl_all!(NodeRef: Debug, Send, Sync);
    assert_impl_all!(Node: Clone, Debug, Send, Sync);
}
