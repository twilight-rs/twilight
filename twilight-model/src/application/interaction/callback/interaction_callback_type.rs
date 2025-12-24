use std::fmt::{Display, Formatter, Result as FmtResult};

use serde_repr::Deserialize_repr;

/// Interaction callback type
/// 
/// See [Discord Docs/Interaction Callback Type].
///
/// [Discord Docs/Interaction Callback Type]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-response-object-interaction-callback-type
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[repr(u8)]
pub enum InteractionCallbackType {
    /// ACK a Ping
    Pong = 1,
    /// Respond to an interaction with a message
    ChannelMessageWithSource = 4,
    /// ACK an interaction and edit a response later, the user sees a loading state
    DeferredChannelMessageWithSource = 5,
    /// For components, ACK an interaction and edit the original message later; the user does not see a loading state
    DeferredUpdateMessage = 6,
    /// For components, edit the message the component was attached to
    UpdateMessage = 7,
    /// Respond to an autocomplete interaction with suggested choices
    ApplicationCommandAutocompleteResult = 8,
    /// Respond to an interaction with a popup modal
    Modal = 9,
    /// Deprecated; respond to an interaction with an upgrade button, only available for apps with monetization enabled
    PremiumRequired = 10,
    /// Launch the Activity associated with the app. Only available for apps with Activities enabled
    LaunchActivity = 12,
}

impl InteractionCallbackType {
    pub const fn kind(self) -> &'static str {
        match self {
            InteractionCallbackType::Pong => "Pong",
            InteractionCallbackType::ChannelMessageWithSource => "ChannelMessageWithSource",
            InteractionCallbackType::DeferredChannelMessageWithSource => {
                "DeferredChannelMessageWithSource"
            }
            InteractionCallbackType::DeferredUpdateMessage => "DeferredUpdateMessage",
            InteractionCallbackType::UpdateMessage => "UpdateMessage",
            InteractionCallbackType::ApplicationCommandAutocompleteResult => {
                "ApplicationCommandAutocompleteResult"
            }
            InteractionCallbackType::Modal => "Modal",
            InteractionCallbackType::PremiumRequired => "PremiumRequired",
            InteractionCallbackType::LaunchActivity => "LaunchActivity",
        }
    }
}

#[derive(Debug)]
pub struct UnknownInteractionCallbackTypeError {
    value: u8,
}

impl Display for UnknownInteractionCallbackTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("unknown interaction callback type: ")?;

        Display::fmt(&self.value, f)
    }
}

impl TryFrom<u8> for InteractionCallbackType {
    type Error = UnknownInteractionCallbackTypeError;

    fn try_from(i: u8) -> Result<Self, Self::Error> {
        match i {
            1 => Ok(Self::Pong),
            4 => Ok(Self::ChannelMessageWithSource),
            5 => Ok(Self::DeferredChannelMessageWithSource),
            6 => Ok(Self::DeferredUpdateMessage),
            7 => Ok(Self::UpdateMessage),
            8 => Ok(Self::ApplicationCommandAutocompleteResult),
            9 => Ok(Self::Modal),
            10 => Ok(Self::PremiumRequired),
            12 => Ok(Self::LaunchActivity),
            other => Err(UnknownInteractionCallbackTypeError { value: other }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::application::interaction::callback::interaction_callback_type::InteractionCallbackType;

    use serde::Deserialize;
    use static_assertions::{assert_impl_all, const_assert_eq};
    use std::{fmt::Debug, hash::Hash};

    use super::UnknownInteractionCallbackTypeError;

    assert_impl_all!(
        InteractionCallbackType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );
    const_assert_eq!(1, InteractionCallbackType::Pong as u8);
    const_assert_eq!(4, InteractionCallbackType::ChannelMessageWithSource as u8);
    const_assert_eq!(
        5,
        InteractionCallbackType::DeferredChannelMessageWithSource as u8
    );
    const_assert_eq!(6, InteractionCallbackType::DeferredUpdateMessage as u8);
    const_assert_eq!(7, InteractionCallbackType::UpdateMessage as u8);
    const_assert_eq!(
        8,
        InteractionCallbackType::ApplicationCommandAutocompleteResult as u8
    );
    const_assert_eq!(9, InteractionCallbackType::Modal as u8);
    const_assert_eq!(10, InteractionCallbackType::PremiumRequired as u8);
    const_assert_eq!(12, InteractionCallbackType::LaunchActivity as u8);

    #[test]
    fn kind() {
        assert_eq!("Pong", InteractionCallbackType::Pong.kind());
        assert_eq!(
            "ChannelMessageWithSource",
            InteractionCallbackType::ChannelMessageWithSource.kind()
        );
        assert_eq!(
            "DeferredChannelMessageWithSource",
            InteractionCallbackType::DeferredChannelMessageWithSource.kind()
        );
        assert_eq!(
            "DeferredUpdateMessage",
            InteractionCallbackType::DeferredUpdateMessage.kind()
        );
        assert_eq!(
            "UpdateMessage",
            InteractionCallbackType::UpdateMessage.kind()
        );
        assert_eq!(
            "ApplicationCommandAutocompleteResult",
            InteractionCallbackType::ApplicationCommandAutocompleteResult.kind()
        );
        assert_eq!("Modal", InteractionCallbackType::Modal.kind());
        assert_eq!(
            "PremiumRequired",
            InteractionCallbackType::PremiumRequired.kind()
        );
        assert_eq!(
            "LaunchActivity",
            InteractionCallbackType::LaunchActivity.kind()
        );
    }

    #[test]
    fn try_from() -> Result<(), UnknownInteractionCallbackTypeError> {
        assert_eq!(
            InteractionCallbackType::Pong,
            InteractionCallbackType::try_from(1)?
        );
        assert_eq!(
            InteractionCallbackType::ChannelMessageWithSource,
            InteractionCallbackType::try_from(4)?
        );
        assert_eq!(
            InteractionCallbackType::DeferredChannelMessageWithSource,
            InteractionCallbackType::try_from(5)?
        );
        assert_eq!(
            InteractionCallbackType::DeferredUpdateMessage,
            InteractionCallbackType::try_from(6)?,
        );
        assert_eq!(
            InteractionCallbackType::UpdateMessage,
            InteractionCallbackType::try_from(7)?
        );
        assert_eq!(
            InteractionCallbackType::ApplicationCommandAutocompleteResult,
            InteractionCallbackType::try_from(8)?
        );
        assert_eq!(
            InteractionCallbackType::Modal,
            InteractionCallbackType::try_from(9)?
        );
        assert_eq!(
            InteractionCallbackType::PremiumRequired,
            InteractionCallbackType::try_from(10)?
        );
        assert_eq!(
            InteractionCallbackType::LaunchActivity,
            InteractionCallbackType::try_from(12)?
        );

        assert!(InteractionCallbackType::try_from(u8::MAX).is_err());

        Ok(())
    }
}
