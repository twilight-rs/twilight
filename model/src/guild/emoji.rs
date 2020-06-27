use crate::{
    id::{EmojiId, RoleId},
    user::User,
};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Emoji {
    #[serde(default)]
    pub animated: bool,
    #[serde(default)]
    pub available: bool,
    // This does not need to be optional here as it can only be optional
    // in a unicode emoji. Which can only happen in reactions, and we use
    // another struct for emojis in that case.
    pub id: EmojiId,
    #[serde(default)]
    pub managed: bool,
    pub name: String,
    #[serde(default)]
    pub require_colons: bool,
    #[serde(default)]
    pub roles: Vec<RoleId>,
    pub user: Option<User>,
}

impl Key<'_, EmojiId> for Emoji {
    fn key(&self) -> EmojiId {
        self.id
    }
}
