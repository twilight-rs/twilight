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
    #[serde(default)]
    pub roles: Vec<RoleId>,
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

        while let Some(emoji) = seq.next_element::<Emoji>()? {
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
