use crate::{
    channel::ChannelType,
    id::{ChannelId, GuildId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChannelMention {
    pub guild_id: GuildId,
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, ChannelMention, ChannelType, GuildId};
    use serde_test::Token;

    #[test]
    fn test_channel_mention() {
        let value = ChannelMention {
            guild_id: GuildId::new(1).expect("non zero"),
            id: ChannelId::new(2).expect("non zero"),
            kind: ChannelType::GuildText,
            name: "channel".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ChannelMention",
                    len: 4,
                },
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(0),
                Token::Str("name"),
                Token::Str("channel"),
                Token::StructEnd,
            ],
        );
    }
}
