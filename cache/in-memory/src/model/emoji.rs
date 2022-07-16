use serde::Serialize;
use twilight_model::{
    guild::Emoji,
    id::{
        marker::{EmojiMarker, RoleMarker, UserMarker},
        Id,
    },
};

/// Represents a cached [`Emoji`].
///
/// [`Emoji`]: twilight_model::guild::Emoji
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedEmoji {
    pub(crate) animated: bool,
    pub(crate) available: bool,
    pub(crate) id: Id<EmojiMarker>,
    pub(crate) managed: bool,
    pub(crate) name: String,
    pub(crate) require_colons: bool,
    pub(crate) roles: Vec<Id<RoleMarker>>,
    pub(crate) user_id: Option<Id<UserMarker>>,
}

impl CachedEmoji {
    /// Whether this emoji can be used.
    ///
    /// May be false due to loss of Server Boosts.
    pub const fn available(&self) -> bool {
        self.available
    }

    /// Whether the emoji is animated.
    pub const fn animated(&self) -> bool {
        self.animated
    }

    /// ID of the Emoji.
    pub const fn id(&self) -> Id<EmojiMarker> {
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
    pub fn roles(&self) -> &[Id<RoleMarker>] {
        &self.roles
    }

    /// ID of the user who created the emoji.
    pub const fn user_id(&self) -> Option<Id<UserMarker>> {
        self.user_id
    }

    /// Construct a cached emoji from its [`twilight_model`] form.
    pub(crate) fn from_model(emoji: Emoji) -> Self {
        let Emoji {
            animated,
            available,
            id,
            managed,
            name,
            require_colons,
            roles,
            user,
        } = emoji;

        CachedEmoji {
            animated,
            available,
            id,
            managed,
            name,
            require_colons,
            roles,
            user_id: user.map(|user| user.id),
        }
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
    use serde::Serialize;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;
    use twilight_model::{guild::Emoji, id::Id};

    assert_fields!(
        CachedEmoji: id,
        animated,
        name,
        managed,
        require_colons,
        roles,
        user_id
    );
    assert_impl_all!(
        CachedEmoji: Clone,
        Debug,
        Eq,
        PartialEq,
        PartialEq<Emoji>,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn eq_emoji() {
        let emoji = Emoji {
            id: Id::new(123),
            animated: true,
            name: "foo".to_owned(),
            managed: false,
            require_colons: true,
            roles: vec![],
            user: None,
            available: true,
        };
        let cached = CachedEmoji {
            id: Id::new(123),
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
