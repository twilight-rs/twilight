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
        activities: impl Into<Option<Vec<Activity>>>,
        afk: bool,
        since: impl Into<Option<u64>>,
        status: impl Into<Status>,
    ) -> Self {
        Self {
            d: UpdateStatusInfo::new(activities, afk, since, status),
            op: OpCode::StatusUpdate,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UpdateStatusInfo {
    pub activities: Option<Vec<Activity>>,
    pub afk: bool,
    pub since: Option<u64>,
    pub status: Status,
}

impl UpdateStatusInfo {
    pub fn new(
        activities: impl Into<Option<Vec<Activity>>>,
        afk: bool,
        since: impl Into<Option<u64>>,
        status: impl Into<Status>,
    ) -> Self {
        Self::_new(activities.into(), afk, since.into(), status.into())
    }

    fn _new(
        activities: Option<Vec<Activity>>,
        afk: bool,
        since: Option<u64>,
        status: Status,
    ) -> Self {
        Self {
            activities,
            afk,
            since,
            status,
        }
    }
}
