use twilight_model::channel::{
    message::{
        component::{SelectDefaultValue, SelectMenu, SelectMenuOption, SelectMenuType},
        EmojiReactionType,
    },
    ChannelType,
};

/// Create a select menu option with a builder
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a select menu option"]
pub struct SelectMenuOptionBuilder(SelectMenuOption);

impl SelectMenuOptionBuilder {
    /// Create a new select menu option builder.
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self(SelectMenuOption {
            default: false,
            description: None,
            emoji: None,
            label: label.into(),
            value: value.into(),
        })
    }

    /// Set whether this option is the default
    pub const fn default(mut self, default: bool) -> Self {
        self.0.default = default;

        self
    }

    /// Set the associated emoji
    pub fn emoji(mut self, emoji: EmojiReactionType) -> Self {
        self.0.emoji.replace(emoji);

        self
    }

    /// Set the description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.0.description.replace(description.into());

        self
    }

    /// Build into a select menu option
    pub fn build(self) -> SelectMenuOption {
        self.0
    }
}

impl From<SelectMenuOptionBuilder> for SelectMenuOption {
    fn from(builder: SelectMenuOptionBuilder) -> Self {
        builder.build()
    }
}

/// Create a select menu with a builder.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "must be built into a select menu"]
pub struct SelectMenuBuilder(SelectMenu);

impl SelectMenuBuilder {
    /// Create a new select menu builder.
    pub fn new(custom_id: impl Into<String>, kind: SelectMenuType) -> Self {
        Self(SelectMenu {
            custom_id: custom_id.into(),
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

    /// Set whether this select menu is disabled.
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;

        self
    }

    /// Set the max values of this select menu.
    pub fn max_values(mut self, max_values: u8) -> Self {
        self.0.max_values.replace(max_values);

        self
    }

    /// Set the min values of this select menu.
    pub fn min_values(mut self, min_values: u8) -> Self {
        self.0.min_values.replace(min_values);

        self
    }

    /// Add an option to this select menu.
    #[allow(clippy::missing-panics-doc)] // this does not panic; unwrap is never called on None
    pub fn option(mut self, option: impl Into<SelectMenuOption>) -> Self {
        if self.0.options.is_none() {
            self.0.options.replace(Vec::new());
        }

        self.0.options.as_mut().unwrap().push(option.into());

        self
    }

    /// Set the placeholder for this select menu.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.0.placeholder.replace(placeholder.into());

        self
    }

    /// Set the identifier of this select menu.
    pub fn id(mut self, id: impl Into<i32>) -> Self {
        self.0.id.replace(id.into());

        self
    }

    /// Set the channel types of this select menu.
    pub fn channel_types(mut self, channel_types: Vec<ChannelType>) -> Self {
        self.0.channel_types.replace(channel_types);

        self
    }

    /// Set the default values of this select menu.
    pub fn default_values(mut self, default_values: Vec<SelectDefaultValue>) -> Self {
        self.0.default_values.replace(default_values);

        self
    }

    /// Build into a select menu,
    pub fn build(self) -> SelectMenu {
        self.0
    }
}

impl From<SelectMenuBuilder> for SelectMenu {
    fn from(builder: SelectMenuBuilder) -> Self {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(SelectMenuBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(SelectMenu: From<SelectMenuBuilder>);

    #[test]
    fn builder() {
        let expected_option = SelectMenuOption {
            default: false,
            description: Some("test".to_string()),
            emoji: None,
            label: "bar".to_string(),
            value: "foo".to_string(),
        };

        let expected = SelectMenu {
            custom_id: "foo".to_string(),
            disabled: false,
            max_values: None,
            min_values: None,
            options: Some(vec![expected_option]),
            placeholder: None,
            id: None,
            channel_types: None,
            default_values: None,
            kind: SelectMenuType::Text,
        };

        let actual = SelectMenuBuilder::new("foo", SelectMenuType::Text)
            .option(SelectMenuOptionBuilder::new("bar", "foo").description("test"))
            .build();

        assert_eq!(actual, expected);
    }
}
