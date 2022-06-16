use crate::id::{marker::ChannelMarker, Id};
use serde::{Deserialize, Serialize};

/// Additional metadata needed during execution for a specific
/// [`AutoModerationActionType`].
///
/// [`AutoModerationActionType`]: super::AutoModerationActionType
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationActionMetadata {
    /// Channel to which user content should be logged.
    pub channel_id: Id<ChannelMarker>,
    /// Timeout duration in seconds.
    ///
    /// Maximum value is 2419200 seconds, or 4 weeks.
    pub duration_seconds: u32,
}
