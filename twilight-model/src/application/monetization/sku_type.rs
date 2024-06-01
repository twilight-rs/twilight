use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum SkuType {
    Subscription,
    SubscriptionGroup,
    Unknown(u8),
}

impl From<u8> for SkuType {
    fn from(value: u8) -> Self {
        match value {
            5 => SkuType::Subscription,
            6 => SkuType::SubscriptionGroup,
            other => SkuType::Unknown(other),
        }
    }
}

impl From<SkuType> for u8 {
    fn from(value: SkuType) -> Self {
        match value {
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
        serde_test::assert_tokens(&SkuType::Subscription, &[Token::U8(5)]);
        serde_test::assert_tokens(&SkuType::SubscriptionGroup, &[Token::U8(6)]);
        serde_test::assert_tokens(&SkuType::Unknown(3), &[Token::U8(3)]);
    }
}
