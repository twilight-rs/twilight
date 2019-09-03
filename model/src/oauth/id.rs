use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct SkuId(pub String);

impl Display for SkuId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct TeamId(pub String);

impl Display for TeamId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}
