use crate::{
    id::{EmojiId, RoleId},
    user::User,
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Emoji {
    // This does not need to be optional here as it can only be optional
    // in a unicode emoji. Which can only happen in reactions, and we use
    // another struct for emojis in that case.
    pub id: EmojiId,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub animated: bool,
    pub name: String,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub managed: bool,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub require_colons: bool,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub roles: Vec<RoleId>,
    pub user: Option<User>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub available: bool,
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
