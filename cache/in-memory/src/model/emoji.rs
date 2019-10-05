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
