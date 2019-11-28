#[cfg_attr(
feature = "serde-support",
derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum WebhookType {
    Incoming = 1,
    ChannelFollower = 2,
}

impl Default for WebhookType {
    fn default() -> Self { Self::Incoming }
}


#[cfg(test)]
mod tests {
    use super::WebhookType;
    use serde_test::Token;

    #[test]
    fn test_default() {
        assert_eq!(WebhookType::Incoming, WebhookType::default());
    }

    #[test]
    fn test_activity_type_incoming() {
        serde_test::assert_tokens(&WebhookType::Incoming, &[Token::U8(1)]);
    }

    #[test]
    fn test_activity_type_channel_follower() {
        serde_test::assert_tokens(&WebhookType::ChannelFollower, &[Token::U8(2)]);
    }
}
