use serde_repr::{Deserialize_repr, Serialize_repr};

/// Behavior to perform when the user's integration expires.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u8)]
pub enum IntegrationExpireBehavior {
    /// Remove the role when the integration expires.
    RemoveRole = 0,
    /// Kick the user when the integration expires.
    Kick = 1,
}

#[cfg(test)]
mod tests {
    use super::IntegrationExpireBehavior;
    use serde_test::Token;

    #[test]
    fn integration_expire_behavior() {
        serde_test::assert_tokens(&IntegrationExpireBehavior::RemoveRole, &[Token::U8(0)]);
        serde_test::assert_tokens(&IntegrationExpireBehavior::Kick, &[Token::U8(1)]);
    }
}
