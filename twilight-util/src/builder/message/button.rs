use twilight_model::channel::message::{
    EmojiReactionType,
    component::{Button, ButtonStyle},
};
use twilight_validate::component::{ComponentValidationError, button};

/// Create a button from a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a button"]
pub struct ButtonBuilder(Button);

impl ButtonBuilder {
    /// Create a new button builder.
    pub const fn new(style: ButtonStyle) -> Self {
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

    /// Sets the custom identifier of the button.
    pub fn custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.0.custom_id.replace(custom_id.into());

        self
    }

    /// Specify whether this button is disabled.
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;

        self
    }

    /// Add an emoji to this button.
    pub fn emoji(mut self, emoji: EmojiReactionType) -> Self {
        self.0.emoji.replace(emoji);

        self
    }

    /// Sets the URL for this button.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.0.url.replace(url.into());

        self
    }

    /// Sets the label for this button.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.0.label.replace(label.into());

        self
    }

    /// Validate the fields in this builder.
    ///
    /// # Errors
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

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ButtonBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Button: From<ButtonBuilder>);

    #[test]
    fn builder() {
        let expected = Button {
            custom_id: None,
            disabled: false,
            emoji: None,
            label: None,
            style: ButtonStyle::Primary,
            url: None,
            id: None,
            sku_id: None,
        };

        let actual = ButtonBuilder::new(ButtonStyle::Primary).build();

        assert_eq!(actual, expected);
    }
}
