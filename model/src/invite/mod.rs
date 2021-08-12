mod channel;
mod guild;
mod metadata;
mod stage_instance;
mod target_type;
mod welcome_screen;

pub use self::{
    channel::InviteChannel,
    guild::InviteGuild,
    metadata::InviteMetadata,
    stage_instance::{InviteStageInstance, InviteStageInstanceMember},
    target_type::TargetType,
    welcome_screen::{WelcomeScreen, WelcomeScreenChannel},
};

use super::user::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Invite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u64>,
    pub channel: InviteChannel,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild: Option<InviteGuild>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_instance: Option<InviteStageInstance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<TargetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::{
        welcome_screen::WelcomeScreenChannel, Invite, InviteChannel, InviteGuild,
        InviteStageInstance, InviteStageInstanceMember, TargetType, User, WelcomeScreen,
    };
    use crate::{
        channel::ChannelType,
        guild::VerificationLevel,
        id::{ChannelId, EmojiId, GuildId, UserId},
    };
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(
        Invite: approximate_member_count,
        approximate_presence_count,
        channel,
        code,
        expires_at,
        guild,
        inviter,
        stage_instance,
        target_type,
        target_user
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
            channel: InviteChannel {
                id: ChannelId(2),
                kind: ChannelType::Group,
                name: None,
            },
            code: "uniquecode".to_owned(),
            expires_at: None,
            guild: None,
            inviter: None,
            stage_instance: None,
            target_type: Some(TargetType::Stream),
            target_user: None,
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
                Token::Struct {
                    name: "InviteChannel",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
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
    fn test_invite_complete() {
        let value = Invite {
            approximate_member_count: Some(31),
            approximate_presence_count: Some(7),
            channel: InviteChannel {
                id: ChannelId(2),
                kind: ChannelType::Group,
                name: None,
            },
            code: "uniquecode".to_owned(),
            guild: Some(InviteGuild {
                banner: Some("banner hash".to_owned()),
                description: Some("a description".to_owned()),
                features: vec!["a feature".to_owned()],
                icon: Some("icon hash".to_owned()),
                id: GuildId(1),
                name: "guild name".to_owned(),
                splash: Some("splash hash".to_owned()),
                vanity_url_code: Some("twilight".to_owned()),
                verification_level: VerificationLevel::Medium,
                welcome_screen: Some(WelcomeScreen {
                    description: Some("welcome description".to_owned()),
                    welcome_channels: vec![
                        WelcomeScreenChannel {
                            channel_id: ChannelId(123),
                            description: "channel description".to_owned(),
                            emoji_id: None,
                            emoji_name: Some("\u{1f352}".to_owned()),
                        },
                        WelcomeScreenChannel {
                            channel_id: ChannelId(456),
                            description: "custom description".to_owned(),
                            emoji_id: Some(EmojiId(789)),
                            emoji_name: Some("custom_name".to_owned()),
                        },
                    ],
                }),
            }),
            expires_at: Some("expires at timestamp".to_owned()),
            inviter: Some(User {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: UserId(2),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            }),
            stage_instance: Some(InviteStageInstance {
                members: Vec::from([InviteStageInstanceMember {
                    avatar: None,
                    joined_at: "joined at".into(),
                    nick: None,
                    pending: None,
                    premium_since: None,
                    roles: Vec::new(),
                    user: User {
                        accent_color: None,
                        avatar: None,
                        banner: None,
                        bot: false,
                        discriminator: 1,
                        email: None,
                        flags: None,
                        id: UserId(2),
                        locale: None,
                        mfa_enabled: None,
                        name: "test".to_owned(),
                        premium_type: None,
                        public_flags: None,
                        system: None,
                        verified: None,
                    },
                }]),
                participant_count: 4,
                speaker_count: 2,
                topic: "who is the best pony".into(),
            }),
            target_type: Some(TargetType::Stream),
            target_user: Some(User {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: UserId(2),
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
            &value,
            &[
                Token::Struct {
                    name: "Invite",
                    len: 10,
                },
                Token::Str("approximate_member_count"),
                Token::Some,
                Token::U64(31),
                Token::Str("approximate_presence_count"),
                Token::Some,
                Token::U64(7),
                Token::Str("channel"),
                Token::Struct {
                    name: "InviteChannel",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(3),
                Token::StructEnd,
                Token::Str("code"),
                Token::Str("uniquecode"),
                Token::Str("expires_at"),
                Token::Some,
                Token::Str("expires at timestamp"),
                Token::Str("guild"),
                Token::Some,
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
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("2"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::Str("stage_instance"),
                Token::Some,
                Token::Struct {
                    name: "InviteStageInstance",
                    len: 4,
                },
                Token::Str("members"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "InviteStageInstanceMember",
                    len: 2,
                },
                Token::Str("joined_at"),
                Token::Str("joined at"),
                Token::Str("user"),
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
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("2"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("participant_count"),
                Token::U64(4),
                Token::Str("speaker_count"),
                Token::U64(2),
                Token::Str("topic"),
                Token::Str("who is the best pony"),
                Token::StructEnd,
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
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("2"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
