use crate::{
    channel::ChannelType,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChannelMention {
    pub guild_id: Id<GuildMarker>,
    pub id: Id<ChannelMarker>,
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
    fn channel_mention() {
        let value = ChannelMention {
            guild_id: Id::new(1),
            id: Id::new(2),
            kind: ChannelType::GUILD_TEXT,
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
                Token::NewtypeStruct {
                    name: "ChannelType",
                },
                Token::U8(0),
                Token::Str("name"),
                Token::Str("channel"),
                Token::StructEnd,
            ],
        );
    }
}
