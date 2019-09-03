use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct SkuId(#[serde(with = "crate::id::string")] pub u64);

impl Display for SkuId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct TeamId(#[serde(with = "crate::id::string")] pub u64);

impl Display for TeamId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}
