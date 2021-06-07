use crate::gateway::{
    opcode::OpCode,
    presence::{Activity, Status},
};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Error emitted when the payload can not be created as configured.
#[derive(Debug)]
pub struct UpdateStatusError {
    kind: UpdateStatusErrorType,
}

impl UpdateStatusError {
    /// Immutable reference to the type of error that occured.
    #[must_use = "retrieving the type has no effect if let unused"]
    pub const fn kind(&self) -> &UpdateStatusErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if let unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (UpdateStatusErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }
}

impl Display for UpdateStatusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            UpdateStatusErrorType::MissingActivity => {
                f.write_str("at least one activity must be provided")
            }
        }
    }
}

impl Error for UpdateStatusError {}

/// Type of [`UpdateStatusError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum UpdateStatusErrorType {
    /// No activities provided.
    MissingActivity,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UpdateStatus {
    pub d: UpdateStatusInfo,
    pub op: OpCode,
}

impl UpdateStatus {
    /// Create a new, valid [`UpdateStatus`] payload.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`UpdateStatusErrorType::MissingActivity`] if
    /// an empty set of activites is provided.
    pub fn new(
        activities: impl Into<Vec<Activity>>,
        afk: bool,
        since: impl Into<Option<u64>>,
        status: impl Into<Status>,
    ) -> Result<Self, UpdateStatusError> {
        let d = UpdateStatusInfo::new(activities, afk, since, status)?;

        Ok(Self {
            d,
            op: OpCode::StatusUpdate,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UpdateStatusInfo {
    /// User's activities.
    ///
    /// At least one is required.
    pub activities: Vec<Activity>,
    pub afk: bool,
    pub since: Option<u64>,
    pub status: Status,
}

impl UpdateStatusInfo {
    /// Create a validated stats update info struct.
    ///
    /// # Errors
    ///
    /// Returns an [`UpdateStatusErrorType::MissingActivity`] error type if an
    /// empty set of activites is provided.
    pub fn new(
        activities: impl Into<Vec<Activity>>,
        afk: bool,
        since: impl Into<Option<u64>>,
        status: impl Into<Status>,
    ) -> Result<Self, UpdateStatusError> {
        Self::_new(activities.into(), afk, since.into(), status.into())
    }

    fn _new(
        activities: Vec<Activity>,
        afk: bool,
        since: Option<u64>,
        status: Status,
    ) -> Result<Self, UpdateStatusError> {
        if activities.is_empty() {
            return Err(UpdateStatusError {
                kind: UpdateStatusErrorType::MissingActivity,
            });
        }

        Ok(Self {
            activities,
            afk,
            since,
            status,
        })
    }
}
