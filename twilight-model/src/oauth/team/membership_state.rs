use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub enum TeamMembershipState {
    Invited,
    Accepted,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for TeamMembershipState {
    fn from(value: u8) -> Self {
        match value {
            1 => TeamMembershipState::Invited,
            2 => TeamMembershipState::Accepted,
            unknown => TeamMembershipState::Unknown(unknown),
        }
    }
}

impl From<TeamMembershipState> for u8 {
    fn from(value: TeamMembershipState) -> Self {
        match value {
            TeamMembershipState::Invited => 1,
            TeamMembershipState::Accepted => 2,
            TeamMembershipState::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TeamMembershipState;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&TeamMembershipState::Invited, &[Token::U8(1)]);
        serde_test::assert_tokens(&TeamMembershipState::Accepted, &[Token::U8(2)]);
        serde_test::assert_tokens(&TeamMembershipState::Unknown(99), &[Token::U8(99)]);
    }
}
