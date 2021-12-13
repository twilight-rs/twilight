use crate::{
    channel::ChannelType,
    id::{marker, Id},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChannelMention {
    pub guild_id: Id<marker::Guild>,
    pub id: Id<marker::Channel>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::{ChannelMention, ChannelType};
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn test_channel_mention() {
        let value = ChannelMention {
            guild_id: Id::new(1).expect("non zero"),
            id: Id::new(2).expect("non zero"),
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
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
