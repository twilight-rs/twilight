use crate::{
    channel::message::ReactionType,
    guild::Member,
    id::{
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct GatewayReaction {
    pub channel_id: Id<ChannelMarker>,
    pub emoji: ReactionType,
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub guild_id: Option<Id<GuildMarker>>,
    pub member: Option<Member>,
    pub message_id: Id<MessageMarker>,
    pub user_id: Id<UserMarker>,
}

#[cfg(test)]
mod tests {
    use super::GatewayReaction;
    use crate::{
        channel::message::ReactionType,
        guild::{Member, MemberFlags},
        id::Id,
        test::image_hash,
        user::User,
        util::Timestamp,
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn reaction_with_member() {
        let joined_at = Timestamp::from_str("2020-01-01T00:00:00.000000+00:00").unwrap();
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let value = GatewayReaction {
            channel_id: Id::new(2),
            emoji: ReactionType::Unicode {
                name: "a".to_owned(),
            },
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
                roles: vec![Id::new(5)],
                user: User {
                    accent_color: None,
                    avatar: Some(image_hash::AVATAR),
                    banner: None,
                    bot: false,
                    discriminator: 1,
                    email: None,
                    flags: None,
                    id: Id::new(4),
                    locale: None,
                    mfa_enabled: None,
                    name: "test".to_owned(),
                    premium_type: None,
                    public_flags: None,
                    system: None,
                    verified: None,
                },
            }),
            message_id: Id::new(3),
            user_id: Id::new(4),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GatewayReaction",
                    len: 6,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("emoji"),
                Token::Struct {
                    name: "ReactionType",
                    len: 1,
                },
                Token::Str("name"),
                Token::Str("a"),
                Token::StructEnd,
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
                Token::Str("5"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("message_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn reaction_without_member() {
        let value = GatewayReaction {
            channel_id: Id::new(2),
            emoji: ReactionType::Unicode {
                name: "a".to_owned(),
            },
            guild_id: None,
            member: None,
            message_id: Id::new(3),
            user_id: Id::new(4),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GatewayReaction",
                    len: 6,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("emoji"),
                Token::Struct {
                    name: "ReactionType",
                    len: 1,
                },
                Token::Str("name"),
                Token::Str("a"),
                Token::StructEnd,
                Token::Str("guild_id"),
                Token::None,
                Token::Str("member"),
                Token::None,
                Token::Str("message_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::StructEnd,
            ],
        );
    }
}
