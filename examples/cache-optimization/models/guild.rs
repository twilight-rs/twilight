use twilight_cache_inmemory::CacheableGuild;
use twilight_model::{
    gateway::payload::incoming::GuildUpdate,
    guild::Guild,
    id::{
        Id,
        marker::{GuildMarker, UserMarker},
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedGuild {
    pub id: Id<GuildMarker>,
    pub owner_id: Id<UserMarker>,
    pub member_count: Option<u64>,
}

impl From<Guild> for MinimalCachedGuild {
    fn from(guild: Guild) -> Self {
        Self {
            id: guild.id,
            owner_id: guild.owner_id,
            member_count: guild.member_count,
        }
    }
}

impl PartialEq<Guild> for MinimalCachedGuild {
    fn eq(&self, other: &Guild) -> bool {
        self.id == other.id
            && self.owner_id == other.owner_id
            && self.member_count == other.member_count
    }
}

impl CacheableGuild for MinimalCachedGuild {
    fn id(&self) -> Id<GuildMarker> {
        self.id
    }

    fn owner_id(&self) -> Id<UserMarker> {
        self.owner_id
    }

    fn set_unavailable(&mut self, _unavailable: Option<bool>) {
        // We don't store this information, so this is a no-op
    }

    fn update_with_guild_update(&mut self, guild_update: &GuildUpdate) {
        self.id = guild_update.id;
        self.owner_id = guild_update.owner_id;
        self.member_count = guild_update.member_count;
    }

    fn increase_member_count(&mut self, amount: u64) {
        if let Some(count) = self.member_count.as_mut() {
            *count += amount;
        }
    }

    fn decrease_member_count(&mut self, amount: u64) {
        if let Some(count) = self.member_count.as_mut() {
            *count -= amount;
        }
    }
}
