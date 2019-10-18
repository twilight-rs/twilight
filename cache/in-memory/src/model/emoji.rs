use dawn_model::{
    guild::Emoji,
    id::{EmojiId, RoleId},
    user::User,
};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CachedEmoji {
    pub id: EmojiId,
    pub animated: bool,
    pub name: String,
    pub managed: bool,
    pub require_colons: bool,
    pub roles: Vec<RoleId>,
    pub user: Option<Arc<User>>,
}

impl PartialEq<Emoji> for CachedEmoji {
    fn eq(&self, other: &Emoji) -> bool {
        self.id == other.id
            && self.animated == other.animated
            && self.managed == other.managed
            && self.name == other.name
            && self.require_colons == other.require_colons
            && self.roles == other.roles
    }
}

#[cfg(test)]
mod tests {
    use super::CachedEmoji;
    use dawn_model::{guild::Emoji, id::EmojiId};
    use std::fmt::Debug;

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
        };
        let cached = CachedEmoji {
            id: EmojiId(123),
            animated: true,
            name: "foo".to_owned(),
            managed: false,
            require_colons: true,
            roles: vec![],
            user: None,
        };

        assert_eq!(cached, emoji);
    }

    #[test]
    fn test_fields() {
        static_assertions::assert_fields!(
            CachedEmoji: id,
            animated,
            name,
            managed,
            require_colons,
            roles,
            user
        );
    }

    #[test]
    fn test_impls() {
        static_assertions::assert_impl_all!(CachedEmoji: Clone, Debug, Eq, PartialEq);
    }
}
