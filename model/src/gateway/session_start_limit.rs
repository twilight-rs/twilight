use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SessionStartLimit {
    pub remaining: u64,
    pub reset_after: u64,
    pub total: u64,
}
