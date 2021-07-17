use crate::id::ChannelId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildWidget {
    pub channel_id: ChannelId,
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, GuildWidget};
    use serde_test::Token;

    #[test]
    fn test_guild_widget() {
        let value = GuildWidget {
            channel_id: ChannelId::new(111_111_111_111_111_111).expect("non zero"),
            enabled: true,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildWidget",
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
