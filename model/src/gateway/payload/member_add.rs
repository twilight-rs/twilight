use crate::guild::Member;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MemberAdd(pub Member);

impl Deref for MemberAdd {
    type Target = Member;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MemberAdd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Member, MemberAdd};
    use crate::{
        id::{GuildId, UserId},
        user::User,
    };
    use serde_test::Token;

    #[test]
    fn test_member_add() {
        let value = MemberAdd(Member {
            deaf: false,
            guild_id: GuildId(1),
            hoisted_role: None,
            joined_at: None,
            mute: false,
            nick: None,
            pending: true,
            premium_since: None,
            roles: vec![],
            user: User {
                id: UserId(2),
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 987,
                name: "ab".to_string(),
                mfa_enabled: None,
                locale: None,
                verified: None,
                email: None,
                flags: None,
                premium_type: None,
                system: None,
                public_flags: None,
            },
        });

        serde_test::assert_tokens(
            &value,
            &[
                Token::NewtypeStruct { name: "MemberAdd" },
                Token::Struct {
                    name: "Member",
                    len: 9,
                },
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("hoisted_role"),
                Token::None,
                Token::Str("joined_at"),
                Token::None,
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::None,
                Token::Str("pending"),
                Token::Bool(true),
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
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
                Token::Str("0987"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("2"),
                Token::Str("username"),
                Token::Str("ab"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
