use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PremiumTier(u8);

impl PremiumTier {
    pub const NONE: Self = Self::new(0);
    pub const TIER_1: Self = Self::new(1);
    pub const TIER_2: Self = Self::new(2);
    pub const TIER_3: Self = Self::new(3);

    /// Create a new premium tier from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`TIER_!`][`Self::TIER_1`].
    pub const fn new(premium_tier: u8) -> Self {
        Self(premium_tier)
    }

    /// Retrieve the value of the premium tier.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::PremiumTier;
    ///
    /// assert_eq!(2, PremiumTier::TIER_2.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl Default for PremiumTier {
    fn default() -> Self {
        Self::NONE
    }
}

impl From<u8> for PremiumTier {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<PremiumTier> for u8 {
    fn from(value: PremiumTier) -> Self {
        value.get()
    }
}

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
