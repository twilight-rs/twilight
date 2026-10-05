use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum EntitlementType {
    /// Entitlement was purchased by user.
    Purchase,
    /// Entitlement for Discord Nitro subscription.
    PremiumSubscription,
    /// Entitlement was gifted by developer.
    DeveloperGift,
    /// Entitlement was purchased by a dev in application test mode.
    TestModePurchase,
    /// Entitlement was granted when the SKU was free.
    FreePurchase,
    /// Entitlement was gifted by another user.
    UserGift,
    /// Entitlement was claimed by user for free as a Nitro Subscriber.
    PremiumPurchase,
    /// Entitlement was purchased as an app subscription.
    ApplicationSubscription,
    Unknown(u8),
}

impl From<u8> for EntitlementType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Purchase,
            2 => Self::PremiumSubscription,
            3 => Self::DeveloperGift,
            4 => Self::TestModePurchase,
            5 => Self::FreePurchase,
            6 => Self::UserGift,
            7 => Self::PremiumPurchase,
            8 => Self::ApplicationSubscription,
            other => Self::Unknown(other),
        }
    }
}

impl From<EntitlementType> for u8 {
    fn from(value: EntitlementType) -> Self {
        match value {
            EntitlementType::Purchase => 1,
            EntitlementType::PremiumSubscription => 2,
            EntitlementType::DeveloperGift => 3,
            EntitlementType::TestModePurchase => 4,
            EntitlementType::FreePurchase => 5,
            EntitlementType::UserGift => 6,
            EntitlementType::PremiumPurchase => 7,
            EntitlementType::ApplicationSubscription => 8,
            EntitlementType::Unknown(other) => other,
        }
    }
}

impl EntitlementType {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Purchase => "Purchase",
            Self::PremiumSubscription => "PremiumSubscription",
            Self::DeveloperGift => "DeveloperGift",
            Self::TestModePurchase => "TestModePurchase",
            Self::FreePurchase => "FreePurchase",
            Self::UserGift => "UserGift",
            Self::PremiumPurchase => "PremiumPurchase",
            Self::ApplicationSubscription => "ApplicationSubscription",
            Self::Unknown(_) => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EntitlementType;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&EntitlementType::Purchase, &[Token::U8(1)]);
        serde_test::assert_tokens(&EntitlementType::PremiumSubscription, &[Token::U8(2)]);
        serde_test::assert_tokens(&EntitlementType::DeveloperGift, &[Token::U8(3)]);
        serde_test::assert_tokens(&EntitlementType::TestModePurchase, &[Token::U8(4)]);
        serde_test::assert_tokens(&EntitlementType::FreePurchase, &[Token::U8(5)]);
        serde_test::assert_tokens(&EntitlementType::UserGift, &[Token::U8(6)]);
        serde_test::assert_tokens(&EntitlementType::PremiumPurchase, &[Token::U8(7)]);
        serde_test::assert_tokens(&EntitlementType::ApplicationSubscription, &[Token::U8(8)]);
        serde_test::assert_tokens(&EntitlementType::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn names() {
        assert_eq!(EntitlementType::Purchase.name(), "Purchase");
        assert_eq!(
            EntitlementType::PremiumSubscription.name(),
            "PremiumSubscription"
        );
        assert_eq!(EntitlementType::DeveloperGift.name(), "DeveloperGift");
        assert_eq!(EntitlementType::TestModePurchase.name(), "TestModePurchase");
        assert_eq!(EntitlementType::FreePurchase.name(), "FreePurchase");
        assert_eq!(EntitlementType::UserGift.name(), "UserGift");
        assert_eq!(EntitlementType::PremiumPurchase.name(), "PremiumPurchase");
        assert_eq!(
            EntitlementType::ApplicationSubscription.name(),
            "ApplicationSubscription"
        );
        assert_eq!(EntitlementType::Unknown(99).name(), "Unknown");
    }
}
