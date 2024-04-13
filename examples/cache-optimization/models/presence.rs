use twilight_cache_inmemory::CacheablePresence;
use twilight_model::{
    gateway::presence::Presence,
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedPresence {
    pub guild_id: Id<GuildMarker>,
    pub user_id: Id<UserMarker>,
}

impl From<Presence> for MinimalCachedPresence {
    fn from(presence: Presence) -> Self {
        Self {
            guild_id: presence.guild_id,
            user_id: presence.user.id(),
        }
    }
}

impl PartialEq<Presence> for MinimalCachedPresence {
    fn eq(&self, other: &Presence) -> bool {
        self.guild_id == other.guild_id && self.user_id == other.user.id()
    }
}

impl CacheablePresence for MinimalCachedPresence {}
