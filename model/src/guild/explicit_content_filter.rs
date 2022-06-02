use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[repr(u8)]
pub enum ExplicitContentFilter {
    None = 0,
    MembersWithoutRole = 1,
    AllMembers = 2,
}

#[cfg(test)]
mod tests {
    use super::ExplicitContentFilter;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&ExplicitContentFilter::None, &[Token::U8(0)]);
        serde_test::assert_tokens(&ExplicitContentFilter::MembersWithoutRole, &[Token::U8(1)]);
        serde_test::assert_tokens(&ExplicitContentFilter::AllMembers, &[Token::U8(2)]);
    }
}
