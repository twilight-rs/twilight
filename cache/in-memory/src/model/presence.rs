use serde::Serialize;
use twilight_model::{
    gateway::presence::{Activity, ClientStatus, Presence, Status, UserOrId},
    id::{GuildId, UserId},
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedPresence {
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
    pub guild_id: GuildId,
    pub status: Status,
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

impl From<&'_ Presence> for CachedPresence {
    fn from(presence: &'_ Presence) -> Self {
        Self {
            activities: presence.activities.clone(),
            client_status: presence.client_status.clone(),
            guild_id: presence.guild_id,
            status: presence.status,
            user_id: presence_user_id(&presence.user),
        }
    }
}

fn presence_user_id(user: &UserOrId) -> UserId {
    match user {
        UserOrId::User(ref u) => u.id,
        UserOrId::UserId { id } => *id,
    }
}
