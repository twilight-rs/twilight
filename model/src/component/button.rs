use crate::id::EmojiId;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Hash)]
pub struct Button {
    pub style: ButtonStyle,
    pub emoji: Option<PartialEmoji>,
    pub label: Option<String>,
    pub custom_id: Option<String>,
    pub url: Option<String>,
    pub disabled: bool,
}

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Hash)]
pub struct PartialEmoji {
    pub id: Option<EmojiId>,
    pub name: String,
    pub animated: bool,
}
