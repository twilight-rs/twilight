use crate::{guild::Member, id::GuildId};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MemberAdd {
    pub guild_id: GuildId,
    pub member: Member,
}
