use crate::{
    guild::Member,
    id::{marker::ScheduledEventMarker, Id},
    user::User,
};
use serde::{Deserialize, Serialize};

/// Container for user and member data returned by Discord.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildScheduledEventUser {
    /// ID of the scheduled event.
    pub guild_scheduled_event_id: Id<ScheduledEventMarker>,
    /// Member object of the user, if requested.
    pub member: Option<Member>,
    /// User object.
    pub user: User,
}
