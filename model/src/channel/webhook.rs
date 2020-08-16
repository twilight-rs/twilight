use crate::{
    channel::WebhookType,
    id::{ChannelId, GuildId, WebhookId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Webhook {
    pub avatar: Option<String>,
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub id: WebhookId,
    #[serde(default = "WebhookType::default", rename = "type")]
    pub kind: WebhookType,
    pub name: Option<String>,
    pub token: Option<String>,
    pub user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, GuildId, Webhook, WebhookId, WebhookType};
    use serde_test::Token;

    #[test]
    fn test_webhook() {
        let value = Webhook {
            avatar: Some("avatar".to_owned()),
            channel_id: ChannelId(1),
            guild_id: Some(GuildId(2)),
            id: WebhookId(3),
            kind: WebhookType::Incoming,
            name: Some("a webhook".to_owned()),
            token: Some("a token".to_owned()),
            user: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Webhook",
                    len: 8,
                },
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
                Token::Str("user"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }
}
