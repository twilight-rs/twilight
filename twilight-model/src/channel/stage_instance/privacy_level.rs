use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PrivacyLevel(u8);

impl PrivacyLevel {
    pub const GUILD_ONLY: Self = Self::new(2);

    /// Create a new privacy level from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`GUILD_ONLY`][`Self::GUILD_ONLY`].
    pub const fn new(privacy_level: u8) -> Self {
        Self(privacy_level)
    }

    /// Retrieve the value of the privacy level.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::stage_instance::PrivacyLevel;
    ///
    /// assert_eq!(2, PrivacyLevel::GUILD_ONLY.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PrivacyLevel {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<PrivacyLevel> for u8 {
    fn from(value: PrivacyLevel) -> Self {
        value.get()
    }
}

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
