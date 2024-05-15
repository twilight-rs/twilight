use crate::{
    guild::Member,
    id::{
        marker::{ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TypingStart {
    pub channel_id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    pub timestamp: u64,
    pub user_id: Id<UserMarker>,
}

#[cfg(test)]
mod tests {
    use super::TypingStart;
    use crate::{
        guild::{Member, MemberFlags},
        id::Id,
        test::image_hash,
        user::User,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn typing_start_with_member() -> Result<(), TimestampParseError> {
        let joined_at = Some(Timestamp::from_str("2020-01-01T00:00:00.000000+00:00")?);
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let value = TypingStart {
            channel_id: Id::new(2),
            guild_id: Some(Id::new(1)),
            member: Some(Member {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                flags,
                joined_at,
                mute: false,
                nick: Some("typing".to_owned()),
                pending: false,
                premium_since: None,
                roles: vec![Id::new(4)],
                user: User {
                    id: Id::new(3),
                    accent_color: None,
                    avatar: Some(image_hash::AVATAR),
                    avatar_decoration: None,
                    avatar_decoration_data: None,
                    banner: None,
                    bot: false,
                    discriminator: 1,
                    global_name: Some("test".to_owned()),
                    name: "test".to_owned(),
                    mfa_enabled: None,
                    locale: None,
                    verified: None,
                    email: None,
                    flags: None,
                    premium_type: None,
                    system: None,
                    public_flags: None,
                },
            }),
            timestamp: 1_500_000_000,
            user_id: Id::new(3),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "TypingStart",
                    len: 5,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "Member",
                    len: 9,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("flags"),
                Token::U64(flags.bits()),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str("2020-01-01T00:00:00.000000+00:00"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("typing"),
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 10,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("avatar_decoration"),
                Token::None,
                Token::Str("avatar_decoration_data"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("global_name"),
                Token::Some,
                Token::Str("test"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("timestamp"),
                Token::U64(1_500_000_000),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }

    #[test]
    fn typing_start_without_member() {
        let value = TypingStart {
            channel_id: Id::new(2),
            guild_id: None,
            member: None,
            timestamp: 1_500_000_000,
            user_id: Id::new(3),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "TypingStart",
                    len: 3,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("timestamp"),
                Token::U64(1_500_000_000),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::StructEnd,
            ],
        );
    }
}
