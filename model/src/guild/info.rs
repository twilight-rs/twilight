use crate::{guild::Permissions, id::GuildId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GuildInfo {
    pub id: GuildId,
    pub icon: Option<String>,
    pub name: String,
    pub owner: bool,
    pub permissions: Permissions,
}
