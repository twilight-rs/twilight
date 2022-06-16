use super::{AutoModerationActionMetadata, AutoModerationActionType};
use serde::{Deserialize, Serialize};

/// An action which will execute whenever a rule is triggered.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationAction {
    /// Type of action.
    pub kind: AutoModerationActionType,
    /// Additional metadata needed during execution for this specific action
    /// type.
    pub metadata: AutoModerationActionMetadata,
}
