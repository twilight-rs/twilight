use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u8)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub enum PrivacyLevel {
    GuildOnly = 2,
}

#[cfg(test)]
mod tests {
    use super::PrivacyLevel;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&PrivacyLevel::GuildOnly, &[Token::U8(2)]);
    }
}
