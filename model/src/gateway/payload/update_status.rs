use crate::gateway::{
    opcode::OpCode,
    presence::{Activity, Status},
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
            op: OpCode::Identify,
        }
    }
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
