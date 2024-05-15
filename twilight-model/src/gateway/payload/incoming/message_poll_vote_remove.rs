use crate::id::{
    marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
    Id,
};
use serde::{Deserialize, Serialize};

/// Sent when a user removes a vote on a poll. If the poll allows multiple selection,
/// one event will be sent per answer.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessagePollVoteRemove {
    /// ID of the answer.
    pub answer_id: u8,
    /// ID of the channel.
    pub channel_id: Id<ChannelMarker>,
    /// ID of the guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// ID of the message.
    pub message_id: Id<MessageMarker>,
    /// ID of the user.
    pub user_id: Id<UserMarker>,
}

#[cfg(test)]
mod tests {
    use super::{Id, MessagePollVoteRemove};
    use serde_test::Token;

    #[test]
    fn test_message_poll_vote_remove() {
        let value = MessagePollVoteRemove {
            answer_id: 1,
            channel_id: Id::new(2),
            guild_id: Some(Id::new(3)),
            message_id: Id::new(4),
            user_id: Id::new(5),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessagePollVoteRemove",
                    len: 5,
                },
                Token::Str("answer_id"),
                Token::U64(1),
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("message_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::StructEnd,
            ],
        );
    }
}
