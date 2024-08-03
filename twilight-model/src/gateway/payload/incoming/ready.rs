use crate::{
    gateway::ShardId, guild::UnavailableGuild, oauth::PartialApplication, user::CurrentUser,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ready {
    pub application: PartialApplication,
    pub guilds: Vec<UnavailableGuild>,
    pub resume_gateway_url: String,
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<ShardId>,
    pub user: CurrentUser,
    #[serde(rename = "v")]
    pub version: u64,
}

#[cfg(test)]
mod tests {
    use super::Ready;
    use crate::{
        gateway::ShardId,
        guild::UnavailableGuild,
        id::Id,
        oauth::{ApplicationFlags, PartialApplication},
        user::CurrentUser,
        util::mustbe::MustBeBool,
    };
    use serde_test::Token;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn ready() {
        let guilds = vec![
            UnavailableGuild {
                id: Id::new(1),
                unavailable: MustBeBool,
            },
            UnavailableGuild {
                id: Id::new(2),
                unavailable: MustBeBool,
            },
        ];

        let ready = Ready {
            application: PartialApplication {
                flags: ApplicationFlags::empty(),
                id: Id::new(100),
            },
            guilds,
            resume_gateway_url: "wss://gateway.discord.gg".into(),
            session_id: "foo".to_owned(),
            shard: Some(ShardId::new(4, 7)),
            user: CurrentUser {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1212,
                email: None,
                flags: None,
                id: Id::new(3),
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
                    len: 7,
                },
                Token::Str("application"),
                Token::Struct {
                    name: "PartialApplication",
                    len: 2,
                },
                Token::Str("flags"),
                Token::U64(0),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100"),
                Token::StructEnd,
                Token::Str("guilds"),
                Token::Seq { len: Some(2) },
                Token::Struct {
                    name: "UnavailableGuild",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("unavailable"),
                Token::Bool(true),
                Token::StructEnd,
                Token::Struct {
                    name: "UnavailableGuild",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("unavailable"),
                Token::Bool(true),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("resume_gateway_url"),
                Token::Str("wss://gateway.discord.gg"),
                Token::Str("session_id"),
                Token::Str("foo"),
                Token::Str("shard"),
                Token::Some,
                Token::Tuple { len: 2 },
                Token::U32(4),
                Token::U32(7),
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
                Token::NewtypeStruct { name: "Id" },
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
