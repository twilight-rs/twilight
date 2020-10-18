use crate::id::{ChannelId, WebhookId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct FollowedChannel {
    pub channel_id: ChannelId,
    pub webhook_id: WebhookId,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, FollowedChannel, WebhookId};
    use serde_test::Token;

    #[test]
    fn test_followed_channel() {
        let value = FollowedChannel {
            channel_id: ChannelId(1),
            webhook_id: WebhookId(2),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "FollowedChannel",
                    len: 2,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("webhook_id"),
                Token::NewtypeStruct { name: "WebhookId" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );
    }
}
