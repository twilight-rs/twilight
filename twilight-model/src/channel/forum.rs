use crate::id::{
    marker::{EmojiMarker, TagMarker},
    Id,
};
use serde::{
    de::{Deserializer, Error as DeError},
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DefaultReaction {
    #[serde(flatten)]
    pub emoji: ForumReaction,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ForumReaction {
    EmojiId(Id<EmojiMarker>),
    EmojiName(String),
}

impl<'de> Deserialize<'de> for ForumReaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let internal = InternalForumReaction::deserialize(deserializer)?;
        match (internal.emoji_id, internal.emoji_name) {
            (None, Some(name)) => Ok(Self::EmojiName(name)),
            (Some(id), None) => Ok(Self::EmojiId(id)),
            _ => Err(DeError::custom("too few or many fields")),
        }
    }
}

impl Serialize for ForumReaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("ForumReaction", 2)?;

        match self {
            Self::EmojiId(id) => {
                state.serialize_field("emoji_id", &Some(id))?;
                state.serialize_field("emoji_name", &None::<()>)?;
            }
            Self::EmojiName(name) => {
                state.serialize_field("emoji_id", &None::<()>)?;
                state.serialize_field("emoji_name", &Some(name))?;
            }
        }

        state.end()
    }
}

/// Helper struct for [`ForumReaction`]'s deserialization implementation.
#[derive(Deserialize)]
#[serde(rename = "ForumReaction")]
struct InternalForumReaction {
    emoji_id: Option<Id<EmojiMarker>>,
    emoji_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ForumTag {
    #[serde(flatten)]
    pub emoji: ForumReaction,
    pub id: Id<TagMarker>,
    pub moderated: bool,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::{DefaultReaction, ForumReaction, ForumTag};
    use crate::id::{
        marker::{EmojiMarker, TagMarker},
        Id,
    };
    use serde_test::Token;

    const EMOJI_ID: Id<EmojiMarker> = Id::new(1);
    const TAG_ID: Id<TagMarker> = Id::new(2);

    #[test]
    fn default_reaction() {
        let value = DefaultReaction {
            emoji: ForumReaction::EmojiName("name".into()),
        };

        // justification for `Token::Map`: https://github.com/serde-rs/serde/issues/1346#issuecomment-451715157
        serde_test::assert_tokens(
            &value,
            &[
                Token::Map { len: None },
                Token::Str("emoji_id"),
                Token::None,
                Token::Str("emoji_name"),
                Token::Some,
                Token::Str("name"),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn forum_tag() {
        let value = ForumTag {
            emoji: ForumReaction::EmojiId(EMOJI_ID),
            id: TAG_ID,
            moderated: false,
            name: "other".into(),
        };

        // justification for `Token::Map`: https://github.com/serde-rs/serde/issues/1346#issuecomment-451715157
        serde_test::assert_tokens(
            &value,
            &[
                Token::Map { len: None },
                Token::Str("emoji_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("emoji_name"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("moderated"),
                Token::Bool(false),
                Token::Str("name"),
                Token::Str("other"),
                Token::MapEnd,
            ],
        );
    }
}
