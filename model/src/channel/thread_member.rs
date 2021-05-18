use crate::id::{ChannelId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMember {
    // Values currently unknown and undocumented.
    pub flags: u64,
    pub id: ChannelId,
    pub join_timestamp: String,
    pub user_id: UserId,
}

#[cfg(test)]
mod tests {
    use super::ThreadMember;
    use crate::id::{ChannelId, UserId};
    use serde_test::Token;

    #[test]
    fn test_thread_member() {
        let value = ThreadMember {
            flags: 3,
            id: ChannelId(1),
            join_timestamp: "123".to_string(),
            user_id: UserId(2),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ThreadMember",
                    len: 4,
                },
                Token::Str("flags"),
                Token::U64(3),
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("join_timestamp"),
                Token::Str("123"),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );
    }
}
