use serde::Serialize;
use twilight_model::{
    gateway::presence::{Activity, ClientStatus, Presence, Status, UserOrId},
    id::{GuildId, UserId},
};

/// Represents a cached [`Presence`].
///
/// [`Presence`]: twilight_model::gateway::presence::Presence
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedPresence {
    /// Current activities.
    pub activities: Vec<Activity>,
    /// Platform-dependent status.
    pub client_status: ClientStatus,
    /// ID of the guild.
    pub guild_id: GuildId,
    /// Status of the user.
    pub status: Status,
    /// ID of the user.
    pub user_id: UserId,
}

impl PartialEq<Presence> for CachedPresence {
    fn eq(&self, other: &Presence) -> bool {
        (
            &self.activities,
            &self.client_status,
            self.guild_id,
            self.status,
            self.user_id,
        ) == (
            &other.activities,
            &other.client_status,
            other.guild_id,
            other.status,
            presence_user_id(&other.user),
        )
    }
}

impl From<Presence> for CachedPresence {
    fn from(presence: Presence) -> Self {
        Self {
            activities: presence.activities,
            client_status: presence.client_status,
            guild_id: presence.guild_id,
            status: presence.status,
            user_id: presence_user_id(&presence.user),
        }
    }
}

const fn presence_user_id(user: &UserOrId) -> UserId {
    match user {
        UserOrId::User(ref u) => u.id,
        UserOrId::UserId { id } => *id,
    }
}
