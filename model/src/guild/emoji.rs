use crate::{
    id::{EmojiId, RoleId},
    user::User,
};
use serde::{
    de::{DeserializeSeed, Deserializer, SeqAccess, Visitor},
    Deserialize, Serialize,
};
use serde_mappable_seq::Key;
use std::{
    collections::HashMap,
    fmt::{Formatter, Result as FmtResult},
};

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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<RoleId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl Key<'_, EmojiId> for Emoji {
    fn key(&self) -> EmojiId {
        self.id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmojiMapDeserializer;

struct EmojiMapVisitor;

impl<'de> Visitor<'de> for EmojiMapVisitor {
    type Value = HashMap<EmojiId, Emoji>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a sequence of emojis")
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let mut map = seq
            .size_hint()
            .map_or_else(HashMap::new, HashMap::with_capacity);

        let span = tracing::trace_span!("adding elements to emoji map");
        let _span_enter = span.enter();

        while let Some(emoji) = seq.next_element::<Emoji>()? {
            tracing::trace!(%emoji.id, ?emoji);

            map.insert(emoji.id, emoji);
        }

        Ok(map)
    }
}

impl<'de> DeserializeSeed<'de> for EmojiMapDeserializer {
    type Value = HashMap<EmojiId, Emoji>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_seq(EmojiMapVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::{Emoji, EmojiId, RoleId, User};
    use crate::id::UserId;
    use serde_test::Token;

    #[test]
    fn test_emoji() {
        let emoji = Emoji {
            animated: false,
            available: true,
            id: EmojiId(100_000_000_000_000_000),
            managed: false,
            name: "test".to_owned(),
            require_colons: true,
            roles: Vec::new(),
            user: Some(User {
                avatar: None,
                bot: false,
                discriminator: "0001".to_owned(),
                email: None,
                flags: None,
                id: UserId(1),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            }),
        };

        serde_test::assert_tokens(
            &emoji,
            &[
                Token::Struct {
                    name: "Emoji",
                    len: 7,
                },
                Token::Str("animated"),
                Token::Bool(false),
                Token::Str("available"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "EmojiId" },
                Token::Str("100000000000000000"),
                Token::Str("managed"),
                Token::Bool(false),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("require_colons"),
                Token::Bool(true),
                Token::Str("user"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 5,
                },
                Token::Str("avatar"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("1"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        )
    }

    #[test]
    fn test_emoji_complete() {
        let emoji = Emoji {
            animated: false,
            available: true,
            id: EmojiId(100_000_000_000_000_000),
            managed: false,
            name: "test".to_owned(),
            require_colons: true,
            roles: vec![RoleId(1)],
            user: Some(User {
                avatar: None,
                bot: false,
                discriminator: "0001".to_owned(),
                email: None,
                flags: None,
                id: UserId(1),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            }),
        };

        serde_test::assert_tokens(
            &emoji,
            &[
                Token::Struct {
                    name: "Emoji",
                    len: 8,
                },
                Token::Str("animated"),
                Token::Bool(false),
                Token::Str("available"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "EmojiId" },
                Token::Str("100000000000000000"),
                Token::Str("managed"),
                Token::Bool(false),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("require_colons"),
                Token::Bool(true),
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("1"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 5,
                },
                Token::Str("avatar"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("1"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        )
    }
}
