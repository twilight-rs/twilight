use twilight_model::channel::message::{component::Container, Component};

/// Create a container with a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a container"]
pub struct ContainerBuilder(Container);

impl ContainerBuilder {
    /// Create a new container builder.
    pub const fn new() -> Self {
        Self(Container {
            accent_color: None,
            components: Vec::new(),
            id: None,
            spoiler: None,
        })
    }

    /// Set the accent color of this container.
    pub fn accent_color(mut self, accent_color: u32) -> Self {
        self.0.accent_color.replace(accent_color);

        self
    }

    /// Add a component to this container.
    pub fn component(mut self, component: impl Into<Component>) -> Self {
        self.0.components.push(component.into());

        self
    }

    /// Set the identifier of this container.
    pub fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    /// Specify whether this container is spoilered.
    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.0.spoiler.replace(spoiler);

        self
    }

    /// Build into a container.
    pub fn build(self) -> Container {
        self.0
    }
}

impl Default for ContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ContainerBuilder> for Container {
    fn from(builder: ContainerBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ContainerBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Container: From<ContainerBuilder>);

    #[test]
    fn builder() {
        let expected = Container {
            accent_color: None,
            components: Vec::new(),
            id: None,
            spoiler: None,
        };

        let actual = ContainerBuilder::new().build();

        assert_eq!(expected, actual);
    }
}
