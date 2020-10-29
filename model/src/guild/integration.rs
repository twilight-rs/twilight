use super::{IntegrationAccount, IntegrationApplication, IntegrationExpireBehavior};
use crate::{
    id::{IntegrationId, RoleId},
    user::User,
};
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildIntegration {
    pub account: IntegrationAccount,
    pub application: Option<IntegrationApplication>,
    pub enable_emoticons: Option<bool>,
    pub enabled: bool,
    pub expire_behavior: Option<IntegrationExpireBehavior>,
    pub expire_grace_period: Option<u64>,
    pub id: IntegrationId,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    pub revoked: Option<bool>,
    pub role_id: Option<RoleId>,
    pub subscriber_count: Option<u64>,
    pub synced_at: Option<String>,
    pub syncing: Option<bool>,
    pub user: Option<User>,
}

impl Key<'_, IntegrationId> for GuildIntegration {
    fn key(&self) -> IntegrationId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GuildIntegration, IntegrationAccount, IntegrationExpireBehavior, IntegrationId, User,
    };
    use crate::id::{RoleId, UserId};
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
            id: IntegrationId(2),
            kind: "a".to_owned(),
            name: "integration name".to_owned(),
            revoked: Some(false),
            role_id: Some(RoleId(3)),
            subscriber_count: Some(1337),
            synced_at: Some("timestamp".to_owned()),
            syncing: Some(false),
            user: Some(User {
                avatar: Some("hash".to_owned()),
                bot: true,
                discriminator: "1000".to_owned(),
                email: None,
                flags: None,
                id: UserId(4),
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
                Token::None,
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
                    len: 13,
                },
                Token::Str("avatar"),
                Token::Some,
                Token::Str("hash"),
                Token::Str("bot"),
                Token::Bool(true),
                Token::Str("discriminator"),
                Token::Str("1000"),
                Token::Str("email"),
                Token::None,
                Token::Str("flags"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("4"),
                Token::Str("locale"),
                Token::None,
                Token::Str("mfa_enabled"),
                Token::None,
                Token::Str("username"),
                Token::Str("user"),
                Token::Str("premium_type"),
                Token::None,
                Token::Str("public_flags"),
                Token::None,
                Token::Str("system"),
                Token::None,
                Token::Str("verified"),
                Token::None,
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
