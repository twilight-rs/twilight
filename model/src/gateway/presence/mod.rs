mod activity;
mod activity_assets;
mod activity_flags;
mod activity_party;
mod activity_secrets;
mod activity_timestamps;
mod activity_type;
mod status;

pub use self::{
    activity::Activity,
    activity_assets::ActivityAssets,
    activity_flags::ActivityFlags,
    activity_party::ActivityParty,
    activity_secrets::ActivitySecrets,
    activity_timestamps::ActivityTimestamps,
    activity_type::ActivityType,
    status::Status,
};

use crate::{id::UserId, user::User};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Presence {
    pub activity: Option<Activity>,
    pub last_modified: Option<u64>,
    pub nick: Option<String>,
    pub status: Status,
    pub user: UserOrId,
}

impl Key<'_, UserId> for Presence {
    fn key(&self) -> UserId {
        match self.user {
            UserOrId::User(ref u) => u.id,
            UserOrId::UserId(id) => id,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum UserOrId {
    User(User),
    UserId(UserId),
}
