use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum WebhookType {
    Incoming,
    ChannelFollower,
    /// Webhooks used with interactions.
    Application,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for WebhookType {
    fn from(value: u8) -> Self {
        match value {
            1 => WebhookType::Incoming,
            2 => WebhookType::ChannelFollower,
            3 => WebhookType::Application,
            unknown => WebhookType::Unknown(unknown),
        }
    }
}

impl From<WebhookType> for u8 {
    fn from(value: WebhookType) -> Self {
        match value {
            WebhookType::Incoming => 1,
            WebhookType::ChannelFollower => 2,
            WebhookType::Application => 3,
            WebhookType::Unknown(unknown) => unknown,
        }
    }
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
        serde_test::assert_tokens(&WebhookType::Unknown(99), &[Token::U8(99)]);
    }
}
