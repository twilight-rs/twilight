use twilight_model::channel::message::component::TextDisplay;

/// Create a text display with a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a text display"]
pub struct TextDisplayBuilder(TextDisplay);

impl TextDisplayBuilder {
    /// Create a new text display builder.
    pub fn new(content: String) -> Self {
        Self(TextDisplay {
            content: content.into(),
            id: None,
        })
    }

    /// Set the content of this text display.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.0.content = content.into();

        self
    }

    /// Set the identifier of this text display.
    pub fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    /// Build into a text display.
    pub fn build(self) -> TextDisplay {
        self.0
    }
}

impl From<TextDisplayBuilder> for TextDisplay {
    fn from(builder: TextDisplayBuilder) -> Self {
        builder.build()
    }
}
