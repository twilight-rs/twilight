use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MfaLevel(u8);

impl MfaLevel {
    pub const NONE: Self = Self::new(0);
    pub const ELEVATED: Self = Self::new(1);

    /// Create a new MFA Level from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`ELEVATED`][`Self::ELEVATED`].
    pub const fn new(mfa_level: u8) -> Self {
        Self(mfa_level)
    }

    /// Retrieve the value of the MFA Level.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::MfaLevel;
    ///
    /// assert_eq!(1, MfaLevel::ELEVATED.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for MfaLevel {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<MfaLevel> for u8 {
    fn from(value: MfaLevel) -> Self {
        value.get()
    }
}

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
