use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum SKUType {
    Subscription,
    SubscriptionGroup,
    Unknown(u8),
}

impl From<u8> for SKUType {
    fn from(value: u8) -> Self {
        match value {
            1 => SKUType::Subscription,
            2 => SKUType::SubscriptionGroup,
            other => SKUType::Unknown(other),
        }
    }
}

impl From<SKUType> for u8 {
    fn from(value: SKUType) -> Self {
        match value {
            SKUType::Subscription => 1,
            SKUType::SubscriptionGroup => 2,
            SKUType::Unknown(other) => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SKUType;
    use serde_test::Token;
    #[test]
    fn sku_type() {
        serde_test::assert_tokens(&SKUType::Subscription, &[Token::U8(1)]);
        serde_test::assert_tokens(&SKUType::SubscriptionGroup, &[Token::U8(2)]);
        serde_test::assert_tokens(&SKUType::Unknown(3), &[Token::U8(3)]);
    }
}
