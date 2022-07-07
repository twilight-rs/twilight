use crate::{
    id::{marker::UserMarker, Id},
    voice::OpCode,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ClientDisconnect {
    pub d: ClientDisconnectInfo,
    pub op: OpCode,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ClientDisconnectInfo {
    pub user_id: Id<UserMarker>,
}
