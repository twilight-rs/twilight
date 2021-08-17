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
    pub(crate) activities: Vec<Activity>,
    pub(crate) client_status: ClientStatus,
    pub(crate) guild_id: GuildId,
    pub(crate) status: Status,
    pub(crate) user_id: UserId,
}

impl CachedPresence {
    /// Current activities.
    pub fn activities(&self) -> &[Activity] {
        &self.activities
    }

    /// Platform-dependent status.
    pub const fn client_status(&self) -> &ClientStatus {
        &self.client_status
    }

    /// ID of the guild.
    pub const fn guild_id(&self) -> GuildId {
        self.guild_id
    }

    /// Status of the user.
    pub const fn status(&self) -> Status {
        self.status
    }

    /// ID of the user.
    pub const fn user_id(&self) -> UserId {
        self.user_id
    }
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
