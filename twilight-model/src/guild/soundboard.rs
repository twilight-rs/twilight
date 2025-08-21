use serde::{Deserialize, Serialize};

use crate::{
    id::{
        marker::{EmojiMarker, GuildMarker, SoundboardSoundMarker},
        Id,
    },
    user::User,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SoundboardSound {
    pub available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<Id<EmojiMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    pub name: String,
    pub sound_id: Id<SoundboardSoundMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    pub volume: f64,
}

#[cfg(test)]
mod tests {
    use crate::id::Id;

    use super::SoundboardSound;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(
        SoundboardSound: available,
        emoji_id,
        emoji_name,
        guild_id,
        name,
        sound_id,
        user,
        volume
    );

    assert_impl_all!(
        SoundboardSound: Clone, Debug, Deserialize<'static>, PartialEq, Serialize
    );

    #[test]
    fn soundboard_sound() {
        let sound = SoundboardSound {
            available: true,
            emoji_id: None,
            emoji_name: None,
            guild_id: None,
            name: "test".to_owned(),
            sound_id: Id::new(123),
            user: None,
            volume: 50.0,
        };

        serde_test::assert_tokens(
            &sound,
            &[
                Token::Struct {
                    name: "SoundboardSound",
                    len: 4,
                },
                Token::Str("available"),
                Token::Bool(true),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("sound_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123"),
                Token::Str("volume"),
                Token::F64(50.0),
                Token::StructEnd,
            ],
        );
    }
}
