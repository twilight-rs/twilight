//! Interactive message elements for use with [`Interaction`]s.
//!
//! Refer to [Discord Docs/Message Components] for additional information.
//!
//! [`Interaction`]: crate::application::interaction::Interaction
//! [Discord Docs/Message Components]: https://discord.com/developers/docs/interactions/message-components

mod action_row;
mod button;
mod kind;
mod select_menu;
mod text_input;

pub use self::{
    action_row::ActionRow,
    button::{Button, ButtonStyle},
    kind::ComponentType,
    select_menu::{SelectDefaultValue, SelectMenu, SelectMenuOption, SelectMenuType},
    text_input::{TextInput, TextInputStyle},
};

use super::EmojiReactionType;
use crate::channel::ChannelType;
use crate::id::marker::SkuMarker;
use crate::id::Id;
use serde::{
    de::{Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor},
    ser::{Error as SerError, SerializeStruct},
    Deserialize, Serialize, Serializer,
};
use serde_value::{DeserializerError, Value};
use std::fmt::{Formatter, Result as FmtResult};

/// Interactive message element.
///
/// Must be either a top level [`ActionRow`] or nested inside one.
///
/// # Examples
///
/// ## Button
///
/// ```
/// use twilight_model::channel::message::component::{ActionRow, Button, ButtonStyle, Component};
///
/// Component::ActionRow(ActionRow {
///     components: Vec::from([Component::Button(Button {
///         custom_id: Some("click_one".to_owned()),
///         disabled: false,
///         emoji: None,
///         label: Some("Click me!".to_owned()),
///         style: ButtonStyle::Primary,
///         url: None,
///         sku_id: None,
///     })]),
/// });
/// ```
///
/// ## Select menu
///
/// ```
/// use twilight_model::{
///     channel::message::{
///         component::{ActionRow, Component, SelectMenu, SelectMenuOption, SelectMenuType},
///         EmojiReactionType,
///     },
///     id::Id,
/// };
///
/// Component::ActionRow(ActionRow {
///     components: vec![Component::SelectMenu(SelectMenu {
///         channel_types: None,
///         custom_id: "class_select_1".to_owned(),
///         default_values: None,
///         disabled: false,
///         kind: SelectMenuType::Text,
///         max_values: Some(3),
///         min_values: Some(1),
///         options: Some(Vec::from([
///             SelectMenuOption {
///                 default: false,
///                 emoji: Some(EmojiReactionType::Custom {
///                     animated: false,
///                     id: Id::new(625891304148303894),
///                     name: Some("rogue".to_owned()),
///                 }),
///                 description: Some("Sneak n stab".to_owned()),
///                 label: "Rogue".to_owned(),
///                 value: "rogue".to_owned(),
///             },
///             SelectMenuOption {
///                 default: false,
///                 emoji: Some(EmojiReactionType::Custom {
///                     animated: false,
///                     id: Id::new(625891304081063986),
///                     name: Some("mage".to_owned()),
///                 }),
///                 description: Some("Turn 'em into a sheep".to_owned()),
///                 label: "Mage".to_owned(),
///                 value: "mage".to_owned(),
///             },
///             SelectMenuOption {
///                 default: false,
///                 emoji: Some(EmojiReactionType::Custom {
///                     animated: false,
///                     id: Id::new(625891303795982337),
///                     name: Some("priest".to_owned()),
///                 }),
///                 description: Some("You get heals when I'm done doing damage".to_owned()),
///                 label: "Priest".to_owned(),
///                 value: "priest".to_owned(),
///             },
///         ])),
///         placeholder: Some("Choose a class".to_owned()),
///     })],
/// });
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Component {
    /// Top level, non-interactive container of other (non action row) components.
    ActionRow(ActionRow),
    /// Clickable item that renders below messages.
    Button(Button),
    /// Dropdown-style item that renders below messages.
    SelectMenu(SelectMenu),
    /// Pop-up item that renders on modals.
    TextInput(TextInput),
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl Component {
    /// Type of component that this is.
    ///
    /// ```
    /// use twilight_model::channel::message::component::{
    ///     Button, ButtonStyle, Component, ComponentType,
    /// };
    ///
    /// let component = Component::Button(Button {
    ///     custom_id: None,
    ///     disabled: false,
    ///     emoji: None,
    ///     label: Some("ping".to_owned()),
    ///     style: ButtonStyle::Primary,
    ///     url: None,
    ///     sku_id: None,
    /// });
    ///
    /// assert_eq!(ComponentType::Button, component.kind());
    /// ```
    pub const fn kind(&self) -> ComponentType {
        match self {
            Self::ActionRow(_) => ComponentType::ActionRow,
            Self::Button(_) => ComponentType::Button,
            Self::SelectMenu(SelectMenu { kind, .. }) => match kind {
                SelectMenuType::Text => ComponentType::TextSelectMenu,
                SelectMenuType::User => ComponentType::UserSelectMenu,
                SelectMenuType::Role => ComponentType::RoleSelectMenu,
                SelectMenuType::Mentionable => ComponentType::MentionableSelectMenu,
                SelectMenuType::Channel => ComponentType::ChannelSelectMenu,
            },
            Self::TextInput(_) => ComponentType::TextInput,
            Component::Unknown(unknown) => ComponentType::Unknown(*unknown),
        }
    }
}

impl From<ActionRow> for Component {
    fn from(action_row: ActionRow) -> Self {
        Self::ActionRow(action_row)
    }
}

impl From<Button> for Component {
    fn from(button: Button) -> Self {
        Self::Button(button)
    }
}

impl From<SelectMenu> for Component {
    fn from(select_menu: SelectMenu) -> Self {
        Self::SelectMenu(select_menu)
    }
}

impl From<TextInput> for Component {
    fn from(text_input: TextInput) -> Self {
        Self::TextInput(text_input)
    }
}

impl<'de> Deserialize<'de> for Component {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ComponentVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum Field {
    ChannelTypes,
    Components,
    CustomId,
    DefaultValues,
    Disabled,
    Emoji,
    Label,
    MaxLength,
    MaxValues,
    MinLength,
    MinValues,
    Options,
    Placeholder,
    Required,
    Style,
    Type,
    Url,
    SkuId,
    Value,
}

struct ComponentVisitor;

impl<'de> Visitor<'de> for ComponentVisitor {
    type Value = Component;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct Component")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        // Required fields.
        let mut components: Option<Vec<Component>> = None;
        let mut kind: Option<ComponentType> = None;
        let mut options: Option<Vec<SelectMenuOption>> = None;
        let mut style: Option<Value> = None;

        // Liminal fields.
        let mut custom_id: Option<Option<Value>> = None;
        let mut label: Option<Option<String>> = None;

        // Optional fields.
        let mut channel_types: Option<Vec<ChannelType>> = None;
        let mut default_values: Option<Vec<SelectDefaultValue>> = None;
        let mut disabled: Option<bool> = None;
        let mut emoji: Option<Option<EmojiReactionType>> = None;
        let mut max_length: Option<Option<u16>> = None;
        let mut max_values: Option<Option<u8>> = None;
        let mut min_length: Option<Option<u16>> = None;
        let mut min_values: Option<Option<u8>> = None;
        let mut placeholder: Option<Option<String>> = None;
        let mut required: Option<Option<bool>> = None;
        let mut url: Option<Option<String>> = None;
        let mut sku_id: Option<Id<SkuMarker>> = None;
        let mut value: Option<Option<String>> = None;

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
                Field::ChannelTypes => {
                    if channel_types.is_some() {
                        return Err(DeError::duplicate_field("channel_types"));
                    }

                    channel_types = Some(map.next_value()?);
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
                Field::DefaultValues => {
                    if default_values.is_some() {
                        return Err(DeError::duplicate_field("default_values"));
                    }

                    default_values = map.next_value()?;
                }
                Field::Disabled => {
                    if disabled.is_some() {
                        return Err(DeError::duplicate_field("disabled"));
                    }

                    disabled = Some(map.next_value()?);
                }
                Field::Emoji => {
                    if emoji.is_some() {
                        return Err(DeError::duplicate_field("emoji"));
                    }

                    emoji = Some(map.next_value()?);
                }
                Field::Label => {
                    if label.is_some() {
                        return Err(DeError::duplicate_field("label"));
                    }

                    label = Some(map.next_value()?);
                }
                Field::MaxLength => {
                    if max_length.is_some() {
                        return Err(DeError::duplicate_field("max_length"));
                    }

                    max_length = Some(map.next_value()?);
                }
                Field::MaxValues => {
                    if max_values.is_some() {
                        return Err(DeError::duplicate_field("max_values"));
                    }

                    max_values = Some(map.next_value()?);
                }
                Field::MinLength => {
                    if min_length.is_some() {
                        return Err(DeError::duplicate_field("min_length"));
                    }

                    min_length = Some(map.next_value()?);
                }
                Field::MinValues => {
                    if min_values.is_some() {
                        return Err(DeError::duplicate_field("min_values"));
                    }

                    min_values = Some(map.next_value()?);
                }
                Field::Options => {
                    if options.is_some() {
                        return Err(DeError::duplicate_field("options"));
                    }

                    options = Some(map.next_value()?);
                }
                Field::Placeholder => {
                    if placeholder.is_some() {
                        return Err(DeError::duplicate_field("placeholder"));
                    }

                    placeholder = Some(map.next_value()?);
                }
                Field::Required => {
                    if required.is_some() {
                        return Err(DeError::duplicate_field("required"));
                    }

                    required = Some(map.next_value()?);
                }
                Field::Style => {
                    if style.is_some() {
                        return Err(DeError::duplicate_field("style"));
                    }

                    style = Some(map.next_value()?);
                }
                Field::Type => {
                    if kind.is_some() {
                        return Err(DeError::duplicate_field("type"));
                    }

                    kind = Some(map.next_value()?);
                }
                Field::Url => {
                    if url.is_some() {
                        return Err(DeError::duplicate_field("url"));
                    }

                    url = Some(map.next_value()?);
                }
                Field::SkuId => {
                    if sku_id.is_some() {
                        return Err(DeError::duplicate_field("sku_id"));
                    }

                    sku_id = map.next_value()?;
                }
                Field::Value => {
                    if value.is_some() {
                        return Err(DeError::duplicate_field("value"));
                    }

                    value = Some(map.next_value()?);
                }
            };
        }

        let kind = kind.ok_or_else(|| DeError::missing_field("type"))?;

        Ok(match kind {
            // Required fields:
            // - components
            ComponentType::ActionRow => {
                let components = components.ok_or_else(|| DeError::missing_field("components"))?;

                Self::Value::ActionRow(ActionRow { components })
            }
            // Required fields:
            // - style
            //
            // Optional fields:
            // - custom_id
            // - disabled
            // - emoji
            // - label
            // - url
            // - sku_id
            ComponentType::Button => {
                let style = style
                    .ok_or_else(|| DeError::missing_field("style"))?
                    .deserialize_into()
                    .map_err(DeserializerError::into_error)?;

                let custom_id = custom_id
                    .flatten()
                    .map(Value::deserialize_into)
                    .transpose()
                    .map_err(DeserializerError::into_error)?;

                Self::Value::Button(Button {
                    custom_id,
                    disabled: disabled.unwrap_or_default(),
                    emoji: emoji.unwrap_or_default(),
                    label: label.flatten(),
                    style,
                    url: url.unwrap_or_default(),
                    sku_id,
                })
            }
            // Required fields:
            // - custom_id
            // - options (if this is a text select menu)
            //
            // Optional fields:
            // - default_values
            // - disabled
            // - max_values
            // - min_values
            // - placeholder
            // - channel_types (if this is a channel select menu)
            kind @ (ComponentType::TextSelectMenu
            | ComponentType::UserSelectMenu
            | ComponentType::RoleSelectMenu
            | ComponentType::MentionableSelectMenu
            | ComponentType::ChannelSelectMenu) => {
                // Verify the individual variants' required fields
                if let ComponentType::TextSelectMenu = kind {
                    if options.is_none() {
                        return Err(DeError::missing_field("options"));
                    }
                }

                let custom_id = custom_id
                    .flatten()
                    .ok_or_else(|| DeError::missing_field("custom_id"))?
                    .deserialize_into()
                    .map_err(DeserializerError::into_error)?;

                Self::Value::SelectMenu(SelectMenu {
                    channel_types,
                    custom_id,
                    default_values,
                    disabled: disabled.unwrap_or_default(),
                    kind: match kind {
                        ComponentType::TextSelectMenu => SelectMenuType::Text,
                        ComponentType::UserSelectMenu => SelectMenuType::User,
                        ComponentType::RoleSelectMenu => SelectMenuType::Role,
                        ComponentType::MentionableSelectMenu => SelectMenuType::Mentionable,
                        ComponentType::ChannelSelectMenu => SelectMenuType::Channel,
                        // This branch is unreachable unless we add a new type above and forget to
                        // also add it here
                        _ => {
                            unreachable!("select menu component type is only partially implemented")
                        }
                    },
                    max_values: max_values.unwrap_or_default(),
                    min_values: min_values.unwrap_or_default(),
                    options,
                    placeholder: placeholder.unwrap_or_default(),
                })
            }
            // Required fields:
            // - custom_id
            // - label
            // - style
            //
            // Optional fields:
            // - max_length
            // - min_length
            // - placeholder
            // - required
            // - value
            ComponentType::TextInput => {
                let custom_id = custom_id
                    .flatten()
                    .ok_or_else(|| DeError::missing_field("custom_id"))?
                    .deserialize_into()
                    .map_err(DeserializerError::into_error)?;

                let label = label
                    .flatten()
                    .ok_or_else(|| DeError::missing_field("label"))?;

                let style = style
                    .ok_or_else(|| DeError::missing_field("style"))?
                    .deserialize_into()
                    .map_err(DeserializerError::into_error)?;

                Self::Value::TextInput(TextInput {
                    custom_id,
                    label,
                    max_length: max_length.unwrap_or_default(),
                    min_length: min_length.unwrap_or_default(),
                    placeholder: placeholder.unwrap_or_default(),
                    required: required.unwrap_or_default(),
                    style,
                    value: value.unwrap_or_default(),
                })
            }
            ComponentType::Unknown(unknown) => Self::Value::Unknown(unknown),
        })
    }
}

impl Serialize for Component {
    #[allow(clippy::too_many_lines)]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let len = match self {
            // Required fields:
            // - type
            // - components
            Component::ActionRow(_) => 2,
            // Required fields:
            // - type
            // - style
            //
            // Optional fields:
            // - custom_id
            // - disabled
            // - emoji
            // - label
            // - url
            // - sku_id
            Component::Button(button) => {
                2 + usize::from(button.custom_id.is_some())
                    + usize::from(button.disabled)
                    + usize::from(button.emoji.is_some())
                    + usize::from(button.label.is_some())
                    + usize::from(button.url.is_some())
                    + usize::from(button.sku_id.is_some())
            }
            // Required fields:
            // - custom_id
            // - options (for text select menus)
            // - type
            //
            // Optional fields:
            // - channel_types (for channel select menus)
            // - default_values
            // - disabled
            // - max_values
            // - min_values
            // - placeholder
            Component::SelectMenu(select_menu) => {
                // We ignore text menus that don't include the `options` field, as those are
                // detected later in the serialization process
                2 + usize::from(select_menu.channel_types.is_some())
                    + usize::from(select_menu.default_values.is_some())
                    + usize::from(select_menu.disabled)
                    + usize::from(select_menu.max_values.is_some())
                    + usize::from(select_menu.min_values.is_some())
                    + usize::from(select_menu.options.is_some())
                    + usize::from(select_menu.placeholder.is_some())
            }
            // Required fields:
            // - custom_id
            // - label
            // - style
            // - type
            //
            // Optional fields:
            // - max_length
            // - min_length
            // - placeholder
            // - required
            // - value
            Component::TextInput(text_input) => {
                4 + usize::from(text_input.max_length.is_some())
                    + usize::from(text_input.min_length.is_some())
                    + usize::from(text_input.placeholder.is_some())
                    + usize::from(text_input.required.is_some())
                    + usize::from(text_input.value.is_some())
            }
            // We are dropping fields here but nothing we can do about that for
            // the time being.
            Component::Unknown(_) => 1,
        };

        let mut state = serializer.serialize_struct("Component", len)?;

        match self {
            Component::ActionRow(action_row) => {
                state.serialize_field("type", &ComponentType::ActionRow)?;

                state.serialize_field("components", &action_row.components)?;
            }
            Component::Button(button) => {
                state.serialize_field("type", &ComponentType::Button)?;

                if button.custom_id.is_some() {
                    state.serialize_field("custom_id", &button.custom_id)?;
                }

                if button.disabled {
                    state.serialize_field("disabled", &button.disabled)?;
                }

                if button.emoji.is_some() {
                    state.serialize_field("emoji", &button.emoji)?;
                }

                if button.label.is_some() {
                    state.serialize_field("label", &button.label)?;
                }

                state.serialize_field("style", &button.style)?;

                if button.url.is_some() {
                    state.serialize_field("url", &button.url)?;
                }

                if button.sku_id.is_some() {
                    state.serialize_field("sku_id", &button.sku_id)?;
                }
            }
            Component::SelectMenu(select_menu) => {
                match &select_menu.kind {
                    SelectMenuType::Text => {
                        state.serialize_field("type", &ComponentType::TextSelectMenu)?;
                        state.serialize_field(
                            "options",
                            &select_menu.options.as_ref().ok_or(SerError::custom(
                                "required field \"option\" missing for text select menu",
                            ))?,
                        )?;
                    }
                    SelectMenuType::User => {
                        state.serialize_field("type", &ComponentType::UserSelectMenu)?;
                    }
                    SelectMenuType::Role => {
                        state.serialize_field("type", &ComponentType::RoleSelectMenu)?;
                    }
                    SelectMenuType::Mentionable => {
                        state.serialize_field("type", &ComponentType::MentionableSelectMenu)?;
                    }
                    SelectMenuType::Channel => {
                        state.serialize_field("type", &ComponentType::ChannelSelectMenu)?;
                        if let Some(channel_types) = &select_menu.channel_types {
                            state.serialize_field("channel_types", channel_types)?;
                        }
                    }
                }

                // Due to `custom_id` being required in some variants and
                // optional in others, serialize as an Option.
                state.serialize_field("custom_id", &Some(&select_menu.custom_id))?;

                if select_menu.default_values.is_some() {
                    state.serialize_field("default_values", &select_menu.default_values)?;
                }

                state.serialize_field("disabled", &select_menu.disabled)?;

                if select_menu.max_values.is_some() {
                    state.serialize_field("max_values", &select_menu.max_values)?;
                }

                if select_menu.min_values.is_some() {
                    state.serialize_field("min_values", &select_menu.min_values)?;
                }

                if select_menu.placeholder.is_some() {
                    state.serialize_field("placeholder", &select_menu.placeholder)?;
                }
            }
            Component::TextInput(text_input) => {
                state.serialize_field("type", &ComponentType::TextInput)?;

                // Due to `custom_id` and `label` being required in some
                // variants and optional in others, serialize as an Option.
                state.serialize_field("custom_id", &Some(&text_input.custom_id))?;
                state.serialize_field("label", &Some(&text_input.label))?;

                if text_input.max_length.is_some() {
                    state.serialize_field("max_length", &text_input.max_length)?;
                }

                if text_input.min_length.is_some() {
                    state.serialize_field("min_length", &text_input.min_length)?;
                }

                if text_input.placeholder.is_some() {
                    state.serialize_field("placeholder", &text_input.placeholder)?;
                }

                if text_input.required.is_some() {
                    state.serialize_field("required", &text_input.required)?;
                }

                state.serialize_field("style", &text_input.style)?;

                if text_input.value.is_some() {
                    state.serialize_field("value", &text_input.value)?;
                }
            }
            // We are not serializing all fields so this will fail to
            // deserialize. But it is all that can be done to avoid losing
            // incoming messages at this time.
            Component::Unknown(unknown) => {
                state.serialize_field("type", &ComponentType::Unknown(*unknown))?;
            }
        }

        state.end()
    }
}

#[cfg(test)]
mod tests {
    // Required due to the use of a unicode emoji in a constant.
    #![allow(clippy::non_ascii_literal)]

    use super::*;
    use crate::id::Id;
    use serde_test::Token;
    use static_assertions::assert_impl_all;

    assert_impl_all!(
        Component: From<ActionRow>,
        From<Button>,
        From<SelectMenu>,
        From<TextInput>
    );

    #[allow(clippy::too_many_lines)]
    #[test]
    fn component_full() {
        let component = Component::ActionRow(ActionRow {
            components: Vec::from([
                Component::Button(Button {
                    custom_id: Some("test custom id".into()),
                    disabled: true,
                    emoji: None,
                    label: Some("test label".into()),
                    style: ButtonStyle::Primary,
                    url: None,
                    sku_id: None,
                }),
                Component::SelectMenu(SelectMenu {
                    channel_types: None,
                    custom_id: "test custom id 2".into(),
                    default_values: None,
                    disabled: false,
                    kind: SelectMenuType::Text,
                    max_values: Some(25),
                    min_values: Some(5),
                    options: Some(Vec::from([SelectMenuOption {
                        label: "test option label".into(),
                        value: "test option value".into(),
                        description: Some("test description".into()),
                        emoji: None,
                        default: false,
                    }])),
                    placeholder: Some("test placeholder".into()),
                }),
            ]),
        });

        serde_test::assert_tokens(
            &component,
            &[
                Token::Struct {
                    name: "Component",
                    len: 2,
                },
                Token::Str("type"),
                Token::U8(ComponentType::ActionRow.into()),
                Token::Str("components"),
                Token::Seq { len: Some(2) },
                Token::Struct {
                    name: "Component",
                    len: 5,
                },
                Token::Str("type"),
                Token::U8(ComponentType::Button.into()),
                Token::Str("custom_id"),
                Token::Some,
                Token::Str("test custom id"),
                Token::Str("disabled"),
                Token::Bool(true),
                Token::Str("label"),
                Token::Some,
                Token::Str("test label"),
                Token::Str("style"),
                Token::U8(ButtonStyle::Primary.into()),
                Token::StructEnd,
                Token::Struct {
                    name: "Component",
                    len: 6,
                },
                Token::Str("type"),
                Token::U8(ComponentType::TextSelectMenu.into()),
                Token::Str("options"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "SelectMenuOption",
                    len: 4,
                },
                Token::Str("default"),
                Token::Bool(false),
                Token::Str("description"),
                Token::Some,
                Token::Str("test description"),
                Token::Str("label"),
                Token::Str("test option label"),
                Token::Str("value"),
                Token::Str("test option value"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("custom_id"),
                Token::Some,
                Token::Str("test custom id 2"),
                Token::Str("disabled"),
                Token::Bool(false),
                Token::Str("max_values"),
                Token::Some,
                Token::U8(25),
                Token::Str("min_values"),
                Token::Some,
                Token::U8(5),
                Token::Str("placeholder"),
                Token::Some,
                Token::Str("test placeholder"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn action_row() {
        let value = Component::ActionRow(ActionRow {
            components: Vec::from([Component::Button(Button {
                custom_id: Some("button-1".to_owned()),
                disabled: false,
                emoji: None,
                style: ButtonStyle::Primary,
                label: Some("Button".to_owned()),
                url: None,
                sku_id: None,
            })]),
        });

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Component",
                    len: 2,
                },
                Token::String("type"),
                Token::U8(ComponentType::ActionRow.into()),
                Token::String("components"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Component",
                    len: 4,
                },
                Token::String("type"),
                Token::U8(2),
                Token::String("custom_id"),
                Token::Some,
                Token::String("button-1"),
                Token::String("label"),
                Token::Some,
                Token::String("Button"),
                Token::String("style"),
                Token::U8(1),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn button() {
        // Free Palestine.
        //
        // Palestinian Flag.
        const FLAG: &str = "🇵🇸";

        let value = Component::Button(Button {
            custom_id: Some("test".to_owned()),
            disabled: false,
            emoji: Some(EmojiReactionType::Unicode {
                name: FLAG.to_owned(),
            }),
            label: Some("Test".to_owned()),
            style: ButtonStyle::Link,
            url: Some("https://twilight.rs".to_owned()),
            sku_id: None,
        });

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Component",
                    len: 6,
                },
                Token::String("type"),
                Token::U8(ComponentType::Button.into()),
                Token::String("custom_id"),
                Token::Some,
                Token::String("test"),
                Token::String("emoji"),
                Token::Some,
                Token::Struct {
                    name: "EmojiReactionType",
                    len: 1,
                },
                Token::String("name"),
                Token::String(FLAG),
                Token::StructEnd,
                Token::String("label"),
                Token::Some,
                Token::String("Test"),
                Token::String("style"),
                Token::U8(ButtonStyle::Link.into()),
                Token::String("url"),
                Token::Some,
                Token::String("https://twilight.rs"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn select_menu() {
        fn check_select(default_values: Option<Vec<(SelectDefaultValue, &'static str)>>) {
            let select_menu = Component::SelectMenu(SelectMenu {
                channel_types: None,
                custom_id: String::from("my_select"),
                default_values: default_values
                    .clone()
                    .map(|values| values.into_iter().map(|pair| pair.0).collect()),
                disabled: false,
                kind: SelectMenuType::User,
                max_values: None,
                min_values: None,
                options: None,
                placeholder: None,
            });
            let mut tokens = vec![
                Token::Struct {
                    name: "Component",
                    len: 2 + usize::from(default_values.is_some()),
                },
                Token::String("type"),
                Token::U8(ComponentType::UserSelectMenu.into()),
                Token::Str("custom_id"),
                Token::Some,
                Token::Str("my_select"),
            ];
            if let Some(default_values) = default_values {
                tokens.extend_from_slice(&[
                    Token::Str("default_values"),
                    Token::Some,
                    Token::Seq {
                        len: Some(default_values.len()),
                    },
                ]);
                for (_, id) in default_values {
                    tokens.extend_from_slice(&[
                        Token::Struct {
                            name: "SelectDefaultValue",
                            len: 2,
                        },
                        Token::Str("type"),
                        Token::UnitVariant {
                            name: "SelectDefaultValue",
                            variant: "user",
                        },
                        Token::Str("id"),
                        Token::NewtypeStruct { name: "Id" },
                        Token::Str(id),
                        Token::StructEnd,
                    ])
                }
                tokens.push(Token::SeqEnd);
            }
            tokens.extend_from_slice(&[
                Token::Str("disabled"),
                Token::Bool(false),
                Token::StructEnd,
            ]);
            serde_test::assert_tokens(&select_menu, &tokens);
        }

        check_select(None);
        check_select(Some(vec![(
            SelectDefaultValue::User(Id::new(1234)),
            "1234",
        )]));
        check_select(Some(vec![
            (SelectDefaultValue::User(Id::new(1234)), "1234"),
            (SelectDefaultValue::User(Id::new(5432)), "5432"),
        ]));
    }

    #[test]
    fn text_input() {
        let value = Component::TextInput(TextInput {
            custom_id: "test".to_owned(),
            label: "The label".to_owned(),
            max_length: Some(100),
            min_length: Some(1),
            placeholder: Some("Taking this place".to_owned()),
            required: Some(true),
            style: TextInputStyle::Short,
            value: Some("Hello World!".to_owned()),
        });

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Component",
                    len: 9,
                },
                Token::String("type"),
                Token::U8(ComponentType::TextInput.into()),
                Token::String("custom_id"),
                Token::Some,
                Token::String("test"),
                Token::String("label"),
                Token::Some,
                Token::String("The label"),
                Token::String("max_length"),
                Token::Some,
                Token::U16(100),
                Token::String("min_length"),
                Token::Some,
                Token::U16(1),
                Token::String("placeholder"),
                Token::Some,
                Token::String("Taking this place"),
                Token::String("required"),
                Token::Some,
                Token::Bool(true),
                Token::String("style"),
                Token::U8(TextInputStyle::Short as u8),
                Token::String("value"),
                Token::Some,
                Token::String("Hello World!"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn premium_button() {
        let value = Component::Button(Button {
            custom_id: None,
            disabled: false,
            emoji: None,
            label: None,
            style: ButtonStyle::Premium,
            url: None,
            sku_id: Some(Id::new(114_941_315_417_899_012)),
        });

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Component",
                    len: 3,
                },
                Token::String("type"),
                Token::U8(ComponentType::Button.into()),
                Token::String("style"),
                Token::U8(ButtonStyle::Premium.into()),
                Token::String("sku_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
                Token::StructEnd,
            ],
        );
    }
}
