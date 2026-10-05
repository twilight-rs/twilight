use serde::{Deserialize, Serialize};

/// Status of a [`Subscription`].
///
/// [`Subscription`]: super::Subscription
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum SubscriptionStatus {
    /// Subscription is active and scheduled to renew.
    Active,
    /// Subscription is inactive and not being charged.
    Inactive,
    /// Subscription is active but will not renew.
    Ending,
    Unknown(u8),
}

impl From<u8> for SubscriptionStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Active,
            1 => Self::Inactive,
            2 => Self::Ending,
            other => Self::Unknown(other),
        }
    }
}

impl From<SubscriptionStatus> for u8 {
    fn from(value: SubscriptionStatus) -> Self {
        match value {
            SubscriptionStatus::Active => 0,
            SubscriptionStatus::Inactive => 1,
            SubscriptionStatus::Ending => 2,
            SubscriptionStatus::Unknown(other) => other,
        }
    }
}

impl SubscriptionStatus {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Inactive => "Inactive",
            Self::Ending => "Ending",
            Self::Unknown(_) => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriptionStatus;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&SubscriptionStatus::Active, &[Token::U8(0)]);
        serde_test::assert_tokens(&SubscriptionStatus::Inactive, &[Token::U8(1)]);
        serde_test::assert_tokens(&SubscriptionStatus::Ending, &[Token::U8(2)]);
        serde_test::assert_tokens(&SubscriptionStatus::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn names() {
        assert_eq!(SubscriptionStatus::Active.name(), "Active");
        assert_eq!(SubscriptionStatus::Inactive.name(), "Inactive");
        assert_eq!(SubscriptionStatus::Ending.name(), "Ending");
        assert_eq!(SubscriptionStatus::Unknown(99).name(), "Unknown");
    }
}
