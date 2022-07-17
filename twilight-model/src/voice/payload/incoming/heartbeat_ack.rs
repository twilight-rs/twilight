use crate::voice::OpCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct HeartbeatAck {
    pub d: u64,
    pub op: OpCode,
}
