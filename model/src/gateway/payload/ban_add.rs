use crate::{id::GuildId, user::User};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BanAdd {
    pub guild_id: GuildId,
    pub user: User,
}
