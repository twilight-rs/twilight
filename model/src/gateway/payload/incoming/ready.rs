use crate::{guild::UnavailableGuild, oauth::PartialApplication, user::CurrentUser};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ready {
    pub application: PartialApplication,
    pub guilds: Vec<UnavailableGuild>,
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<[u64; 2]>,
    pub user: CurrentUser,
    #[serde(rename = "v")]
    pub version: u64,
}

#[cfg(test)]
mod tests {
    use super::Ready;
    use crate::{
        guild::UnavailableGuild,
        id::{ApplicationId, GuildId, UserId},
        oauth::{current_application_info::ApplicationFlags, PartialApplication},
        user::CurrentUser,
    };
    use serde_test::Token;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_ready() {
        let guilds = vec![
            UnavailableGuild {
                id: GuildId::new(1).expect("non zero"),
                unavailable: true,
            },
            UnavailableGuild {
                id: GuildId::new(2).expect("non zero"),
                unavailable: true,
            },
        ];

        let ready = Ready {
            application: PartialApplication {
                flags: ApplicationFlags::empty(),
                id: ApplicationId::new(100).expect("non zero"),
            },
            guilds,
            session_id: "foo".to_owned(),
            shard: Some([4, 7]),
            user: CurrentUser {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1212,
                email: None,
                flags: None,
                id: UserId::new(3).expect("non zero"),
                locale: None,
                mfa_enabled: false,
                name: "bar".to_owned(),
                premium_type: None,
                public_flags: None,
                verified: None,
            },
            version: 8,
        };

        serde_test::assert_tokens(
            &ready,
            &[
                Token::Struct {
                    name: "Ready",
                    len: 6,
                },
                Token::Str("application"),
                Token::Struct {
                    name: "PartialApplication",
                    len: 2,
                },
                Token::Str("flags"),
                Token::U64(0),
                Token::Str("id"),
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::Str("100"),
                Token::StructEnd,
                Token::Str("guilds"),
                Token::Seq { len: Some(2) },
                Token::Struct {
                    name: "UnavailableGuild",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("unavailable"),
                Token::Bool(true),
                Token::StructEnd,
                Token::Struct {
                    name: "UnavailableGuild",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("unavailable"),
                Token::Bool(true),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("session_id"),
                Token::Str("foo"),
                Token::Str("shard"),
                Token::Some,
                Token::Tuple { len: 2 },
                Token::U64(4),
                Token::U64(7),
                Token::TupleEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "CurrentUser",
                    len: 8,
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
                Token::Str("1212"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::Str("mfa_enabled"),
                Token::Bool(false),
                Token::Str("username"),
                Token::Str("bar"),
                Token::StructEnd,
                Token::Str("v"),
                Token::U64(8),
                Token::StructEnd,
            ],
        );
    }
}
