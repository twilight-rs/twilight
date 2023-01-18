use crate::channel::ChannelType;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Eq,
    collections::HashMap,
    fmt::{Debug, Formatter, Result as FmtResult},
};

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
    /// Applicable for options of type [`INTEGER`], [`NUMBER`], and [`STRING`].
    ///
    /// Defaults to `false`.
    ///
    /// **Note**: may not be set to `true` if `choices` are set.
    ///
    /// [`INTEGER`]: CommandOptionType::INTEGER
    /// [`NUMBER`]: CommandOptionType::NUMBER
    /// [`STRING`]: CommandOptionType::STRING
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>,
    /// List of possible channel types users can select from.
    ///
    /// Applicable for options of type [`Channel`].
    ///
    /// Defaults to any channel type.
    ///
    /// [`CHANNEL`]: CommandOptionType::CHANNEL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_types: Option<Vec<ChannelType>>,
    /// List of predetermined choices users can select from.
    ///
    /// Applicable for options of type [`INTEGER`], [`NUMBER`], and [`STRING`].
    ///
    /// Defaults to no choices; users may input a value of their choice.
    ///
    /// Must be at most 25 options.
    ///
    /// **Note**: all choices must be of the same type.
    ///
    /// [`INTEGER`]: CommandOptionType::INTEGER
    /// [`NUMBER`]: CommandOptionType::NUMBER
    /// [`STRING`]: CommandOptionType::STRING
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
    /// [`STRING`]: CommandOptionType::STRING
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,
    /// Maximum allowed value.
    ///
    /// Applicable for options of type [`INTEGER`] and [`NUMBER`].
    ///
    /// Defaults to no maximum.
    ///
    /// [`INTEGER`]: CommandOptionType::INTEGER
    /// [`NUMBER`]: CommandOptionType::NUMBER
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<CommandOptionValue>,
    /// Minimum allowed value length.
    ///
    /// Applicable for options of type [`STRING`].
    ///
    /// Defaults to `0`.
    ///
    /// Must be at most `6000`.
    ///
    /// [`STRING`]: CommandOptionType::STRING
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,
    /// Minimum allowed value.
    ///
    /// Applicable for options of type [`INTEGER`] and [`NUMBER`].
    ///
    /// Defaults to no minimum.
    ///
    /// [`INTEGER`]: CommandOptionType::INTEGER
    /// [`NUMBER`]: CommandOptionType::NUMBER
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
    /// Applicable for options of type [`SUB_COMMAND`] and [`SUB_COMMAND_GROUP`].
    ///
    /// Defaults to no options.
    ///
    /// **Note**: at least one option is required and [`SUB_COMMAND_GROUP`] may
    /// only contain [`SUB_COMMAND`]s.
    ///
    /// See [Discord Docs/Subcommands and Subcommand Groups].
    ///
    /// [Discord Docs/Subcommands and Subcommand Groups]: https://discord.com/developers/docs/interactions/application-commands#subcommands-and-subcommand-groups
    /// [`SUB_COMMAND`]: CommandOptionType::SUB_COMMAND
    /// [`SUB_COMMAND_GROUP`]: CommandOptionType::SUB_COMMAND_GROUP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CommandOption>>,
    /// Whether the option is required.
    ///
    /// Applicable for all options except those of type [`SUB_COMMAND`] and
    /// [`SUB_COMMAND_GROUP`].
    ///
    /// Defaults to `false`.
    ///
    /// [`SUB_COMMAND`]: CommandOptionType::SUB_COMMAND
    /// [`SUB_COMMAND_GROUP`]: CommandOptionType::SUB_COMMAND_GROUP
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
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandOptionChoiceData<T> {
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
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CommandOptionType(u8);

impl CommandOptionType {
    pub const SUB_COMMAND: Self = Self::new(1);

    pub const SUB_COMMAND_GROUP: Self = Self::new(2);

    pub const STRING: Self = Self::new(3);

    pub const INTEGER: Self = Self::new(4);

    pub const BOOLEAN: Self = Self::new(5);

    pub const USER: Self = Self::new(6);

    pub const CHANNEL: Self = Self::new(7);

    pub const ROLE: Self = Self::new(8);

    pub const MENTIONABLE: Self = Self::new(9);

    pub const NUMBER: Self = Self::new(10);

    pub const ATTACHMENT: Self = Self::new(11);

    /// Create a new command option type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`STRING`][`Self::STRING`].
    pub const fn new(command_option_type: u8) -> Self {
        Self(command_option_type)
    }

    /// Retrieve the value of the command option type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::application::command::CommandOptionType;
    ///
    /// assert_eq!(4, CommandOptionType::INTEGER.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::ATTACHMENT => "ATTACHMENT",
            Self::BOOLEAN => "BOOLEAN",
            Self::CHANNEL => "CHANNEL",
            Self::INTEGER => "INTEGER",
            Self::MENTIONABLE => "MENTIONABLE",
            Self::NUMBER => "NUMBER",
            Self::ROLE => "ROLE",
            Self::STRING => "STRING",
            Self::SUB_COMMAND => "SUB_COMMAND",
            Self::SUB_COMMAND_GROUP => "SUB_COMMAND_GROUP",
            Self::USER => "USER",
            _ => return None,
        })
    }
}

impl Debug for CommandOptionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("CommandOptionType")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("CommandOptionType").field(&self.0).finish()
        }
    }
}

impl From<u8> for CommandOptionType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<CommandOptionType> for u8 {
    fn from(value: CommandOptionType) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::CommandOptionType;
    use serde_test::Token;

    const MAP: &[(CommandOptionType, u8)] = &[
        (CommandOptionType::SUB_COMMAND, 1),
        (CommandOptionType::SUB_COMMAND_GROUP, 2),
        (CommandOptionType::STRING, 3),
        (CommandOptionType::INTEGER, 4),
        (CommandOptionType::BOOLEAN, 5),
        (CommandOptionType::USER, 6),
        (CommandOptionType::CHANNEL, 7),
        (CommandOptionType::ROLE, 8),
        (CommandOptionType::MENTIONABLE, 9),
        (CommandOptionType::NUMBER, 10),
        (CommandOptionType::ATTACHMENT, 11),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "CommandOptionType",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, CommandOptionType::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
