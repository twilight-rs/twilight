use crate::id::{ChannelId, EmojiId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct WelcomeScreen {
    /// Guild description.
    pub description: Option<String>,
    /// Channels shown in the welcome screen.
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct WelcomeScreenChannel {
    /// ID of the channel.
    pub channel_id: ChannelId,
    /// Description of the channel.
    pub description: String,
    /// ID of the emoji if the emoji is custom.
    pub emoji_id: Option<EmojiId>,
    /// Emoji's name if it is custom, or the unicode character.
    pub emoji_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, EmojiId, WelcomeScreen, WelcomeScreenChannel};
    use serde_test::Token;

    #[test]
    fn test_welcome_screen() {
        let value = WelcomeScreen {
            description: Some("welcome description".to_owned()),
            welcome_channels: vec![
                WelcomeScreenChannel {
                    channel_id: ChannelId::new(123).expect("non zero"),
                    description: "channel description".to_owned(),
                    emoji_id: None,
                    emoji_name: Some("\u{1f352}".to_owned()),
                },
                WelcomeScreenChannel {
                    channel_id: ChannelId::new(456).expect("non zero"),
                    description: "custom description".to_owned(),
                    emoji_id: Some(EmojiId::new(789).expect("non zero")),
                    emoji_name: Some("custom_name".to_owned()),
                },
            ],
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "WelcomeScreen",
                    len: 2,
                },
                Token::Str("description"),
                Token::Some,
                Token::Str("welcome description"),
                Token::Str("welcome_channels"),
                Token::Seq { len: Some(2) },
                Token::Struct {
                    name: "WelcomeScreenChannel",
                    len: 4,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("123"),
                Token::Str("description"),
                Token::Str("channel description"),
                Token::Str("emoji_id"),
                Token::None,
                Token::Str("emoji_name"),
                Token::Some,
                Token::Str("\u{1f352}"),
                Token::StructEnd,
                Token::Struct {
                    name: "WelcomeScreenChannel",
                    len: 4,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("456"),
                Token::Str("description"),
                Token::Str("custom description"),
                Token::Str("emoji_id"),
                Token::Some,
                Token::NewtypeStruct { name: "EmojiId" },
                Token::Str("789"),
                Token::Str("emoji_name"),
                Token::Some,
                Token::Str("custom_name"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
