use twilight_model::channel::message::component::TextDisplay;

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a text display"]
pub struct TextDisplayBuilder(TextDisplay);

impl TextDisplayBuilder {
    pub fn new(content: String) -> Self {
        Self(TextDisplay {
            content: content.into(),
            id: None,
        })
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.0.content = content.into();

        self
    }

    pub fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    pub fn build(self) -> TextDisplay {
        self.0
    }
}

impl From<TextDisplay> for TextDisplayBuilder {
    fn from(text_display: TextDisplay) -> Self {
        Self(text_display)
    }
}
