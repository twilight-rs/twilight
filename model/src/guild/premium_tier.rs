#[cfg_attr(
    feature = "serde-support",
    derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PremiumTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}

impl Default for PremiumTier {
    fn default() -> Self {
        PremiumTier::None
    }
}
