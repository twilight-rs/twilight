use crate::{
    id::{GuildId, RoleId},
    user::User,
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Member {
    pub deaf: bool,
    pub guild_id: Option<GuildId>,
    pub joined_at: Option<String>,
    pub mute: bool,
    pub nick: Option<String>,
    pub premium_since: Option<String>,
    pub roles: Vec<RoleId>,
    pub user: User,
}

#[cfg(feature = "serde-support")]
mod serde_support {
    use super::Member;
    use crate::id::UserId;
    use serde_mappable_seq::Key;

    impl Key<'_, UserId> for Member {
        fn key(&self) -> UserId {
            self.user.id
        }
    }
}
