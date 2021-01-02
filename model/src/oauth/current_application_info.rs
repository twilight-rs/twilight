use crate::{
    id::{ApplicationId, GuildId},
    oauth::{id::SkuId, team::Team},
    user::{User, UserFlags},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CurrentApplicationInfo {
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub cover_image: Option<String>,
    pub description: String,
    pub flags: Option<UserFlags>,
    pub guild_id: Option<GuildId>,
    pub icon: Option<String>,
    pub id: ApplicationId,
    pub name: String,
    pub owner: User,
    pub primary_sku_id: Option<SkuId>,
    #[serde(default)]
    pub rpc_origins: Vec<String>,
    pub slug: Option<String>,
    pub summary: String,
    pub team: Option<Team>,
    pub verify_key: String,
}

#[cfg(test)]
mod tests {
    use super::{CurrentApplicationInfo, GuildId, SkuId, Team, User};
    use crate::{
        id::{ApplicationId, UserId},
        oauth::id::TeamId,
        user::UserFlags,
    };
    use serde_test::Token;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_current_application_info() {
        let value = CurrentApplicationInfo {
            bot_public: true,
            bot_require_code_grant: false,
            cover_image: Some("cover image hash".to_owned()),
            description: "a pretty cool application".to_owned(),
            flags: Some(UserFlags::empty()),
            guild_id: Some(GuildId(1)),
            icon: Some("icon hash".to_owned()),
            id: ApplicationId(2),
            name: "cool application".to_owned(),
            owner: User {
                avatar: None,
                bot: false,
                discriminator: "0001".to_owned(),
                email: None,
                flags: None,
                id: UserId(3),
                locale: None,
                mfa_enabled: None,
                name: "app dev".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
            primary_sku_id: Some(SkuId(4)),
            rpc_origins: vec!["one".to_owned()],
            slug: Some("app slug".to_owned()),
            summary: "a summary".to_owned(),
            team: Some(Team {
                icon: None,
                id: TeamId(5),
                members: Vec::new(),
                owner_user_id: UserId(6),
            }),
            verify_key: "key".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CurrentApplicationInfo",
                    len: 16,
                },
                Token::Str("bot_public"),
                Token::Bool(true),
                Token::Str("bot_require_code_grant"),
                Token::Bool(false),
                Token::Str("cover_image"),
                Token::Some,
                Token::Str("cover image hash"),
                Token::Str("description"),
                Token::Str("a pretty cool application"),
                Token::Str("flags"),
                Token::Some,
                Token::U64(0),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("icon"),
                Token::Some,
                Token::Str("icon hash"),
                Token::Str("id"),
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::Str("2"),
                Token::Str("name"),
                Token::Str("cool application"),
                Token::Str("owner"),
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
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("app dev"),
                Token::StructEnd,
                Token::Str("primary_sku_id"),
                Token::Some,
                Token::NewtypeStruct { name: "SkuId" },
                Token::Str("4"),
                Token::Str("rpc_origins"),
                Token::Seq { len: Some(1) },
                Token::Str("one"),
                Token::SeqEnd,
                Token::Str("slug"),
                Token::Some,
                Token::Str("app slug"),
                Token::Str("summary"),
                Token::Str("a summary"),
                Token::Str("team"),
                Token::Some,
                Token::Struct {
                    name: "Team",
                    len: 4,
                },
                Token::Str("icon"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "TeamId" },
                Token::Str("5"),
                Token::Str("members"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("owner_user_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("6"),
                Token::StructEnd,
                Token::Str("verify_key"),
                Token::Str("key"),
                Token::StructEnd,
            ],
        );
    }
}
