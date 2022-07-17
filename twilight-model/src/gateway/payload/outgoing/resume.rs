use crate::gateway::OpCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Resume {
    pub d: ResumeInfo,
    pub op: OpCode,
}

impl Resume {
    pub fn new(seq: u64, session_id: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            d: ResumeInfo::new(seq, session_id, token),
            op: OpCode::Resume,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ResumeInfo {
    pub seq: u64,
    pub session_id: String,
    pub token: String,
}

impl ResumeInfo {
    pub fn new(seq: u64, session_id: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            seq,
            session_id: session_id.into(),
            token: token.into(),
        }
    }
}
