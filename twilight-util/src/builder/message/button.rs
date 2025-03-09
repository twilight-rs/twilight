use twilight_model::channel::message::{
    component::{Button, ButtonStyle},
    EmojiReactionType,
};
use twilight_validate::component::{button, ComponentValidationError};

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a button"]
pub struct ButtonBuilder(Button);

impl ButtonBuilder {
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

    pub fn custom_id(mut self, custom_id: String) -> Self {
        self.0.custom_id.replace(custom_id);

        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;

        self
    }

    pub fn emoji(mut self, emoji: EmojiReactionType) -> Self {
        self.0.emoji.replace(emoji);

        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.0.url.replace(url);

        self
    }

    pub fn validate(self) -> Result<Self, ComponentValidationError> {
        button(&self.0)?;

        Ok(self)
    }

    pub fn build(self) -> Button {
        self.0
    }
}
