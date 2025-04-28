use twilight_model::channel::message::{component::Section, Component};

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into section"]
pub struct SectionBuilder(Section);

impl SectionBuilder {
    pub fn new(accessory: impl Into<Component>) -> Self {
        Self(Section {
            components: Vec::new(),
            id: None,
            accessory: Box::new(accessory.into()),
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

    pub fn accessory(mut self, accessory: impl Into<Component>) -> Self {
        self.0.accessory = Box::new(accessory.into());

        self
    }

    pub fn build(self) -> Section {
        self.0
    }
}
