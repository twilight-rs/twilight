use crate::id::{ChannelId, GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct WebhooksUpdate {
    pub channel_id: ChannelId,
    pub guild_id: GuildId,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, GuildId, WebhooksUpdate};
    use serde_test::Token;

    #[test]
    fn test_webhooks_update() {
        let value = WebhooksUpdate {
            channel_id: ChannelId::new(1).expect("non zero"),
            guild_id: GuildId::new(2).expect("non zero"),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "WebhooksUpdate",
                    len: 2,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );
    }
}
