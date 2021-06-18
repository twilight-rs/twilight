use serde::Serialize;
use twilight_model::{
    guild::Emoji,
    id::{EmojiId, RoleId, UserId},
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedEmoji {
    pub id: EmojiId,
    pub animated: bool,
    pub name: String,
    pub managed: bool,
    pub require_colons: bool,
    pub roles: Vec<RoleId>,
    /// ID of the user who created the emoji.
    pub user_id: Option<UserId>,
    pub available: bool,
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
