use crate::id::{
    marker::{EmojiMarker, TagMarker},
    Id,
};
use serde::{
    de::Error as DeError, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DefaultReaction {
    EmojiId(Id<EmojiMarker>),
    EmojiName(String),
}

impl<'de> Deserialize<'de> for DefaultReaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let internal = InternalDefaultReaction::deserialize(deserializer)?;
        match (internal.emoji_id, internal.emoji_name) {
            (None, Some(name)) => Ok(Self::EmojiName(name)),
            (Some(id), None) => Ok(Self::EmojiId(id)),
            _ => Err(DeError::custom("too few or many fields")),
        }
    }
}

impl Serialize for DefaultReaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("DefaultReaction", 2)?;

        match self {
            DefaultReaction::EmojiId(id) => {
                state.serialize_field("emoji_id", &Some(id))?;
                state.serialize_field("emoji_name", &None::<()>)?;
            }
            DefaultReaction::EmojiName(name) => {
                state.serialize_field("emoji_id", &None::<()>)?;
                state.serialize_field("emoji_name", &Some(name))?;
            }
        }

        state.end()
    }
}

/// Helper struct for [`DefaultReaction`]'s deserialization implementation.
#[derive(Deserialize)]
#[serde(rename = "DefaultReaction")]
struct InternalDefaultReaction {
    emoji_id: Option<Id<EmojiMarker>>,
    emoji_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ForumTag {
    #[serde(flatten)]
    pub emoji: DefaultReaction,
    pub id: Id<TagMarker>,
    pub moderated: bool,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::{DefaultReaction, ForumTag};
    use crate::id::{
        marker::{EmojiMarker, TagMarker},
        Id,
    };
    use serde_test::Token;

    const EMOJI_ID: Id<EmojiMarker> = Id::new(1);
    const TAG_ID: Id<TagMarker> = Id::new(2);

    #[test]
    fn default_reaction() {
        let value = DefaultReaction::EmojiName("name".into());

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "DefaultReaction",
                    len: 2,
                },
                Token::Str("emoji_id"),
                Token::None,
                Token::Str("emoji_name"),
                Token::Some,
                Token::Str("name"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn forum_tag() {
        let value = ForumTag {
            emoji: DefaultReaction::EmojiId(EMOJI_ID),
            id: TAG_ID,
            moderated: false,
            name: "other".into(),
        };

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
