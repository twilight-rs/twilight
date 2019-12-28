use crate::voice::VoiceState;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VoiceStateUpdate(VoiceState);

#[cfg(test)]
mod tests {
    use super::{VoiceState, VoiceStateUpdate};
    use crate::{
        guild::Member,
        id::{GuildId, RoleId, UserId},
        user::User,
    };
    use serde_test::Token;

    #[test]
    fn test_voice_state_update() {
        let update = VoiceStateUpdate(VoiceState {
            channel_id: None,
            deaf: false,
            guild_id: Some(GuildId(1)),
            member: Some(Member {
                deaf: false,
                guild_id: None,
                hoisted_role: Some(RoleId(4)),
                joined_at: None,
                mute: false,
                nick: None,
                premium_since: None,
                roles: vec![RoleId(4)],
                user: User {
                    id: UserId(1),
                    avatar: None,
                    bot: false,
                    discriminator: "0909".to_string(),
                    name: "foo".to_string(),
                    mfa_enabled: None,
                    locale: None,
                    verified: None,
                    email: None,
                    flags: None,
                    premium_type: None,
                    system: None,
                },
            }),
            mute: false,
            self_deaf: false,
            self_mute: false,
            self_stream: false,
            session_id: "a".to_owned(),
            suppress: false,
            token: None,
            user_id: UserId(1),
        });

        serde_test::assert_tokens(
            &update,
            &[
                Token::NewtypeStruct {
                    name: "VoiceStateUpdate",
                },
                Token::Struct {
                    name: "VoiceState",
                    len: 12,
                },
                Token::Str("channel_id"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct {
                    name: "GuildId",
                },
                Token::Str("1"),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "Member",
                    len: 9,
                },
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::None,
                Token::Str("hoisted_role"),
                Token::Some,
                Token::NewtypeStruct {
                    name: "RoleId",
                },
                Token::Str("4"),
                Token::Str("joined_at"),
                Token::None,
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::None,
                Token::Str("premium_since"),
                Token::None,
                Token::Str("roles"),
                Token::Seq {
                    len: Some(1),
                },
                Token::NewtypeStruct {
                    name: "RoleId",
                },
                Token::Str("4"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 12,
                },
                Token::Str("id"),
                Token::NewtypeStruct {
                    name: "UserId",
                },
                Token::Str("1"),
                Token::Str("avatar"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0909"),
                Token::Str("username"),
                Token::Str("foo"),
                                Token::Str("mfa_enabled"),
                Token::None,
                Token::Str("locale"),
                Token::None,
                Token::Str("verified"),
                Token::None,
                Token::Str("email"),
                Token::None,
                Token::Str("flags"),
                Token::None,
                Token::Str("premium_type"),
                Token::None,
                Token::Str("system"),
                Token::None,
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("self_deaf"),
                Token::Bool(false),
                Token::Str("self_mute"),
                Token::Bool(false),
                Token::Str("self_stream"),
                Token::Bool(false),
                Token::Str("session_id"),
                Token::Str("a"),
                Token::Str("suppress"),
                Token::Bool(false),
                Token::Str("token"),
                Token::None,
                Token::Str("user_id"),
                Token::NewtypeStruct {
                    name: "UserId",
                },
                Token::Str("1"),
                Token::StructEnd,
            ],
        );
    }
}
