//! Create a [`Command`] with a builder.
//!
//! # Examples
//!
//! ```
//! use twilight_model::application::command::CommandType;
//! use twilight_util::builder::command::{BooleanBuilder, CommandBuilder, StringBuilder};
//!
//! CommandBuilder::new(
//!     "blep",
//!     "Send a random adorable animal photo",
//!     CommandType::ChatInput,
//! )
//! .option(
//!     StringBuilder::new("animal", "The type of animal")
//!         .required(true)
//!         .choices([
//!             ("Dog", "animal_dog"),
//!             ("Cat", "animal_cat"),
//!             ("Penguin", "animal_penguin"),
//!         ]),
//! )
//! .option(BooleanBuilder::new(
//!     "only_smol",
//!     "Whether to show only baby animals",
//! ));
//! ```
//!
//! ```
//! use twilight_model::application::command::CommandType;
//! use twilight_util::builder::command::{CommandBuilder, NumberBuilder};
//!
//! CommandBuilder::new(
//!     "birthday",
//!     "Wish a friend a happy birthday",
//!     CommandType::ChatInput,
//! )
//! .name_localizations([("zh-CN", "生日"), ("el", "γενέθλια")])
//! .description_localizations([("zh-Cn", "祝你朋友生日快乐")])
//! .option(
//!     NumberBuilder::new("age", "Your friend's age")
//!         .name_localizations([("zh-CN", "岁数")])
//!         .description_localizations([("zh-CN", "你朋友的岁数")]),
//! );
//! ```

use twilight_model::{
    application::command::{
        BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData, Command,
        CommandOption, CommandOptionChoice, CommandOptionValue, CommandType, Number,
        NumberCommandOptionData, OptionsCommandOptionData,
    },
    channel::ChannelType,
    guild::Permissions,
    id::{marker::GuildMarker, Id},
};
use twilight_validate::command::{command as validate_command, CommandValidationError};

/// Builder to create a [`Command`].
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
#[must_use = "must be built into a command"]
pub struct CommandBuilder(Command);

impl CommandBuilder {
    /// Create a new default [`Command`] builder.
    #[must_use = "builders have no effect if unused"]
    pub fn new(name: impl Into<String>, description: impl Into<String>, kind: CommandType) -> Self {
        Self(Command {
            application_id: None,
            default_member_permissions: None,
            dm_permission: None,
            description: description.into(),
            description_localizations: None,
            guild_id: None,
            id: None,
            kind,
            name: name.into(),
            name_localizations: None,
            options: Vec::new(),
            version: Id::new(1),
        })
    }

    /// Consume the builder, returning a [`Command`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "must be built into a command"]
    pub fn build(self) -> Command {
        self.0
    }

    /// Ensure the command is valid.
    ///
    /// # Errors
    ///
    /// Refer to the errors section of [`twilight_validate::command::command`]
    /// for possible errors.
    pub fn validate(self) -> Result<Self, CommandValidationError> {
        validate_command(&self.0)?;

        Ok(self)
    }

    /// Set the guild ID of the command.
    ///
    /// Defaults to [`None`].
    pub const fn guild_id(mut self, guild_id: Id<GuildMarker>) -> Self {
        self.0.guild_id = Some(guild_id);

        self
    }

    /// Set the default member permission required to run the command.
    ///
    /// Defaults to [`None`].
    pub const fn default_member_permissions(
        mut self,
        default_member_permissions: Permissions,
    ) -> Self {
        self.0.default_member_permissions = Some(default_member_permissions);

        self
    }

    /// Set whether the command is available in DMs.
    ///
    /// Defaults to [`None`].
    pub const fn dm_permission(mut self, dm_permission: bool) -> Self {
        self.0.dm_permission = Some(dm_permission);

        self
    }

    /// Set the localization dictionary for the command description.
    ///
    /// Defaults to [`None`].
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set the localization dictionary for the command name.
    ///
    /// Defaults to [`None`].
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Add an option to the command.
    ///
    /// Defaults to an empty list.
    pub fn option(self, option: impl Into<CommandOption>) -> Self {
        self._option(option.into())
    }

    fn _option(mut self, option: CommandOption) -> Self {
        self.0.options.push(option);

        self
    }
}

/// Create an attachment option with a builder.
#[derive(Clone, Debug)]
#[must_use = "should be used in a command builder"]
pub struct AttachmentBuilder(BaseCommandOptionData);

impl AttachmentBuilder {
    /// Create a new default [`AttachmentBuilder`].
    #[must_use = "builders have no effect if unused"]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(BaseCommandOptionData {
            description: description.into(),
            description_localizations: None,
            name: name.into(),
            name_localizations: None,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::Attachment(self.0)
    }

    /// Set the localization dictionary for the option description.
    ///
    /// Defaults to [`None`].
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set the localization dictionary for the option name.
    ///
    /// Defaults to [`None`].
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set whether this option is required.
    ///
    /// Defaults to false.
    pub const fn required(mut self, required: bool) -> Self {
        self.0.required = required;

        self
    }
}

impl From<AttachmentBuilder> for CommandOption {
    fn from(builder: AttachmentBuilder) -> CommandOption {
        builder.build()
    }
}

/// Create a boolean option with a builder.
#[derive(Clone, Debug)]
#[must_use = "should be used in a command builder"]
pub struct BooleanBuilder(BaseCommandOptionData);

impl BooleanBuilder {
    /// Create a new default [`BooleanBuilder`].
    #[must_use = "builders have no effect if unused"]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(BaseCommandOptionData {
            description: description.into(),
            description_localizations: None,
            name: name.into(),
            name_localizations: None,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::Boolean(self.0)
    }

    /// Set the localization dictionary for the option description.
    ///
    /// Defaults to [`None`].
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set the localization dictionary for the option name.
    ///
    /// Defaults to [`None`].
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set whether this option is required.
    ///
    /// Defaults to false.
    pub const fn required(mut self, required: bool) -> Self {
        self.0.required = required;

        self
    }
}

impl From<BooleanBuilder> for CommandOption {
    fn from(builder: BooleanBuilder) -> CommandOption {
        builder.build()
    }
}

/// Create a channel option with a builder.
#[derive(Clone, Debug)]
#[must_use = "should be used in a command builder"]
pub struct ChannelBuilder(ChannelCommandOptionData);

impl ChannelBuilder {
    /// Create a new default [`ChannelBuilder`].
    #[must_use = "builders have no effect if unused"]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(ChannelCommandOptionData {
            channel_types: Vec::new(),
            description: description.into(),
            description_localizations: None,
            name: name.into(),
            name_localizations: None,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::Channel(self.0)
    }

    /// Restricts the channel choice to specific types.
    ///
    /// Defaults to all channel types allowed.
    pub fn channel_types(mut self, channel_types: impl IntoIterator<Item = ChannelType>) -> Self {
        self.0.channel_types = channel_types.into_iter().collect();

        self
    }

    /// Set the localization dictionary for the option description.
    ///
    /// Defaults to [`None`].
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set the localization dictionary for the option name.
    ///
    /// Defaults to [`None`].
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set whether this option is required.
    ///
    /// Defaults to false.
    pub const fn required(mut self, required: bool) -> Self {
        self.0.required = required;

        self
    }
}

impl From<ChannelBuilder> for CommandOption {
    fn from(builder: ChannelBuilder) -> CommandOption {
        builder.build()
    }
}
/// Create a integer option with a builder.
#[derive(Clone, Debug)]
#[must_use = "should be used in a command builder"]
pub struct IntegerBuilder(NumberCommandOptionData);

impl IntegerBuilder {
    /// Create a new default [`IntegerBuilder`].
    #[must_use = "builders have no effect if unused"]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(NumberCommandOptionData {
            autocomplete: false,
            choices: Vec::new(),
            description: description.into(),
            description_localizations: None,
            max_value: None,
            min_value: None,
            name: name.into(),
            name_localizations: None,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::Integer(self.0)
    }

    /// Set whether this option supports autocomplete.
    ///
    /// Defaults to false.
    pub const fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.0.autocomplete = autocomplete;

        self
    }

    /// Set localization for a particular choice, by name.
    ///
    /// Choices must be set with the [`choices`] method before updating their
    /// localization.
    ///
    /// [`choices`]: Self::choices
    pub fn choice_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        choice_name: &str,
        name_localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        let choice = self.0.choices.iter_mut().find(
            |choice| matches!(choice, CommandOptionChoice::Int { name, .. } if name == choice_name),
        );

        if let Some(choice) = choice {
            set_choice_localizations(choice, name_localizations);
        }

        self
    }

    /// Set the list of choices for an option.
    ///
    /// Accepts tuples of `(String, i64)` corresponding to the name and value.
    /// Localization may be added with [`choice_localizations`].
    ///
    /// Defaults to no choices.
    ///
    /// [`choice_localizations`]: Self::choice_localizations
    pub fn choices<K: Into<String>>(mut self, choices: impl IntoIterator<Item = (K, i64)>) -> Self {
        self.0.choices = choices
            .into_iter()
            .map(|(name, value, ..)| CommandOptionChoice::Int {
                name: name.into(),
                name_localizations: None,
                value,
            })
            .collect();

        self
    }

    /// Set the localization dictionary for the option description.
    ///
    /// Defaults to [`None`].
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set the maximum value permitted.
    ///
    /// Defaults to no limit.
    pub const fn max_value(mut self, value: i64) -> Self {
        self.0.max_value = Some(CommandOptionValue::Integer(value));

        self
    }

    /// Set the minimum value permitted.
    ///
    /// Defaults to no limit.
    pub const fn min_value(mut self, value: i64) -> Self {
        self.0.min_value = Some(CommandOptionValue::Integer(value));

        self
    }

    /// Set the localization dictionary for the option name.
    ///
    /// Defaults to [`None`].
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set whether this option is required.
    ///
    /// Defaults to false.
    pub const fn required(mut self, required: bool) -> Self {
        self.0.required = required;

        self
    }
}

impl From<IntegerBuilder> for CommandOption {
    fn from(builder: IntegerBuilder) -> CommandOption {
        builder.build()
    }
}

/// Create a mentionable option with a builder.
#[derive(Clone, Debug)]
#[must_use = "should be used in a command builder"]
pub struct MentionableBuilder(BaseCommandOptionData);

impl MentionableBuilder {
    /// Create a new default [`MentionableBuilder`].
    #[must_use = "builders have no effect if unused"]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(BaseCommandOptionData {
            description: description.into(),
            description_localizations: None,
            name: name.into(),
            name_localizations: None,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::Mentionable(self.0)
    }

    /// Set the localization dictionary for the option description.
    ///
    /// Defaults to [`None`].
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set the localization dictionary for the option name.
    ///
    /// Defaults to [`None`].
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set whether this option is required.
    ///
    /// Defaults to false.
    pub const fn required(mut self, required: bool) -> Self {
        self.0.required = required;

        self
    }
}

impl From<MentionableBuilder> for CommandOption {
    fn from(builder: MentionableBuilder) -> CommandOption {
        builder.build()
    }
}

/// Create a [`Number`] option with a builder.
#[derive(Clone, Debug)]
#[must_use = "should be used in a command builder"]
pub struct NumberBuilder(NumberCommandOptionData);

impl NumberBuilder {
    /// Create a new default [`NumberBuilder`].
    #[must_use = "builders have no effect if unused"]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(NumberCommandOptionData {
            autocomplete: false,
            choices: Vec::new(),
            description: description.into(),
            description_localizations: None,
            max_value: None,
            min_value: None,
            name: name.into(),
            name_localizations: None,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::Number(self.0)
    }

    /// Set whether this option supports autocomplete.
    ///
    /// Defaults to false.
    pub const fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.0.autocomplete = autocomplete;

        self
    }

    /// Set localization for a particular choice, by name.
    ///
    /// Choices must be set with the [`choices`] method before updating their
    /// localization.
    ///
    /// [`choices`]: Self::choices
    pub fn choice_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        choice_name: &str,
        name_localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        let choice = self.0.choices.iter_mut().find(
            |choice| matches!(choice, CommandOptionChoice::Number { name, .. } if name == choice_name),
        );

        if let Some(choice) = choice {
            set_choice_localizations(choice, name_localizations);
        }

        self
    }

    /// Set the list of choices for an option.
    ///
    /// Accepts tuples of `(String, Number)` corresponding to the name and
    /// value. Localization may be added with [`choice_localizations`].
    ///
    /// Defaults to no choices.
    ///
    /// [`choice_localizations`]: Self::choice_localizations
    pub fn choices<K: Into<String>>(mut self, choices: impl IntoIterator<Item = (K, f64)>) -> Self {
        self.0.choices = choices
            .into_iter()
            .map(|(name, value, ..)| CommandOptionChoice::Number {
                name: name.into(),
                name_localizations: None,
                value: Number(value),
            })
            .collect();

        self
    }

    /// Set the localization dictionary for the option description.
    ///
    /// Defaults to [`None`].
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set the maximum value permitted.
    ///
    /// Defaults to no limit.
    pub const fn max_value(mut self, value: f64) -> Self {
        self.0.max_value = Some(CommandOptionValue::Number(Number(value)));

        self
    }

    /// Set the minimum value permitted.
    ///
    /// Defaults to no limit.
    pub const fn min_value(mut self, value: f64) -> Self {
        self.0.min_value = Some(CommandOptionValue::Number(Number(value)));

        self
    }

    /// Set the localization dictionary for the option name.
    ///
    /// Defaults to [`None`].
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set whether this option is required.
    ///
    /// Defaults to false.
    pub const fn required(mut self, required: bool) -> Self {
        self.0.required = required;

        self
    }
}

impl From<NumberBuilder> for CommandOption {
    fn from(builder: NumberBuilder) -> CommandOption {
        builder.build()
    }
}

/// Create a role option with a builder.
#[derive(Clone, Debug)]
#[must_use = "should be used in a command builder"]
pub struct RoleBuilder(BaseCommandOptionData);

impl RoleBuilder {
    /// Create a new default [`RoleBuilder`].
    #[must_use = "builders have no effect if unused"]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(BaseCommandOptionData {
            description: description.into(),
            description_localizations: None,
            name: name.into(),
            name_localizations: None,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::Role(self.0)
    }

    /// Set the localization dictionary for the option description.
    ///
    /// Defaults to [`None`].
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set the localization dictionary for the option name.
    ///
    /// Defaults to [`None`].
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set whether this option is required.
    ///
    /// Defaults to false.
    pub const fn required(mut self, required: bool) -> Self {
        self.0.required = required;

        self
    }
}

impl From<RoleBuilder> for CommandOption {
    fn from(builder: RoleBuilder) -> CommandOption {
        builder.build()
    }
}

/// Create a string option with a builder.
#[derive(Clone, Debug)]
#[must_use = "should be used in a command builder"]
pub struct StringBuilder(ChoiceCommandOptionData);

impl StringBuilder {
    /// Create a new default [`StringBuilder`].
    #[must_use = "builders have no effect if unused"]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(ChoiceCommandOptionData {
            autocomplete: false,
            choices: Vec::new(),
            description: description.into(),
            description_localizations: None,
            max_length: None,
            min_length: None,
            name: name.into(),
            name_localizations: None,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::String(self.0)
    }

    /// Set whether this option supports autocomplete.
    ///
    /// Defaults to false.
    pub const fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.0.autocomplete = autocomplete;

        self
    }

    /// Set localization for a particular choice, by name.
    ///
    /// Choices must be set with the [`choices`] method before updating their
    /// localization.
    ///
    /// [`choices`]: Self::choices
    pub fn choice_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        choice_name: &str,
        name_localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        let choice = self.0.choices.iter_mut().find(
            |choice| matches!(choice, CommandOptionChoice::String { name, .. } if name == choice_name),
        );

        if let Some(choice) = choice {
            set_choice_localizations(choice, name_localizations);
        }

        self
    }

    /// Set the list of choices for an option.
    ///
    /// Accepts tuples of `(String, String)` corresponding to the name and
    /// value. Localization may be added with [`choice_localizations`].
    ///
    /// Defaults to no choices.
    ///
    /// [`choice_localizations`]: Self::choice_localizations
    pub fn choices<K: Into<String>, V: Into<String>>(
        mut self,
        choices: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.choices = choices
            .into_iter()
            .map(|(name, value, ..)| CommandOptionChoice::String {
                name: name.into(),
                name_localizations: None,
                value: value.into(),
            })
            .collect();

        self
    }

    /// Set the localization dictionary for the option description.
    ///
    /// Defaults to [`None`].
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set the maximum allowed length.
    ///
    /// Defaults to no limit.
    pub const fn max_length(mut self, value: u16) -> Self {
        self.0.max_length = Some(value);

        self
    }

    /// Set the minimum allowed length.
    ///
    /// Defaults to no limit.
    pub const fn min_length(mut self, value: u16) -> Self {
        self.0.min_length = Some(value);

        self
    }

    /// Set the localization dictionary for the option name.
    ///
    /// Defaults to [`None`].
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set whether this option is required.
    ///
    /// Defaults to false.
    pub const fn required(mut self, required: bool) -> Self {
        self.0.required = required;

        self
    }
}

impl From<StringBuilder> for CommandOption {
    fn from(builder: StringBuilder) -> CommandOption {
        builder.build()
    }
}

/// Create a subcommand option with a builder.
#[derive(Clone, Debug)]
#[must_use = "should be used in a command builder"]
pub struct SubCommandBuilder(OptionsCommandOptionData);

impl SubCommandBuilder {
    /// Create a new default [`SubCommandBuilder`].
    #[must_use = "builders have no effect if unused"]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(OptionsCommandOptionData {
            description: description.into(),
            description_localizations: None,
            name: name.into(),
            name_localizations: None,
            options: Vec::new(),
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::SubCommand(self.0)
    }

    /// Set the localization dictionary for the option description.
    ///
    /// Defaults to [`None`].
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set the localization dictionary for the option name.
    ///
    /// Defaults to [`None`].
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Add an option to the sub command.
    ///
    /// Defaults to an empty list.
    pub fn option(self, option: impl Into<CommandOption>) -> Self {
        self._option(option.into())
    }

    fn _option(mut self, option: CommandOption) -> Self {
        self.0.options.push(option);

        self
    }
}

impl From<SubCommandBuilder> for CommandOption {
    fn from(builder: SubCommandBuilder) -> CommandOption {
        builder.build()
    }
}

/// Create a subcommand group option with a builder.
#[derive(Clone, Debug)]
#[must_use = "should be used in a command builder"]
pub struct SubCommandGroupBuilder(OptionsCommandOptionData);

impl SubCommandGroupBuilder {
    /// Create a new default [`SubCommandGroupBuilder`].
    #[must_use = "builders have no effect if unused"]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(OptionsCommandOptionData {
            description: description.into(),
            description_localizations: None,
            name: name.into(),
            name_localizations: None,
            options: Vec::new(),
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::SubCommandGroup(self.0)
    }

    /// Set the localization dictionary for the option description.
    ///
    /// Defaults to [`None`].
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set the localization dictionary for the option name.
    ///
    /// Defaults to [`None`].
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set the list of sub commands to the group.
    ///
    /// Defaults to no subcommands.
    pub fn subcommands(mut self, subcommands: impl IntoIterator<Item = SubCommandBuilder>) -> Self {
        self.0.options = subcommands.into_iter().map(Into::into).collect();

        self
    }
}

impl From<SubCommandGroupBuilder> for CommandOption {
    fn from(builder: SubCommandGroupBuilder) -> CommandOption {
        builder.build()
    }
}

/// Create a user option with a builder.
#[derive(Clone, Debug)]
#[must_use = "should be used in a command builder"]
pub struct UserBuilder(BaseCommandOptionData);

impl UserBuilder {
    /// Create a new default [`UserBuilder`].
    #[must_use = "builders have no effect if unused"]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(BaseCommandOptionData {
            description: description.into(),
            description_localizations: None,
            name: name.into(),
            name_localizations: None,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::User(self.0)
    }

    /// Set the localization dictionary for the option description.
    ///
    /// Defaults to [`None`].
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set the localization dictionary for the option name.
    ///
    /// Defaults to [`None`].
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.0.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(a, b)| (a.into(), b.into()))
                .collect(),
        );

        self
    }

    /// Set whether this option is required.
    ///
    /// Defaults to false.
    pub const fn required(mut self, required: bool) -> Self {
        self.0.required = required;

        self
    }
}

impl From<UserBuilder> for CommandOption {
    fn from(builder: UserBuilder) -> CommandOption {
        builder.build()
    }
}

fn set_choice_localizations<K: Into<String>, V: Into<String>>(
    choice: &mut CommandOptionChoice,
    localizations: impl IntoIterator<Item = (K, V)>,
) {
    let name_localizations = match choice {
        CommandOptionChoice::String {
            name_localizations, ..
        }
        | CommandOptionChoice::Int {
            name_localizations, ..
        }
        | CommandOptionChoice::Number {
            name_localizations, ..
        } => name_localizations,
    };

    *name_localizations = Some(
        localizations
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(AttachmentBuilder: Clone, Debug, Send, Sync);
    assert_impl_all!(CommandBuilder: Clone, Debug, Send, Sync);
    assert_impl_all!(BooleanBuilder: Clone, Debug, Send, Sync);
    assert_impl_all!(ChannelBuilder: Clone, Debug, Send, Sync);
    assert_impl_all!(IntegerBuilder: Clone, Debug, Send, Sync);
    assert_impl_all!(MentionableBuilder: Clone, Debug, Send, Sync);
    assert_impl_all!(RoleBuilder: Clone, Debug, Send, Sync);
    assert_impl_all!(StringBuilder: Clone, Debug, Send, Sync);
    assert_impl_all!(SubCommandBuilder: Clone, Debug, Send, Sync);
    assert_impl_all!(SubCommandGroupBuilder: Clone, Debug, Send, Sync);
    assert_impl_all!(UserBuilder: Clone, Debug, Send, Sync);

    #[test]
    #[allow(clippy::too_many_lines)]
    fn construct_command_with_builder() {
        let command =
            CommandBuilder::new(
                "permissions",
                "Get or edit permissions for a user or a role",
                CommandType::ChatInput,
            )
            .option(
                SubCommandGroupBuilder::new("user", "Get or edit permissions for a user")
                    .subcommands([
                        SubCommandBuilder::new("get", "Get permissions for a user")
                            .option(UserBuilder::new("user", "The user to get").required(true))
                            .option(ChannelBuilder::new(
                                "channel",
                                "The channel permissions to get. If omitted, the guild \
                                 permissions will be returned",
                            )),
                        SubCommandBuilder::new("edit", "Edit permissions for a user")
                            .option(UserBuilder::new("user", "The user to edit").required(true))
                            .option(ChannelBuilder::new(
                                "channel",
                                "The channel permissions to edit. If omitted, the guild \
                                 permissions will be edited",
                            )),
                    ]),
            )
            .option(
                SubCommandGroupBuilder::new("role", "Get or edit permissions for a role")
                    .subcommands([
                        SubCommandBuilder::new("get", "Get permissions for a role")
                            .option(RoleBuilder::new("role", "The role to get").required(true))
                            .option(ChannelBuilder::new(
                                "channel",
                                "The channel permissions to get. If omitted, the guild \
                                 permissions will be returned",
                            )),
                        SubCommandBuilder::new("edit", "Edit permissions for a role")
                            .option(RoleBuilder::new("role", "The role to edit").required(true))
                            .option(ChannelBuilder::new(
                                "channel",
                                "The channel permissions to edit. If omitted, the guild \
                                 permissions will be edited",
                            ))
                            .option(
                                NumberBuilder::new("position", "The position of the new role")
                                    .autocomplete(true),
                            ),
                    ]),
            )
            .build();

        let command_manual = Command {
            application_id: None,
            default_member_permissions: None,
            dm_permission: None,
            description: String::from("Get or edit permissions for a user or a role"),
            guild_id: None,
            id: None,
            kind: CommandType::ChatInput,
            name: String::from("permissions"),
            name_localizations: None,
            description_localizations: None,
            options: Vec::from([
                CommandOption::SubCommandGroup(OptionsCommandOptionData {
                    description: String::from("Get or edit permissions for a user"),
                    description_localizations: None,
                    name: String::from("user"),
                    name_localizations: None,
                    options: Vec::from([
                        CommandOption::SubCommand(OptionsCommandOptionData {
                            description: String::from("Get permissions for a user"),
                            description_localizations: None,
                            name: String::from("get"),
                            name_localizations: None,
                            options: Vec::from([
                                CommandOption::User(BaseCommandOptionData {
                                    description: String::from("The user to get"),
                                    description_localizations: None,
                                    name: String::from("user"),
                                    name_localizations: None,
                                    required: true,
                                }),
                                CommandOption::Channel(ChannelCommandOptionData {
                                    channel_types: Vec::new(),
                                    description: String::from(
                                        "The channel permissions to get. If omitted, the guild \
                                         permissions will be returned",
                                    ),
                                    description_localizations: None,
                                    name: String::from("channel"),
                                    name_localizations: None,
                                    required: false,
                                }),
                            ]),
                        }),
                        CommandOption::SubCommand(OptionsCommandOptionData {
                            description: String::from("Edit permissions for a user"),
                            description_localizations: None,
                            name: String::from("edit"),
                            name_localizations: None,
                            options: Vec::from([
                                CommandOption::User(BaseCommandOptionData {
                                    description: String::from("The user to edit"),
                                    description_localizations: None,
                                    name: String::from("user"),
                                    name_localizations: None,
                                    required: true,
                                }),
                                CommandOption::Channel(ChannelCommandOptionData {
                                    channel_types: Vec::new(),
                                    description: String::from(
                                        "The channel permissions to edit. If omitted, the guild \
                                         permissions will be edited",
                                    ),
                                    description_localizations: None,
                                    name: String::from("channel"),
                                    name_localizations: None,
                                    required: false,
                                }),
                            ]),
                        }),
                    ]),
                }),
                CommandOption::SubCommandGroup(OptionsCommandOptionData {
                    description: String::from("Get or edit permissions for a role"),
                    description_localizations: None,
                    name: String::from("role"),
                    name_localizations: None,
                    options: Vec::from([
                        CommandOption::SubCommand(OptionsCommandOptionData {
                            description: String::from("Get permissions for a role"),
                            description_localizations: None,
                            name: String::from("get"),
                            name_localizations: None,
                            options: Vec::from([
                                CommandOption::Role(BaseCommandOptionData {
                                    description: String::from("The role to get"),
                                    description_localizations: None,
                                    name: String::from("role"),
                                    name_localizations: None,
                                    required: true,
                                }),
                                CommandOption::Channel(ChannelCommandOptionData {
                                    channel_types: Vec::new(),
                                    description: String::from(
                                        "The channel permissions to get. If omitted, the guild \
                                         permissions will be returned",
                                    ),
                                    description_localizations: None,
                                    name: String::from("channel"),
                                    name_localizations: None,
                                    required: false,
                                }),
                            ]),
                        }),
                        CommandOption::SubCommand(OptionsCommandOptionData {
                            description: String::from("Edit permissions for a role"),
                            description_localizations: None,
                            name: String::from("edit"),
                            name_localizations: None,
                            options: Vec::from([
                                CommandOption::Role(BaseCommandOptionData {
                                    description: String::from("The role to edit"),
                                    description_localizations: None,
                                    name: String::from("role"),
                                    name_localizations: None,
                                    required: true,
                                }),
                                CommandOption::Channel(ChannelCommandOptionData {
                                    channel_types: Vec::new(),
                                    description: String::from(
                                        "The channel permissions to edit. If omitted, the guild \
                                         permissions will be edited",
                                    ),
                                    description_localizations: None,
                                    name: String::from("channel"),
                                    name_localizations: None,
                                    required: false,
                                }),
                                CommandOption::Number(NumberCommandOptionData {
                                    autocomplete: true,
                                    choices: Vec::new(),
                                    description: String::from("The position of the new role"),
                                    description_localizations: None,
                                    max_value: None,
                                    min_value: None,
                                    name: String::from("position"),
                                    name_localizations: None,
                                    required: false,
                                }),
                            ]),
                        }),
                    ]),
                }),
            ]),
            version: Id::new(1),
        };

        assert_eq!(command, command_manual);
    }

    #[test]
    fn validate() {
        let result = CommandBuilder::new("", "", CommandType::ChatInput).validate();

        assert!(result.is_err());
    }
}
