use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct WebhookType(u8);

impl WebhookType {
    pub const INCOMING: Self = Self::new(1);

    pub const CHANNEL_FOLLOWER: Self = Self::new(2);

    /// Webhooks used with interactions.
    pub const APPLICATION: Self = Self::new(3);

    /// Create a new webhook type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`INCOMING`][`Self::INCOMING`].
    pub const fn new(webhook_type: u8) -> Self {
        Self(webhook_type)
    }

    /// Retrieve the value of the webhook type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::WebhookType;
    ///
    /// assert_eq!(2, WebhookType::CHANNEL_FOLLOWER.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

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

impl Debug for WebhookType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("WebhookType")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("WebhookType").field(&self.0).finish()
        }
    }
}

impl Default for WebhookType {
    fn default() -> Self {
        Self::INCOMING
    }
}

impl From<u8> for WebhookType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<WebhookType> for u8 {
    fn from(value: WebhookType) -> Self {
        value.get()
    }
}

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
