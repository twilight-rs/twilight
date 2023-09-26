use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum EntitlementType {
    /// Entitlement was purchased as an app subscription.
    ApplicationSubscription,
    Unknown(u8),
}

impl From<u8> for EntitlementType {
    fn from(value: u8) -> Self {
        match value {
            8 => Self::ApplicationSubscription,
            other => Self::Unknown(other),
        }
    }
}

impl From<EntitlementType> for u8 {
    fn from(value: EntitlementType) -> Self {
        match value {
            EntitlementType::ApplicationSubscription => 8,
            EntitlementType::Unknown(other) => other,
        }
    }
}

impl EntitlementType {
    pub const fn name(self) -> &'static str {
        match self {
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
        serde_test::assert_tokens(&EntitlementType::ApplicationSubscription, &[Token::U8(8)]);
        serde_test::assert_tokens(&EntitlementType::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn names() {
        assert_eq!(
            EntitlementType::ApplicationSubscription.name(),
            "ApplicationSubscription"
        );
        assert_eq!(EntitlementType::Unknown(99).name(), "Unknown");
    }
}
