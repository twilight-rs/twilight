use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct WebhookType(u8);

impl WebhookType {
    pub const INCOMING: Self = Self::new(1);

    pub const CHANNEL_FOLLOWER: Self = Self::new(2);

    /// Webhooks used with interactions.
    pub const APPLICATION: Self = Self::new(3);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::APPLICATION => "APPLICATION",
            Self::CHANNEL_FOLLOWER => "CHANNEL_FOLLOWER",
            Self::INCOMING => "INCOMING",
            _ => return None,
        })
    }
}

impl Default for WebhookType {
    fn default() -> Self {
        Self::INCOMING
    }
}

impl_typed!(WebhookType, u8);

#[cfg(test)]
mod tests {
    use super::WebhookType;
    use serde_test::Token;

    const MAP: &[(WebhookType, u8)] = &[
        (WebhookType::INCOMING, 1),
        (WebhookType::CHANNEL_FOLLOWER, 2),
        (WebhookType::APPLICATION, 3),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "WebhookType",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, WebhookType::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
