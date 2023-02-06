use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VerificationLevel(u8);

impl VerificationLevel {
    pub const NONE: Self = Self::new(0);

    pub const LOW: Self = Self::new(1);

    pub const MEDIUM: Self = Self::new(2);

    pub const HIGH: Self = Self::new(3);

    pub const VERY_HIGH: Self = Self::new(4);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::NONE => "NONE",
            Self::LOW => "LOW",
            Self::MEDIUM => "MEDIUM",
            Self::HIGH => "HIGH",
            Self::VERY_HIGH => "VERY_HIGH",
            _ => return None,
        })
    }
}

impl_typed!(VerificationLevel, u8);

#[cfg(test)]
mod tests {
    use super::VerificationLevel;
    use serde_test::Token;

    const MAP: &[(VerificationLevel, u8)] = &[
        (VerificationLevel::NONE, 0),
        (VerificationLevel::LOW, 1),
        (VerificationLevel::MEDIUM, 2),
        (VerificationLevel::HIGH, 3),
        (VerificationLevel::VERY_HIGH, 4),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "VerificationLevel",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, VerificationLevel::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
