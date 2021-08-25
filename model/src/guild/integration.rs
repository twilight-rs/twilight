use super::{IntegrationAccount, IntegrationApplication, IntegrationExpireBehavior};
use crate::{
    id::{GuildId, IntegrationId, RoleId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildIntegration {
    pub account: IntegrationAccount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<IntegrationApplication>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_emoticons: Option<bool>,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_behavior: Option<IntegrationExpireBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_grace_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    pub id: IntegrationId,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<RoleId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syncing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::{
        GuildIntegration, IntegrationAccount, IntegrationApplication, IntegrationExpireBehavior,
        IntegrationId, User,
    };
    use crate::id::{ApplicationId, RoleId, UserId};
    use serde_test::Token;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_guild_integration() {
        let value = GuildIntegration {
            account: IntegrationAccount {
                id: "abcd".to_owned(),
                name: "account name".to_owned(),
            },
            application: None,
            enable_emoticons: Some(true),
            enabled: true,
            expire_behavior: Some(IntegrationExpireBehavior::Kick),
            expire_grace_period: Some(3_600),
            guild_id: None,
            id: IntegrationId::new(2).expect("non zero"),
            kind: "a".to_owned(),
            name: "integration name".to_owned(),
            revoked: Some(false),
            role_id: Some(RoleId::new(3).expect("non zero")),
            subscriber_count: Some(1337),
            synced_at: Some("timestamp".to_owned()),
            syncing: Some(false),
            user: Some(User {
                accent_color: None,
                avatar: Some("hash".to_owned()),
                banner: None,
                bot: true,
                discriminator: 1000,
                email: None,
                flags: None,
                id: UserId::new(4).expect("non zero"),
                locale: None,
                mfa_enabled: None,
                name: "user".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            }),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildIntegration",
                    len: 14,
                },
                Token::Str("account"),
                Token::Struct {
                    name: "IntegrationAccount",
                    len: 2,
                },
                Token::Str("id"),
                Token::Str("abcd"),
                Token::Str("name"),
                Token::Str("account name"),
                Token::StructEnd,
                Token::Str("enable_emoticons"),
                Token::Some,
                Token::Bool(true),
                Token::Str("enabled"),
                Token::Bool(true),
                Token::Str("expire_behavior"),
                Token::Some,
                Token::U8(1),
                Token::Str("expire_grace_period"),
                Token::Some,
                Token::U64(3_600),
                Token::Str("id"),
                Token::NewtypeStruct {
                    name: "IntegrationId",
                },
                Token::Str("2"),
                Token::Str("type"),
                Token::Str("a"),
                Token::Str("name"),
                Token::Str("integration name"),
                Token::Str("revoked"),
                Token::Some,
                Token::Bool(false),
                Token::Str("role_id"),
                Token::Some,
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("3"),
                Token::Str("subscriber_count"),
                Token::Some,
                Token::U64(1337),
                Token::Str("synced_at"),
                Token::Some,
                Token::Str("timestamp"),
                Token::Str("syncing"),
                Token::Some,
                Token::Bool(false),
                Token::Str("user"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str("hash"),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(true),
                Token::Str("discriminator"),
                Token::Str("1000"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("4"),
                Token::Str("username"),
                Token::Str("user"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_guild_integration_complete() {
        let value = GuildIntegration {
            account: IntegrationAccount {
                id: "abcd".to_owned(),
                name: "account name".to_owned(),
            },
            application: Some(IntegrationApplication {
                bot: None,
                description: "Friendship is Magic".to_string(),
                icon: None,
                id: ApplicationId::new(123).expect("non zero"),
                name: "Twilight".to_string(),
                summary: "A cool pony".to_string(),
            }),
            enable_emoticons: Some(true),
            enabled: true,
            expire_behavior: Some(IntegrationExpireBehavior::Kick),
            expire_grace_period: Some(3_600),
            guild_id: None,
            id: IntegrationId::new(2).expect("non zero"),
            kind: "a".to_owned(),
            name: "integration name".to_owned(),
            revoked: Some(false),
            role_id: Some(RoleId::new(3).expect("non zero")),
            subscriber_count: Some(1337),
            synced_at: Some("timestamp".to_owned()),
            syncing: Some(false),
            user: Some(User {
                accent_color: None,
                avatar: Some("hash".to_owned()),
                banner: None,
                bot: true,
                discriminator: 1000,
                email: None,
                flags: None,
                id: UserId::new(4).expect("non zero"),
                locale: None,
                mfa_enabled: None,
                name: "user".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            }),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildIntegration",
                    len: 15,
                },
                Token::Str("account"),
                Token::Struct {
                    name: "IntegrationAccount",
                    len: 2,
                },
                Token::Str("id"),
                Token::Str("abcd"),
                Token::Str("name"),
                Token::Str("account name"),
                Token::StructEnd,
                Token::Str("application"),
                Token::Some,
                Token::Struct {
                    name: "IntegrationApplication",
                    len: 5,
                },
                Token::Str("description"),
                Token::Str("Friendship is Magic"),
                Token::Str("icon"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::Str("123"),
                Token::Str("name"),
                Token::Str("Twilight"),
                Token::Str("summary"),
                Token::Str("A cool pony"),
                Token::StructEnd,
                Token::Str("enable_emoticons"),
                Token::Some,
                Token::Bool(true),
                Token::Str("enabled"),
                Token::Bool(true),
                Token::Str("expire_behavior"),
                Token::Some,
                Token::U8(1),
                Token::Str("expire_grace_period"),
                Token::Some,
                Token::U64(3_600),
                Token::Str("id"),
                Token::NewtypeStruct {
                    name: "IntegrationId",
                },
                Token::Str("2"),
                Token::Str("type"),
                Token::Str("a"),
                Token::Str("name"),
                Token::Str("integration name"),
                Token::Str("revoked"),
                Token::Some,
                Token::Bool(false),
                Token::Str("role_id"),
                Token::Some,
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("3"),
                Token::Str("subscriber_count"),
                Token::Some,
                Token::U64(1337),
                Token::Str("synced_at"),
                Token::Some,
                Token::Str("timestamp"),
                Token::Str("syncing"),
                Token::Some,
                Token::Bool(false),
                Token::Str("user"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str("hash"),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(true),
                Token::Str("discriminator"),
                Token::Str("1000"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("4"),
                Token::Str("username"),
                Token::Str("user"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
