use twilight_model::channel::message::component::Checkbox;

/// Create a checkbox from a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a checkbox"]
pub struct CheckboxBuilder(Checkbox);
impl CheckboxBuilder {
    /// Create a new checkbox builder
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self(Checkbox {
            id: None,
            custom_id: custom_id.into(),
            default: None,
        })
    }

    /// Set the identifier of this checkbox.
    pub const fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    /// Set if this checkbox is checked by default
    pub const fn default(mut self, default: bool) -> Self {
        self.0.default.replace(default);

        self
    }

    /// Build into a checkbox
    pub fn build(self) -> Checkbox {
        self.0
    }
}
