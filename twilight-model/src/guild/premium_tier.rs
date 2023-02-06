use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PremiumTier(u8);

impl PremiumTier {
    pub const NONE: Self = Self::new(0);

    pub const TIER_1: Self = Self::new(1);

    pub const TIER_2: Self = Self::new(2);

    pub const TIER_3: Self = Self::new(3);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::NONE => "NONE",
            Self::TIER_1 => "TIER_1",
            Self::TIER_2 => "TIER_2",
            Self::TIER_3 => "TIER_3",
            _ => return None,
        })
    }
}

impl Default for PremiumTier {
    fn default() -> Self {
        Self::NONE
    }
}

impl_typed!(PremiumTier, u8);

#[cfg(test)]
mod tests {
    use super::PremiumTier;
    use serde_test::Token;

    const MAP: &[(PremiumTier, u8)] = &[
        (PremiumTier::NONE, 0),
        (PremiumTier::TIER_1, 1),
        (PremiumTier::TIER_2, 2),
        (PremiumTier::TIER_3, 3),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "PremiumTier",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, PremiumTier::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
