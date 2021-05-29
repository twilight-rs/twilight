use crate::component::Component;
use crate::id::EmojiId;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Hash)]
pub struct Button {
    pub style: ButtonStyle,
    pub emoji: Option<PartialEmoji>,
    pub label: Option<String>,
    pub custom_id: Option<String>,
    pub url: Option<String>,
    pub disabled: bool,
}

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Hash)]
pub struct PartialEmoji {
    pub id: Option<EmojiId>,
    pub name: String,
    pub animated: bool,
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

pub enum ButtonBuilderError {
    MissingLabelOrEmoji,
    LabelTooLong,
    CustomIdTooLong,
    LinkTooLong,
}

impl ButtonBuilder {
    /// Makes a new button that links to an external site
    /// You will also need to set the emoji or label as well (or both)
    pub fn new_link(url: impl Into<String>) -> Result<Self, ButtonBuilderError> {
        let url = url.into();
        if url.len() > 512 {
            Err(ButtonBuilderError::LinkTooLong)
        } else {
            Ok(Self {
                style: ButtonStyle::Link,
                emoji: None,
                label: None,
                custom_id: None,
                url: Some(url.into()),
                disabled: false,
            })
        }
    }

    /// Make a new button with a specific style and id
    /// You will also need to set the emoji or label as well (or both)
    pub fn new_button(
        style: ButtonStyle,
        custom_id: impl Into<String>,
    ) -> Result<Self, ButtonBuilderError> {
        let custom_id = custom_id.into();
        if custom_id.len() > 100 {
            Err(ButtonBuilderError::CustomIdTooLong)
        } else {
            Ok(Self {
                style,
                emoji: None,
                label: None,
                custom_id: Some(custom_id),
                url: None,
                disabled: false,
            })
        }
    }

    /// Set an emoji for the button
    pub fn with_emoji(mut self, emoji: PartialEmoji) -> Self {
        self.emoji = Some(emoji);
        self
    }

    /// Set the button label text
    pub fn with_label(mut self, label: impl Into<String>) -> Result<Self, ButtonBuilderError> {
        let label = label.into();
        if label.len() > 80 {
            Err(ButtonBuilderError::LabelTooLong)
        } else {
            self.label = Some(label);
            Ok(self)
        }
    }

    /// Disable the button
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Finish the builder and assemble into the component
    ///
    /// # Errors
    /// Emits a
    pub fn build(self) -> Result<Component, ButtonBuilderError> {
        if self.label.is_none() && self.emoji.is_none() {
            Err(ButtonBuilderError::MissingLabelOrEmoji)
        } else {
            Ok(Component::Button(Button {
                style: self.style,
                emoji: self.emoji,
                label: self.label,
                custom_id: self.custom_id,
                url: self.url,
                disabled: self.disabled,
            }))
        }
    }
}
