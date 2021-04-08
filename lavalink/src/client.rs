//! Client to manage nodes and players.

use crate::{
    model::{IncomingEvent, OutgoingEvent, SlimVoiceServerUpdate, VoiceUpdate},
    node::{Node, NodeConfig, NodeError, Resume},
    player::{Player, PlayerManager},
};
use dashmap::{mapref::one::Ref, DashMap};
use futures_channel::mpsc::{TrySendError, UnboundedReceiver};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    net::SocketAddr,
    sync::Arc,
};
use twilight_model::{
    gateway::event::Event,
    id::{GuildId, UserId},
};

/// An error that can occur while interacting with the client.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ClientError {
    /// A node isn't configured, so the operation isn't possible to fulfill.
    NodesUnconfigured,
    /// Sending a voice update event to the node failed because the node's
    /// connection was shutdown.
    SendingVoiceUpdate {
        /// The source of the error.
        source: TrySendError<OutgoingEvent>,
    },
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NodesUnconfigured => f.write_str("no node has been configured"),
            Self::SendingVoiceUpdate { .. } => f.write_str("couldn't send voice update to node"),
        }
    }
}

impl Error for ClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NodesUnconfigured => None,
            Self::SendingVoiceUpdate { source } => Some(source),
        }
    }
}

#[derive(Debug, Default)]
struct LavalinkRef {
    guilds: DashMap<GuildId, SocketAddr>,
    nodes: DashMap<SocketAddr, Node>,
    players: PlayerManager,
    resume: Option<Resume>,
    shard_count: u64,
    user_id: UserId,
    server_updates: DashMap<GuildId, SlimVoiceServerUpdate>,
    sessions: DashMap<GuildId, Box<str>>,
}

/// The lavalink client that manages nodes, players, and processes events from
/// Discord to tie it all together.
///
/// **Note**: You must call the [`process`] method with every Voice State Update
/// and Voice Server Update event you receive from Discord. It will
/// automatically forward these events to Lavalink. See its documentation for
/// more information.
///
/// You can retrieve players using the [`player`] method. Players contain
/// information about the active playing information of a guild and allows you to send events to the
/// connected node, such as [`Play`] events.
///
/// # Cloning
///
/// The client internally wraps its data within an Arc. This means that the
/// client can be cloned and passed around tasks and threads cheaply.
///
/// [`Play`]: crate::model::outgoing::Play
/// [`player`]: Self::player
/// [`process`]: Self::process
#[derive(Clone, Debug)]
pub struct Lavalink(Arc<LavalinkRef>);

impl Lavalink {
    /// Create a new Lavalink client instance.
    ///
    /// The user ID and number of shards provided may not be modified during
    /// runtime, and the client must be re-created. These parameters are
    /// automatically passed to new nodes created via [`add`].
    ///
    /// See also [`new_with_resume`], which allows you to specify session resume
    /// capability.
    ///
    /// [`add`]: Self::add
    /// [`new_with_resume`]: Self::new_with_resume
    pub fn new(user_id: UserId, shard_count: u64) -> Self {
        Self::_new_with_resume(user_id, shard_count, None)
    }

    /// Like [`new`], but allows you to specify resume capability (if any).
    ///
    /// Provide `None` for the `resume` parameter to disable session resume
    /// capability. See the [`Resume`] documentation for defaults.
    ///
    /// [`Resume`]: crate::node::Resume
    /// [`new`]: Self::new
    pub fn new_with_resume(
        user_id: UserId,
        shard_count: u64,
        resume: impl Into<Option<Resume>>,
    ) -> Self {
        Self::_new_with_resume(user_id, shard_count, resume.into())
    }

    fn _new_with_resume(user_id: UserId, shard_count: u64, resume: Option<Resume>) -> Self {
        Self(Arc::new(LavalinkRef {
            guilds: DashMap::new(),
            nodes: DashMap::new(),
            players: PlayerManager::new(),
            resume,
            shard_count,
            user_id,
            server_updates: DashMap::new(),
            sessions: DashMap::new(),
        }))
    }

    /// Process an event into the Lavalink client.
    ///
    /// **Note**: calling this method in your event loop is required. See the
    /// [crate documentation] for an example.
    ///
    /// This requires the `VoiceServerUpdate` and `VoiceStateUpdate` events that
    /// you receive from Discord over the gateway to send voice updates to
    /// nodes. For simplicity in some applications' event loops, any event can
    /// be provided, but they will just be ignored.
    ///
    /// The Ready event can optionally be provided to do some cleaning of
    /// stalled voice states that never received their voice server update half
    /// or vice versa. It is recommended that you process Ready events.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::NodesUnconfigured`] if no nodes have been added
    /// to the client when attempting to retrieve a guild's player.
    ///
    /// [crate documentation]: crate#examples
    pub async fn process(&self, event: &Event) -> Result<(), ClientError> {
        tracing::trace!("processing event: {:?}", event);

        let guild_id = match event {
            Event::Ready(e) => {
                let shard_id = e.shard.map_or(0, |[id, _]| id);

                self.clear_shard_states(shard_id);

                return Ok(());
            }
            Event::VoiceServerUpdate(e) => {
                if let Some(guild_id) = e.guild_id {
                    self.0.server_updates.insert(guild_id, e.clone().into());
                    guild_id
                } else {
                    tracing::trace!("event has no guild ID: {:?}", e);
                    return Ok(());
                }
            }
            Event::VoiceStateUpdate(e) => {
                if e.0.user_id != self.0.user_id {
                    tracing::trace!("got voice state update from another user");

                    return Ok(());
                }

                if let Some(guild_id) = e.0.guild_id {
                    // Update player if it exists and update the connected channel ID.
                    if let Some(kv) = self.0.players.get(&guild_id) {
                        kv.value().set_channel_id(e.0.channel_id);
                    }

                    if e.0.channel_id.is_none() {
                        self.0.sessions.remove(&guild_id);
                    } else {
                        self.0
                            .sessions
                            .insert(guild_id, e.0.session_id.clone().into_boxed_str());
                    }
                    guild_id
                } else {
                    tracing::trace!("event has no guild ID: {:?}", e);
                    return Ok(());
                }
            }
            _ => return Ok(()),
        };

        tracing::debug!(
            "got voice server/state update for {:?}: {:?}",
            guild_id,
            event
        );

        let update = {
            let server = self.0.server_updates.get(&guild_id);
            let session = self.0.sessions.get(&guild_id);
            match (server, session) {
                (Some(server), Some(session)) => {
                    let server = server.value();
                    let session = session.value();
                    tracing::debug!(
                        "got both halves for {}: {:?}; Session ID: {:?}",
                        guild_id,
                        server,
                        session,
                    );
                    VoiceUpdate::new(guild_id, session.as_ref(), server.clone())
                }
                (Some(server), None) => {
                    tracing::debug!(
                        "guild {} is now waiting for other half; got: {:?}",
                        guild_id,
                        server.value()
                    );
                    return Ok(());
                }
                (None, Some(session)) => {
                    tracing::debug!(
                        "guild {} is now waiting for other half; got session ID: {:?}",
                        guild_id,
                        session.value()
                    );
                    return Ok(());
                }
                (None, None) => return Ok(()),
            }
        };

        tracing::debug!("getting player for guild {}", guild_id);
        let player = self.player(guild_id).await?;
        tracing::debug!("sending voice update for guild {}: {:?}", guild_id, update);
        player
            .send(update)
            .map_err(|source| ClientError::SendingVoiceUpdate { source })?;
        tracing::debug!("sent voice update for guild {}", guild_id);

        Ok(())
    }

    /// Add a new node to be managed by the Lavalink client.
    ///
    /// If a node already exists with the provided address, then it will be
    /// replaced.
    pub async fn add(
        &self,
        address: SocketAddr,
        authorization: impl Into<String>,
    ) -> Result<(Node, UnboundedReceiver<IncomingEvent>), NodeError> {
        let config = NodeConfig {
            address,
            authorization: authorization.into(),
            resume: self.0.resume.clone(),
            shard_count: self.0.shard_count,
            user_id: self.0.user_id,
        };

        let (node, rx) = Node::connect(config, self.0.players.clone()).await?;
        self.0.nodes.insert(address, node.clone());

        Ok((node, rx))
    }

    /// Remove a node from the list of nodes being managed by the Lavalink
    /// client.
    ///
    /// The node is returned if it existed.
    pub async fn remove(&self, address: SocketAddr) -> Option<(SocketAddr, Node)> {
        self.0.nodes.remove(&address)
    }

    /// Determine the "best" node for new players according to available nodes'
    /// penalty scores.
    ///
    /// Refer to [`Node::penalty`] for how this is calculated.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::NodesUnconfigured`] if there are no configured
    /// nodes available in the client.
    ///
    /// [`Node::penalty`]: crate::node::Node::penalty
    pub async fn best(&self) -> Result<Node, ClientError> {
        let mut lowest = i32::MAX;
        let mut best = None;

        for node in self.0.nodes.iter() {
            let penalty = node.value().penalty().await;

            if penalty < lowest {
                lowest = penalty;
                best.replace(node.clone());
            }
        }

        best.ok_or(ClientError::NodesUnconfigured)
    }

    /// Retrieve an immutable reference to the player manager.
    pub fn players(&self) -> &PlayerManager {
        &self.0.players
    }

    /// Retrieve a player for the guild.
    ///
    /// Creates a player configured to use the best available node if a player
    /// for the guild doesn't already exist. Use [`PlayerManager::get`] to only
    /// retrieve and not create.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::NodesUnconfigured`] if no node has been
    /// configured via [`add`].
    ///
    /// [`PlayerManager::get`]: crate::player::PlayerManager::get
    /// [`add`]: Self::add
    pub async fn player(&self, guild_id: GuildId) -> Result<Ref<'_, GuildId, Player>, ClientError> {
        if let Some(player) = self.players().get(&guild_id) {
            return Ok(player);
        }

        let node = self.best().await?;

        Ok(self.players().get_or_insert(guild_id, node).downgrade())
    }

    /// Clear out the map of guild states/updates for a shard that are waiting
    /// for their other half.
    ///
    /// We can do this by iterating over the map and removing the ones that we
    /// can calculate came from a shard.
    ///
    /// This map should be small or empty, and if it isn't, then it needs to be
    /// cleared out anyway.
    fn clear_shard_states(&self, shard_id: u64) {
        let shard_count = self.0.shard_count;

        self.0
            .server_updates
            .retain(|k, _| (k.0 >> 22) % shard_count != shard_id);
        self.0
            .sessions
            .retain(|k, _| (k.0 >> 22) % shard_count != shard_id);
    }
}

#[cfg(test)]
mod tests {
    use super::{ClientError, Lavalink};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{error::Error, fmt::Debug};

    assert_fields!(ClientError::SendingVoiceUpdate: source);
    assert_impl_all!(ClientError: Clone, Debug, Error, PartialEq, Send, Sync);
    assert_impl_all!(Lavalink: Clone, Debug, Send, Sync);
}
