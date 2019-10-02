use crate::gateway::OpCode;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
