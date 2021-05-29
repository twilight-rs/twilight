mod button;


pub use button::{Button, ButtonBuilder, ButtonStyle, PartialEmoji};


use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
#[non_exhaustive]
pub enum ComponentError {
    InvalidSubComponent,
    TooManyButtons,
}

impl Display for ComponentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ComponentError::InvalidSubComponent => f.write_str("You can not nest ActionRows"),
            ComponentError::TooManyButtons => {
                f.write_str("ActionRows can only have up to 5 buttons")
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Component {
    ActionRow(Vec<Component>),
    Button(Button),
}

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum ComponentType {
    ActionRow = 1,
    Button = 2,
}

impl Component {
    pub(crate) fn get_kind(&self) -> ComponentType {
        match self {
            Component::ActionRow(_) => ComponentType::ActionRow,
            Component::Button(_) => ComponentType::Button,
        }
    }

    fn field_count(&self) -> usize {
        match self {
            Component::ActionRow(_) => 1,
            Component::Button(button) => {
                // always present:
                // - type
                // - style
                // - either custom_id or url (mutally exclusive)
                3 + if button.label.is_some() { 1 } else { 0 }
                    + if button.emoji.is_some() { 1 } else { 0 }
                    + if button.url.is_some() { 1 } else { 0 }
                    + if button.disabled { 1 } else { 0 }
            }
        }
    }

    /// Create a new action row containing subcomponent, this can not contain other
    ///
    /// # Errors
    /// Returns a `ComponentError` if this contains too many subcomponents (more then 5 buttons)
    /// or other action rows
    pub fn create_action_row(components: Vec<Component>) -> Result<Self, ComponentError> {
        for component in &components {
            if let Component::ActionRow(_) = component {
                return Err(ComponentError::InvalidSubComponent);
            }
        }
        if components.len() > 5 {
            return Err(ComponentError::TooManyButtons);
        }
        Ok(Component::ActionRow(components))
    }
}

impl Serialize for Component {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Component", self.field_count())?;
        state.serialize_field("type", &self.get_kind())?;
        match self {
            Component::ActionRow(components) => {
                state.serialize_field("components", components)?;
            }
            Component::Button(button) => {
                state.serialize_field("style", &button.style)?;
                if button.label.is_some() {
                    state.serialize_field("label", button.label.as_ref().unwrap())?;
                }

                if button.emoji.is_some() {
                    state.serialize_field("emoji", button.emoji.as_ref().unwrap())?;
                }

                if button.custom_id.is_some() {
                    state.serialize_field("custom_id", button.custom_id.as_ref().unwrap())?;
                }

                if button.url.is_some() {
                    state.serialize_field("url", button.url.as_ref().unwrap())?;
                }

                if button.disabled {
                    state.serialize_field("disabled", &button.disabled)?;
                }
            }
        }

        state.end()
    }
}
