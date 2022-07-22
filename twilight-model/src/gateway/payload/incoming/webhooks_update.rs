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
    fn webhooks_update() {
        let value = WebhooksUpdate {
            channel_id: Id::new(1),
            guild_id: Id::new(2),
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
