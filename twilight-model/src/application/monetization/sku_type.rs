use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum SkuType {
    /// Durable one-time purchase.
    Durable,
    /// Consumable one-time purchase.
    Consumable,
    /// Represents a recurring subscription.
    Subscription,
    /// System-generated group for each [`Subscription`] SKU created.
    ///
    /// [`Subscription`]: super::Subscription
    SubscriptionGroup,
    Unknown(u8),
}

impl From<u8> for SkuType {
    fn from(value: u8) -> Self {
        match value {
            2 => Self::Durable,
            3 => Self::Consumable,
            5 => SkuType::Subscription,
            6 => SkuType::SubscriptionGroup,
            other => SkuType::Unknown(other),
        }
    }
}

impl From<SkuType> for u8 {
    fn from(value: SkuType) -> Self {
        match value {
            SkuType::Durable => 2,
            SkuType::Consumable => 3,
            SkuType::Subscription => 5,
            SkuType::SubscriptionGroup => 6,
            SkuType::Unknown(other) => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SkuType;
    use serde_test::Token;
    #[test]
    fn sku_type() {
        serde_test::assert_tokens(&SkuType::Durable, &[Token::U8(2)]);
        serde_test::assert_tokens(&SkuType::Consumable, &[Token::U8(3)]);
        serde_test::assert_tokens(&SkuType::Subscription, &[Token::U8(5)]);
        serde_test::assert_tokens(&SkuType::SubscriptionGroup, &[Token::U8(6)]);
        serde_test::assert_tokens(&SkuType::Unknown(u8::MAX), &[Token::U8(u8::MAX)]);
    }
}
