use std::fmt::{Formatter, Result as FmtResult};

use serde::{
    de::{Error as DeError, IgnoredAny, MapAccess, Unexpected, Visitor},
    ser::{SerializeStruct, Serializer},
    Deserialize, Deserializer, Serialize,
};

use super::ComponentType;
use crate::channel::ReactionType;

/// Dropdown-style interactive components that render on messages.
///
/// Refer to [Discord Docs/Message Components] for additional information.
///
/// [Discord Docs/Message Components]: https://discord.com/developers/docs/interactions/message-components#select-menus
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SelectMenu {
    /// Developer defined identifier.
    pub custom_id: String,
    /// Whether the select menu is disabled.
    ///
    /// Defaults to `false`.
    pub disabled: bool,
    /// Maximum number of options that may be chosen.
    pub max_values: Option<u8>,
    /// Minimum number of options that must be chosen.
    pub min_values: Option<u8>,
    /// List of available choices.
    pub options: Vec<SelectMenuOption>,
    /// Custom placeholder text if no option is selected.
    pub placeholder: Option<String>,
}

/// Dropdown options that are part of [`SelectMenu`].
///
/// Refer to [Discord Docs/Message Components] for additional information.
///
/// [Discord Docs/Message Components]: https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SelectMenuOption {
    /// Whether the option will be selected by default.
    #[serde(default)]
    pub default: bool,
    /// Additional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<ReactionType>,
    /// User-facing name.
    pub label: String,
    /// Developer defined value.
    pub value: String,
}

impl<'de> Deserialize<'de> for SelectMenu {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(SelectMenuVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum SelectMenuField {
    Type,
    CustomId,
    Disabled,
    Options,
    Placeholder,
    MinValues,
    MaxValues,
}

struct SelectMenuVisitor;

impl<'de> Visitor<'de> for SelectMenuVisitor {
    type Value = SelectMenu;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct SelectMenu")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut kind: Option<ComponentType> = None;
        let mut custom_id: Option<String> = None;
        let mut disabled: Option<bool> = None;
        let mut options: Option<Vec<SelectMenuOption>> = None;
        let mut placeholder: Option<String> = None;
        let mut max_values: Option<u8> = None;
        let mut min_values: Option<u8> = None;

        #[cfg(feature = "tracing")]
        let span = tracing::trace_span!("deserializing select menu");
        #[cfg(feature = "tracing")]
        let _span_enter = span.enter();

        loop {
            #[cfg(feature = "tracing")]
            let span_child = tracing::trace_span!("iterating over select menu");
            #[cfg(feature = "tracing")]
            let _span_child_enter = span_child.enter();

            let key = match map.next_key() {
                Ok(Some(key)) => {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(?key, "found key");

                    key
                }
                Ok(None) => break,
                Err(why) => {
                    // Encountered when we run into an unknown key.
                    map.next_value::<IgnoredAny>()?;

                    #[cfg(feature = "tracing")]
                    tracing::trace!("ran into an unknown key: {:?}", why);

                    continue;
                }
            };

            match key {
                SelectMenuField::Type => {
                    if kind.is_some() {
                        return Err(DeError::duplicate_field("type"));
                    }

                    let value: ComponentType = map.next_value()?;

                    if value != ComponentType::SelectMenu {
                        return Err(DeError::invalid_value(
                            Unexpected::Unsigned(value as u64),
                            &"a select menu type",
                        ));
                    }

                    kind = Some(value)
                }
                SelectMenuField::CustomId => {
                    if custom_id.is_some() {
                        return Err(DeError::duplicate_field("custom_id"));
                    }

                    custom_id = Some(map.next_value()?);
                }
                SelectMenuField::Disabled => {
                    if disabled.is_some() {
                        return Err(DeError::duplicate_field("disabled"));
                    }

                    disabled = Some(map.next_value()?);
                }
                SelectMenuField::MaxValues => {
                    if max_values.is_some() {
                        return Err(DeError::duplicate_field("max_values"));
                    }

                    max_values = Some(map.next_value()?);
                }
                SelectMenuField::MinValues => {
                    if min_values.is_some() {
                        return Err(DeError::duplicate_field("min_values"));
                    }

                    min_values = Some(map.next_value()?);
                }
                SelectMenuField::Options => {
                    if options.is_some() {
                        return Err(DeError::duplicate_field("options"));
                    }

                    options = Some(map.next_value()?);
                }
                SelectMenuField::Placeholder => {
                    if placeholder.is_some() {
                        return Err(DeError::duplicate_field("placeholder"));
                    }

                    placeholder = Some(map.next_value()?);
                }
            }
        }

        if kind.is_none() {
            return Err(DeError::missing_field("kind"));
        }

        let custom_id = custom_id.ok_or_else(|| DeError::missing_field("custom_id"))?;
        // defaults to false
        let disabled = disabled.unwrap_or(false);
        let options = options.ok_or_else(|| DeError::missing_field("options"))?;

        #[cfg(feature = "tracing")]
        tracing::trace!(?custom_id, %disabled, ?options, ?kind, "all fields of SelectMenu exist");

        Ok(SelectMenu {
            custom_id,
            disabled,
            max_values,
            min_values,
            options,
            placeholder,
        })
    }
}

impl Serialize for SelectMenu {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Base of 4 to account for the fields that are always present:
        //
        // - `custom_id`
        // - `disabled`
        // - `options`
        // - `type`
        let field_count = 4
            + usize::from(self.placeholder.is_some())
            + usize::from(self.min_values.is_some())
            + usize::from(self.max_values.is_some());
        let mut state = serializer.serialize_struct("SelectMenu", field_count)?;

        state.serialize_field("custom_id", &self.custom_id)?;

        state.serialize_field("disabled", &self.disabled)?;

        if self.max_values.is_some() {
            state.serialize_field("max_values", &self.max_values)?;
        }

        if self.min_values.is_some() {
            state.serialize_field("min_values", &self.min_values)?;
        }

        state.serialize_field("options", &self.options)?;

        if self.placeholder.is_some() {
            state.serialize_field("placeholder", &self.placeholder)?;
        }

        state.serialize_field("type", &ComponentType::SelectMenu)?;

        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::{SelectMenu, SelectMenuOption};
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(SelectMenuOption: default, description, emoji, label, value);
    assert_fields!(
        SelectMenu: custom_id,
        disabled,
        max_values,
        min_values,
        options,
        placeholder
    );
    assert_impl_all!(
        SelectMenuOption: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
    assert_impl_all!(
        SelectMenu: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
}
