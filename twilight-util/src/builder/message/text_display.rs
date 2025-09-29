use twilight_model::channel::message::component::TextDisplay;

/// Create a text display with a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a text display"]
pub struct TextDisplayBuilder(TextDisplay);

impl TextDisplayBuilder {
    /// Create a new text display builder.
    pub fn new(content: impl Into<String>) -> Self {
        Self(TextDisplay {
            content: content.into(),
            id: None,
        })
    }

    /// Set the identifier of this text display.
    pub const fn id(mut self, id: i32) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(TextDisplayBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(TextDisplay: From<TextDisplayBuilder>);

    #[test]
    fn builder() {
        let expected = TextDisplay {
            content: "Lorem ipsum".to_string(),
            id: None,
        };

        let actual = TextDisplayBuilder::new("Lorem ipsum").build();

        assert_eq!(actual, expected);
    }
}
