use twilight_model::channel::message::{component::ActionRow, Component};

/// Create an action row from a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an ActionRow"]
pub struct ActionRowBuilder(ActionRow);

impl ActionRowBuilder {
    /// Create a new action row builder.
    pub fn new() -> Self {
        Self(ActionRow {
            id: None,
            components: Vec::new(),
        })
    }

    /// Add a component.
    pub fn component(mut self, component: impl Into<Component>) -> Self {
        self.0.components.push(component.into());

        self
    }

    /// The component's identifier.
    pub fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    /// Build into an action row.
    pub fn build(self) -> ActionRow {
        self.0
    }
}

impl From<ActionRowBuilder> for ActionRow {
    fn from(builder: ActionRowBuilder) -> Self {
        builder.build()
    }
}
