use crate::guild::PartialGuild;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuildDelete {
    pub guild: PartialGuild,
}
