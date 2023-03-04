use serde::{Deserialize, Serialize};

/// Type of premium tier for a [`User`].
///
/// [`User`]: super::User
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub enum PremiumType {
    /// User doesn't have premium.
    None,
    /// User has Nitro Classic.
    NitroClassic,
    /// User has the standard Nitro.
    Nitro,
    /// User has Nitro Basic.
    NitroBasic,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for PremiumType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::NitroClassic,
            2 => Self::Nitro,
            3 => Self::NitroBasic,
            unknown => PremiumType::Unknown(unknown),
        }
    }
}

impl From<PremiumType> for u8 {
    fn from(value: PremiumType) -> Self {
        match value {
            PremiumType::None => 0,
            PremiumType::NitroClassic => 1,
            PremiumType::Nitro => 2,
            PremiumType::NitroBasic => 3,
            PremiumType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PremiumType;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        PremiumType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn variants() {
        serde_test::assert_tokens(&PremiumType::None, &[Token::U8(0)]);
        serde_test::assert_tokens(&PremiumType::NitroClassic, &[Token::U8(1)]);
        serde_test::assert_tokens(&PremiumType::Nitro, &[Token::U8(2)]);
        serde_test::assert_tokens(&PremiumType::NitroBasic, &[Token::U8(3)]);
        serde_test::assert_tokens(&PremiumType::Unknown(42), &[Token::U8(42)]);
    }
}
