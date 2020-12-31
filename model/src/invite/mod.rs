mod channel;
mod guild;
mod metadata;
mod target_user_type;

pub use self::{
    channel::InviteChannel, guild::InviteGuild, metadata::InviteMetadata,
    target_user_type::TargetUserType,
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
    pub guild: Option<InviteGuild>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_type: Option<TargetUserType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::{Invite, InviteChannel, InviteGuild, TargetUserType, User};
    use crate::{
        channel::ChannelType,
        guild::VerificationLevel,
        id::{ChannelId, GuildId, UserId},
    };
    use serde_test::Token;

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
            guild: None,
            inviter: None,
            target_user_type: Some(TargetUserType::Stream),
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
                Token::Str("target_user_type"),
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
            }),
            inviter: Some(User {
                avatar: None,
                bot: false,
                discriminator: "0001".to_owned(),
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
            target_user_type: Some(TargetUserType::Stream),
            target_user: Some(User {
                avatar: None,
                bot: false,
                discriminator: "0001".to_owned(),
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
                    len: 8,
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
                Token::Str("guild"),
                Token::Some,
                Token::Struct {
                    name: "InviteGuild",
                    len: 9,
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
                Token::StructEnd,
                Token::Str("inviter"),
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
                Token::Str("2"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::Str("target_user_type"),
                Token::Some,
                Token::U8(1),
                Token::Str("target_user"),
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
                Token::Str("2"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
