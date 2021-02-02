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

use crate::{model::*, node::Node};
use dashmap::DashMap;
use futures_channel::mpsc::TrySendError;
use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, AtomicI64, AtomicU16, Ordering},
        Arc,
    },
};
use twilight_model::id::{ChannelId, GuildId};

/// Retrieve and create players for guilds.
///
/// The player manager contains all of the players for all guilds over all
/// nodes, and can be used to read player information and send events to nodes.
#[derive(Clone, Debug, Default)]
pub struct PlayerManager {
    pub(crate) players: Arc<DashMap<GuildId, Player>>,
}

impl PlayerManager {
    /// Create a new player manager.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Return an immutable reference to a player by guild ID.
    pub fn get(&self, guild_id: &GuildId) -> Option<Player> {
        // Clippy recommends removing the `map` call, which is just wrong.
        #[allow(clippy::map_clone)]
        self.players.get(guild_id).map(|player| player.clone())
    }

    /// Return a mutable reference to a player by guild ID or insert a new
    /// player linked to a given node.
    pub fn get_or_insert(&self, guild_id: GuildId, node: Node) -> Player {
        self.players
            .entry(guild_id)
            .or_insert_with(|| Player::new(guild_id, node))
            .clone()
    }
}

#[derive(Debug)]
struct PlayerRef {
    channel_id: Option<ChannelId>,
    guild_id: GuildId,
    node: Node,
    paused: AtomicBool,
    playing: Option<()>,
    position: AtomicI64,
    time: AtomicI64,
    volume: AtomicU16,
}

/// A player for a guild connected to a node.
///
/// This can be used to send events over a node and to read the details of a
/// player for a guild.
#[derive(Clone, Debug)]
pub struct Player(Arc<PlayerRef>);

impl Player {
    pub(crate) fn new(guild_id: GuildId, node: Node) -> Self {
        Self(Arc::new(PlayerRef {
            channel_id: None,
            guild_id,
            node,
            paused: AtomicBool::new(false),
            playing: None,
            position: AtomicI64::new(0),
            time: AtomicI64::new(0),
            volume: AtomicU16::new(0),
        }))
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
    /// use twilight_lavalink::{model::{Play, Pause}, Lavalink};
    /// # use twilight_model::id::{GuildId, UserId};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let (guild_id, user_id) = (GuildId(1), UserId(2));
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
    /// [`Pause`]: crate::model::outgoing::Pause
    /// [`Play`]: crate::model::outgoing::Play
    pub fn send(&self, event: impl Into<OutgoingEvent>) -> Result<(), TrySendError<OutgoingEvent>> {
        self._send(event.into())
    }

    fn _send(&self, event: OutgoingEvent) -> Result<(), TrySendError<OutgoingEvent>> {
        tracing::debug!(
            "sending event on guild player {}: {:?}",
            self.0.guild_id,
            event
        );

        if let OutgoingEvent::Pause(ref event) = event {
            self.0.paused.store(event.pause, Ordering::Release);
        }

        self.0.node.send(event)
    }

    /// Return an immutable reference to the node linked to the player.
    pub fn node(&self) -> &Node {
        &self.0.node
    }

    /// Return the player's channel ID.
    pub fn channel_id(&self) -> Option<ChannelId> {
        self.0.channel_id.as_ref().copied()
    }

    /// Return the player's guild ID.
    pub fn guild_id(&self) -> GuildId {
        self.0.guild_id
    }

    /// Return whether the player is paused.
    pub fn paused(&self) -> bool {
        self.0.paused.load(Ordering::Acquire)
    }

    /// Return the player's position.
    pub fn position(&self) -> i64 {
        self.0.position.load(Ordering::Relaxed)
    }

    /// Set the player's position.
    pub(crate) fn set_position(&self, position: i64) {
        self.0.position.store(position, Ordering::Release)
    }

    /// Return the player's time.
    pub fn time(&mut self) -> i64 {
        self.0.time.load(Ordering::Relaxed)
    }

    /// Set the player's time.
    pub(crate) fn set_time(&self, time: i64) {
        self.0.time.store(time, Ordering::Release)
    }

    /// Return the player's volume.
    pub fn volume(&self) -> u16 {
        self.0.volume.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::{Player, PlayerManager};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(PlayerManager: Clone, Debug, Default, Send, Sync);
    assert_impl_all!(Player: Debug, Send, Sync);
}
