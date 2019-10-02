use crate::{
    id::UserId,
    user::{PremiumType, UserFlags},
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UserProfile {
    pub id: UserId,
    pub avatar: Option<String>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub bot: bool,
    pub discriminator: String,
    pub email: Option<String>,
    pub flags: Option<UserFlags>,
    pub locale: Option<String>,
    pub mfa_enabled: Option<bool>,
    #[cfg_attr(feature = "serde-support", serde(rename = "username"))]
    pub name: String,
    pub premium_type: Option<PremiumType>,
    pub verified: Option<bool>,
}
