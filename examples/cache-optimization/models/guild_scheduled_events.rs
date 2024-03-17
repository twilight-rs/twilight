use twilight_cache_inmemory::traits::CacheableGuildScheduledEvent;
use twilight_model::{id::{marker::{GuildMarker, ScheduledEventMarker}, Id}, guild::scheduled_event::GuildScheduledEvent};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalGuildScheduledEvent {
    guild_id: Id<GuildMarker>,
    id: Id<ScheduledEventMarker>,
    user_count: Option<u64>,
}

impl From<GuildScheduledEvent> for MinimalGuildScheduledEvent {
    fn from(event: GuildScheduledEvent) -> Self {
        Self { guild_id: event.guild_id, id: event.id, user_count: event.user_count }
    }
}

impl PartialEq<GuildScheduledEvent> for MinimalGuildScheduledEvent {
    fn eq(&self, other: &GuildScheduledEvent) -> bool {
        self.id == other.id
    }
}

impl CacheableGuildScheduledEvent for MinimalGuildScheduledEvent {
    fn add_user(
        &mut self,
        _guild_id: Id<GuildMarker>,
        _event_id: Id<ScheduledEventMarker>,
        _user_id: Id<twilight_model::id::marker::UserMarker>,
    ) {
        self.user_count = self.user_count.map(|c| c.saturating_add(1));
    }

    fn remove_user(
        &mut self,
        _guild_id: Id<GuildMarker>,
        _event_id: Id<ScheduledEventMarker>,
        _user_id: Id<twilight_model::id::marker::UserMarker>,
    ) {
        self.user_count = self.user_count.map(|c| c.saturating_sub(1));
    }
}
