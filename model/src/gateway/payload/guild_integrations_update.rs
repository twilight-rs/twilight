use crate::id::GuildId;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GuildIntegrationsUpdate {
    pub guild_id: GuildId,
}
