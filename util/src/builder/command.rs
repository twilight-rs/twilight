//! Create a [`Command`] with a builder.
//!
//! # Examples
//!
//! ```
//! use twilight_model::application::command::CommandType;
//! use twilight_util::builder::command::{BooleanBuilder, CommandBuilder, StringBuilder};
//!
//! CommandBuilder::new(
//!     "blep".into(),
//!     "Send a random adorable animal photo".into(),
//!     CommandType::ChatInput,
//! )
//! .option(
//!     StringBuilder::new("animal".into(), "The type of animal".into())
//!         .required(true)
//!         .choices([
//!             ("Dog".into(), "animal_dog".into()),
//!             ("Cat".into(), "animal_cat".into()),
//!             ("Penguin".into(), "animal_penguin".into()),
//!         ]),
//! )
//! .option(BooleanBuilder::new(
//!     "only_smol".into(),
//!     "Whether to show only baby animals".into(),
//! ));
//! ```

use twilight_model::{
    application::command::{
        BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData, Command,
        CommandOption, CommandOptionChoice, CommandOptionValue, CommandType, Number,
        NumberCommandOptionData, OptionsCommandOptionData,
    },
    channel::ChannelType,
    id::{marker::GuildMarker, Id},
};

#[cfg(feature = "validate")]
use twilight_validate::command::{command as validate_command, CommandValidationError};

/// Builder to create a [`Command`].
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
#[must_use = "must be built into a command"]
pub struct CommandBuilder(Command);

impl CommandBuilder {
    /// Create a new default [`Command`] builder.
    #[must_use = "builders have no effect if unused"]
    pub const fn new(name: String, description: String, kind: CommandType) -> Self {
        Self(Command {
            application_id: None,
            default_permission: None,
            description,
            guild_id: None,
            id: None,
            kind,
            name,
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
    #[cfg(feature = "validate")]
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

    /// Set the default permission of the command.
    ///
    /// Defaults to [`None`].
    pub const fn default_permission(mut self, default_permission: bool) -> Self {
        self.0.default_permission = Some(default_permission);

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
    pub const fn new(name: String, description: String) -> Self {
        Self(BaseCommandOptionData {
            description,
            name,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::Attachment(self.0)
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
    pub const fn new(name: String, description: String) -> Self {
        Self(BaseCommandOptionData {
            description,
            name,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::Boolean(self.0)
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
    pub const fn new(name: String, description: String) -> Self {
        Self(ChannelCommandOptionData {
            channel_types: Vec::new(),
            description,
            name,
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
    pub const fn new(name: String, description: String) -> Self {
        Self(NumberCommandOptionData {
            autocomplete: false,
            choices: Vec::new(),
            description,
            max_value: None,
            min_value: None,
            name,
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

    /// Set the list of choices for an option.
    ///
    /// Accepts tuples of `(String, i64)` corresponding to the name and value.
    ///
    /// Defaults to no choices.
    pub fn choices(mut self, choices: impl IntoIterator<Item = (String, i64)>) -> Self {
        self.0.choices = choices
            .into_iter()
            .map(|(name, value)| CommandOptionChoice::Int { name, value })
            .collect();

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
    pub const fn new(name: String, description: String) -> Self {
        Self(BaseCommandOptionData {
            description,
            name,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::Mentionable(self.0)
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
    pub const fn new(name: String, description: String) -> Self {
        Self(NumberCommandOptionData {
            autocomplete: false,
            choices: Vec::new(),
            description,
            max_value: None,
            min_value: None,
            name,
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

    /// Set the list of choices for an option.
    ///
    /// Accepts tuples of `(String, Number)` corresponding to the name and
    /// value.
    ///
    /// Defaults to no choices.
    pub fn choices(mut self, choices: impl IntoIterator<Item = (String, Number)>) -> Self {
        self.0.choices = choices
            .into_iter()
            .map(|(name, value)| CommandOptionChoice::Number { name, value })
            .collect();

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
    pub const fn new(name: String, description: String) -> Self {
        Self(BaseCommandOptionData {
            description,
            name,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::Role(self.0)
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
    pub const fn new(name: String, description: String) -> Self {
        Self(ChoiceCommandOptionData {
            autocomplete: false,
            choices: Vec::new(),
            description,
            name,
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

    /// Set the list of choices for an option.
    ///
    /// Accepts tuples of `(String, String)` corresponding to the name and
    /// value.
    ///
    /// Defaults to no choices.
    pub fn choices(mut self, choices: impl IntoIterator<Item = (String, String)>) -> Self {
        self.0.choices = choices
            .into_iter()
            .map(|(name, value)| CommandOptionChoice::String { name, value })
            .collect();

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
    pub const fn new(name: String, description: String) -> Self {
        Self(OptionsCommandOptionData {
            description,
            name,
            options: Vec::new(),
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::SubCommand(self.0)
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
    pub const fn new(name: String, description: String) -> Self {
        Self(OptionsCommandOptionData {
            description,
            name,
            options: Vec::new(),
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::SubCommandGroup(self.0)
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
    pub const fn new(name: String, description: String) -> Self {
        Self(BaseCommandOptionData {
            description,
            name,
            required: false,
        })
    }

    /// Consume the builder, returning the built command option.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "should be used in a command builder"]
    pub fn build(self) -> CommandOption {
        CommandOption::User(self.0)
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
        let command = CommandBuilder::new(
            "permissions".into(),
            "Get or edit permissions for a user or a role".into(),
            CommandType::ChatInput,
        )
        .option(
            SubCommandGroupBuilder::new("user".into(), "Get or edit permissions for a user".into())
                .subcommands([
                    SubCommandBuilder::new("get".into(), "Get permissions for a user".into())
                        .option(
                            UserBuilder::new("user".into(), "The user to get".into())
                                .required(true),
                        )
                        .option(ChannelBuilder::new(
                            "channel".into(),
                            "The channel permissions to get. If omitted, the guild permissions \
                             will be returned"
                                .into(),
                        )),
                    SubCommandBuilder::new("edit".into(), "Edit permissions for a user".into())
                        .option(
                            UserBuilder::new("user".into(), "The user to edit".into())
                                .required(true),
                        )
                        .option(ChannelBuilder::new(
                            "channel".into(),
                            "The channel permissions to edit. If omitted, the guild permissions \
                             will be edited"
                                .into(),
                        )),
                ]),
        )
        .option(
            SubCommandGroupBuilder::new("role".into(), "Get or edit permissions for a role".into())
                .subcommands([
                    SubCommandBuilder::new("get".into(), "Get permissions for a role".into())
                        .option(
                            RoleBuilder::new("role".into(), "The role to get".into())
                                .required(true),
                        )
                        .option(ChannelBuilder::new(
                            "channel".into(),
                            "The channel permissions to get. If omitted, the guild permissions \
                             will be returned"
                                .into(),
                        )),
                    SubCommandBuilder::new("edit".into(), "Edit permissions for a role".into())
                        .option(
                            RoleBuilder::new("role".into(), "The role to edit".into())
                                .required(true),
                        )
                        .option(ChannelBuilder::new(
                            "channel".into(),
                            "The channel permissions to edit. If omitted, the guild permissions \
                             will be edited"
                                .into(),
                        ))
                        .option(
                            NumberBuilder::new(
                                "position".into(),
                                "The position of the new role".into(),
                            )
                            .autocomplete(true),
                        ),
                ]),
        )
        .build();

        let command_manual = Command {
            application_id: None,
            guild_id: None,
            kind: CommandType::ChatInput,
            name: String::from("permissions"),
            default_permission: None,
            description: String::from("Get or edit permissions for a user or a role"),
            id: None,
            options: Vec::from([
                CommandOption::SubCommandGroup(OptionsCommandOptionData {
                    description: String::from("Get or edit permissions for a user"),
                    name: String::from("user"),
                    options: Vec::from([
                        CommandOption::SubCommand(OptionsCommandOptionData {
                            description: String::from("Get permissions for a user"),
                            name: String::from("get"),
                            options: Vec::from([
                                CommandOption::User(BaseCommandOptionData {
                                    description: String::from("The user to get"),
                                    name: String::from("user"),
                                    required: true,
                                }),
                                CommandOption::Channel(ChannelCommandOptionData {
                                    channel_types: Vec::new(),
                                    description: String::from(
                                        "The channel permissions to get. If omitted, the guild \
                                         permissions will be returned",
                                    ),
                                    name: String::from("channel"),
                                    required: false,
                                }),
                            ]),
                        }),
                        CommandOption::SubCommand(OptionsCommandOptionData {
                            description: String::from("Edit permissions for a user"),
                            name: String::from("edit"),
                            options: Vec::from([
                                CommandOption::User(BaseCommandOptionData {
                                    description: String::from("The user to edit"),
                                    name: String::from("user"),
                                    required: true,
                                }),
                                CommandOption::Channel(ChannelCommandOptionData {
                                    channel_types: Vec::new(),
                                    description: String::from(
                                        "The channel permissions to edit. If omitted, the guild \
                                         permissions will be edited",
                                    ),
                                    name: String::from("channel"),
                                    required: false,
                                }),
                            ]),
                        }),
                    ]),
                }),
                CommandOption::SubCommandGroup(OptionsCommandOptionData {
                    description: String::from("Get or edit permissions for a role"),
                    name: String::from("role"),
                    options: Vec::from([
                        CommandOption::SubCommand(OptionsCommandOptionData {
                            description: String::from("Get permissions for a role"),
                            name: String::from("get"),
                            options: Vec::from([
                                CommandOption::Role(BaseCommandOptionData {
                                    description: String::from("The role to get"),
                                    name: String::from("role"),
                                    required: true,
                                }),
                                CommandOption::Channel(ChannelCommandOptionData {
                                    channel_types: Vec::new(),
                                    description: String::from(
                                        "The channel permissions to get. If omitted, the guild \
                                         permissions will be returned",
                                    ),
                                    name: String::from("channel"),
                                    required: false,
                                }),
                            ]),
                        }),
                        CommandOption::SubCommand(OptionsCommandOptionData {
                            description: String::from("Edit permissions for a role"),
                            name: String::from("edit"),
                            options: Vec::from([
                                CommandOption::Role(BaseCommandOptionData {
                                    description: String::from("The role to edit"),
                                    name: String::from("role"),
                                    required: true,
                                }),
                                CommandOption::Channel(ChannelCommandOptionData {
                                    channel_types: Vec::new(),
                                    description: String::from(
                                        "The channel permissions to edit. If omitted, the guild \
                                         permissions will be edited",
                                    ),
                                    name: String::from("channel"),
                                    required: false,
                                }),
                                CommandOption::Number(NumberCommandOptionData {
                                    autocomplete: true,
                                    choices: Vec::new(),
                                    description: String::from("The position of the new role"),
                                    max_value: None,
                                    min_value: None,
                                    name: String::from("position"),
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

    #[cfg(feature = "validate")]
    #[test]
    fn test_validate() {
        let result = CommandBuilder::new("".into(), "".into(), CommandType::ChatInput).validate();

        assert!(result.is_err());
    }
}
