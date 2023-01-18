use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// Type of interaction.
///
/// See [Discord Docs/Interaction Object].
///
/// [Discord Docs/Interaction Object]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-type
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InteractionType(u8);

impl InteractionType {
    /// Interaction involves a ping (webhook-based interactions).
    ///
    /// See [Discord Docs/Receiving an Interaction].
    ///
    /// [Discord Docs/Receiving an Interaction]: https://discord.com/developers/docs/interactions/receiving-and-responding#receiving-an-interaction
    pub const PING: Self = Self::new(1);

    /// Interaction involves an application command.
    pub const APPLICATION_COMMAND: Self = Self::new(2);

    /// Interaction involves a message [`Component`].
    ///
    /// [`Component`]: crate::channel::message::Component
    pub const MESSAGE_COMPONENT: Self = Self::new(3);

    /// Interaction involves an autocomplete request.
    pub const APPLICATION_COMMAND_AUTOCOMPLETE: Self = Self::new(4);

    /// Interaction involves a modal submit.
    pub const MODAL_SUBMIT: Self = Self::new(5);

    /// Create a new interaction type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`MODAL_SUBMIT`][`Self::MODAL_SUBMIT`].
    pub const fn new(connection_visibility: u8) -> Self {
        Self(connection_visibility)
    }

    /// Retrieve the value of the interaction type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::application::interaction::InteractionType;
    ///
    /// assert_eq!(3, InteractionType::MESSAGE_COMPONENT.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::APPLICATION_COMMAND => "APPLICATION_COMMAND",
            Self::APPLICATION_COMMAND_AUTOCOMPLETE => "APPLICATION_COMMAND_AUTOCOMPLETE",
            Self::MESSAGE_COMPONENT => "MESSAGE_COMPONENT",
            Self::MODAL_SUBMIT => "MODAL_SUBMIT",
            Self::PING => "PING",
            _ => return None,
        })
    }
}

impl Debug for InteractionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("InteractionType")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("InteractionType").field(&self.0).finish()
        }
    }
}

impl From<u8> for InteractionType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<InteractionType> for u8 {
    fn from(value: InteractionType) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::InteractionType;
    use serde_test::Token;

    const MAP: &[(InteractionType, u8)] = &[
        (InteractionType::PING, 1),
        (InteractionType::APPLICATION_COMMAND, 2),
        (InteractionType::MESSAGE_COMPONENT, 3),
        (InteractionType::APPLICATION_COMMAND_AUTOCOMPLETE, 4),
        (InteractionType::MODAL_SUBMIT, 5),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "InteractionType",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, InteractionType::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
