mod flags;

pub use self::flags::ApplicationFlags;

use crate::{
    id::{ApplicationId, GuildId},
    oauth::{id::SkuId, team::Team},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CurrentApplicationInfo {
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub cover_image: Option<String>,
    pub description: String,
    pub guild_id: Option<GuildId>,
    /// Public flags of the application.
    pub flags: Option<ApplicationFlags>,
    pub icon: Option<String>,
    pub id: ApplicationId,
    pub name: String,
    pub owner: User,
    pub primary_sku_id: Option<SkuId>,
    /// URL of the application's privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,
    #[serde(default)]
    pub rpc_origins: Vec<String>,
    pub slug: Option<String>,
    pub summary: String,
    pub team: Option<Team>,
    /// URL of the application's terms of service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,
    pub verify_key: String,
}

#[cfg(test)]
mod tests {
    use super::{ApplicationFlags, CurrentApplicationInfo, GuildId, SkuId, Team, User};
    use crate::{id::ApplicationId, id::UserId, oauth::id::TeamId};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        CurrentApplicationInfo: bot_public,
        bot_require_code_grant,
        cover_image,
        description,
        guild_id,
        flags,
        icon,
        id,
        name,
        owner,
        primary_sku_id,
        privacy_policy_url,
        rpc_origins,
        slug,
        summary,
        team,
        terms_of_service_url,
        verify_key
    );

    assert_impl_all!(
        CurrentApplicationInfo: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize
    );

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_current_application_info() {
        let value = CurrentApplicationInfo {
            bot_public: true,
            bot_require_code_grant: false,
            cover_image: Some("cover image hash".to_owned()),
            description: "a pretty cool application".to_owned(),
            guild_id: Some(GuildId::new(1).expect("non zero")),
            flags: Some(ApplicationFlags::EMBEDDED),
            icon: Some("icon hash".to_owned()),
            id: ApplicationId::new(2).expect("non zero"),
            name: "cool application".to_owned(),
            owner: User {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: UserId::new(3).expect("non zero"),
                locale: None,
                mfa_enabled: None,
                name: "app dev".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
            primary_sku_id: Some(SkuId(4)),
            privacy_policy_url: Some("https://privacypolicy".into()),
            rpc_origins: vec!["one".to_owned()],
            slug: Some("app slug".to_owned()),
            summary: "a summary".to_owned(),
            team: Some(Team {
                icon: None,
                id: TeamId::new(5).expect("non zero"),
                members: Vec::new(),
                name: "team name".into(),
                owner_user_id: UserId::new(6).expect("non zero"),
            }),
            terms_of_service_url: Some("https://termsofservice".into()),
            verify_key: "key".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CurrentApplicationInfo",
                    len: 18,
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
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("flags"),
                Token::Some,
                Token::U64(131_072),
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
                Token::Str("privacy_policy_url"),
                Token::Some,
                Token::Str("https://privacypolicy"),
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
                    len: 5,
                },
                Token::Str("icon"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "TeamId" },
                Token::Str("5"),
                Token::Str("members"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("name"),
                Token::Str("team name"),
                Token::Str("owner_user_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("6"),
                Token::StructEnd,
                Token::Str("terms_of_service_url"),
                Token::Some,
                Token::Str("https://termsofservice"),
                Token::Str("verify_key"),
                Token::Str("key"),
                Token::StructEnd,
            ],
        );
    }
}
