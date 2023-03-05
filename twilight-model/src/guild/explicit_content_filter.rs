use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub enum ExplicitContentFilter {
    None,
    MembersWithoutRole,
    AllMembers,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for ExplicitContentFilter {
    fn from(value: u8) -> Self {
        match value {
            0 => ExplicitContentFilter::None,
            1 => ExplicitContentFilter::MembersWithoutRole,
            2 => ExplicitContentFilter::AllMembers,
            unknown => ExplicitContentFilter::Unknown(unknown),
        }
    }
}

impl From<ExplicitContentFilter> for u8 {
    fn from(value: ExplicitContentFilter) -> Self {
        match value {
            ExplicitContentFilter::None => 0,
            ExplicitContentFilter::MembersWithoutRole => 1,
            ExplicitContentFilter::AllMembers => 2,
            ExplicitContentFilter::Unknown(unknown) => unknown,
        }
    }
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
        serde_test::assert_tokens(&ExplicitContentFilter::Unknown(99), &[Token::U8(99)]);
    }
}
