use crate::{channel::ChannelType, id::ChannelId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteChannel {
    /// ID of the channel.
    pub id: ChannelId,
    /// Name of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Type of the channel.
    #[serde(rename = "type")]
    pub kind: ChannelType,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, ChannelType, InviteChannel};
    use serde_test::Token;

    #[test]
    fn test_invite_channel() {
        let value = InviteChannel {
            id: ChannelId::new(1).expect("non zero"),
            name: Some("channel name".to_owned()),
            kind: ChannelType::GuildText,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InviteChannel",
                    len: 3,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Some,
                Token::Str("channel name"),
                Token::Str("type"),
                Token::U8(0),
                Token::StructEnd,
            ],
        )
    }
}
