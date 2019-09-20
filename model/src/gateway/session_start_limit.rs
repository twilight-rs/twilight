use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SessionStartLimit {
    pub remaining: u64,
    pub reset_after: u64,
    pub total: u64,
}
