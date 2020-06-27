mod connection;
mod connection_visibility;
mod current_user;
mod flags;
mod premium_type;
mod profile;

pub use self::{
    connection::Connection, connection_visibility::ConnectionVisibility, current_user::CurrentUser,
    flags::UserFlags, premium_type::PremiumType, profile::UserProfile,
};

use crate::id::UserId;
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct User {
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: bool,
    pub discriminator: String,
    pub email: Option<String>,
    pub flags: Option<UserFlags>,
    pub id: UserId,
    pub locale: Option<String>,
    pub mfa_enabled: Option<bool>,
    #[serde(rename = "username")]
    pub name: String,
    pub premium_type: Option<PremiumType>,
    pub public_flags: Option<UserFlags>,
    pub system: Option<bool>,
    pub verified: Option<bool>,
}

impl Key<'_, UserId> for User {
    fn key(&self) -> UserId {
        self.id
    }
}
