use crate::{
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    voice::OpCode,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Identify {
    pub d: IdentifyInfo,
    pub op: OpCode,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IdentifyInfo {
    pub server_id: Id<GuildMarker>,
    pub session_id: String,
    pub token: String,
    pub user_id: Id<UserMarker>,
}
