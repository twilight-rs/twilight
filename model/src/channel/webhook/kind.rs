use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u8)]
pub enum WebhookType {
    Incoming = 1,
    ChannelFollower = 2,
    /// Webhooks used with interactions.
    Application = 3,
}

impl Default for WebhookType {
    fn default() -> Self {
        Self::Incoming
    }
}

#[cfg(test)]
mod tests {
    use super::WebhookType;
    use serde_test::Token;

    #[test]
    fn default() {
        assert_eq!(WebhookType::Incoming, WebhookType::default());
    }

    #[test]
    fn variants() {
        serde_test::assert_tokens(&WebhookType::Incoming, &[Token::U8(1)]);
        serde_test::assert_tokens(&WebhookType::ChannelFollower, &[Token::U8(2)]);
        serde_test::assert_tokens(&WebhookType::Application, &[Token::U8(3)]);
    }
}
