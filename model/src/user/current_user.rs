use crate::id::UserId;
use std::hash::{Hash, Hasher};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct CurrentUser {
    pub id: UserId,
    pub avatar: Option<String>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub bot: bool,
    pub discriminator: String,
    pub email: Option<String>,
    pub mfa_enabled: bool,
    #[cfg_attr(feature = "serde-support", serde(rename = "username"))]
    pub name: String,
    pub verified: bool,
}

impl Hash for CurrentUser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
