//! [`ModalSubmit`] interaction.
//!
//!
//! [`ModalSubmit`]: crate::application::interaction::InteractionType::ModalSubmit

use crate::channel::message::component::ComponentType;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};

/// Data received when an [`ModalSubmit`] interaction is executed.
///
/// See [Discord Docs/Modal Submit Data Structure].
///
/// [`ModalSubmit`]: crate::application::interaction::InteractionType::ModalSubmit
/// [Discord Docs/Modal Submit Data Structure]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-modal-submit-data-structure
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModalInteractionData {
    /// List of user inputs.
    pub components: Vec<ModalInteractionDataActionRow>,
    /// User defined identifier for the modal.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// [Discord Docs/Custom ID]: https://discord.com/developers/docs/interactions/message-components#custom-id
    pub custom_id: String,
}

/// User filled in [`ActionRow`].
///
/// See [Discord Docs/Modal Submit Data Structure].
///
/// [`ActionRow`]: crate::application::interaction::modal::ModalInteractionDataActionRow
/// [Discord Docs/Modal Submit Data Structure]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-modal-submit-data-structure
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ModalInteractionDataActionRow {
    /// List of components.
    pub components: Vec<ModalInteractionDataComponent>,
}

impl Serialize for ModalInteractionDataActionRow {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("ModalInteractionDataActionRow", 2)?;

        state.serialize_field("type", &ComponentType::ActionRow)?;
        state.serialize_field("components", &self.components)?;

        state.end()
    }
}

/// User filled in modal component.
///
/// See [Discord Docs/Message Components].
///
/// [Discord Docs/Message Components]: https://discord.com/developers/docs/interactions/message-components
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModalInteractionDataComponent {
    /// User defined identifier for the component.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// [Discord Docs/Custom ID]: https://discord.com/developers/docs/interactions/message-components#custom-id
    pub custom_id: String,
    /// Type of the component.
    #[serde(rename = "type")]
    pub kind: ComponentType,
    /// Value submitted by the user.
    pub value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(ModalInteractionData: custom_id, components);
    assert_impl_all!(
        ModalInteractionData: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    assert_fields!(ModalInteractionDataActionRow: components);
    assert_impl_all!(
        ModalInteractionDataActionRow: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    assert_fields!(ModalInteractionDataComponent: custom_id, value);
    assert_impl_all!(
        ModalInteractionDataComponent: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn modal_data() {
        let value = ModalInteractionData {
            custom_id: "test-modal".to_owned(),
            components: Vec::from([ModalInteractionDataActionRow {
                components: Vec::from([ModalInteractionDataComponent {
                    custom_id: "the-data-id".to_owned(),
                    kind: ComponentType::TextInput,
                    value: Some("input value".into()),
                }]),
            }]),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ModalInteractionData",
                    len: 2,
                },
                Token::String("components"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "ModalInteractionDataActionRow",
                    len: 2,
                },
                Token::String("type"),
                Token::U8(ComponentType::ActionRow.into()),
                Token::String("components"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "ModalInteractionDataComponent",
                    len: 3,
                },
                Token::String("custom_id"),
                Token::String("the-data-id"),
                Token::String("type"),
                Token::U8(ComponentType::TextInput.into()),
                Token::String("value"),
                Token::Some,
                Token::String("input value"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::String("custom_id"),
                Token::String("test-modal"),
                Token::StructEnd,
            ],
        );
    }
}
