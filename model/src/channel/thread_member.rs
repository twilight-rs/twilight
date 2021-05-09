use serde::{Deserialize, Serialize};
use crate::id::{ChannelId, UserId};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMember {
    pub id: ChannelId,
    pub user_id: UserId,
    pub join_timestamp: String,
    pub flags: u64, // Values currently unknown and undocumented
}

#[cfg(test)]
mod tests {
    use super::ThreadMember;
    use crate::id::{ChannelId, UserId};
    use serde_test::Token;

    #[test]
    fn test_thread_member() {
        let value = ThreadMember {
            id: ChannelId(1),
            user_id: UserId(2),
            join_timestamp: "123".to_string(),
            flags: 3,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ThreadMember",
                    len: 4,
                },
                Token::Str("id"),
                Token::NewtypeStruct {
                    name: "ChannelId",
                },
                Token::Str("1"),
                Token::Str("user_id"),
                Token::NewtypeStruct {
                    name: "UserId",
                },
                Token::Str("2"),
                Token::Str("join_timestamp"),
                Token::Str("123"),
                Token::Str("flags"),
                Token::U64(3),
                Token::StructEnd,
            ],
        );
    }
}