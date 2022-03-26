use crate::application::component::ComponentType;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

/// Data received when an [`ModalSubmit`] interaction is executed.
///
/// Refer to [the discord docs] for more information.
///
/// [`ModalSubmit`]: crate::application::interaction::Interaction::ModalSubmit
/// [the discord docs]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data-structure
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModalInteractionData {
    /// List of parsed user inputs.
    pub components: Vec<ModalInteractionDataActionRow>,
    /// User defined identifier for the input text.
    pub custom_id: String,
}

/// The parsed [`ActionRow`] of the users input.
///
/// Refer to [the discord docs] for more information.
///
/// [`ActionRow`]: crate::application::interaction::modal::ModalInteractionDataActionRow
/// [the discord docs]: https://discord.com/developers/docs/interactions/message-components#component-object-component-structure
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ModalInteractionDataActionRow {
    /// Parsed components.
    pub components: Vec<ModalInteractionDataComponent>,
}

impl Serialize for ModalInteractionDataActionRow {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("ModalInteractionDataActionRow", 2)?;

        state.serialize_field("component_type", &ComponentType::ActionRow)?;
        state.serialize_field("components", &self.components)?;

        state.end()
    }
}

/// Data received when a user fills in a modal component.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/message-components#component-object-component-structure
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModalInteractionDataComponent {
    pub custom_id: String,
    #[serde(rename = "type")]
    pub kind: ComponentType,
    pub value: String,
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
    fn test_modal_data() {
        let value = ModalInteractionData {
            custom_id: "test-modal".to_owned(),
            components: Vec::from([ModalInteractionDataActionRow {
                components: Vec::from([ModalInteractionDataComponent {
                    custom_id: "the-data-id".to_owned(),
                    kind: ComponentType::TextInput,
                    value: "input value".into(),
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
                Token::String("component_type"),
                Token::U8(ComponentType::ActionRow as u8),
                Token::String("components"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "ModalInteractionDataComponent",
                    len: 3,
                },
                Token::String("custom_id"),
                Token::String("the-data-id"),
                Token::String("type"),
                Token::U8(ComponentType::TextInput as u8),
                Token::String("value"),
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
