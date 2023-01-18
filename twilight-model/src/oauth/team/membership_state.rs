use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TeamMembershipState(u8);

impl TeamMembershipState {
    pub const INVITED: Self = Self::new(1);
    pub const ACCEPTED: Self = Self::new(2);

    /// Create a new membership state from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`ACCEPTED`][`Self::ACCEPTED`].
    pub const fn new(membership_state: u8) -> Self {
        Self(membership_state)
    }

    /// Retrieve the value of the membership state.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::oauth::team::TeamMembershipState;
    ///
    /// assert_eq!(1, TeamMembershipState::INVITED.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::ACCEPTED => "ACCEPTED",
            Self::INVITED => "INVITED",
            _ => return None,
        })
    }
}

impl Debug for TeamMembershipState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("TeamMembershipState")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("TeamMembershipState").field(&self.0).finish()
        }
    }
}

impl From<u8> for TeamMembershipState {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<TeamMembershipState> for u8 {
    fn from(value: TeamMembershipState) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::TeamMembershipState;
    use serde_test::Token;

    const MAP: &[(TeamMembershipState, u8)] = &[
        (TeamMembershipState::INVITED, 1),
        (TeamMembershipState::ACCEPTED, 2),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "TeamMembershipState",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, TeamMembershipState::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
