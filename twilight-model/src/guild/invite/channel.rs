use crate::{
    channel::ChannelType,
    id::{marker::ChannelMarker, Id},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteChannel {
    /// ID of the channel.
    pub id: Id<ChannelMarker>,
    /// Name of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Type of the channel.
    #[serde(rename = "type")]
    pub kind: ChannelType,
}

#[cfg(test)]
mod tests {
    use super::{ChannelType, InviteChannel};
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn invite_channel() {
        let value = InviteChannel {
            id: Id::new(1),
            name: Some("channel name".to_owned()),
            kind: ChannelType::GUILD_TEXT,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InviteChannel",
                    len: 3,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Some,
                Token::Str("channel name"),
                Token::Str("type"),
                Token::NewtypeStruct {
                    name: "ChannelType",
                },
                Token::U8(0),
                Token::StructEnd,
            ],
        )
    }
}
