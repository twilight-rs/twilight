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
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use futures_channel::mpsc::TrySendError;
use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, AtomicU16, AtomicU64, Ordering},
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
    pub fn get(&self, guild_id: &GuildId) -> Option<Ref<'_, GuildId, Player>> {
        self.players.get(guild_id)
    }

    /// Return a mutable reference to a player by guild ID.
    pub(crate) fn get_mut(&self, guild_id: &GuildId) -> Option<RefMut<'_, GuildId, Player>> {
        self.players.get_mut(guild_id)
    }

    /// Return a mutable reference to a player by guild ID or insert a new
    /// player linked to a given node.
    pub fn get_or_insert(&self, guild_id: GuildId, node: Node) -> RefMut<'_, GuildId, Player> {
        self.players
            .entry(guild_id)
            .or_insert_with(|| Player::new(guild_id, node))
    }

    /// Destroy a player on the remote node and remove it from the [`PlayerManager`].
    ///
    /// Returns an error if the associated node no longer is connected.
    pub fn destroy(&self, guild_id: GuildId) -> Result<(), TrySendError<OutgoingEvent>> {
        if let Some(node) = self.get(&guild_id).map(|kv| kv.value().node().clone()) {
            node.send(OutgoingEvent::from(Destroy::new(guild_id)))?;
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
    guild_id: GuildId,
    node: Node,
    paused: AtomicBool,
    position: i64,
    time: i64,
    volume: AtomicU16,
}

impl Player {
    pub(crate) fn new(guild_id: GuildId, node: Node) -> Self {
        Self {
            channel_id: AtomicU64::new(0),
            guild_id,
            node,
            paused: AtomicBool::new(false),
            position: 0,
            time: 0,
            volume: AtomicU16::new(100),
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
            self.guild_id,
            event
        );

        match event {
            OutgoingEvent::Pause(ref event) => self.paused.store(event.pause, Ordering::Release),
            OutgoingEvent::Volume(ref event) => {
                self.volume.store(event.volume as u16, Ordering::Release)
            }
            _ => {}
        }

        self.node.send(event)
    }

    /// Return an immutable reference to the node linked to the player.
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Return a copy of the player's channel ID.
    pub fn channel_id(&self) -> Option<ChannelId> {
        let channel_id = self.channel_id.load(Ordering::Acquire);

        if channel_id == 0 {
            None
        } else {
            Some(ChannelId(channel_id))
        }
    }

    /// Sets the channel ID the player is currently connected to.
    pub(crate) fn set_channel_id(&self, channel_id: Option<ChannelId>) {
        self.channel_id.store(
            channel_id.map(|id| id.0).unwrap_or(0_u64),
            Ordering::Release,
        );
    }

    /// Return an copy of the player's guild ID.
    pub fn guild_id(&self) -> GuildId {
        self.guild_id
    }

    /// Return a copy of whether the player is paused.
    pub fn paused(&self) -> bool {
        self.paused.load(Ordering::Acquire)
    }

    /// Return a copy of the player's position.
    pub fn position(&self) -> i64 {
        self.position
    }

    /// Return a mmutable reference to the player's channel ID.
    pub(crate) fn position_mut(&mut self) -> &mut i64 {
        &mut self.position
    }

    /// Return a copy of the player's time.
    pub fn time_ref(&self) -> i64 {
        self.time
    }

    /// Return a mutable reference to the player's channel ID.
    pub(crate) fn time_mut(&mut self) -> &mut i64 {
        &mut self.time
    }

    /// Return a copy of the player's volume.
    pub fn volume_ref(&self) -> u16 {
        self.volume.load(Ordering::Acquire)
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
