mod connection;
mod connection_visibility;
mod current_user;
mod flags;
mod premium_type;
mod profile;

pub use self::{
    connection::Connection,
    connection_visibility::ConnectionVisibility,
    current_user::CurrentUser,
    flags::UserFlags,
    premium_type::PremiumType,
    profile::UserProfile,
};

use crate::id::UserId;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct User {
    pub id: UserId,
    pub avatar: Option<String>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub bot: bool,
    pub discriminator: String,
    #[cfg_attr(feature = "serde-support", serde(rename = "username"))]
    pub name: String,
    pub mfa_enabled: Option<bool>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<UserFlags>,
    pub premium_type: Option<PremiumType>,
    pub system: Option<bool>,
}

#[cfg(feature = "serde-support")]
mod serde_support {
    use super::User;
    use crate::id::UserId;
    use serde_mappable_seq::Key;

    impl Key<'_, UserId> for User {
        fn key(&self) -> UserId {
            self.id
        }
    }
}
