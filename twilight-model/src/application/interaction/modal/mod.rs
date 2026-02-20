//! [`ModalSubmit`] interaction.
//!
//!
//! [`ModalSubmit`]: crate::application::interaction::InteractionType::ModalSubmit
mod action_row;
mod checkbox;
mod checkbox_group;
mod file_upload;
mod label;
mod select_menu;
mod text_display;
mod text_input;

pub use self::{
    action_row::ModalInteractionActionRow,
    checkbox::ModalInteractionCheckbox,
    checkbox_group::ModalInteractionCheckboxGroup,
    file_upload::ModalInteractionFileUpload,
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
    /// List of modal component responses.
    pub components: Vec<ModalInteractionComponent>,
    /// User defined identifier for the modal.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// [Discord Docs/Custom ID]: https://discord.com/developers/docs/components/reference#anatomy-of-a-component-custom-id
    pub custom_id: String,
    /// Resolved data from select menus.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<InteractionDataResolved>,
}

/// User filled in modal component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModalInteractionComponent {
    /// Top-level layout component including a string label and optional description.
    Label(ModalInteractionLabel),
    /// Top-level layout component. In modals, layout components are preferred over action rows.
    ActionRow(ModalInteractionActionRow),
    /// Dropdown selection component for text.
    StringSelect(ModalInteractionStringSelect),
    /// Dropdown selection component for users.
    UserSelect(ModalInteractionUserSelect),
    /// Dropdown selection component for roles.
    RoleSelect(ModalInteractionRoleSelect),
    /// Dropdown selection component for mentionables.
    MentionableSelect(ModalInteractionMentionableSelect),
    /// Dropdown selection component for channels.
    ChannelSelect(ModalInteractionChannelSelect),
    /// Text input component.
    TextInput(ModalInteractionTextInput),
    /// Markdown text.
    TextDisplay(ModalInteractionTextDisplay),
    /// File upload component.
    FileUpload(ModalInteractionFileUpload),
    /// Checkbox Group Component.
    CheckboxGroup(ModalInteractionCheckboxGroup),

    Checkbox(ModalInteractionCheckbox),
    /// Variant value is unknown to the library in the context of modals.
    Unknown(u8),
}

impl ModalInteractionComponent {
    /// Type of component that this is.
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
            ModalInteractionComponent::FileUpload(_) => ComponentType::FileUpload,
            ModalInteractionComponent::Checkbox(_) => ComponentType::Checkbox,
            ModalInteractionComponent::CheckboxGroup(_) => ComponentType::CheckboxGroup,
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

impl From<ModalInteractionFileUpload> for ModalInteractionComponent {
    fn from(file_upload: ModalInteractionFileUpload) -> Self {
        Self::FileUpload(file_upload)
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
            ComponentType::FileUpload => {
                let custom_id = custom_id.ok_or_else(|| DeError::missing_field("custom_id"))?;
                let values = values
                    .ok_or_else(|| DeError::missing_field("values"))?
                    .into_iter()
                    .map(Value::deserialize_into)
                    .collect::<Result<_, _>>()
                    .map_err(DeserializerError::into_error)?;

                Self::Value::FileUpload(ModalInteractionFileUpload {
                    id,
                    custom_id,
                    values,
                })
            }
            ComponentType::CheckboxGroup => {
                let custom_id = custom_id.ok_or_else(|| DeError::missing_field("custom_id"))?;
                let values = values
                    .ok_or_else(|| DeError::missing_field("values"))?
                    .into_iter()
                    .map(Value::deserialize_into)
                    .collect::<Result<_, _>>()
                    .map_err(DeserializerError::into_error)?;

                Self::Value::CheckboxGroup(ModalInteractionCheckboxGroup {
                    id,
                    custom_id,
                    values,
                })
            }
            ComponentType::Checkbox => {
                let custom_id = custom_id.ok_or_else(|| DeError::missing_field("custom_id"))?;
                let value = value.ok_or_else(|| DeError::missing_field("value"))?;

                Self::Value::TextInput(ModalInteractionTextInput {
                    custom_id,
                    id,
                    value,
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
            state.serialize_field("custom_id", &select.custom_id)?;
            state.serialize_field("id", &select.id)?;
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
            ModalInteractionComponent::ActionRow(_) => 3,
            // Required fields:
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
            // Required fields:
            // - type
            // - id
            // - custom_id
            // - values
            ModalInteractionComponent::FileUpload(_) => 4,
            ModalInteractionComponent::CheckboxGroup(_) => 4,
            ModalInteractionComponent::Checkbox(_) => 4,
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
            ModalInteractionComponent::FileUpload(file_upload) => {
                state.serialize_field("custom_id", &file_upload.custom_id)?;
                state.serialize_field("id", &file_upload.id)?;
                state.serialize_field("values", &file_upload.values)?;
            }
            ModalInteractionComponent::CheckboxGroup(checkbox_group) => {
                state.serialize_field("custom_id", &checkbox_group.custom_id)?;
                state.serialize_field("id", &checkbox_group.id)?;
                state.serialize_field("values", &checkbox_group.values)?;
            }
            ModalInteractionComponent::Checkbox(checkbox) => {
                state.serialize_field("custom_id", &checkbox.custom_id)?;
                state.serialize_field("id", &checkbox.id)?;
                state.serialize_field("value", &checkbox.value)?;
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
    use crate::application::interaction::InteractionChannel;
    use crate::channel::ChannelType;
    use crate::guild::Permissions;
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::collections::HashMap;
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

    fn label_tokens(id: i32, component_tokens: &[Token]) -> Vec<Token> {
        let mut label_tokens = vec![
            Token::Struct {
                name: "ModalInteractionComponent",
                len: 3,
            },
            Token::String("type"),
            Token::U8(ComponentType::Label.into()),
            Token::String("id"),
            Token::I32(id),
            Token::String("component"),
        ];
        label_tokens.extend_from_slice(component_tokens);
        label_tokens.push(Token::StructEnd);

        label_tokens
    }

    #[test]
    fn modal_action_rows() {
        let value = ModalInteractionData {
            custom_id: "test-modal".to_owned(),
            components: vec![ModalInteractionComponent::ActionRow(
                ModalInteractionActionRow {
                    id: 1,
                    components: vec![ModalInteractionComponent::TextInput(
                        ModalInteractionTextInput {
                            custom_id: "the-data-id".to_owned(),
                            id: 2,
                            value: "input value".to_owned(),
                        },
                    )],
                },
            )],
            resolved: None,
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
                    name: "ModalInteractionComponent",
                    len: 3,
                },
                Token::String("type"),
                Token::U8(ComponentType::ActionRow.into()),
                Token::String("id"),
                Token::I32(1),
                Token::String("components"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "ModalInteractionComponent",
                    len: 4,
                },
                Token::String("type"),
                Token::U8(ComponentType::TextInput.into()),
                Token::String("custom_id"),
                Token::String("the-data-id"),
                Token::String("id"),
                Token::I32(2),
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

    #[test]
    #[allow(clippy::too_many_lines)]
    fn modal_labels() {
        let value = ModalInteractionData {
            custom_id: "test-modal".to_owned(),
            components: vec![
                ModalInteractionComponent::Label(ModalInteractionLabel {
                    id: 1,
                    component: Box::new(ModalInteractionComponent::TextInput(
                        ModalInteractionTextInput {
                            custom_id: "the-text-input-id".to_owned(),
                            id: 2,
                            value: "input value".to_owned(),
                        },
                    )),
                }),
                ModalInteractionComponent::Label(ModalInteractionLabel {
                    id: 3,
                    component: Box::new(ModalInteractionComponent::TextDisplay(
                        ModalInteractionTextDisplay { id: 4 },
                    )),
                }),
                ModalInteractionComponent::Label(ModalInteractionLabel {
                    id: 5,
                    component: Box::new(ModalInteractionComponent::ChannelSelect(
                        ModalInteractionChannelSelect {
                            id: 6,
                            custom_id: "the-channel-select-id".to_owned(),
                            values: vec![Id::new(42)],
                        },
                    )),
                }),
            ],
            resolved: Some(InteractionDataResolved {
                attachments: HashMap::new(),
                channels: HashMap::from([(
                    Id::new(42),
                    InteractionChannel {
                        id: Id::new(42),
                        kind: ChannelType::GuildText,
                        name: "the-channel-name".to_owned(),
                        parent_id: None,
                        permissions: Permissions::empty(),
                        thread_metadata: None,
                    },
                )]),
                members: HashMap::new(),
                messages: HashMap::new(),
                roles: HashMap::new(),
                users: HashMap::new(),
            }),
        };

        let text_input_tokens = [
            Token::Struct {
                name: "ModalInteractionComponent",
                len: 4,
            },
            Token::String("type"),
            Token::U8(ComponentType::TextInput.into()),
            Token::String("custom_id"),
            Token::String("the-text-input-id"),
            Token::String("id"),
            Token::I32(2),
            Token::String("value"),
            Token::String("input value"),
            Token::StructEnd,
        ];

        let text_display_tokens = [
            Token::Struct {
                name: "ModalInteractionComponent",
                len: 2,
            },
            Token::String("type"),
            Token::U8(ComponentType::TextDisplay.into()),
            Token::String("id"),
            Token::I32(4),
            Token::StructEnd,
        ];

        let channel_select_tokens = [
            Token::Struct {
                name: "ModalInteractionComponent",
                len: 4,
            },
            Token::String("type"),
            Token::U8(ComponentType::ChannelSelectMenu.into()),
            Token::String("custom_id"),
            Token::String("the-channel-select-id"),
            Token::String("id"),
            Token::I32(6),
            Token::String("values"),
            Token::Seq { len: Some(1) },
            Token::NewtypeStruct { name: "Id" },
            Token::String("42"),
            Token::SeqEnd,
            Token::StructEnd,
        ];

        let mut all_tokens = vec![
            Token::Struct {
                name: "ModalInteractionData",
                len: 3,
            },
            Token::String("components"),
            Token::Seq { len: Some(3) },
        ];

        all_tokens.extend_from_slice(&label_tokens(1, &text_input_tokens));
        all_tokens.extend_from_slice(&label_tokens(3, &text_display_tokens));
        all_tokens.extend_from_slice(&label_tokens(5, &channel_select_tokens));

        all_tokens.extend_from_slice(&[
            Token::SeqEnd,
            Token::String("custom_id"),
            Token::String("test-modal"),
            Token::String("resolved"),
            Token::Some,
            Token::Struct {
                name: "InteractionDataResolved",
                len: 1,
            },
            Token::String("channels"),
            Token::Map { len: Some(1) },
            Token::NewtypeStruct { name: "Id" },
            Token::String("42"),
            Token::Struct {
                name: "InteractionChannel",
                len: 4,
            },
            Token::String("id"),
            Token::NewtypeStruct { name: "Id" },
            Token::String("42"),
            Token::String("type"),
            Token::U8(0),
            Token::String("name"),
            Token::String("the-channel-name"),
            Token::String("permissions"),
            Token::String("0"),
            Token::StructEnd,
            Token::MapEnd,
            Token::StructEnd,
            Token::StructEnd,
        ]);

        serde_test::assert_tokens(&value, &all_tokens);
    }

    #[test]
    fn modal_file_upload() {
        let value = ModalInteractionData {
            custom_id: "test-modal".to_owned(),
            components: vec![ModalInteractionComponent::FileUpload(
                ModalInteractionFileUpload {
                    id: 42,
                    custom_id: "file-upload".to_owned(),
                    values: vec![Id::new(1), Id::new(2)],
                },
            )],
            // Having a None resolved data for the file upload response is not realistic,
            // but (de)serialization of InteractionDataResolved is already tested sufficiently.
            resolved: None,
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
                    name: "ModalInteractionComponent",
                    len: 4,
                },
                Token::String("type"),
                Token::U8(ComponentType::FileUpload.into()),
                Token::String("custom_id"),
                Token::String("file-upload"),
                Token::String("id"),
                Token::I32(42),
                Token::String("values"),
                Token::Seq { len: Some(2) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::String("custom_id"),
                Token::String("test-modal"),
                Token::StructEnd,
            ],
        )
    }
}
