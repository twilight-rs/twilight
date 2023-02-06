use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TeamMembershipState(u8);

impl TeamMembershipState {
    pub const INVITED: Self = Self::new(1);

    pub const ACCEPTED: Self = Self::new(2);

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

impl_typed!(TeamMembershipState, u8);

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
