use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PrivacyLevel(u8);

impl PrivacyLevel {
    pub const GUILD_ONLY: Self = Self::new(2);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::GUILD_ONLY => "GUILD_ONLY",
            _ => return None,
        })
    }
}

impl_typed!(PrivacyLevel, u8);

#[cfg(test)]
mod tests {
    use super::PrivacyLevel;
    use serde_test::Token;

    const MAP: &[(PrivacyLevel, u8)] = &[(PrivacyLevel::GUILD_ONLY, 2)];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "PrivacyLevel",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, PrivacyLevel::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
