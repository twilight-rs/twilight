//! Create a [`Command`] with a builder.

use super::{
    BaseCommandOptionData, ChoiceCommandOptionData, Command, CommandOption, CommandOptionChoice,
    OptionsCommandOptionData,
};
use crate::id::{ApplicationId, CommandId, GuildId};

/// Builder to create a [`Command`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CommandBuilder(Command);

impl CommandBuilder {
    /// Maximum total textual length of the command in UTF-16 code points.
    ///
    /// This combines the text of the name, description and value for the
    /// command and its subcommands and groups.
    pub const COMMAND_LENGTH_LIMIT: usize = 4000;

    /// Maximum number of options within a command.
    pub const COMMAND_OPTIONS_LIMIT: usize = 25;

    /// Maximum number of UTF-16 code points that can be in a description.
    pub const DESCRIPTION_LENGTH_LIMIT: usize = 100;

    /// Maximum number of UTF-16 code points in a name.
    pub const NAME_LENGTH_LIMIT: usize = 32;

    /// Maximum number of choices within a option.
    pub const OPTIONS_CHOICES_LIMIT: usize = 25;

    /// Maximum number of subcommand groups within a command.
    pub const SUBCOMMAND_GROUP_LIMIT: usize = 25;

    /// Maximum number of subcommands within a subcommand group.
    pub const SUBCOMMAND_LIMIT: usize = 25;

    /// Creates a new default [`Command`] builder.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::_new(name.into(), description.into())
    }

    const fn _new(name: String, description: String) -> Self {
        Self(Command {
            application_id: None,
            guild_id: None,
            name,
            default_permission: None,
            description,
            id: None,
            options: Vec::new(),
        })
    }

    /// Consume the builder, returning a [`Command`].
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> Command {
        self.0
    }

    /// Sets the application ID of the command.
    ///
    /// Defaults to [`None`].
    pub const fn application_id(mut self, application_id: ApplicationId) -> Self {
        self.0.application_id = Some(application_id);

        self
    }

    /// Sets the guild ID of the command.
    ///
    /// Defaults to [`None`].
    pub const fn guild_id(mut self, guild_id: GuildId) -> Self {
        self.0.guild_id = Some(guild_id);

        self
    }

    /// Sets the default permission of the command.
    ///
    /// Defaults to [`None`].
    pub const fn default_permission(mut self, default_permission: bool) -> Self {
        self.0.default_permission = Some(default_permission);

        self
    }

    /// Sets the ID of the command.
    ///
    /// Defaults to [`None`].
    pub const fn id(mut self, id: CommandId) -> Self {
        self.0.id = Some(id);

        self
    }

    /// Adds an option to the command.
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

/// Create a boolean option with a builder.
pub struct BooleanBuilder(BaseCommandOptionData);

impl BooleanBuilder {
    /// Creates a new default [`BooleanBuilder`].
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::_new(name.into(), description.into())
    }

    const fn _new(name: String, description: String) -> Self {
        Self(BaseCommandOptionData {
            description,
            name,
            required: false,
        })
    }

    /// Builds this into a command option.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> CommandOption {
        CommandOption::Boolean(self.0)
    }

    /// Sets whether this option is required.
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
pub struct ChannelBuilder(BaseCommandOptionData);

impl ChannelBuilder {
    /// Creates a new default [`ChannelBuilder`].
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::_new(name.into(), description.into())
    }

    const fn _new(name: String, description: String) -> Self {
        Self(BaseCommandOptionData {
            description,
            name,
            required: false,
        })
    }

    /// Builds this into a command option.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> CommandOption {
        CommandOption::Channel(self.0)
    }

    /// Sets whether this option is required.
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
pub struct IntegerBuilder(ChoiceCommandOptionData);

impl IntegerBuilder {
    /// Creates a new default [`IntegerBuilder`].
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::_new(name.into(), description.into())
    }

    const fn _new(name: String, description: String) -> Self {
        Self(ChoiceCommandOptionData {
            choices: Vec::new(),
            description,
            name,
            required: false,
        })
    }

    /// Builds this into a command option.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> CommandOption {
        CommandOption::Integer(self.0)
    }

    /// Adds a choice to the command.
    ///
    /// Defaults to no choices.
    pub fn choice(self, name: impl Into<String>, value: i64) -> Self {
        self._choice(name.into(), value)
    }

    fn _choice(mut self, name: String, value: i64) -> Self {
        self.0
            .choices
            .push(CommandOptionChoice::Int { name, value });

        self
    }

    /// Sets whether this option is required.
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
pub struct MentionableBuilder(BaseCommandOptionData);

impl MentionableBuilder {
    /// Creates a new default [`MentionableBuilder`].
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::_new(name.into(), description.into())
    }

    const fn _new(name: String, description: String) -> Self {
        Self(BaseCommandOptionData {
            description,
            name,
            required: false,
        })
    }

    /// Builds this into a command option.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> CommandOption {
        CommandOption::Mentionable(self.0)
    }

    /// Sets whether this option is required.
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

/// Create a role option with a builder.
pub struct RoleBuilder(BaseCommandOptionData);

impl RoleBuilder {
    /// Creates a new default [`RoleBuilder`].
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::_new(name.into(), description.into())
    }

    const fn _new(name: String, description: String) -> Self {
        Self(BaseCommandOptionData {
            description,
            name,
            required: false,
        })
    }

    /// Builds this into a command option.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> CommandOption {
        CommandOption::Role(self.0)
    }

    /// Sets whether this option is required.
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
pub struct StringBuilder(ChoiceCommandOptionData);

impl StringBuilder {
    /// Creates a new default [`StringBuilder`].
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::_new(name.into(), description.into())
    }

    const fn _new(name: String, description: String) -> Self {
        Self(ChoiceCommandOptionData {
            choices: Vec::new(),
            description,
            name,
            required: false,
        })
    }

    /// Builds this into a command option.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> CommandOption {
        CommandOption::String(self.0)
    }

    /// Adds a choice to the command.
    ///
    /// Defaults to no choices.
    pub fn choice(self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self._choice(name.into(), value.into())
    }

    fn _choice(mut self, name: String, value: String) -> Self {
        self.0
            .choices
            .push(CommandOptionChoice::String { name, value });

        self
    }

    /// Sets whether this option is required.
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
pub struct SubCommandBuilder(OptionsCommandOptionData);

impl SubCommandBuilder {
    /// Creates a new default [`SubCommandBuilder`].
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::_new(name.into(), description.into())
    }

    const fn _new(name: String, description: String) -> Self {
        Self(OptionsCommandOptionData {
            description,
            name,
            options: Vec::new(),
            required: false,
        })
    }

    /// Builds this into a command option.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> CommandOption {
        CommandOption::SubCommand(self.0)
    }

    /// Adds an option to the sub command.
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
pub struct SubCommandGroupBuilder(OptionsCommandOptionData);

impl SubCommandGroupBuilder {
    /// Creates a new default [`SubCommandGroupBuilder`].
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::_new(name.into(), description.into())
    }

    const fn _new(name: String, description: String) -> Self {
        Self(OptionsCommandOptionData {
            description,
            name,
            options: Vec::new(),
            required: false,
        })
    }

    /// Builds this into a command option.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> CommandOption {
        CommandOption::SubCommandGroup(self.0)
    }

    /// Adds an option to the sub command group.
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

impl From<SubCommandGroupBuilder> for CommandOption {
    fn from(builder: SubCommandGroupBuilder) -> CommandOption {
        builder.build()
    }
}

/// Create a user option with a builder.
pub struct UserBuilder(BaseCommandOptionData);

impl UserBuilder {
    /// Creates a new default [`UserBuilder`].
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self::_new(name.into(), description.into())
    }

    const fn _new(name: String, description: String) -> Self {
        Self(BaseCommandOptionData {
            description,
            name,
            required: false,
        })
    }

    /// Builds this into a command option.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> CommandOption {
        CommandOption::User(self.0)
    }

    /// Sets whether this option is required.
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

    #[test]
    #[allow(clippy::too_many_lines)]
    fn construct_command_with_builder() {
        // from <https://discord.com/developers/docs/interactions/slash-commands#example-walkthrough>
        let command = CommandBuilder::new("permissions","Get or edit permissions for a user or a role")
            .option(
                SubCommandGroupBuilder::new("user", "Get or edit permissions for a user")
                   .option(
                        SubCommandBuilder::new("get", "Get permissions for a user")
                            .option(UserBuilder::new("user", "The user to get").required(true))
                            .option(ChannelBuilder::new("channel", "The channel permissions to get. If omitted, the guild permissions will be returned")),
                    )
                    .option(SubCommandBuilder::new("edit", "Edit permissions for a user")
                        .option(UserBuilder::new("user", "The user to edit").required(true))
                        .option(ChannelBuilder::new("channel", "The channel permissions to edit. If omitted, the guild permissions will be edited"))),
                    )
            .option(
                SubCommandGroupBuilder::new("role", "Get or edit permissions for a role")
                    .option(SubCommandBuilder::new("get", "Get permissions for a role")
                        .option(RoleBuilder::new("role", "The role to get").required(true))
                        .option(ChannelBuilder::new("channel", "The channel permissions to get. If omitted, the guild permissions will be returned")))
                    .option(SubCommandBuilder::new("edit","Edit permissions for a role")
                        .option(RoleBuilder::new("role", "The role to edit").required(true))
                        .option(ChannelBuilder::new("channel", "The channel permissions to edit. If omitted, the guild permissions will be edited")),
                    )
            )
            .build();
        let command_manual = Command {
            application_id: None,
            guild_id: None,
            name: String::from("permissions"),
            default_permission: None,
            description: String::from("Get or edit permissions for a user or a role"),
            id: None,
            options: vec![
                CommandOption::SubCommandGroup(OptionsCommandOptionData {
                    description: String::from("Get or edit permissions for a user"),
                    name: String::from("user"),
                    options: vec![
                        CommandOption::SubCommand(OptionsCommandOptionData {
                            description: String::from("Get permissions for a user"),
                            name: String::from("get"),
                            options: vec![
                                CommandOption::User(BaseCommandOptionData {
                                    description: String::from("The user to get"),
                                    name: String::from("user"),
                                    required: true,
                                }),
                                CommandOption::Channel(BaseCommandOptionData {
                                    description: String::from("The channel permissions to get. If omitted, the guild permissions will be returned"),
                                    name: String::from("channel"),
                                    required: false,
                                })
                            ],
                            required: false,
                        }),
                        CommandOption::SubCommand(OptionsCommandOptionData {
                            description: String::from("Edit permissions for a user"),
                            name: String::from("edit"),
                            options: vec![
                                CommandOption::User(BaseCommandOptionData {
                                    description: String::from("The user to edit"),
                                    name: String::from("user"),
                                    required: true,
                                }),
                                CommandOption::Channel(BaseCommandOptionData {
                                    description: String::from("The channel permissions to edit. If omitted, the guild permissions will be edited"),
                                    name: String::from("channel"),
                                    required: false,
                                })
                            ],
                            required: false,
                        }),
                    ],
                    required: false,
                }),
                CommandOption::SubCommandGroup(OptionsCommandOptionData {
                    description: String::from("Get or edit permissions for a role"),
                    name: String::from("role"),
                    options: vec![
                        CommandOption::SubCommand(OptionsCommandOptionData {
                            description: String::from("Get permissions for a role"),
                            name: String::from("get"),
                            options: vec![
                                CommandOption::Role(BaseCommandOptionData {
                                    description: String::from("The role to get"),
                                    name: String::from("role"),
                                    required: true,
                                }),
                                CommandOption::Channel(BaseCommandOptionData {
                                    description: String::from("The channel permissions to get. If omitted, the guild permissions will be returned"),
                                    name: String::from("channel"),
                                    required: false,
                                })
                            ],
                            required: false,
                        }),
                        CommandOption::SubCommand(OptionsCommandOptionData {
                            description: String::from("Edit permissions for a role"),
                            name: String::from("edit"),
                            options: vec![
                                CommandOption::Role(BaseCommandOptionData {
                                    description: String::from("The role to edit"),
                                    name: String::from("role"),
                                    required: true,
                                }),
                                CommandOption::Channel(BaseCommandOptionData {
                                    description: String::from("The channel permissions to edit. If omitted, the guild permissions will be edited"),
                                    name: String::from("channel"),
                                    required: false,
                                })
                            ],
                            required: false,
                        }),
                    ],
                    required: false,
                }),
            ],
        };
        assert_eq!(command, command_manual);
    }
}
