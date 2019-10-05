use crate::id::UserId;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Default, Debug, Eq, Hash, PartialEq)]
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
