use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum TeamMembershipState {
    Invited = 1,
    Accepted = 2,
}

#[cfg(test)]
mod tests {
    use super::TeamMembershipState;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&TeamMembershipState::Invited, &[Token::U8(1)]);
        serde_test::assert_tokens(&TeamMembershipState::Accepted, &[Token::U8(2)]);
    }
}
