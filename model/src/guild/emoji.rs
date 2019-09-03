use crate::{
    id::{EmojiId, RoleId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Emoji {
    pub id: EmojiId,
    #[serde(default)]
    pub animated: bool,
    pub name: String,
    pub managed: bool,
    pub require_colons: bool,
    pub roles: Vec<RoleId>,
    pub uesr: Option<User>,
}
