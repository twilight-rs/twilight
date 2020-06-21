use crate::gateway::{
    opcode::OpCode,
    presence::{Activity, Status},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UpdateStatus {
    pub d: UpdateStatusInfo,
    pub op: OpCode,
}

impl UpdateStatus {
    pub fn new(
        afk: bool,
        game: impl Into<Option<Activity>>,
        since: impl Into<Option<u64>>,
        status: impl Into<Status>,
    ) -> Self {
        Self {
            d: UpdateStatusInfo::new(afk, game, since, status),
            op: OpCode::StatusUpdate,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UpdateStatusInfo {
    pub afk: bool,
    pub game: Option<Activity>,
    pub since: Option<u64>,
    pub status: Status,
}

impl UpdateStatusInfo {
    pub fn new(
        afk: bool,
        game: impl Into<Option<Activity>>,
        since: impl Into<Option<u64>>,
        status: impl Into<Status>,
    ) -> Self {
        Self::_new(afk, game.into(), since.into(), status.into())
    }

    fn _new(afk: bool, game: Option<Activity>, since: Option<u64>, status: Status) -> Self {
        Self {
            afk,
            game,
            since,
            status,
        }
    }
}
