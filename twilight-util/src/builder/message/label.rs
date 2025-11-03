use twilight_model::channel::message::Component;
use twilight_model::channel::message::component::Label;

/// Create a label from a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a label"]
pub struct LabelBuilder(Label);

impl LabelBuilder {
    /// Create a new label builder.
    pub fn new(label: impl Into<String>, component: Component) -> Self {
        Self(Label {
            id: None,
            label: label.into(),
            description: None,
            component: Box::new(component),
        })
    }

    /// Set the identifier of this label.
    pub const fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    /// Set the description of this label.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.0.description.replace(description.into());

        self
    }

    /// Build into a label.
    pub fn build(self) -> Label {
        self.0
    }
}

impl From<LabelBuilder> for Label {
    fn from(builder: LabelBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(LabelBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Label: From<LabelBuilder>);

    #[test]
    fn builder() {
        let expected = Label {
            id: Some(42),
            label: "Label label".to_string(),
            description: Some("Label description".to_string()),
            component: Box::new(Component::Unknown(43)),
        };

        let actual = LabelBuilder::new("Label label", Component::Unknown(43))
            .description("Label description")
            .id(42)
            .build();

        assert_eq!(actual, expected);
    }
}
