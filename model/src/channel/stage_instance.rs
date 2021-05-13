use crate::id::{ChannelId, GuildId, StageId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StageInstance {
    pub channel_id: ChannelId,
    pub guild_id: GuildId,
    pub id: StageId,
    pub topic: String,
}

#[cfg(test)]
mod test {
    use super::StageInstance;
    use crate::id::{ChannelId, GuildId, StageId};
    use serde_test::Token;

    #[test]
    fn test_stage_instance() {
        let value = StageInstance {
            channel_id: ChannelId(100),
            guild_id: GuildId(200),
            id: StageId(300),
            topic: "a topic".into(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct { name: "StageInstance", len: 4 },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("100"),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("200"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "StageId" },
                Token::Str("300"),
                Token::Str("topic"),
                Token::Str("a topic"),
                Token::StructEnd,
            ]
        );
    }
}
