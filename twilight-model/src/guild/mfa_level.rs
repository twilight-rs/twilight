use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MfaLevel(u8);

impl MfaLevel {
    pub const NONE: Self = Self::new(0);

    pub const ELEVATED: Self = Self::new(1);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::ELEVATED => "ELEVATED",
            Self::NONE => "NONE",
            _ => return None,
        })
    }
}

impl_typed!(MfaLevel, u8);

#[cfg(test)]
mod tests {
    use super::MfaLevel;
    use serde_test::Token;

    const MAP: &[(MfaLevel, u8)] = &[(MfaLevel::NONE, 0), (MfaLevel::ELEVATED, 1)];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[Token::NewtypeStruct { name: "MfaLevel" }, Token::U8(*num)],
            );
            assert_eq!(*kind, MfaLevel::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
