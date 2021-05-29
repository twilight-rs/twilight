mod button;

pub use button::{Button, ButtonStyle, PartialEmoji};
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
                return Err(ComponentError::InvalidSubComponent)
            }
        }
        if components.len() > 5 {
            return Err(ComponentError::TooManyButtons);
        }
        Ok(Component::ActionRow(components))
    }
}

/// Helper struct for assembling  the Button struct
pub struct ButtonBuilder {
    pub(crate) style: ButtonStyle,
    pub(crate) emoji: Option<PartialEmoji>,
    pub(crate) label: Option<String>,
    pub(crate) custom_id: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) disabled: bool,
}

impl ButtonBuilder {
    /// Makes a new button that links to an external site
    /// You will also need to set the emoji or label as well (or both)
    pub fn new_link(url: impl Into<String>) -> Self {
        Self {
            style: ButtonStyle::Link,
            emoji: None,
            label: None,
            custom_id: None,
            url: Some(url.into()),
            disabled: false,
        }
    }

    /// Make a new button with a specific style and id
    /// You will also need to set the emoji or label as well (or both)
    pub fn new_button(style: ButtonStyle, custom_id: impl Into<String>) -> Self {
        Self {
            style,
            emoji: None,
            label: None,
            custom_id: Some(custom_id.into()),
            url: None,
            disabled: false,
        }
    }

    /// Set an emoji for the button
    pub fn with_emoji(mut self, emoji: PartialEmoji) -> Self {
        self.emoji = Some(emoji);
        self
    }

    /// Set the button label text
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Disable the button
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Finish the builder and assemble into the component
    pub fn build(self) -> Component {
        Component::Button(Button {
            style: self.style,
            emoji: self.emoji,
            label: self.label,
            custom_id: self.custom_id,
            url: self.url,
            disabled: self.disabled,
        })
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
