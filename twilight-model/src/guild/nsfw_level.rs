use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct NSFWLevel(u8);

impl NSFWLevel {
    pub const DEFAULT: Self = Self::new(0);

    pub const EXPLICIT: Self = Self::new(1);

    pub const SAFE: Self = Self::new(2);

    pub const AGE_RESTRICTED: Self = Self::new(3);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::AGE_RESTRICTED => "AGE_RESTRICTED",
            Self::DEFAULT => "DEFAULT",
            Self::EXPLICIT => "EXPLICIT",
            Self::SAFE => "SAFE",
            _ => return None,
        })
    }
}

impl_typed!(NSFWLevel, u8);

#[cfg(test)]
mod tests {
    use super::NSFWLevel;
    use serde_test::Token;

    const MAP: &[(NSFWLevel, u8)] = &[
        (NSFWLevel::DEFAULT, 0),
        (NSFWLevel::EXPLICIT, 1),
        (NSFWLevel::SAFE, 2),
        (NSFWLevel::AGE_RESTRICTED, 3),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[Token::NewtypeStruct { name: "NSFWLevel" }, Token::U8(*num)],
            );
            assert_eq!(*kind, NSFWLevel::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
