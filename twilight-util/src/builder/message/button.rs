use twilight_model::channel::message::{
    component::{Button, ButtonStyle},
    EmojiReactionType,
};
use twilight_validate::component::{button, ComponentValidationError};

/// Create a button from a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a button"]
pub struct ButtonBuilder(Button);

impl ButtonBuilder {
    /// Create a new button builder.
    pub fn new(style: ButtonStyle) -> Self {
        Self(Button {
            custom_id: None,
            disabled: false,
            emoji: None,
            label: None,
            style,
            url: None,
            id: None,
            sku_id: None,
        })
    }

    /// The custom identifier of the button.
    pub fn custom_id(mut self, custom_id: String) -> Self {
        self.0.custom_id.replace(custom_id);

        self
    }

    /// Specify whether this button is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;

        self
    }

    /// Add an emoji to this button.
    pub fn emoji(mut self, emoji: EmojiReactionType) -> Self {
        self.0.emoji.replace(emoji);

        self
    }

    /// Sets the URL for this button.
    pub fn url(mut self, url: String) -> Self {
        self.0.url.replace(url);

        self
    }

    /// Validate the fields in this builder.
    pub fn validate(self) -> Result<Self, ComponentValidationError> {
        button(&self.0)?;

        Ok(self)
    }

    /// Build into a button.
    pub fn build(self) -> Button {
        self.0
    }
}

impl From<ButtonBuilder> for Button {
    fn from(builder: ButtonBuilder) -> Self {
        builder.build()
    }
}
