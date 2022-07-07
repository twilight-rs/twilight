use crate::channel::ChannelType;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{cmp::Eq, collections::HashMap};

/// Option for a [`Command`].
///
/// Can be nested under other [`CommandOption`]s of type [`SubCommand`] and
/// [`SubCommandGroup`].
///
/// [`Command`]: super::Command
/// [`SubCommand`]: CommandOptionType::SubCommand
/// [`SubCommandGroup`]: CommandOptionType::SubCommandGroup
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CommandOption {
    /// Whether the command supports autocomplete.
    ///
    /// May not be set to `true` if `choices` are set.
    ///
    /// Applicable for [`CommandOption`]s of type [`Integer`], [`Number`], and
    /// [`String`].
    ///
    /// [`Integer`]: CommandOptionType::Integer
    /// [`Number`]: CommandOptionType::Number
    /// [`String`]: CommandOptionType::String
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>,
    /// List of possible channel types users can select from.
    ///
    /// The user is free to choose any channel type if no types are configured.
    ///
    /// Applicable for [`CommandOption`] of type [`Channel`].
    ///
    /// [`Channel`]: CommandOptionType::Channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_types: Option<Vec<ChannelType>>,
    /// List of predetermined choices users can select from.
    ///
    /// The user must input a value manually if no predetermined choices are
    /// configured.
    ///
    /// All choices must be the same type.
    ///
    /// Applicable for [`CommandOption`]s of type [`Integer`], [`Number`], and
    /// [`String`].
    ///
    /// [`Integer`]: CommandOptionType::Integer
    /// [`Number`]: CommandOptionType::Number
    /// [`String`]: CommandOptionType::String
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<CommandOptionChoice>>,
    /// Description of the option. Must be 100 characters or less.
    pub description: String,
    /// Localization dictionary for the `description` field.
    ///
    /// See [Discord Docs/Localization].
    ///
    /// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    /// Type of option.
    #[serde(rename = "type")]
    pub kind: CommandOptionType,
    /// Maximum allowed length.
    ///
    /// Must be at least `1` and at most `6000`.
    ///
    /// Applicable for [`CommandOption`]s of type [`String`].
    ///
    /// [`String`]: CommandOptionType::String
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,
    /// Minimum allowed length.
    ///
    /// Must be at most `6000`.
    ///
    /// Applicable for [`CommandOption`]s of type [`String`].
    ///
    /// [`String`]: CommandOptionType::String
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,
    /// Maximum allowed value.
    ///
    /// Applicable for [`CommandOption`]s of type [`Integer`] and [`Number`].
    ///
    /// [`Integer`]: CommandOptionType::Integer
    /// [`Number`]: CommandOptionType::Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<CommandOptionValue>,
    /// Minimum allowed value.
    ///
    /// Applicable for [`CommandOption`]s of type [`Integer`] and [`Number`].
    ///
    /// [`Integer`]: CommandOptionType::Integer
    /// [`Number`]: CommandOptionType::Number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<CommandOptionValue>,
    /// Name of the option. Must be 32 characters or less.
    pub name: String,
    /// Localization dictionary for the `name` field.
    ///
    /// Keys should be valid locales. See [Discord Docs/Locales],
    /// [Discord Docs/Localization].
    ///
    /// [Discord Docs/Locales]: https://discord.com/developers/docs/reference#locales
    /// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,
    /// Nested [`CommandOption`]s.
    ///
    /// Applicable for [`CommandOption`]s of type [`SubCommand`] and
    /// [`SubCommandGroup`]. Note that [`SubCommandGroup`] may only contain
    /// [`SubCommand`]s.
    ///
    /// [`SubCommand`]: CommandOptionType::SubCommand
    /// [`SubCommandGroup`]: CommandOptionType::SubCommandGroup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CommandOption>>,
    /// Whether the option is required to be completed by a user.
    ///
    /// Defaults to `false`.
    ///
    /// Applicable for all [`CommandOption`]s, except those of type
    /// [`SubCommand`] and [`SubCommandGroup`].
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
#[serde(untagged)]
pub enum CommandOptionChoice {
    /// String choice.
    String(CommandOptionChoiceData<String>),
    /// Integer choice.
    Integer(CommandOptionChoiceData<i64>),
    /// Number choice.
    Number(CommandOptionChoiceData<f64>),
}

/// Data of [`CommandOptionChoice`].
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CommandOptionChoiceData<T> {
    /// Name of the choice. Must be 100 characters or less.
    pub name: String,
    /// Localization dictionary for the `name` field.
    ///
    /// Keys should be valid locales. See [Discord Docs/Locales],
    /// [Discord Docs/Localization].
    ///
    /// [Discord Docs/Locales]: https://discord.com/developers/docs/reference#locales
    /// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
    pub name_localizations: Option<HashMap<String, String>>,
    /// Value of the choice. Must be 100 characters or less if it is a string.
    pub value: T,
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
