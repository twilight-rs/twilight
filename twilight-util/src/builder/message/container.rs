use twilight_model::channel::message::{component::Container, Component};

/// Builder interface for creating a [`Container`] struct.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a container"]
pub struct ContainerBuilder(Container);

impl ContainerBuilder {
    pub fn new() -> Self {
        Self(Container {
            accent_color: None,
            components: Vec::new(),
            id: None,
            spoiler: None,
        })
    }

    pub fn accent_color(mut self, accent_color: u32) -> Self {
        self.0.accent_color.replace(accent_color);

        self
    }

    pub fn component(mut self, component: impl Into<Component>) -> Self {
        self.0.components.push(component.into());

        self
    }

    pub fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.0.spoiler.replace(spoiler);

        self
    }

    pub fn build(self) -> Container {
        self.0
    }
}

impl Default for ContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Container> for ContainerBuilder {
    fn from(container: Container) -> Self {
        Self(container)
    }
}
