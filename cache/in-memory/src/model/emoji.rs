use serde::Serialize;
use twilight_model::{
    guild::Emoji,
    id::{EmojiId, RoleId, UserId},
};

/// Represents a cached [`Emoji`].
///
/// [`Emoji`]: twilight_model::guild::Emoji
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedEmoji {
    pub(crate) animated: bool,
    pub(crate) available: bool,
    pub(crate) id: EmojiId,
    pub(crate) managed: bool,
    pub(crate) name: String,
    pub(crate) require_colons: bool,
    pub(crate) roles: Vec<RoleId>,
    pub(crate) user_id: Option<UserId>,
}

impl CachedEmoji {
    /// Whether the emoji is animated.
    pub const fn available(&self) -> bool {
        self.available
    }

    /// Whether this emoji can be used.
    ///
    /// May be false due to loss of Server Boosts.
    pub const fn animated(&self) -> bool {
        self.animated
    }

    /// ID of the Emoji.
    pub const fn id(&self) -> EmojiId {
        self.id
    }

    /// Whether the emoji is managed.
    pub const fn managed(&self) -> bool {
        self.managed
    }

    /// Name of the Emoji.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whether the emoji must be wrapped in colons.
    pub const fn require_colons(&self) -> bool {
        self.require_colons
    }

    /// List of roles allowed to use this emoji.
    pub fn roles(&self) -> &[RoleId] {
        &self.roles
    }

    /// ID of the user who created the emoji.
    pub const fn user_id(&self) -> Option<UserId> {
        self.user_id
    }
}

impl PartialEq<Emoji> for CachedEmoji {
    fn eq(&self, other: &Emoji) -> bool {
        self.id == other.id
            && self.animated == other.animated
            && self.managed == other.managed
            && self.name == other.name
            && self.require_colons == other.require_colons
            && self.roles == other.roles
            && self.user_id == other.user.as_ref().map(|user| user.id)
            && self.available == other.available
    }
}

#[cfg(test)]
mod tests {
    use super::CachedEmoji;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;
    use twilight_model::{guild::Emoji, id::EmojiId};

    assert_fields!(
        CachedEmoji: id,
        animated,
        name,
        managed,
        require_colons,
        roles,
        user_id
    );
    assert_impl_all!(CachedEmoji: Clone, Debug, Eq, PartialEq);

    #[test]
    fn test_eq_emoji() {
        let emoji = Emoji {
            id: EmojiId(123),
            animated: true,
            name: "foo".to_owned(),
            managed: false,
            require_colons: true,
            roles: vec![],
            user: None,
            available: true,
        };
        let cached = CachedEmoji {
            id: EmojiId(123),
            animated: true,
            name: "foo".to_owned(),
            managed: false,
            require_colons: true,
            roles: vec![],
            user_id: None,
            available: true,
        };

        assert_eq!(cached, emoji);
    }
}
