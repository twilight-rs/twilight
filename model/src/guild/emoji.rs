use crate::{
    id::{EmojiId, RoleId},
    user::User,
};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Emoji {
    pub id: EmojiId,
    #[serde(default)]
    pub animated: bool,
    pub name: String,
    pub managed: bool,
    pub require_colons: bool,
    pub roles: Vec<RoleId>,
    pub user: Option<User>,
}

impl Key<'_, EmojiId> for Emoji {
    fn key(&self) -> EmojiId {
        self.id
    }
}
