use crate::id::{
    marker::{EmojiMarker, TagMarker},
    Id,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DefaultReaction {
    #[serde(flatten)]
    emoji: ForumReaction,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ForumReaction {
    EmojiId(Id<EmojiMarker>),
    EmojiName(String),
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
                Token::Str("emoji_name"),
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
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
