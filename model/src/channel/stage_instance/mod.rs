mod privacy_level;

pub use self::privacy_level::PrivacyLevel;

use crate::id::{ChannelId, GuildId, StageId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct StageInstance {
    pub channel_id: ChannelId,
    pub guild_id: GuildId,
    pub id: StageId,
    pub privacy_level: PrivacyLevel,
    pub topic: String,
}

#[cfg(test)]
mod test {
    use super::{PrivacyLevel, StageInstance};
    use crate::id::{ChannelId, GuildId, StageId};
    use serde_test::Token;

    #[test]
    fn test_stage_instance() {
        let value = StageInstance {
            channel_id: ChannelId::new(100).expect("non zero"),
            guild_id: GuildId::new(200).expect("non zero"),
            id: StageId::new(300).expect("non zero"),
            privacy_level: PrivacyLevel::GuildOnly,
            topic: "a topic".into(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "StageInstance",
                    len: 5,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("100"),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("200"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "StageId" },
                Token::Str("300"),
                Token::Str("privacy_level"),
                Token::U8(2),
                Token::Str("topic"),
                Token::Str("a topic"),
                Token::StructEnd,
            ],
        );
    }
}
