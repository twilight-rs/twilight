use serde::{Deserialize, Serialize};

/// Behavior to perform when the user's integration expires.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum IntegrationExpireBehavior {
    /// Remove the role when the integration expires.
    RemoveRole,
    /// Kick the user when the integration expires.
    Kick,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for IntegrationExpireBehavior {
    fn from(value: u8) -> Self {
        match value {
            0 => IntegrationExpireBehavior::RemoveRole,
            1 => IntegrationExpireBehavior::Kick,
            unknown => IntegrationExpireBehavior::Unknown(unknown),
        }
    }
}

impl From<IntegrationExpireBehavior> for u8 {
    fn from(value: IntegrationExpireBehavior) -> Self {
        match value {
            IntegrationExpireBehavior::RemoveRole => 0,
            IntegrationExpireBehavior::Kick => 1,
            IntegrationExpireBehavior::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IntegrationExpireBehavior;
    use serde_test::Token;

    #[test]
    fn test_integration_expire_behavior() {
        serde_test::assert_tokens(&IntegrationExpireBehavior::RemoveRole, &[Token::U8(0)]);
        serde_test::assert_tokens(&IntegrationExpireBehavior::Kick, &[Token::U8(1)]);
        serde_test::assert_tokens(&IntegrationExpireBehavior::Unknown(99), &[Token::U8(99)]);
    }
}
