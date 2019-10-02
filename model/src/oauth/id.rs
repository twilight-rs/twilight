use std::fmt::{Display, Formatter, Result as FmtResult};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SkuId(#[cfg_attr(feature = "serde-support", serde(with = "crate::id::string"))] pub u64);

impl Display for SkuId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TeamId(
    #[cfg_attr(feature = "serde-support", serde(with = "crate::id::string"))] pub u64,
);

impl Display for TeamId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}
