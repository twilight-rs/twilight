use crate::{
    id::{EmojiId, RoleId},
    user::User,
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Emoji {
    pub id: EmojiId,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub animated: bool,
    pub name: String,
    pub managed: bool,
    pub require_colons: bool,
    pub roles: Vec<RoleId>,
    pub user: Option<User>,
}

#[cfg(feature = "serde-support")]
mod serde_support {
    use super::Emoji;
    use crate::id::EmojiId;
    use serde_mappable_seq::Key;

    impl Key<'_, EmojiId> for Emoji {
        fn key(&self) -> EmojiId {
            self.id
        }
    }
}
