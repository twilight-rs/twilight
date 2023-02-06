use serde::{Deserialize, Serialize};

/// Type of premium tier for a [`User`].
///
/// [`User`]: super::User
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PremiumType(u8);

impl PremiumType {
    /// User doesn't have premium.
    pub const NONE: Self = Self::new(0);

    /// User has Nitro Classic.
    pub const NITRO_CLASSIC: Self = Self::new(1);

    /// User has the standard Nitro.
    pub const NITRO: Self = Self::new(2);

    /// User has Nitro Basic.
    pub const NITRO_BASIC: Self = Self::new(3);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::NITRO_BASIC => "NITRO_BASIC",
            Self::NITRO_CLASSIC => "NITRO_CLASSIC",
            Self::NITRO => "NITRO",
            Self::NONE => "NONE",
            _ => return None,
        })
    }
}

impl_typed!(PremiumType, u8);

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

    const MAP: &[(PremiumType, u8)] = &[
        (PremiumType::NONE, 0),
        (PremiumType::NITRO_CLASSIC, 1),
        (PremiumType::NITRO, 2),
        (PremiumType::NITRO_BASIC, 3),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "PremiumType",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, PremiumType::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
