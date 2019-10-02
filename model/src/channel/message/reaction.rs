use crate::channel::ReactionType;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MessageReaction {
    pub count: u64,
    pub emoji: ReactionType,
    pub me: bool,
}
