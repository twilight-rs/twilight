use crate::gateway::OpCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Heartbeat {
    pub d: u64,
    pub op: OpCode,
}

impl Heartbeat {
    pub fn new(seq: u64) -> Self {
        Self {
            d: seq,
            op: OpCode::Heartbeat,
        }
    }
}
