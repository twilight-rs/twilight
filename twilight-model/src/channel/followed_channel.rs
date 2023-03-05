use crate::id::{
    marker::{ChannelMarker, WebhookMarker},
    Id,
};
use serde::{Deserialize, Serialize};

/// An object indicating that following a news channel
/// was successful.
///
/// It contains the [`Id<ChannelMarker>`] that is being followed
/// and the [`Id<WebhookMarker>`] that was created in the
/// target channel.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct FollowedChannel {
    pub channel_id: Id<ChannelMarker>,
    pub webhook_id: Id<WebhookMarker>,
}

#[cfg(test)]
mod tests {
    use super::FollowedChannel;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn followed_channel() {
        let value = FollowedChannel {
            channel_id: Id::new(1),
            webhook_id: Id::new(2),
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
