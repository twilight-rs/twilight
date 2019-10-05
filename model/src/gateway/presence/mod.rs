mod activity;
mod activity_assets;
mod activity_flags;
mod activity_party;
mod activity_secrets;
mod activity_timestamps;
mod activity_type;
mod client_status;
mod status;

pub use self::{
    activity::Activity,
    activity_assets::ActivityAssets,
    activity_flags::ActivityFlags,
    activity_party::ActivityParty,
    activity_secrets::ActivitySecrets,
    activity_timestamps::ActivityTimestamps,
    activity_type::ActivityType,
    client_status::ClientStatus,
    status::Status,
};

use crate::{
    id::{GuildId, UserId},
    user::User,
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Presence {
    #[serde(default)]
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
    pub game: Option<Activity>,
    pub guild_id: Option<GuildId>,
    pub nick: Option<String>,
    pub status: Status,
    pub user: UserOrId,
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[cfg_attr(feature = "serde-support", serde(untagged))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UserOrId {
    User(User),
    UserId(UserId),
}

#[cfg(feature = "serde-support")]
mod serde_support {
    use super::{Presence, UserOrId};
    use crate::id::UserId;
    use serde_mappable_seq::Key;

    impl Key<'_, UserId> for Presence {
        fn key(&self) -> UserId {
            match self.user {
                UserOrId::User(ref u) => u.id,
                UserOrId::UserId(id) => id,
            }
        }
    }
}
