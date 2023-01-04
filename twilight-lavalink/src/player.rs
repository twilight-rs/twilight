//! Players containing information about active playing state within guilds and
//! allowing you to send events to connected nodes.
//!
//! Use the [`PlayerManager`] to retrieve existing [players] for guilds and
//! use those players to do things like [send events] or [read the position] of
//! the active audio.
//!
//! [players]: Player
//! [send events]: Player::send
//! [read the position]: Player::position

use crate::{
    model::{Destroy, OutgoingEvent},
    node::{Node, NodeSenderError},
};
use dashmap::DashMap;
use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering},
        Arc,
    },
};
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};

/// Retrieve and create players for guilds.
///
/// The player manager contains all of the players for all guilds over all
/// nodes, and can be used to read player information and send events to nodes.
#[derive(Clone, Debug, Default)]
pub struct PlayerManager {
    pub(crate) players: Arc<DashMap<Id<GuildMarker>, Arc<Player>>>,
}

impl PlayerManager {
    /// Create a new player manager.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Return an immutable reference to a player by guild ID.
    pub fn get(&self, guild_id: &Id<GuildMarker>) -> Option<Arc<Player>> {
        self.players.get(guild_id).map(|r| Arc::clone(r.value()))
    }

    /// Return a mutable reference to a player by guild ID or insert a new
    /// player linked to a given node.
    pub fn get_or_insert(&self, guild_id: Id<GuildMarker>, node: Arc<Node>) -> Arc<Player> {
        let player = self
            .players
            .entry(guild_id)
            .or_insert_with(|| Arc::new(Player::new(guild_id, node)));

        Arc::clone(&player)
    }

    /// Destroy a player on the remote node and remove it from the [`PlayerManager`].
    ///
    /// # Errors
    ///
    /// Returns a [`NodeSenderErrorType::Sending`] error type if node is no
    /// longer connected.
    ///
    /// [`NodeSenderErrorType::Sending`]: crate::node::NodeSenderErrorType::Sending
    pub fn destroy(&self, guild_id: Id<GuildMarker>) -> Result<(), NodeSenderError> {
        if let Some(player) = self.get(&guild_id) {
            player
                .node()
                .send(OutgoingEvent::from(Destroy::new(guild_id)))?;
            self.players.remove(&guild_id);
        }

        Ok(())
    }
}

/// A player for a guild connected to a node.
///
/// This can be used to send events over a node and to read the details of a
/// player for a guild.
#[derive(Debug)]
pub struct Player {
    channel_id: AtomicU64,
    guild_id: Id<GuildMarker>,
    node: Arc<Node>,
    paused: AtomicBool,
    position: AtomicI64,
    time: AtomicI64,
    volume: AtomicI64,
}

impl Player {
    pub(crate) const fn new(guild_id: Id<GuildMarker>, node: Arc<Node>) -> Self {
        Self {
            channel_id: AtomicU64::new(0),
            guild_id,
            node,
            paused: AtomicBool::new(false),
            position: AtomicI64::new(0),
            time: AtomicI64::new(0),
            volume: AtomicI64::new(100),
        }
    }

    /// Send an event to the player's node.
    ///
    /// Returns a `futures_channel` `TrySendError` if the node has been removed.
    ///
    /// # Examples
    ///
    /// Send a [`Play`] and [`Pause`] event:
    ///
    /// ```
    /// use twilight_lavalink::{
    ///     model::{Pause, Play},
    ///     Lavalink,
    /// };
    /// # use twilight_model::id::Id;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let (guild_id, user_id) = (Id::new(1), Id::new(2));
    /// # let track = String::new();
    ///
    /// let lavalink = Lavalink::new(user_id, 10);
    /// let players = lavalink.players();
    ///
    /// if let Some(player) = players.get(&guild_id) {
    ///     player.send(Play::from((guild_id, track)))?;
    ///     player.send(Pause::from((guild_id, true)))?;
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`NodeSenderErrorType::Sending`] error type if node is no
    /// longer connected.
    ///
    /// [`NodeSenderErrorType::Sending`]: crate::node::NodeSenderErrorType::Sending
    /// [`Pause`]: crate::model::outgoing::Pause
    /// [`Play`]: crate::model::outgoing::Play
    pub fn send(&self, event: impl Into<OutgoingEvent>) -> Result<(), NodeSenderError> {
        self._send(event.into())
    }

    fn _send(&self, event: OutgoingEvent) -> Result<(), NodeSenderError> {
        tracing::debug!("sending event on guild player {}: {event:?}", self.guild_id);

        match &event {
            OutgoingEvent::Pause(event) => self.paused.store(event.pause, Ordering::Release),
            OutgoingEvent::Volume(event) => {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                self.volume.store(event.volume, Ordering::Release);
            }
            _ => {}
        }

        self.node.send(event)
    }

    /// Return an immutable reference to the node linked to the player.
    pub const fn node(&self) -> &Arc<Node> {
        &self.node
    }

    /// Return the player's channel ID.
    pub fn channel_id(&self) -> Option<Id<ChannelMarker>> {
        let channel_id = self.channel_id.load(Ordering::Acquire);

        if channel_id == 0 {
            None
        } else {
            Some(Id::new(channel_id))
        }
    }

    /// Sets the channel ID the player is currently connected to.
    pub(crate) fn set_channel_id(&self, channel_id: Option<Id<ChannelMarker>>) {
        self.channel_id
            .store(channel_id.map_or(0_u64, Id::get), Ordering::Release);
    }

    /// Return the player's guild ID.
    pub const fn guild_id(&self) -> Id<GuildMarker> {
        self.guild_id
    }

    /// Return whether the player is paused.
    pub fn paused(&self) -> bool {
        self.paused.load(Ordering::Acquire)
    }

    /// Return the player's position.
    pub fn position(&self) -> i64 {
        self.position.load(Ordering::Relaxed)
    }

    /// Set the player's position.
    pub(crate) fn set_position(&self, position: i64) {
        self.position.store(position, Ordering::Release);
    }

    /// Return the player's time.
    pub fn time(&mut self) -> i64 {
        self.time.load(Ordering::Relaxed)
    }

    /// Set the player's time.
    pub(crate) fn set_time(&self, time: i64) {
        self.time.store(time, Ordering::Release);
    }

    /// Return the player's volume.
    pub fn volume(&self) -> i64 {
        self.volume.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::{Player, PlayerManager};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(PlayerManager: Debug, Default, Send, Sync);
    assert_impl_all!(Player: Debug, Send, Sync);
}
