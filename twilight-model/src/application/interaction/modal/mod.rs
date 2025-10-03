//! [`ModalSubmit`] interaction.
//!
//!
//! [`ModalSubmit`]: crate::application::interaction::InteractionType::ModalSubmit
mod action_row;
mod label;
mod select_menu;
mod text_display;
mod text_input;

pub use self::{
    action_row::ModalInteractionActionRow,
    label::ModalInteractionLabel,
    select_menu::{
        ModalInteractionChannelSelect, ModalInteractionMentionableSelect,
        ModalInteractionRoleSelect, ModalInteractionStringSelect, ModalInteractionUserSelect,
    },
    text_display::ModalInteractionTextDisplay,
    text_input::ModalInteractionTextInput,
};
use crate::application::interaction::InteractionDataResolved;
use crate::application::interaction::modal::select_menu::ModalInteractionSelectMenu;
use crate::channel::message::component::ComponentType;
use crate::id::Id;
use crate::id::marker::{ChannelMarker, GenericMarker, RoleMarker, UserMarker};
use serde::{
    Deserialize, Serialize, Serializer,
    de::{Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor},
    ser::SerializeStruct,
};
use serde_value::{DeserializerError, Value};
use std::fmt::Formatter;

/// Data received when an [`ModalSubmit`] interaction is executed.
///
/// See [Discord Docs/Modal Submit Data Structure].
///
/// [`ModalSubmit`]: crate::application::interaction::InteractionType::ModalSubmit
/// [Discord Docs/Modal Submit Data Structure]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-modal-submit-data-structure
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModalInteractionData {
    /// List of user inputs.
    pub components: Vec<ModalInteractionComponent>,
    /// User defined identifier for the modal.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// [Discord Docs/Custom ID]: https://discord.com/developers/docs/interactions/message-components#custom-id
    pub custom_id: String,
    pub resolved: Option<InteractionDataResolved>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModalInteractionComponent {
    Label(ModalInteractionLabel),
    ActionRow(ModalInteractionActionRow),
    StringSelect(ModalInteractionStringSelect),
    UserSelect(ModalInteractionUserSelect),
    RoleSelect(ModalInteractionRoleSelect),
    MentionableSelect(ModalInteractionMentionableSelect),
    ChannelSelect(ModalInteractionChannelSelect),
    TextInput(ModalInteractionTextInput),
    TextDisplay(ModalInteractionTextDisplay),
    Unknown(u8),
}

impl ModalInteractionComponent {
    pub fn kind(&self) -> ComponentType {
        match self {
            ModalInteractionComponent::Label(_) => ComponentType::Label,
            ModalInteractionComponent::ActionRow(_) => ComponentType::ActionRow,
            ModalInteractionComponent::StringSelect(_) => ComponentType::TextSelectMenu,
            ModalInteractionComponent::UserSelect(_) => ComponentType::UserSelectMenu,
            ModalInteractionComponent::RoleSelect(_) => ComponentType::RoleSelectMenu,
            ModalInteractionComponent::MentionableSelect(_) => ComponentType::MentionableSelectMenu,
            ModalInteractionComponent::ChannelSelect(_) => ComponentType::ChannelSelectMenu,
            ModalInteractionComponent::TextInput(_) => ComponentType::TextInput,
            ModalInteractionComponent::TextDisplay(_) => ComponentType::TextDisplay,
            ModalInteractionComponent::Unknown(unknown) => ComponentType::from(*unknown),
        }
    }
}

impl From<ModalInteractionLabel> for ModalInteractionComponent {
    fn from(label: ModalInteractionLabel) -> Self {
        Self::Label(label)
    }
}

impl From<ModalInteractionActionRow> for ModalInteractionComponent {
    fn from(action_row: ModalInteractionActionRow) -> Self {
        Self::ActionRow(action_row)
    }
}

impl From<ModalInteractionStringSelect> for ModalInteractionComponent {
    fn from(select: ModalInteractionStringSelect) -> Self {
        Self::StringSelect(select)
    }
}

impl From<ModalInteractionUserSelect> for ModalInteractionComponent {
    fn from(select: ModalInteractionUserSelect) -> Self {
        Self::UserSelect(select)
    }
}

impl From<ModalInteractionRoleSelect> for ModalInteractionComponent {
    fn from(select: ModalInteractionRoleSelect) -> Self {
        Self::RoleSelect(select)
    }
}

impl From<ModalInteractionMentionableSelect> for ModalInteractionComponent {
    fn from(select: ModalInteractionMentionableSelect) -> Self {
        Self::MentionableSelect(select)
    }
}

impl From<ModalInteractionChannelSelect> for ModalInteractionComponent {
    fn from(select: ModalInteractionChannelSelect) -> Self {
        Self::ChannelSelect(select)
    }
}

impl From<ModalInteractionTextInput> for ModalInteractionComponent {
    fn from(text_input: ModalInteractionTextInput) -> Self {
        Self::TextInput(text_input)
    }
}

impl From<ModalInteractionTextDisplay> for ModalInteractionComponent {
    fn from(text_display: ModalInteractionTextDisplay) -> Self {
        Self::TextDisplay(text_display)
    }
}

impl<'de> Deserialize<'de> for ModalInteractionComponent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ModalInteractionDataComponentVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum Field {
    Component,
    Components,
    CustomId,
    Id,
    Type,
    Value,
    Values,
}

struct ModalInteractionDataComponentVisitor;

impl<'de> Visitor<'de> for ModalInteractionDataComponentVisitor {
    type Value = ModalInteractionComponent;

    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str("struct ModalInteractionDataComponent")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        fn deserialize_select_menu<'de, ValueType: Deserialize<'de>, Error: DeError>(
            id: i32,
            custom_id: Option<String>,
            values: Option<Vec<Value>>,
        ) -> Result<ModalInteractionSelectMenu<ValueType>, Error> {
            let custom_id = custom_id.ok_or_else(|| DeError::missing_field("custom_id"))?;
            let values = values
                .ok_or_else(|| DeError::missing_field("values"))?
                .into_iter()
                .map(Value::deserialize_into)
                .collect::<Result<_, _>>()
                .map_err(DeserializerError::into_error)?;

            Ok(ModalInteractionSelectMenu {
                id,
                custom_id,
                values,
            })
        }

        // Required fields
        let mut id: Option<i32> = None;
        let mut kind: Option<ComponentType> = None;
        let mut custom_id: Option<String> = None;
        let mut values: Option<Vec<Value>> = None;
        let mut components: Option<Vec<ModalInteractionComponent>> = None;
        let mut component: Option<ModalInteractionComponent> = None;
        let mut value: Option<String> = None;

        loop {
            let key = match map.next_key() {
                Ok(Some(key)) => key,
                Ok(None) => break,
                Err(_) => {
                    map.next_value::<IgnoredAny>()?;

                    continue;
                }
            };

            match key {
                Field::Component => {
                    if component.is_some() {
                        return Err(DeError::duplicate_field("component"));
                    }

                    component = Some(map.next_value()?);
                }
                Field::Components => {
                    if components.is_some() {
                        return Err(DeError::duplicate_field("components"));
                    }

                    components = Some(map.next_value()?);
                }
                Field::CustomId => {
                    if custom_id.is_some() {
                        return Err(DeError::duplicate_field("custom_id"));
                    }

                    custom_id = Some(map.next_value()?);
                }
                Field::Id => {
                    if id.is_some() {
                        return Err(DeError::duplicate_field("id"));
                    }

                    id = Some(map.next_value()?);
                }
                Field::Type => {
                    if kind.is_some() {
                        return Err(DeError::duplicate_field("kind"));
                    }

                    kind = Some(map.next_value()?);
                }
                Field::Value => {
                    if value.is_some() {
                        return Err(DeError::duplicate_field("value"));
                    }

                    value = Some(map.next_value()?);
                }
                Field::Values => {
                    if values.is_some() {
                        return Err(DeError::duplicate_field("values"));
                    }

                    values = Some(map.next_value()?);
                }
            }
        }

        let kind = kind.ok_or_else(|| DeError::missing_field("type"))?;
        let id = id.ok_or_else(|| DeError::missing_field("id"))?;

        Ok(match kind {
            ComponentType::ActionRow => {
                let components = components.ok_or_else(|| DeError::missing_field("components"))?;

                Self::Value::ActionRow(ModalInteractionActionRow { id, components })
            }
            ComponentType::TextSelectMenu => {
                Self::Value::StringSelect(deserialize_select_menu::<String, _>(
                    id, custom_id, values,
                )?)
            }
            ComponentType::UserSelectMenu => {
                Self::Value::UserSelect(deserialize_select_menu::<Id<UserMarker>, _>(
                    id, custom_id, values,
                )?)
            }
            ComponentType::RoleSelectMenu => {
                Self::Value::RoleSelect(deserialize_select_menu::<Id<RoleMarker>, _>(
                    id, custom_id, values,
                )?)
            }
            ComponentType::MentionableSelectMenu => Self::Value::MentionableSelect(
                deserialize_select_menu::<Id<GenericMarker>, _>(id, custom_id, values)?,
            ),
            ComponentType::ChannelSelectMenu => Self::Value::ChannelSelect(
                deserialize_select_menu::<Id<ChannelMarker>, _>(id, custom_id, values)?,
            ),
            ComponentType::TextInput => {
                let custom_id = custom_id.ok_or_else(|| DeError::missing_field("custom_id"))?;
                let value = value.ok_or_else(|| DeError::missing_field("value"))?;

                Self::Value::TextInput(ModalInteractionTextInput {
                    custom_id,
                    id,
                    value,
                })
            }
            ComponentType::TextDisplay => {
                Self::Value::TextDisplay(ModalInteractionTextDisplay { id })
            }
            ComponentType::Label => {
                let component = component.ok_or_else(|| DeError::missing_field("component"))?;

                Self::Value::Label(ModalInteractionLabel {
                    id,
                    component: Box::new(component),
                })
            }
            ComponentType::Button
            | ComponentType::Section
            | ComponentType::Thumbnail
            | ComponentType::MediaGallery
            | ComponentType::File
            | ComponentType::Separator
            | ComponentType::Container
            | ComponentType::Unknown(_) => Self::Value::Unknown(kind.into()),
        })
    }
}

impl Serialize for ModalInteractionComponent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        fn serialize_select_menu<State: SerializeStruct, ValueType: Serialize>(
            state: &mut State,
            select: &ModalInteractionSelectMenu<ValueType>,
        ) -> Result<(), <State as SerializeStruct>::Error> {
            state.serialize_field("id", &select.id)?;
            state.serialize_field("custom_id", &select.custom_id)?;
            state.serialize_field("values", &select.values)?;
            Ok(())
        }

        #[allow(clippy::match_same_arms)]
        let len = match self {
            // Required fields:
            // - type
            // - id
            // - component
            ModalInteractionComponent::Label(_) => 3,
            // Required fields:
            // - type
            // - id
            // - components
            ModalInteractionComponent::ActionRow(_) => 3, // Required fields:
            // - type
            // - id
            // - custom_id
            // - values
            ModalInteractionComponent::StringSelect(_)
            | ModalInteractionComponent::UserSelect(_)
            | ModalInteractionComponent::RoleSelect(_)
            | ModalInteractionComponent::MentionableSelect(_)
            | ModalInteractionComponent::ChannelSelect(_) => 4,
            // Required fields:
            // - type
            // - id
            // - custom_id
            // - value
            ModalInteractionComponent::TextInput(_) => 4,
            // Required fields:
            // - type
            // - id
            ModalInteractionComponent::TextDisplay(_) => 2,
            // We are dropping all fields but type here but nothing we can do about that for
            // the time being
            ModalInteractionComponent::Unknown(_) => 1,
        };

        let mut state = serializer.serialize_struct("ModalInteractionComponent", len)?;
        state.serialize_field("type", &self.kind())?;

        match self {
            ModalInteractionComponent::Label(label) => {
                state.serialize_field("id", &label.id)?;
                state.serialize_field("component", &label.component)?;
            }
            ModalInteractionComponent::ActionRow(action_row) => {
                state.serialize_field("id", &action_row.id)?;
                state.serialize_field("components", &action_row.components)?;
            }
            ModalInteractionComponent::StringSelect(select) => {
                serialize_select_menu(&mut state, select)?;
            }
            ModalInteractionComponent::UserSelect(select) => {
                serialize_select_menu(&mut state, select)?;
            }
            ModalInteractionComponent::RoleSelect(select) => {
                serialize_select_menu(&mut state, select)?;
            }
            ModalInteractionComponent::MentionableSelect(select) => {
                serialize_select_menu(&mut state, select)?;
            }
            ModalInteractionComponent::ChannelSelect(select) => {
                serialize_select_menu(&mut state, select)?;
            }
            ModalInteractionComponent::TextInput(text_input) => {
                state.serialize_field("custom_id", &text_input.custom_id)?;
                state.serialize_field("id", &text_input.id)?;
                state.serialize_field("value", &text_input.value)?;
            }
            ModalInteractionComponent::TextDisplay(text_display) => {
                state.serialize_field("id", &text_display.id)?;
            }
            // We are not serializing all fields so this will fail to
            // deserialize. But it is all that can be done to avoid losing
            // incoming messages at this time.
            ModalInteractionComponent::Unknown(_) => {}
        }

        state.end()
    }
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
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    assert_fields!(ModalInteractionActionRow: components);
    assert_impl_all!(
        ModalInteractionActionRow: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    assert_fields!(ModalInteractionComponent: custom_id, value);
    assert_impl_all!(
        ModalInteractionComponent: Clone,
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
            components: Vec::from([ModalInteractionActionRow {
                components: Vec::from([ModalInteractionComponent {
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
