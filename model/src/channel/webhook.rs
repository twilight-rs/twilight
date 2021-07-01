use crate::{
    channel::WebhookType,
    id::{ApplicationId, ChannelId, GuildId, WebhookId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Webhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,
    pub avatar: Option<String>,
    pub channel_id: ChannelId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    pub id: WebhookId,
    #[serde(default = "WebhookType::default", rename = "type")]
    pub kind: WebhookType,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Url used for executing the webhook.
    ///
    /// Returned by the [`webhooks` OAuth2] flow.
    ///
    /// [`webhooks` OAuth2]: https://discord.com/developers/docs/topics/oauth2#webhooks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::{ApplicationId, ChannelId, GuildId, User, Webhook, WebhookId, WebhookType};
    use crate::id::UserId;
    use serde_test::Token;

    #[test]
    fn test_webhook() {
        let value = Webhook {
            application_id: Some(ApplicationId(4)),
            avatar: Some("avatar".to_owned()),
            channel_id: ChannelId(1),
            guild_id: Some(GuildId(2)),
            id: WebhookId(3),
            kind: WebhookType::Incoming,
            name: Some("a webhook".to_owned()),
            token: Some("a token".to_owned()),
            url: None,
            user: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Webhook",
                    len: 8,
                },
                Token::Str("application_id"),
                Token::Some,
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::Str("4"),
                Token::Str("avatar"),
                Token::Some,
                Token::Str("avatar"),
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "WebhookId" },
                Token::Str("3"),
                Token::Str("type"),
                Token::U8(1),
                Token::Str("name"),
                Token::Some,
                Token::Str("a webhook"),
                Token::Str("token"),
                Token::Some,
                Token::Str("a token"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_webhook_complete() {
        let value = Webhook {
            application_id: Some(ApplicationId(4)),
            avatar: Some("avatar".to_owned()),
            channel_id: ChannelId(1),
            guild_id: Some(GuildId(2)),
            id: WebhookId(3),
            kind: WebhookType::Incoming,
            name: Some("a webhook".to_owned()),
            token: Some("a token".to_owned()),
            url: Some("https://a-url".to_owned()),
            user: Some(User {
                avatar: None,
                bot: false,
                discriminator: "0001".to_owned(),
                email: None,
                flags: None,
                id: UserId(2),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
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
                    name: "Webhook",
                    len: 10,
                },
                Token::Str("application_id"),
                Token::Some,
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::Str("4"),
                Token::Str("avatar"),
                Token::Some,
                Token::Str("avatar"),
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "WebhookId" },
                Token::Str("3"),
                Token::Str("type"),
                Token::U8(1),
                Token::Str("name"),
                Token::Some,
                Token::Str("a webhook"),
                Token::Str("token"),
                Token::Some,
                Token::Str("a token"),
                Token::Str("url"),
                Token::Some,
                Token::Str("https://a-url"),
                Token::Str("user"),
                Token::Some,
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
                Token::Str("2"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
