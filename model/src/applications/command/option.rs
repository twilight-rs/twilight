use std::convert::From;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
struct CommandOptionEnvelope {
    #[serde(default)]
    pub choices: Option<Vec<CommandOptionChoice>>,
    pub description: String,
    #[serde(rename = "type")]
    pub kind: CommandOptionType,
    pub name: String,
    #[serde(default)]
    pub options: Option<Vec<CommandOption>>,
    #[serde(default)]
    pub required: bool,
}

/// Option for a [`Command`].
///
/// It can also be nested under another [`CommandOption`] of type [`SubCommand`]
/// or [`SubCommandGroup`].
///
/// Choices and options are mutually exclusive.
///
/// [`Command`]: super::Command
/// [`SubCommand`]: CommandOption::SubCommand
/// [`SubCommandGroup`]: CommandOption::SubCommandGroup
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CommandOption {
    SubCommand(OptionsCommandOptionData),
    SubCommandGroup(OptionsCommandOptionData),
    String(ChoiceCommandOptionData),
    Integer(ChoiceCommandOptionData),
    Boolean(BaseCommandOptionData),
    User(BaseCommandOptionData),
    Channel(BaseCommandOptionData),
    Role(BaseCommandOptionData),
    Mentionable(BaseCommandOptionData),
}

impl CommandOption {
    pub fn kind(&self) -> CommandOptionType {
        match self {
            CommandOption::SubCommand(_) => CommandOptionType::SubCommand,
            CommandOption::SubCommandGroup(_) => CommandOptionType::SubCommandGroup,
            CommandOption::String(_) => CommandOptionType::String,
            CommandOption::Integer(_) => CommandOptionType::Integer,
            CommandOption::Boolean(_) => CommandOptionType::Boolean,
            CommandOption::User(_) => CommandOptionType::User,
            CommandOption::Channel(_) => CommandOptionType::Channel,
            CommandOption::Role(_) => CommandOptionType::Role,
            CommandOption::Mentionable(_) => CommandOptionType::Mentionable,
        }
    }
}

impl<'de> Deserialize<'de> for CommandOption {
    fn deserialize<D>(deserializer: D) -> Result<CommandOption, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CommandOptionEnvelope::deserialize(deserializer)?.into())
    }
}

impl Serialize for CommandOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let base = match self {
            CommandOption::SubCommand(d) | CommandOption::SubCommandGroup(d) => d.base(),
            CommandOption::String(d) | CommandOption::Integer(d) => d.base(),
            CommandOption::Boolean(d)
            | CommandOption::User(d)
            | CommandOption::Channel(d)
            | CommandOption::Role(d)
            | CommandOption::Mentionable(d) => d.clone(),
        };

        let choices = match self {
            CommandOption::String(d) | CommandOption::Integer(d) => Some(d.choices.clone()),
            _ => None,
        };

        let options = match self {
            CommandOption::SubCommand(d) | CommandOption::SubCommandGroup(d) => {
                Some(d.options.clone())
            }
            _ => None,
        };

        CommandOptionEnvelope {
            choices,
            description: base.description,
            kind: self.kind(),
            name: base.name,
            options,
            required: base.required,
        }
        .serialize(serializer)
    }
}

impl From<CommandOptionEnvelope> for CommandOption {
    fn from(mut envelope: CommandOptionEnvelope) -> Self {
        match envelope.kind {
            CommandOptionType::SubCommand => {
                CommandOption::SubCommand(OptionsCommandOptionData::from((
                    envelope.options.take().unwrap_or_default(),
                    envelope.into(),
                )))
            }
            CommandOptionType::SubCommandGroup => {
                CommandOption::SubCommandGroup(OptionsCommandOptionData::from((
                    envelope.options.take().unwrap_or_default(),
                    envelope.into(),
                )))
            }
            CommandOptionType::String => CommandOption::String(ChoiceCommandOptionData::from((
                envelope.choices.take().unwrap_or_default(),
                envelope.into(),
            ))),
            CommandOptionType::Integer => CommandOption::Integer(ChoiceCommandOptionData::from((
                envelope.choices.take().unwrap_or_default(),
                envelope.into(),
            ))),
            CommandOptionType::Boolean => CommandOption::Boolean(envelope.into()),
            CommandOptionType::User => CommandOption::User(envelope.into()),
            CommandOptionType::Channel => CommandOption::Channel(envelope.into()),
            CommandOptionType::Role => CommandOption::Role(envelope.into()),
            CommandOptionType::Mentionable => CommandOption::Mentionable(envelope.into()),
        }
    }
}

/// Data supplied to a [`CommandOption`] of type [`Boolean`], [`User`],
/// [`Channel`], or [`Role`].
///
/// [`Boolean`]: CommandOption::Boolean
/// [`User`]: CommandOption::User
/// [`Channel`]: CommandOption::Channel
/// [`Role`]: CommandOption::Role
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BaseCommandOptionData {
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Name of the option. It must be 32 characters or less.
    pub name: String,
    /// Whether the option is required to be completed by a user.
    #[serde(default)]
    pub required: bool,
}

impl From<CommandOptionEnvelope> for BaseCommandOptionData {
    fn from(envelope: CommandOptionEnvelope) -> Self {
        Self {
            description: envelope.description,
            name: envelope.name,
            required: envelope.required,
        }
    }
}

/// Data supplied to a [`CommandOption`] of type [`SubCommand`] or
/// [`SubCommandGroup`].
///
/// [`SubCommand`]: CommandOption::SubCommand
/// [`SubCommandGroup`]: CommandOption::SubCommandGroup
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OptionsCommandOptionData {
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Name of the option. It must be 32 characters or less.
    pub name: String,
    /// Used for specifying the nested options in a [`SubCommand`] or
    /// [`SubCommandGroup`].
    ///
    /// [`SubCommand`]: CommandOptionType::SubCommand
    /// [`SubCommandGroup`]: CommandOptionType::SubCommandGroup
    #[serde(default)]
    pub options: Vec<CommandOption>,
    /// Whether the option is required to be completed by a user.
    #[serde(default)]
    pub required: bool,
}

impl From<(Vec<CommandOption>, BaseCommandOptionData)> for OptionsCommandOptionData {
    fn from(opt: (Vec<CommandOption>, BaseCommandOptionData)) -> Self {
        Self {
            description: opt.1.description,
            name: opt.1.name,
            options: opt.0,
            required: opt.1.required,
        }
    }
}

impl OptionsCommandOptionData {
    fn base(&self) -> BaseCommandOptionData {
        BaseCommandOptionData {
            description: self.description.clone(),
            name: self.name.clone(),
            required: self.required,
        }
    }
}

/// Data supplied to a [`CommandOption`] of type [`String`] or [`Integer`].
///
/// [`String`]: CommandOption::String
/// [`Integer`]: CommandOption::Integer
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChoiceCommandOptionData {
    /// Predetermined choices may be defined for a user to select.
    ///
    /// When completing this option, the user is prompted with a selector of all
    /// available choices.
    #[serde(default)]
    pub choices: Vec<CommandOptionChoice>,
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Name of the option. It must be 32 characters or less.
    pub name: String,
    /// Whether or not the option is required to be completed by a user.
    #[serde(default)]
    pub required: bool,
}

impl From<(Vec<CommandOptionChoice>, BaseCommandOptionData)> for ChoiceCommandOptionData {
    fn from(opt: (Vec<CommandOptionChoice>, BaseCommandOptionData)) -> Self {
        Self {
            choices: opt.0,
            description: opt.1.description,
            name: opt.1.name,
            required: opt.1.required,
        }
    }
}

impl ChoiceCommandOptionData {
    fn base(&self) -> BaseCommandOptionData {
        BaseCommandOptionData {
            name: self.name.clone(),
            description: self.description.clone(),
            required: self.required,
        }
    }
}

/// Specifies an option that a user must choose from in a dropdown.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#applicationcommandoptionchoice
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CommandOptionChoice {
    String { name: String, value: String },
    Int { name: String, value: i64 },
}

/// Type of a [`CommandOption`].
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
    Mentionable = 9,
}

impl CommandOptionType {
    pub fn kind(self) -> &'static str {
        match self {
            CommandOptionType::SubCommand => "SubCommand",
            CommandOptionType::SubCommandGroup => "SubCommandGroup",
            CommandOptionType::String => "String",
            CommandOptionType::Integer => "Integer",
            CommandOptionType::Boolean => "Boolean",
            CommandOptionType::User => "User",
            CommandOptionType::Channel => "Channel",
            CommandOptionType::Role => "Role",
            CommandOptionType::Mentionable => "Mentionable",
        }
    }
}
