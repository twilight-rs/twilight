use crate::{
    id::{GuildId, RoleId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MemberUpdate {
    pub guild_id: GuildId,
    pub joined_at: String,
    pub nick: Option<String>,
    pub premium_since: Option<String>,
    pub roles: Vec<RoleId>,
    pub user: User,
}

#[cfg(test)]
mod tests {
    use super::MemberUpdate;
    use crate::user::User;
    use serde_test::Token;

    #[test]
    fn test_member_update() {
        let value = MemberUpdate {
            user: User {
                name: "Twilight Sparkle".to_string(),
                public_flags: None,
                id: 424_242.into(),
                discriminator: 1_234.to_string(),
                avatar: Some("cool image".to_string()),
                bot: false,
                email: None,
                flags: None,
                locale: None,
                mfa_enabled: None,
                premium_type: None,
                system: None,
                verified: None,
            },
            roles: vec![],
            premium_since: None,
            nick: Some("Twilight".to_string()),
            joined_at: "2017-02-27T22:21:50.121000+00:00".to_string(),
            guild_id: 1_234.into(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MemberUpdate",
                    len: 6,
                },
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1234"),
                Token::Str("joined_at"),
                Token::Str("2017-02-27T22:21:50.121000+00:00"),
                Token::Str("nick"),
                Token::Some,
                Token::Str("Twilight"),
                Token::Str("premium_since"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 5,
                },
                Token::Str("avatar"),
                Token::Some,
                Token::Str("cool image"),
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("1234"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("424242"),
                Token::Str("username"),
                Token::Str("Twilight Sparkle"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        )
    }
}
