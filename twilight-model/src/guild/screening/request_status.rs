use serde::{Deserialize, Serialize};
use std::fmt;

/// Status of a member guild application.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JoinRequestStatus {
    /// Join request approved
    Approved,
    /// Join request rejected
    Rejected,
    /// Applicant started but not yet submitted join request
    Started,
    /// Applicant submitted join request that is awaiting review
    Submitted,
}

impl std::fmt::Display for JoinRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::Approved => "APPROVED",
            Self::Rejected => "REJECTED",
            Self::Started => "STARTED",
            Self::Submitted => "SUBMITTED",
        };
        f.write_str(string)
    }
}
