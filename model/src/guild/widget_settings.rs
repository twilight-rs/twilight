use crate::id::{Id, marker::ChannelMarker};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildWidgetSettings {
    pub channel_id: Id<ChannelMarker>,
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::{Id, GuildWidgetSettings};
    use serde_test::Token;

    #[test]
    fn test_guild_widget() {
        let value = GuildWidgetSettings {
            channel_id: Id::new(111_111_111_111_111_111),
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("111111111111111111"),
                Token::Str("enabled"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
