use serde::{Deserialize, Serialize};

/// Behavior to perform when the user's integration expires.
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntegrationExpireBehavior(u8);

impl IntegrationExpireBehavior {
    /// Remove the role when the integration expires.
    pub const REMOVE_ROLE: Self = Self::new(0);

    /// Kick the user when the integration expires.
    pub const KICK: Self = Self::new(1);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::KICK => "KICK",
            Self::REMOVE_ROLE => "REMOVE_ROLE",
            _ => return None,
        })
    }
}

impl_typed!(IntegrationExpireBehavior, u8);

#[cfg(test)]
mod tests {
    use super::IntegrationExpireBehavior;
    use serde_test::Token;

    const MAP: &[(IntegrationExpireBehavior, u8)] = &[
        (IntegrationExpireBehavior::REMOVE_ROLE, 0),
        (IntegrationExpireBehavior::KICK, 1),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "IntegrationExpireBehavior",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, IntegrationExpireBehavior::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
