use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// Behavior to perform when the user's integration expires.
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntegrationExpireBehavior(u8);

impl IntegrationExpireBehavior {
    /// Remove the role when the integration expires.
    pub const REMOVE_ROLE: Self = Self::new(0);

    /// Kick the user when the integration expires.
    pub const KICK: Self = Self::new(1);

    /// Create a new integration expire behavior from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`REMOVE_ROLE`][`Self::REMOVE_ROLE`].
    pub const fn new(integration_expire_behavior: u8) -> Self {
        Self(integration_expire_behavior)
    }

    /// Retrieve the value of the integration expire behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::IntegrationExpireBehavior;
    ///
    /// assert_eq!(1, IntegrationExpireBehavior::KICK.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

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

impl Debug for IntegrationExpireBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("IntegrationExpireBehavior")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("IntegrationExpireBehavior")
                .field(&self.0)
                .finish()
        }
    }
}

impl From<u8> for IntegrationExpireBehavior {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<IntegrationExpireBehavior> for u8 {
    fn from(value: IntegrationExpireBehavior) -> Self {
        value.get()
    }
}

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
