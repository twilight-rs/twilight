use crate::{
    gateway::presence::Presence,
    guild::Member,
    id::{ChannelId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMember {
    // Values currently unknown and undocumented.
    pub flags: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ChannelId>,
    pub join_timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<Presence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, ThreadMember, UserId};
    use serde_test::Token;

    #[test]
    fn test_thread_member() {
        let value = ThreadMember {
            flags: 3,
            id: Some(ChannelId::new(1).expect("non zero")),
            member: None,
            presence: None,
            join_timestamp: "123".to_string(),
            user_id: Some(UserId::new(2).expect("non zero")),
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
                Token::Some,
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("join_timestamp"),
                Token::Str("123"),
                Token::Str("user_id"),
                Token::Some,
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );
    }
}
