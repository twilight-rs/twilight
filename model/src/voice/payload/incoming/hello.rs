use crate::voice::OpCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Hello {
    pub d: HelloInfo,
    pub op: OpCode,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct HelloInfo {
    /// Interval in milliseconds heartbeats must be sent at to maintain the
    /// websocket connection.
    pub heartbeat_interval: u32,
}
