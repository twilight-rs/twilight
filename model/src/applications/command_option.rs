use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// CommandOption is an option that can be supplied to an ApplicationCommand, or
/// nested under another CommandOption of type SubCommand or SubCommandGroup.
///
/// Choices and options are mutually exclusive.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CommandOption {
    #[serde(rename = "type")]
    pub kind: CommandOptionType,
    /// The name of the option. It must be 32 characters or less.
    pub name: String,
    /// A description of the option. It must be 100 characters or less.
    pub description: String,
    /// The first required option that you wish the user to complete. Only one
    /// CommandOption may be default per command.
    ///
    /// For example, given a simple kick command:
    ///     `/kick @user [reason]`
    /// You would make the `@user` option default, as it's the first thing you
    /// would like the user to complete.
    ///
    /// NOTE: THIS IS CURRENTLY BROKEN. IT ALWAYS ERRORS WHEN SET.
    #[serde(default)]
    pub default: bool,
    /// Whether or not the option is required to be completed by a user.
    #[serde(default)]
    pub required: bool,
    /// If the CommandOption is of type String or Int, predetermined choices may
    /// be defined for a user to select. When completing this option, the user is
    /// prompted with a selector of all available choices.
    #[serde(default)]
    pub choices: Vec<CommandOptionChoice>,
    /// If the CommandOption is of type SubCommand or SubCommandGroup, the nested
    /// options are supplied here.
    #[serde(default)]
    pub options: Vec<CommandOption>,
}

/// CommandOptionType specifies the type of a CommandOption.
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum CommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
}

/// Specifies an option that a user must choose from in a dropdown.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#applicationcommandoptionchoice
#[serde(untagged)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum CommandOptionChoice {
    String { name: String, value: String },
    Int { name: String, value: i64 },
}
