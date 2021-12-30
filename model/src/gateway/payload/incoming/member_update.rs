use crate::{
    datetime::Timestamp,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MemberUpdate {
    /// Member's guild avatar.
    pub avatar: Option<String>,
    pub guild_id: Id<GuildMarker>,
    pub deaf: Option<bool>,
    pub joined_at: Timestamp,
    pub mute: Option<bool>,
    pub nick: Option<String>,
    /// Whether the user has yet to pass the guild's [Membership Screening]
    /// requirements.
    ///
    /// Note: This field is still under refactoring by Discord. For more info,
    /// check this [issue] and [pull request].
    ///
    /// [Membership Screening]: https://support.discord.com/hc/en-us/articles/1500000466882
    /// [issue]: https://github.com/discord/discord-api-docs/issues/2567
    /// [pull request]: https://github.com/discord/discord-api-docs/pull/2547
    #[serde(default)]
    pub pending: bool,
    pub premium_since: Option<Timestamp>,
    pub roles: Vec<Id<RoleMarker>>,
    pub user: User,
}

#[cfg(test)]
mod tests {
    use super::MemberUpdate;
    use crate::{datetime::Timestamp, id::Id, user::User};
    use serde_test::Token;

    #[test]
    fn test_member_update() {
        let joined_at = Timestamp::from_micros(1_488_234_110_121_000).expect("non zero");

        let value = MemberUpdate {
            avatar: None,
            guild_id: Id::new_checked(1_234),

            deaf: Some(false),
            joined_at,
            mute: Some(false),
            nick: Some("Twilight".to_string()),
            pending: false,
            premium_since: None,
            roles: vec![],
            user: User {
                accent_color: None,
                banner: None,
                name: "Twilight Sparkle".to_string(),
                public_flags: None,
                id: Id::new_checked(424_242),
                discriminator: 1234,
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
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MemberUpdate",
                    len: 10,
                },
                Token::Str("avatar"),
                Token::None,
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1234"),
                Token::Str("deaf"),
                Token::Some,
                Token::Bool(false),
                Token::Str("joined_at"),
                Token::Str("2017-02-27T22:21:50.121000+00:00"),
                Token::Str("mute"),
                Token::Some,
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("Twilight"),
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("premium_since"),
                Token::None,
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
                Token::Some,
                Token::Str("cool image"),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("1234"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("424242"),
                Token::Str("username"),
                Token::Str("Twilight Sparkle"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        )
    }
}
