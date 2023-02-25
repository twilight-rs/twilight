use crate::id::{marker::ChannelMarker, Id};
use serde::{Deserialize, Serialize};

/// An action which will execute whenever a rule is triggered.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationAction {
    /// Type of action.
    #[serde(rename = "type")]
    pub kind: AutoModerationActionType,
    /// Additional metadata needed during execution for this specific action
    /// type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AutoModerationActionMetadata>,
}

/// Additional metadata needed during execution for a specific
/// [`AutoModerationActionType`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationActionMetadata {
    /// Channel to which user content should be logged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    /// Additional explanation that will be shown to members whenever their message is blocked.
    ///
    /// Maximum value length is 150 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    /// Timeout duration in seconds.
    ///
    /// Maximum value is 2419200 seconds, or 4 weeks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u32>,
}

/// Type of [`AutoModerationAction`].
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum AutoModerationActionType {
    /// Blocks the content of a message according to the rule.
    BlockMessage,
    /// Logs user content to a specified channel.
    SendAlertMessage,
    /// Timeout user for a specified duration.
    ///
    /// A `Timeout` action can only be setup for [`Keyword`] rules.
    /// [`Permissions::MODERATE_MEMBERS`] is required to use the `Timeout` action
    /// type.
    ///
    /// [`Keyword`]: super::AutoModerationTriggerType::Keyword
    /// [`Permissions::MODERATE_MEMBERS`]: crate::guild::Permissions::MODERATE_MEMBERS
    Timeout,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for AutoModerationActionType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::BlockMessage,
            2 => Self::SendAlertMessage,
            3 => Self::Timeout,
            _ => Self::Unknown(value),
        }
    }
}

impl From<AutoModerationActionType> for u8 {
    fn from(value: AutoModerationActionType) -> Self {
        match value {
            AutoModerationActionType::BlockMessage => 1,
            AutoModerationActionType::SendAlertMessage => 2,
            AutoModerationActionType::Timeout => 3,
            AutoModerationActionType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AutoModerationAction, AutoModerationActionMetadata, AutoModerationActionType};
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(AutoModerationAction: kind, metadata);
    assert_fields!(AutoModerationActionMetadata: channel_id, duration_seconds);
    assert_impl_all!(
        AutoModerationAction: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );
    assert_impl_all!(
        AutoModerationActionMetadata: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );
    assert_impl_all!(
        AutoModerationActionType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );

    #[test]
    fn values() {
        assert_eq!(1, u8::from(AutoModerationActionType::BlockMessage));
        assert_eq!(2, u8::from(AutoModerationActionType::SendAlertMessage));
        assert_eq!(3, u8::from(AutoModerationActionType::Timeout));
        assert_eq!(250, u8::from(AutoModerationActionType::Unknown(250)));
    }
}
