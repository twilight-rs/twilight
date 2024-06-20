use super::{
    GuildIntegrationType, IntegrationAccount, IntegrationApplication, IntegrationExpireBehavior,
};
use crate::{
    id::{
        marker::{GuildMarker, IntegrationMarker, RoleMarker},
        Id,
    },
    user::User,
    util::Timestamp,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildIntegration {
    pub account: IntegrationAccount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<IntegrationApplication>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_emoticons: Option<bool>,
    /// Whether the integration has been enabled.
    ///
    /// May be provided on some non-Discord application integrations.
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_behavior: Option<IntegrationExpireBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_grace_period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<IntegrationMarker>,
    #[serde(rename = "type")]
    pub kind: GuildIntegrationType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<Id<RoleMarker>>,
    /// An array of [OAuth2 scopes] which the application has been authorized for.
    ///
    /// [OAuth2 scopes]: crate::oauth::scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syncing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::{
        GuildIntegration, IntegrationAccount, IntegrationApplication, IntegrationExpireBehavior,
        User,
    };
    use crate::{
        guild::GuildIntegrationType,
        id::Id,
        oauth::scope,
        test::image_hash,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn guild_integration() -> Result<(), TimestampParseError> {
        let synced_at = Timestamp::from_str("2021-01-01T01:01:01+00:00")?;

        let value = GuildIntegration {
            account: IntegrationAccount {
                id: "abcd".to_owned(),
                name: "account name".to_owned(),
            },
            application: None,
            enable_emoticons: Some(true),
            enabled: Some(true),
            expire_behavior: Some(IntegrationExpireBehavior::Kick),
            expire_grace_period: Some(3_600),
            guild_id: None,
            id: Id::new(2),
            kind: GuildIntegrationType::Discord,
            name: "integration name".to_owned(),
            revoked: Some(false),
            role_id: Some(Id::new(3)),
            scopes: Some(Vec::from([
                scope::APPLICATIONS_COMMANDS.to_owned(),
                scope::BOT.to_owned(),
            ])),
            subscriber_count: Some(1337),
            synced_at: Some(synced_at),
            syncing: Some(false),
            user: Some(User {
                accent_color: None,
                avatar: Some(image_hash::AVATAR),
                avatar_decoration: None,
                avatar_decoration_data: None,
                banner: None,
                bot: true,
                discriminator: 1000,
                email: None,
                flags: None,
                global_name: Some("test".to_owned()),
                id: Id::new(4),
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
                Token::Str("enable_emoticons"),
                Token::Some,
                Token::Bool(true),
                Token::Str("enabled"),
                Token::Some,
                Token::Bool(true),
                Token::Str("expire_behavior"),
                Token::Some,
                Token::U8(1),
                Token::Str("expire_grace_period"),
                Token::Some,
                Token::U64(3_600),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("type"),
                Token::Str("discord"),
                Token::Str("name"),
                Token::Str("integration name"),
                Token::Str("revoked"),
                Token::Some,
                Token::Bool(false),
                Token::Str("role_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("scopes"),
                Token::Some,
                Token::Seq { len: Some(2) },
                Token::Str(scope::APPLICATIONS_COMMANDS),
                Token::Str(scope::BOT),
                Token::SeqEnd,
                Token::Str("subscriber_count"),
                Token::Some,
                Token::U64(1337),
                Token::Str("synced_at"),
                Token::Some,
                Token::Str("2021-01-01T01:01:01.000000+00:00"),
                Token::Str("syncing"),
                Token::Some,
                Token::Bool(false),
                Token::Str("user"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 10,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("avatar_decoration"),
                Token::None,
                Token::Str("avatar_decoration_data"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(true),
                Token::Str("discriminator"),
                Token::Str("1000"),
                Token::Str("global_name"),
                Token::Some,
                Token::Str("test"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("username"),
                Token::Str("user"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );

        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn guild_integration_complete() -> Result<(), TimestampParseError> {
        let synced_at = Timestamp::from_str("2021-01-01T01:01:01+00:00")?;

        let value = GuildIntegration {
            account: IntegrationAccount {
                id: "abcd".to_owned(),
                name: "account name".to_owned(),
            },
            application: Some(IntegrationApplication {
                bot: None,
                description: "Friendship is Magic".to_string(),
                icon: None,
                id: Id::new(123),
                name: "Twilight".to_string(),
            }),
            enable_emoticons: Some(true),
            enabled: None,
            expire_behavior: Some(IntegrationExpireBehavior::Kick),
            expire_grace_period: Some(3_600),
            guild_id: None,
            id: Id::new(2),
            kind: GuildIntegrationType::Discord,
            name: "integration name".to_owned(),
            revoked: Some(false),
            role_id: Some(Id::new(3)),
            scopes: Some(Vec::from([
                scope::APPLICATIONS_COMMANDS.to_owned(),
                scope::BOT.to_owned(),
            ])),
            subscriber_count: Some(1337),
            synced_at: Some(synced_at),
            syncing: Some(false),
            user: Some(User {
                accent_color: None,
                avatar: Some(image_hash::AVATAR),
                avatar_decoration: None,
                avatar_decoration_data: None,
                banner: None,
                bot: true,
                discriminator: 1000,
                email: None,
                flags: None,
                global_name: Some("test".to_owned()),
                id: Id::new(4),
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
                    len: 16,
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
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("Friendship is Magic"),
                Token::Str("icon"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123"),
                Token::Str("name"),
                Token::Str("Twilight"),
                Token::StructEnd,
                Token::Str("enable_emoticons"),
                Token::Some,
                Token::Bool(true),
                Token::Str("enabled"),
                Token::None,
                Token::Str("expire_behavior"),
                Token::Some,
                Token::U8(1),
                Token::Str("expire_grace_period"),
                Token::Some,
                Token::U64(3_600),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("type"),
                Token::Str("discord"),
                Token::Str("name"),
                Token::Str("integration name"),
                Token::Str("revoked"),
                Token::Some,
                Token::Bool(false),
                Token::Str("role_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("scopes"),
                Token::Some,
                Token::Seq { len: Some(2) },
                Token::Str(scope::APPLICATIONS_COMMANDS),
                Token::Str(scope::BOT),
                Token::SeqEnd,
                Token::Str("subscriber_count"),
                Token::Some,
                Token::U64(1337),
                Token::Str("synced_at"),
                Token::Some,
                Token::Str("2021-01-01T01:01:01.000000+00:00"),
                Token::Str("syncing"),
                Token::Some,
                Token::Bool(false),
                Token::Str("user"),
                Token::Some,
                Token::Struct {
                    name: "User",
                    len: 10,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("avatar_decoration"),
                Token::None,
                Token::Str("avatar_decoration_data"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(true),
                Token::Str("discriminator"),
                Token::Str("1000"),
                Token::Str("global_name"),
                Token::Some,
                Token::Str("test"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("username"),
                Token::Str("user"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
