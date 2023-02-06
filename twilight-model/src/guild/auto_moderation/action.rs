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
    /// Timeout duration in seconds.
    ///
    /// Maximum value is 2419200 seconds, or 4 weeks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u32>,
}

/// Type of [`AutoModerationAction`].
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationActionType(u8);

impl AutoModerationActionType {
    /// Blocks the content of a message according to the rule.
    pub const BLOCK_MESSAGE: Self = Self::new(1);

    /// Logs user content to a specified channel.
    pub const SEND_ALERT_MESSAGE: Self = Self::new(2);

    /// Timeout user for a specified duration.
    ///
    /// A `Timeout` action can only be setup for [`KEYWORD`] rules.
    /// [`Permissions::MODERATE_MEMBERS`] is required to use the `Timeout` action
    /// type.
    ///
    /// [`KEYWORD`]: super::AutoModerationTriggerType::KEYWORD
    /// [`Permissions::MODERATE_MEMBERS`]: crate::guild::Permissions::MODERATE_MEMBERS
    pub const TIMEOUT: Self = Self::new(3);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::BLOCK_MESSAGE => "BLOCK_MESSAGE",
            Self::SEND_ALERT_MESSAGE => "SEND_ALERT_MESSAGE",
            Self::TIMEOUT => "TIMEOUT",
            _ => return None,
        })
    }
}

impl_typed!(AutoModerationActionType, u8);

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
        assert_eq!(1, u8::from(AutoModerationActionType::BLOCK_MESSAGE));
        assert_eq!(2, u8::from(AutoModerationActionType::SEND_ALERT_MESSAGE));
        assert_eq!(3, u8::from(AutoModerationActionType::TIMEOUT));
        assert_eq!(250, u8::from(AutoModerationActionType::new(250)));
    }
}
