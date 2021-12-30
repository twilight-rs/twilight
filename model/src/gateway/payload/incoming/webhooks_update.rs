use crate::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct WebhooksUpdate {
    pub channel_id: Id<ChannelMarker>,
    pub guild_id: Id<GuildMarker>,
}

#[cfg(test)]
mod tests {
    use super::WebhooksUpdate;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn test_webhooks_update() {
        let value = WebhooksUpdate {
            channel_id: Id::new_checked(1).expect("non zero"),
            guild_id: Id::new_checked(2).expect("non zero"),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "WebhooksUpdate",
                    len: 2,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );
    }
}
