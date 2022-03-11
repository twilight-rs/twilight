use crate::id::{
    marker::{GuildMarker, ScheduledEventMarker, UserMarker},
    Id,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildScheduledEventUserAdd {
    pub guild_scheduled_event_id: Id<ScheduledEventMarker>,
    pub user_id: Id<UserMarker>,
    pub guild_id: Id<GuildMarker>,
}
