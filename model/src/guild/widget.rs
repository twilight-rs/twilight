use crate::id::{marker::ChannelMarker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildWidget {
    pub channel_id: Option<Id<ChannelMarker>>,
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::GuildWidget;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn test_guild_widget() {
        let value = GuildWidget {
            channel_id: Some(Id::new(111_111_111_111_111_111)),
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
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("111111111111111111"),
                Token::Str("enabled"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
