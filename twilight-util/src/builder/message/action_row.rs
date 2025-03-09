use twilight_model::channel::message::{component::ActionRow, Component};

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into an ActionRow"]
pub struct ActionRowBuilder(ActionRow);

impl ActionRowBuilder {
    pub fn new() -> Self {
        Self(ActionRow {
            id: None,
            components: Vec::new(),
        })
    }

    pub fn component(mut self, component: impl Into<Component>) -> Self {
        self.0.components.push(component.into());

        self
    }

    pub fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    pub fn build(self) -> ActionRow {
        self.0
    }
}

impl Default for ActionRowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ActionRow> for ActionRowBuilder {
    fn from(action_row: ActionRow) -> Self {
        Self(action_row)
    }
}
