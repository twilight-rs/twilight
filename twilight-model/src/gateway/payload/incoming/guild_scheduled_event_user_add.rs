use crate::id::{
    marker::{GuildMarker, ScheduledEventMarker, UserMarker},
    Id,
};
use serde::{Deserialize, Serialize};

/// Sent when a user has subscribed to a guild scheduled event.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildScheduledEventUserAdd {
    /// Guild ID of the scheduled event.
    pub guild_id: Id<GuildMarker>,
    /// ID of the guild scheduled event.
    pub guild_scheduled_event_id: Id<ScheduledEventMarker>,
    /// ID of the user who has subscribed to the guild scheduled event.
    pub user_id: Id<UserMarker>,
}
