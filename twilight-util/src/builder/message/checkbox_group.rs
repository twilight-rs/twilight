use twilight_model::channel::message::component::{CheckboxGroup, CheckboxGroupOption};

/// Create a checkbox group from a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a checkbox group"]
pub struct CheckboxGroupBuilder(CheckboxGroup);
impl CheckboxGroupBuilder {
    /// Create a new checkbox group builder
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self(CheckboxGroup {
            id: None,
            custom_id: custom_id.into(),
            options: vec![],
            min_values: None,
            max_values: None,
            required: None,
        })
    }

    /// Set the identifier of this checkbox group.
    pub const fn id(mut self, id: i32) -> Self {
        self.0.id.replace(id);

        self
    }

    /// Add an option to this checkbox group
    pub fn option(mut self, option: impl Into<CheckboxGroupOption>) -> Self {
        self.0.options.push(option.into());

        self
    }

    /// Set the minimum values for this checkbox group
    pub const fn min_values(mut self, min_values: u8) -> Self {
        self.0.min_values.replace(min_values);

        self
    }

    /// Set the maximum values for this checkbox group
    pub const fn max_values(mut self, max_values: u8) -> Self {
        self.0.max_values.replace(max_values);

        self
    }

    /// Set if this checkbox group is required or not
    pub const fn required(mut self, required: bool) -> Self {
        self.0.required.replace(required);

        self
    }

    /// Build into a checkbox group
    pub fn build(self) -> CheckboxGroup {
        self.0
    }
}

/// Create a checkbox group option with a builder
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a checkbox group option"]
pub struct CheckboxGroupOptionBuilder(CheckboxGroupOption);

impl CheckboxGroupOptionBuilder {
    /// Create a new checkbox group option builder
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self(CheckboxGroupOption {
            value: value.into(),
            label: label.into(),
            description: None,
            default: None,
        })
    }

    /// Set if this option is selected by default
    pub const fn default(mut self, default: bool) -> Self {
        self.0.default.replace(default);

        self
    }

    /// Set the description of this option
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.0.description.replace(description.into());

        self
    }

    /// Build into a checkbox group option
    pub fn build(self) -> CheckboxGroupOption {
        self.0
    }
}
