use crate::channel::ChannelType;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{cmp::Eq, collections::HashMap};

/// Option for a [`Command`].
///
/// Fields not applicable to the command option's [`CommandOptionType`] should
/// be set to [`None`].
///
/// Fields' default values may be used by setting them to [`None`].
///
/// Choices, descriptions and names may be localized in any [available locale],
/// see [Discord Docs/Localization].
///
/// [available locale]: https://discord.com/developers/docs/reference#locales
/// [`Command`]: super::Command
/// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CommandOption {
    /// Whether the command supports autocomplete.
    ///
    /// Applicable for options of type [`Integer`], [`Number`], and [`String`].
    ///
    /// Defaults to `false`.
    ///
    /// **Note**: may not be set to `true` if `choices` are set.
    ///
    /// [`Integer`]: CommandOptionType::Integer
    /// [`Number`]: CommandOptionType::Number
    /// [`String`]: CommandOptionType::String
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>,
    /// List of possible channel types users can select from.
    ///
    /// Applicable for options of type [`Channel`].
    ///
    /// Defaults to any channel type.
    ///
    /// [`Channel`]: CommandOptionType::Channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_types: Option<Vec<ChannelType>>,
    /// List of predetermined choices users can select from.
    ///
    /// Applicable for options of type [`Integer`], [`Number`], and [`String`].
    ///
    /// Defaults to no choices; users may input a value of their choice.
    ///
    /// Must be at most 25 options.
    ///
    /// **Note**: all choices must be of the same type.
    ///
    /// [`Integer`]: CommandOptionType::Integer
    /// [`Number`]: CommandOptionType::Number
    /// [`String`]: CommandOptionType::String
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<CommandOptionChoice>>,
    /// Description of the option. Must be 100 characters or less.
    pub description: String,
    /// Localization dictionary for the [`description`] field.
    ///
    /// Defaults to no localizations.
    ///
    /// Keys must be valid locales and values must be 100 characters or less.
    ///
    /// [`description`]: Self::description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    /// Type of option.
    #[serde(rename = "type")]
    pub kind: CommandOptionType,
    /// Maximum allowed value length.
    ///
    /// Applicable for options of type [`String`].
    ///
    /// Defaults to `6000`.
    ///
    /// Must be at least `1` and at most `6000`.
    ///
    /// [`String`]: CommandOptionType::String
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,
    /// Maximum allowed value.
    ///
    /// Applicable for options of type [`Integer`] and [`Number`].
    ///
    /// Defaults to no maximum.
    ///
    /// [`Integer`]: CommandOptionType::Integer
    /// [`Number`]: CommandOptionType::Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<CommandOptionValue>,
    /// Minimum allowed value length.
    ///
    /// Applicable for options of type [`String`].
    ///
    /// Defaults to `0`.
    ///
    /// Must be at most `6000`.
    ///
    /// [`String`]: CommandOptionType::String
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,
    /// Minimum allowed value.
    ///
    /// Applicable for options of type [`Integer`] and [`Number`].
    ///
    /// Defaults to no minimum.
    ///
    /// [`Integer`]: CommandOptionType::Integer
    /// [`Number`]: CommandOptionType::Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<CommandOptionValue>,
    /// Name of the option. Must be 32 characters or less.
    pub name: String,
    /// Localization dictionary for the [`name`] field.
    ///
    /// Defaults to no localizations.
    ///
    /// Keys must be valid locales and values must be 32 characters or less.
    ///
    /// [`name`]: Self::name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,
    /// Nested options.
    ///
    /// Applicable for options of type [`SubCommand`] and [`SubCommandGroup`].
    ///
    /// Defaults to no options.
    ///
    /// **Note**: at least one option is required and [`SubCommandGroup`] may
    /// only contain [`SubCommand`]s.
    ///
    /// See [Discord Docs/Subcommands and Subcommand Groups].
    ///
    /// [Discord Docs/Subcommands and Subcommand Groups]: https://discord.com/developers/docs/interactions/application-commands#subcommands-and-subcommand-groups
    /// [`SubCommand`]: CommandOptionType::SubCommand
    /// [`SubCommandGroup`]: CommandOptionType::SubCommandGroup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CommandOption>>,
    /// Whether the option is required.
    ///
    /// Applicable for all options except those of type [`SubCommand`] and
    /// [`SubCommandGroup`].
    ///
    /// Defaults to `false`.
    ///
    /// [`SubCommand`]: CommandOptionType::SubCommand
    /// [`SubCommandGroup`]: CommandOptionType::SubCommandGroup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// A predetermined choice users can select.
///
/// Note that the right variant must be selected based on the
/// [`CommandOption`]'s [`CommandOptionType`].
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CommandOptionChoice {
    /// Name of the choice. Must be 100 characters or less.
    pub name: String,
    /// Localization dictionary for the [`name`] field.
    ///
    /// Defaults to no localizations.
    ///
    /// Keys must be valid locales and values must be 100 characters or less.
    ///
    /// See [`CommandOption`]'s documentation for more info.
    ///
    /// [`name`]: Self::name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,

    /// Value of the choice.
    pub value: CommandOptionChoiceValue,
}

/// The value of a [`CommandOptionChoice`].
///
/// Note that the right variant must be selected based on the
/// [`CommandOption`]'s [`CommandOptionType`].
///
/// See [`CommandOptionChoice`]'s documentation for more info.
///
/// [`CommandOption`]: CommandOption
/// [`CommandOptionChoice`]: CommandOptionChoice
/// [`CommandOptionType`]: CommandOptionType
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CommandOptionChoiceValue {
    /// String choice. Must be 100 characters or less.
    String(String),
    /// Integer choice.
    Integer(i64),
    /// Number choice.
    Number(f64),
}

/// Type used in the `max_value` and `min_value` [`CommandOption`] field.
///
/// Note that the right variant must be selected based on the
/// [`CommandOption`]'s [`CommandOptionType`].
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CommandOptionValue {
    /// Integer type.
    Integer(i64),
    /// Number type.
    Number(f64),
}

/// Type of a [`CommandOption`].
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
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
    Number = 10,
    Attachment = 11,
}

impl CommandOptionType {
    pub const fn kind(self) -> &'static str {
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
            CommandOptionType::Number => "Number",
            CommandOptionType::Attachment => "Attachment",
        }
    }
}
