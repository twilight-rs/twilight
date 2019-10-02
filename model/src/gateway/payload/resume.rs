use crate::gateway::OpCode;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
