use twilight_cache_inmemory::CacheableChannel;
use twilight_model::{
    channel::{Channel, ChannelType, permission_overwrite::PermissionOverwrite},
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker},
    },
    util::Timestamp,
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedChannel {
    pub id: Id<ChannelMarker>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub kind: ChannelType,
    pub parent_id: Option<Id<ChannelMarker>>,
}

impl From<Channel> for MinimalCachedChannel {
    fn from(channel: Channel) -> Self {
        Self {
            id: channel.id,
            guild_id: channel.guild_id,
            kind: channel.kind,
            parent_id: channel.parent_id,
        }
    }
}

impl PartialEq<Channel> for MinimalCachedChannel {
    fn eq(&self, other: &Channel) -> bool {
        self.id == other.id
            && self.guild_id == other.guild_id
            && self.kind == other.kind
            && self.parent_id == other.parent_id
    }
}

impl CacheableChannel for MinimalCachedChannel {
    fn guild_id(&self) -> Option<Id<GuildMarker>> {
        self.guild_id
    }

    fn id(&self) -> Id<ChannelMarker> {
        self.id
    }

    fn kind(&self) -> ChannelType {
        self.kind
    }

    fn parent_id(&self) -> Option<Id<ChannelMarker>> {
        self.parent_id
    }

    fn permission_overwrites(&self) -> Option<&[PermissionOverwrite]> {
        None
    }

    fn set_last_pin_timestamp(&mut self, _timestamp: Option<Timestamp>) {
        // We don't store this information, so this is a no-op
    }
}
