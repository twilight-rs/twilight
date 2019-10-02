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
use std::hash::{Hash, Hasher};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub id: UserId,
    pub avatar: Option<String>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub bot: bool,
    pub discriminator: String,
    #[cfg_attr(feature = "serde-support", serde(rename = "username"))]
    pub name: String,
}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(feature = "serde-support")]
mod serde_mappable_seq_support {
    use super::User;
    use crate::id::UserId;
    use serde_mappable_seq::Key;

    impl Key<'_, UserId> for User {
        fn key(&self) -> UserId {
            self.id
        }
    }
}
