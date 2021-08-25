use crate::{guild::VerificationLevel, id::GuildId, invite::WelcomeScreen};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteGuild {
    /// Hash of the banner image.
    pub banner: Option<String>,
    /// Description used for guild discovery.
    pub description: Option<String>,
    /// List of features that the guild has had enabled.
    pub features: Vec<String>,
    /// Hash of the icon image.
    pub icon: Option<String>,
    /// ID of the guild.
    pub id: GuildId,
    /// Name of the guild.
    pub name: String,
    /// Hash of the splash image.
    pub splash: Option<String>,
    /// Vanity code unique to the guild for invites.
    pub vanity_url_code: Option<String>,
    /// Account verification level required to participate.
    pub verification_level: VerificationLevel,
    /// Welcome screen for a Community guild.
    pub welcome_screen: Option<WelcomeScreen>,
}

#[cfg(test)]
mod tests {
    use super::{GuildId, InviteGuild, VerificationLevel, WelcomeScreen};
    use crate::{
        id::{ChannelId, EmojiId},
        invite::WelcomeScreenChannel,
    };
    use serde_test::Token;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_invite_guild() {
        let value = InviteGuild {
            banner: Some("banner hash".to_owned()),
            description: Some("a description".to_owned()),
            features: vec!["a feature".to_owned()],
            icon: Some("icon hash".to_owned()),
            id: GuildId::new(1).expect("non zero"),
            name: "guild name".to_owned(),
            splash: Some("splash hash".to_owned()),
            vanity_url_code: Some("twilight".to_owned()),
            verification_level: VerificationLevel::Medium,
            welcome_screen: Some(WelcomeScreen {
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
            }),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InviteGuild",
                    len: 10,
                },
                Token::Str("banner"),
                Token::Some,
                Token::Str("banner hash"),
                Token::Str("description"),
                Token::Some,
                Token::Str("a description"),
                Token::Str("features"),
                Token::Seq { len: Some(1) },
                Token::Str("a feature"),
                Token::SeqEnd,
                Token::Str("icon"),
                Token::Some,
                Token::Str("icon hash"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("guild name"),
                Token::Str("splash"),
                Token::Some,
                Token::Str("splash hash"),
                Token::Str("vanity_url_code"),
                Token::Some,
                Token::Str("twilight"),
                Token::Str("verification_level"),
                Token::U8(2),
                Token::Str("welcome_screen"),
                Token::Some,
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
                Token::StructEnd,
            ],
        )
    }
}
