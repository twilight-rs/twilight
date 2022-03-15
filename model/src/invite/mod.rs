mod channel;
mod guild;
mod target_type;
mod welcome_screen;

pub use self::{
    channel::InviteChannel,
    guild::InviteGuild,
    target_type::TargetType,
    welcome_screen::{WelcomeScreen, WelcomeScreenChannel},
};

use super::user::User;
use crate::util::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Invite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u64>,
    pub channel: Option<InviteChannel>,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild: Option<InviteGuild>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<TargetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::{
        welcome_screen::WelcomeScreenChannel, Invite, InviteChannel, InviteGuild, TargetType, User,
        WelcomeScreen,
    };
    use crate::{
        channel::ChannelType,
        guild::VerificationLevel,
        id::Id,
        test::image_hash,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, str::FromStr};

    assert_fields!(
        Invite: approximate_member_count,
        approximate_presence_count,
        channel,
        code,
        created_at,
        expires_at,
        guild,
        inviter,
        max_age,
        max_uses,
        target_type,
        target_user,
        temporary,
        uses
    );

    assert_impl_all!(
        Invite: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize,
        Send,
        Sync,
    );

    #[test]
    fn test_invite() {
        let value = Invite {
            approximate_member_count: Some(31),
            approximate_presence_count: Some(7),
            channel: Some(InviteChannel {
                id: Id::new(2),
                kind: ChannelType::Group,
                name: None,
            }),
            code: "uniquecode".to_owned(),
            created_at: None,
            expires_at: None,
            guild: None,
            inviter: None,
            max_age: None,
            max_uses: None,
            target_type: Some(TargetType::Stream),
            target_user: None,
            temporary: None,
            uses: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Invite",
                    len: 5,
                },
                Token::Str("approximate_member_count"),
                Token::Some,
                Token::U64(31),
                Token::Str("approximate_presence_count"),
                Token::Some,
                Token::U64(7),
                Token::Str("channel"),
                Token::Some,
                Token::Struct {
                    name: "InviteChannel",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(3),
                Token::StructEnd,
                Token::Str("code"),
                Token::Str("uniquecode"),
                Token::Str("target_type"),
                Token::Some,
                Token::U8(1),
                Token::StructEnd,
            ],
        );
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_invite_complete() -> Result<(), TimestampParseError> {
        let created_at = Timestamp::from_str("2021-08-03T16:08:36.325000+00:00")?;
        let expires_at = Timestamp::from_str("2021-08-10T16:08:36.325000+00:00")?;

        let value = Invite {
            approximate_member_count: Some(31),
            approximate_presence_count: Some(7),
            channel: Some(InviteChannel {
                id: Id::new(2),
                kind: ChannelType::Group,
                name: None,
            }),
            code: "uniquecode".to_owned(),
            created_at: Some(created_at),
            expires_at: Some(expires_at),
            guild: Some(InviteGuild {
                banner: Some(image_hash::BANNER),
                description: Some("a description".to_owned()),
                features: vec!["a feature".to_owned()],
                icon: Some(image_hash::ICON),
                id: Id::new(1),
                name: "guild name".to_owned(),
                splash: Some(image_hash::SPLASH),
                vanity_url_code: Some("twilight".to_owned()),
                verification_level: VerificationLevel::Medium,
                welcome_screen: Some(WelcomeScreen {
                    description: Some("welcome description".to_owned()),
                    welcome_channels: vec![
                        WelcomeScreenChannel {
                            channel_id: Id::new(123),
                            description: "channel description".to_owned(),
                            emoji_id: None,
                            emoji_name: Some("\u{1f352}".to_owned()),
                        },
                        WelcomeScreenChannel {
                            channel_id: Id::new(456),
                            description: "custom description".to_owned(),
                            emoji_id: Some(Id::new(789)),
                            emoji_name: Some("custom_name".to_owned()),
                        },
                    ],
                }),
            }),
            inviter: Some(User {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: Id::new(2),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            }),
            max_age: Some(86_400),
            max_uses: Some(10),
            target_type: Some(TargetType::Stream),
            target_user: Some(User {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: Id::new(2),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            }),
            temporary: Some(false),
            uses: Some(3),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Invite",
                    len: 14,
                },
                Token::Str("approximate_member_count"),
                Token::Some,
                Token::U64(31),
                Token::Str("approximate_presence_count"),
                Token::Some,
                Token::U64(7),
                Token::Str("channel"),
                Token::Some,
                Token::Struct {
                    name: "InviteChannel",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(3),
                Token::StructEnd,
                Token::Str("code"),
                Token::Str("uniquecode"),
                Token::Str("created_at"),
                Token::Some,
                Token::Str("2021-08-03T16:08:36.325000+00:00"),
                Token::Str("expires_at"),
                Token::Some,
                Token::Str("2021-08-10T16:08:36.325000+00:00"),
                Token::Str("guild"),
                Token::Some,
                Token::Struct {
                    name: "InviteGuild",
                    len: 10,
                },
                Token::Str("banner"),
                Token::Some,
                Token::Str(image_hash::BANNER_INPUT),
                Token::Str("description"),
                Token::Some,
                Token::Str("a description"),
                Token::Str("features"),
                Token::Seq { len: Some(1) },
                Token::Str("a feature"),
                Token::SeqEnd,
                Token::Str("icon"),
                Token::Some,
                Token::Str(image_hash::ICON_INPUT),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("guild name"),
                Token::Str("splash"),
                Token::Some,
                Token::Str(image_hash::SPLASH_INPUT),
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
                Token::NewtypeStruct { name: "Id" },
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("456"),
                Token::Str("description"),
                Token::Str("custom description"),
                Token::Str("emoji_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("789"),
                Token::Str("emoji_name"),
                Token::Some,
                Token::Str("custom_name"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("inviter"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::Str("max_age"),
                Token::Some,
                Token::U64(86_400),
                Token::Str("max_uses"),
                Token::Some,
                Token::U64(10),
                Token::Str("target_type"),
                Token::Some,
                Token::U8(1),
                Token::Str("target_user"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::Str("temporary"),
                Token::Some,
                Token::Bool(false),
                Token::Str("uses"),
                Token::Some,
                Token::U64(3),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
