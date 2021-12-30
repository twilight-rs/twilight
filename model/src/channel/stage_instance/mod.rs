mod privacy_level;

pub use self::privacy_level::PrivacyLevel;

use crate::id::{
    marker::{ChannelMarker, GuildMarker, StageMarker},
    Id,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct StageInstance {
    pub channel_id: Id<ChannelMarker>,
    pub discoverable_disabled: bool,
    pub guild_id: Id<GuildMarker>,
    pub id: Id<StageMarker>,
    pub privacy_level: PrivacyLevel,
    pub topic: String,
}

#[cfg(test)]
mod test {
    use super::{PrivacyLevel, StageInstance};
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn test_stage_instance() {
        let value = StageInstance {
            channel_id: Id::new(100),
            discoverable_disabled: false,
            guild_id: Id::new(200),
            id: Id::new(300),
            privacy_level: PrivacyLevel::Public,
            topic: "a topic".into(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "StageInstance",
                    len: 6,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100"),
                Token::Str("discoverable_disabled"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("200"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("300"),
                Token::Str("privacy_level"),
                Token::U8(1),
                Token::Str("topic"),
                Token::Str("a topic"),
                Token::StructEnd,
            ],
        );
    }
}
