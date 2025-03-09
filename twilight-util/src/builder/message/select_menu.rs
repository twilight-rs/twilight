use twilight_model::channel::{
    message::component::{SelectDefaultValue, SelectMenu, SelectMenuOption, SelectMenuType},
    ChannelType,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a select menu"]
pub struct SelectMenuBuilder(SelectMenu);

impl SelectMenuBuilder {
    pub fn new(custom_id: String, kind: SelectMenuType) -> Self {
        Self(SelectMenu {
            custom_id,
            disabled: false,
            max_values: None,
            min_values: None,
            options: None,
            placeholder: None,
            id: None,
            channel_types: None,
            default_values: None,
            kind,
        })
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;

        self
    }

    pub fn max_values(mut self, max_values: u8) -> Self {
        self.0.max_values.replace(max_values);

        self
    }

    pub fn min_values(mut self, min_values: u8) -> Self {
        self.0.min_values.replace(min_values);

        self
    }

    pub fn option(mut self, option: impl Into<SelectMenuOption>) -> Self {
        if self.0.options.is_none() {
            self.0.options.replace(Vec::new());
        }

        self.0.options.as_mut().unwrap().push(option.into());

        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.0.placeholder.replace(placeholder.into());

        self
    }

    pub fn id(mut self, id: impl Into<i32>) -> Self {
        self.0.id.replace(id.into());

        self
    }

    pub fn channel_types(mut self, channel_types: Vec<ChannelType>) -> Self {
        self.0.channel_types.replace(channel_types);

        self
    }

    pub fn default_values(mut self, default_values: Vec<SelectDefaultValue>) -> Self {
        self.0.default_values.replace(default_values);

        self
    }

    pub fn build(self) -> SelectMenu {
        self.0
    }
}

impl From<SelectMenu> for SelectMenuBuilder {
    fn from(select_menu: SelectMenu) -> Self {
        Self(select_menu)
    }
}
