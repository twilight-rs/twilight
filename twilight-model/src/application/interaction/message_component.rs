//! [`MessageComponent`] interaction.
//!
//! [`MessageComponent`]: crate::application::interaction::InteractionType::MessageComponent

use crate::application::interaction::InteractionDataResolved;
use crate::channel::message::component::ComponentType;
use serde::{Deserialize, Serialize};

/// Data received when a [`MessageComponent`] interaction is executed.
///
/// See [Discord Docs/Message Component Data Structure].
///
/// [`MessageComponent`]: crate::application::interaction::InteractionType::MessageComponent
/// [Discord Docs/Message Component Data Structure]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-message-component-data-structure
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageComponentInteractionData {
    /// Type of the component.
    pub component_type: ComponentType,
    /// User defined identifier for the component.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// [Discord Docs/Custom ID]: https://discord.com/developers/docs/interactions/message-components#custom-id
    pub custom_id: String,
    /// Converted users, roles, channels, or attachments.
    ///
    /// Only used for [`SelectMenu`] components.
    ///
    /// [`SelectMenu`]: crate::channel::message::component::SelectMenu
    pub resolved: Option<InteractionDataResolved>,
    /// Values selected by the user.
    ///
    /// Only used for [`SelectMenu`] components.
    ///
    /// [`SelectMenu`]: crate::channel::message::component::SelectMenu
    #[serde(default)]
    pub values: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::MessageComponentInteractionData;
    use crate::channel::message::component::ComponentType;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(
        MessageComponentInteractionData: component_type,
        custom_id,
        resolved,
        values
    );
    assert_impl_all!(
        MessageComponentInteractionData: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn message_component_interaction_data() {
        let value = MessageComponentInteractionData {
            component_type: ComponentType::Button,
            custom_id: "test".to_owned(),
            resolved: None,
            values: Vec::from(["1".to_owned(), "2".to_owned()]),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageComponentInteractionData",
                    len: 4,
                },
                Token::String("component_type"),
                Token::U8(ComponentType::Button.into()),
                Token::String("custom_id"),
                Token::String("test"),
                Token::String("resolved"),
                Token::None,
                Token::String("values"),
                Token::Seq { len: Some(2) },
                Token::String("1"),
                Token::String("2"),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        )
    }
}
