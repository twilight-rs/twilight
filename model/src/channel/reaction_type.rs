use crate::id::EmojiId;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[cfg_attr(feature = "serde-support", serde(untagged))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ReactionType {
    Custom {
        animated: bool,
        id: EmojiId,
        name: Option<String>,
    },
    Unicode(String),
}
