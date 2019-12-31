use crate::id::GuildId;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuildDelete {
    pub id: GuildId,
    // If `unavailable` is `None` the user was removed from the guild.
    pub unavailable: Option<bool>,
}
