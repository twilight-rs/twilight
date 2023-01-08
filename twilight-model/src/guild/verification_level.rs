use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VerificationLevel(u8);

impl VerificationLevel {
    pub const NONE: Self = Self::new(0);
    pub const LOW: Self = Self::new(1);
    pub const MEDIUM: Self = Self::new(2);
    pub const HIGH: Self = Self::new(3);
    pub const VERY_HIGH: Self = Self::new(4);

    /// Create a new verification level from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`MEDIUM`][`Self::MEDIUM`].
    pub const fn new(verification_level: u8) -> Self {
        Self(verification_level)
    }

    /// Retrieve the value of the verification level.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::VerificationLevel;
    ///
    /// assert_eq!(1, VerificationLevel::LOW.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for VerificationLevel {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<VerificationLevel> for u8 {
    fn from(value: VerificationLevel) -> Self {
        value.get()
    }
}

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
