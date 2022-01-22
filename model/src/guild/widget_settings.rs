use crate::id::ChannelId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildWidgetSettings {
    pub channel_id: ChannelId,
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, GuildWidgetSettings};
    use serde_test::Token;

    #[test]
    fn test_guild_widget() {
        let value = GuildWidgetSettings {
            channel_id: ChannelId::new(111_111_111_111_111_111).expect("non zero"),
            enabled: true,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildWidgetSettings",
                    len: 2,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("111111111111111111"),
                Token::Str("enabled"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
