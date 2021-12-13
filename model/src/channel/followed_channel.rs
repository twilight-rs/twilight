use crate::id::{marker, Id};
use serde::{Deserialize, Serialize};

/// An object indicating that following a news channel
/// was successful.
///
/// It contains the [`Id<marker::Channel>`] that is being followed
/// and the [`Id<marker::Webhook>`] that was created in the
/// target channel.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct FollowedChannel {
    pub channel_id: Id<marker::Channel>,
    pub webhook_id: Id<marker::Webhook>,
}

#[cfg(test)]
mod tests {
    use super::FollowedChannel;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn test_followed_channel() {
        let value = FollowedChannel {
            channel_id: Id::new(1).expect("non zero"),
            webhook_id: Id::new(2).expect("non zero"),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "FollowedChannel",
                    len: 2,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("webhook_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );
    }
}
