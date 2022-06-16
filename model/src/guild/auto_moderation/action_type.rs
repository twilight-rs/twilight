use serde::{Deserialize, Serialize};

/// Type of [`AutoModerationAction`].
///
/// [`AutoModerationAction`]: super::AutoModerationAction
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum AutoModerationActionType {
    /// Blocks the content of a message according to the rule.
    BlockMessage,
    /// Blocks the content of a message according to the rule.
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
    use super::AutoModerationActionType;
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

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
