use crate::gateway::OpCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Heartbeat {
    pub d: u64,
    pub op: OpCode,
}

impl Heartbeat {
    pub const fn new(seq: u64) -> Self {
        Self {
            d: seq,
            op: OpCode::Heartbeat,
        }
    }
}
