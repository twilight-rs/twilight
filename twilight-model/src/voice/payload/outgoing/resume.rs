use crate::{
    id::{marker::GuildMarker, Id},
    voice::OpCode,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Resume {
    pub d: ResumeInfo,
    pub op: OpCode,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ResumeInfo {
    pub server_id: Id<GuildMarker>,
    pub session_id: String,
    pub token: String,
}
