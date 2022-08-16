use crate::{channel::ChannelType, util::is_false};
use serde::{
    de::{Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{
    cmp::Eq,
    collections::HashMap,
    fmt::{Formatter, Result as FmtResult},
};

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
#[derive(Clone, Debug, PartialEq)]
pub enum CommandOption {
    SubCommand(OptionsCommandOptionData),
    SubCommandGroup(OptionsCommandOptionData),
    String(ChoiceCommandOptionData),
    Integer(NumberCommandOptionData),
    Boolean(BaseCommandOptionData),
    User(BaseCommandOptionData),
    Channel(ChannelCommandOptionData),
    Role(BaseCommandOptionData),
    Mentionable(BaseCommandOptionData),
    Number(NumberCommandOptionData),
    Attachment(BaseCommandOptionData),
}

impl CommandOption {
    pub const fn kind(&self) -> CommandOptionType {
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
            CommandOption::Number(_) => CommandOptionType::Number,
            CommandOption::Attachment(_) => CommandOptionType::Attachment,
        }
    }

    /// Whether the command supports autocomplete.
    pub const fn is_autocomplete(&self) -> bool {
        match self {
            CommandOption::String(data) => data.autocomplete,
            CommandOption::Integer(data) | CommandOption::Number(data) => data.autocomplete,
            CommandOption::Boolean(_)
            | CommandOption::User(_)
            | CommandOption::Role(_)
            | CommandOption::Mentionable(_)
            | CommandOption::SubCommand(_)
            | CommandOption::SubCommandGroup(_)
            | CommandOption::Channel(_)
            | CommandOption::Attachment(_) => false,
        }
    }

    pub const fn is_required(&self) -> bool {
        match self {
            CommandOption::SubCommand(_) | CommandOption::SubCommandGroup(_) => false,
            CommandOption::String(data) => data.required,
            CommandOption::Integer(data) | CommandOption::Number(data) => data.required,
            CommandOption::Channel(data) => data.required,
            CommandOption::Boolean(data)
            | CommandOption::User(data)
            | CommandOption::Role(data)
            | CommandOption::Mentionable(data)
            | CommandOption::Attachment(data) => data.required,
        }
    }
}

impl<'de> Deserialize<'de> for CommandOption {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(OptionVisitor)
    }
}

#[derive(Serialize)]
struct CommandOptionEnvelope<'ser> {
    #[serde(skip_serializing_if = "is_false")]
    autocomplete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_types: Option<&'ser [ChannelType]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    choices: Option<&'ser [CommandOptionChoice]>,
    description: &'ser str,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_value: Option<CommandOptionValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_value: Option<CommandOptionValue>,
    name: &'ser str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<&'ser [CommandOption]>,
    #[serde(skip_serializing_if = "is_false")]
    required: bool,
    #[serde(rename = "type")]
    kind: CommandOptionType,
}

impl Serialize for CommandOption {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let envelope = match self {
            Self::SubCommand(data) | Self::SubCommandGroup(data) => CommandOptionEnvelope {
                autocomplete: false,
                channel_types: None,
                choices: None,
                description: data.description.as_ref(),
                max_length: None,
                max_value: None,
                min_length: None,
                min_value: None,
                name: data.name.as_ref(),
                options: Some(data.options.as_ref()),
                required: false,
                kind: self.kind(),
            },
            Self::String(data) => CommandOptionEnvelope {
                autocomplete: data.autocomplete,
                channel_types: None,
                choices: Some(data.choices.as_ref()),
                description: data.description.as_ref(),
                max_length: data.max_length,
                max_value: None,
                min_length: data.min_length,
                min_value: None,
                name: data.name.as_ref(),
                options: None,
                required: data.required,
                kind: self.kind(),
            },
            Self::Integer(data) | Self::Number(data) => CommandOptionEnvelope {
                autocomplete: data.autocomplete,
                channel_types: None,
                choices: Some(data.choices.as_ref()),
                description: data.description.as_ref(),
                max_length: None,
                max_value: data.max_value,
                min_length: None,
                min_value: data.min_value,
                name: data.name.as_ref(),
                options: None,
                required: data.required,
                kind: self.kind(),
            },
            Self::Channel(data) => CommandOptionEnvelope {
                autocomplete: false,
                channel_types: Some(data.channel_types.as_ref()),
                choices: None,
                description: data.description.as_ref(),
                max_length: None,
                max_value: None,
                min_length: None,
                min_value: None,
                name: data.name.as_ref(),
                options: None,
                required: data.required,
                kind: self.kind(),
            },
            Self::Boolean(data)
            | Self::User(data)
            | Self::Role(data)
            | Self::Mentionable(data)
            | Self::Attachment(data) => CommandOptionEnvelope {
                autocomplete: false,
                channel_types: None,
                choices: None,
                description: data.description.as_ref(),
                max_length: None,
                max_value: None,
                min_length: None,
                min_value: None,
                name: data.name.as_ref(),
                options: None,
                required: data.required,
                kind: self.kind(),
            },
        };

        envelope.serialize(serializer)
    }
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum OptionField {
    Autocomplete,
    ChannelTypes,
    Choices,
    Description,
    DescriptionLocalizations,
    MaxLength,
    MaxValue,
    MinLength,
    MinValue,
    Name,
    NameLocalizations,
    Options,
    Required,
    Type,
}

struct OptionVisitor;

impl<'de> Visitor<'de> for OptionVisitor {
    type Value = CommandOption;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct CommandOption")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut autocomplete: Option<bool> = None;
        let mut channel_types: Option<Option<Vec<ChannelType>>> = None;
        let mut choices: Option<Option<Vec<CommandOptionChoice>>> = None;
        let mut description: Option<String> = None;
        let mut description_localizations: Option<Option<HashMap<String, String>>> = None;
        let mut kind: Option<CommandOptionType> = None;
        let mut max_length: Option<u16> = None;
        let mut max_value: Option<Option<CommandOptionValue>> = None;
        let mut min_length: Option<u16> = None;
        let mut min_value: Option<Option<CommandOptionValue>> = None;
        let mut name: Option<String> = None;
        let mut name_localizations: Option<Option<HashMap<String, String>>> = None;
        let mut options: Option<Option<Vec<CommandOption>>> = None;
        let mut required: Option<bool> = None;

        let span = tracing::trace_span!("deserializing command option");
        let _span_enter = span.enter();

        loop {
            let span_child = tracing::trace_span!("iterating over command option");
            let _span_child_enter = span_child.enter();

            let key = match map.next_key() {
                Ok(Some(key)) => {
                    tracing::trace!(?key, "found key");

                    key
                }
                Ok(None) => break,
                Err(why) => {
                    map.next_value::<IgnoredAny>()?;

                    tracing::trace!("ran into an unknown key: {why:?}");

                    continue;
                }
            };

            match key {
                OptionField::Autocomplete => {
                    if autocomplete.is_some() {
                        return Err(DeError::duplicate_field("autocomplete"));
                    }

                    autocomplete = Some(map.next_value()?);
                }
                OptionField::ChannelTypes => {
                    if channel_types.is_some() {
                        return Err(DeError::duplicate_field("channel_types"));
                    }

                    channel_types = Some(map.next_value()?);
                }
                OptionField::Choices => {
                    if choices.is_some() {
                        return Err(DeError::duplicate_field("choices"));
                    }

                    choices = Some(map.next_value()?);
                }
                OptionField::Description => {
                    if description.is_some() {
                        return Err(DeError::duplicate_field("description"));
                    }

                    description = Some(map.next_value()?);
                }
                OptionField::DescriptionLocalizations => {
                    if description_localizations.is_some() {
                        return Err(DeError::duplicate_field("description_localizations"));
                    }

                    description_localizations = Some(map.next_value()?);
                }
                OptionField::MaxLength => {
                    if max_length.is_some() {
                        return Err(DeError::duplicate_field("max_length"));
                    }

                    max_length = map.next_value()?;
                }
                OptionField::MaxValue => {
                    if max_value.is_some() {
                        return Err(DeError::duplicate_field("max_value"));
                    }

                    max_value = Some(map.next_value()?);
                }
                OptionField::MinLength => {
                    if min_length.is_some() {
                        return Err(DeError::duplicate_field("min_length"));
                    }

                    min_length = map.next_value()?;
                }
                OptionField::MinValue => {
                    if min_value.is_some() {
                        return Err(DeError::duplicate_field("min_value"));
                    }

                    min_value = Some(map.next_value()?);
                }
                OptionField::Name => {
                    if name.is_some() {
                        return Err(DeError::duplicate_field("name"));
                    }

                    name = Some(map.next_value()?);
                }
                OptionField::NameLocalizations => {
                    if name_localizations.is_some() {
                        return Err(DeError::duplicate_field("name_localizations"));
                    }

                    name_localizations = Some(map.next_value()?);
                }
                OptionField::Options => {
                    if options.is_some() {
                        return Err(DeError::duplicate_field("options"));
                    }

                    options = Some(map.next_value()?);
                }
                OptionField::Required => {
                    if required.is_some() {
                        return Err(DeError::duplicate_field("required"));
                    }

                    required = Some(map.next_value()?);
                }
                OptionField::Type => {
                    if kind.is_some() {
                        return Err(DeError::duplicate_field("type"));
                    }

                    kind = Some(map.next_value()?);
                }
            }
        }

        let description = description.ok_or_else(|| DeError::missing_field("description"))?;
        let kind = kind.ok_or_else(|| DeError::missing_field("type"))?;
        let name = name.ok_or_else(|| DeError::missing_field("name"))?;

        tracing::trace!(
            %description,
            ?kind,
            %name,
            "common fields of all variants exist"
        );

        let autocomplete = autocomplete.unwrap_or_default();
        let required = required.unwrap_or_default();

        Ok(match kind {
            CommandOptionType::SubCommand => {
                let options = options.flatten().unwrap_or_default();

                CommandOption::SubCommand(OptionsCommandOptionData {
                    description,
                    description_localizations: description_localizations.flatten(),
                    name,
                    name_localizations: name_localizations.flatten(),
                    options,
                })
            }
            CommandOptionType::SubCommandGroup => {
                let options = options.flatten().unwrap_or_default();

                CommandOption::SubCommandGroup(OptionsCommandOptionData {
                    description,
                    description_localizations: description_localizations.flatten(),
                    name,
                    name_localizations: name_localizations.flatten(),
                    options,
                })
            }
            CommandOptionType::String => CommandOption::String(ChoiceCommandOptionData {
                autocomplete,
                choices: choices.flatten().unwrap_or_default(),
                description,
                description_localizations: description_localizations.flatten(),
                max_length,
                min_length,
                name,
                name_localizations: name_localizations.flatten(),
                required,
            }),
            CommandOptionType::Integer => CommandOption::Integer(NumberCommandOptionData {
                autocomplete,
                choices: choices.flatten().unwrap_or_default(),
                description,
                description_localizations: description_localizations.flatten(),
                max_value: max_value.flatten(),
                min_value: min_value.flatten(),
                name,
                name_localizations: name_localizations.flatten(),
                required,
            }),
            CommandOptionType::Boolean => CommandOption::Boolean(BaseCommandOptionData {
                description,
                description_localizations: description_localizations.flatten(),
                name,
                name_localizations: name_localizations.flatten(),
                required,
            }),
            CommandOptionType::User => CommandOption::User(BaseCommandOptionData {
                description,
                description_localizations: description_localizations.flatten(),
                name,
                name_localizations: name_localizations.flatten(),
                required,
            }),
            CommandOptionType::Channel => CommandOption::Channel(ChannelCommandOptionData {
                channel_types: channel_types.flatten().unwrap_or_default(),
                description,
                description_localizations: description_localizations.flatten(),
                name,
                name_localizations: name_localizations.flatten(),
                required,
            }),
            CommandOptionType::Role => CommandOption::Role(BaseCommandOptionData {
                description,
                description_localizations: description_localizations.flatten(),
                name,
                name_localizations: name_localizations.flatten(),
                required,
            }),
            CommandOptionType::Mentionable => CommandOption::Mentionable(BaseCommandOptionData {
                description,
                description_localizations: description_localizations.flatten(),
                name,
                name_localizations: name_localizations.flatten(),
                required,
            }),
            CommandOptionType::Number => CommandOption::Number(NumberCommandOptionData {
                autocomplete,
                choices: choices.flatten().unwrap_or_default(),
                description,
                description_localizations: description_localizations.flatten(),
                max_value: max_value.flatten(),
                min_value: min_value.flatten(),
                name,
                name_localizations: name_localizations.flatten(),
                required,
            }),
            CommandOptionType::Attachment => CommandOption::Attachment(BaseCommandOptionData {
                description,
                description_localizations: description_localizations.flatten(),
                name,
                name_localizations: name_localizations.flatten(),
                required,
            }),
        })
    }
}

/// Data supplied to a [`CommandOption`] of type [`Boolean`], [`User`],
/// [`Channel`], [`Role`], or [`Mentionable`].
///
/// [`Boolean`]: CommandOption::Boolean
/// [`User`]: CommandOption::User
/// [`Channel`]: CommandOption::Channel
/// [`Role`]: CommandOption::Role
/// [`Mentionable`]: CommandOption::Mentionable
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct BaseCommandOptionData {
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Localization dictionary for the `description` field.
    ///
    /// See [Discord Docs/Localization].
    ///
    /// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    /// Name of the option. It must be 32 characters or less.
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
    /// Whether the option is required to be completed by a user.
    #[serde(default)]
    pub required: bool,
}

/// Data supplied to a [`CommandOption`] of type [`SubCommand`] or
/// [`SubCommandGroup`].
///
/// [`SubCommand`]: CommandOption::SubCommand
/// [`SubCommandGroup`]: CommandOption::SubCommandGroup
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OptionsCommandOptionData {
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Localization dictionary for the `description` field.
    ///
    /// See [Discord Docs/Localization].
    ///
    /// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    /// Name of the option. It must be 32 characters or less.
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
    /// Used for specifying the nested options in a [`SubCommand`] or
    /// [`SubCommandGroup`].
    ///
    /// [`SubCommand`]: CommandOptionType::SubCommand
    /// [`SubCommandGroup`]: CommandOptionType::SubCommandGroup
    #[serde(default)]
    pub options: Vec<CommandOption>,
}

/// Data supplied to a [`CommandOption`] of type [`String`].
///
/// [`String`]: CommandOption::String
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ChoiceCommandOptionData {
    /// Whether the command supports autocomplete.
    #[serde(default)]
    pub autocomplete: bool,
    /// Predetermined choices may be defined for a user to select.
    ///
    /// When completing this option, the user is prompted with a selector of all
    /// available choices.
    ///
    /// If no choices are available, the user must input a value manually.
    #[serde(default)]
    pub choices: Vec<CommandOptionChoice>,
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Localization dictionary for the `description` field.
    ///
    /// See [Discord Docs/Localization].
    ///
    /// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    /// Maximum allowed length.
    ///
    /// Must be at least `1` and at most `6000`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,
    /// Minimum allowed length.
    ///
    /// Must be at most `6000`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,
    /// Name of the option. It must be 32 characters or less.
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
    /// Whether or not the option is required to be completed by a user.
    #[serde(default)]
    pub required: bool,
}

/// Data supplied to a [`CommandOption`] of type [`Channel`].
///
/// [`Channel`]: CommandOption::Channel
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChannelCommandOptionData {
    /// Restricts the channel choice to specific types.
    ///
    /// If no types are selected, all channel types can be sent.
    #[serde(default)]
    pub channel_types: Vec<ChannelType>,
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Localization dictionary for the `description` field.
    ///
    /// See [Discord Docs/Localization].
    ///
    /// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    /// Name of the option. It must be 32 characters or less.
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
    /// Whether or not the option is required to be completed by a user.
    #[serde(default)]
    pub required: bool,
}

/// Data supplied to a [`CommandOption`] of type [`Integer`] or [`Number`].
///
/// [`Integer`]: CommandOption::Integer
/// [`Number`]: CommandOption::Number
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NumberCommandOptionData {
    /// Whether the command supports autocomplete.
    #[serde(default)]
    pub autocomplete: bool,
    /// Predetermined choices may be defined for a user to select.
    ///
    /// When completing this option, the user is prompted with a selector of all
    /// available choices.
    ///
    /// If no choices are available, the user must input a value manually.
    #[serde(default)]
    pub choices: Vec<CommandOptionChoice>,
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Localization dictionary for the `description` field.
    ///
    /// See [Discord Docs/Localization].
    ///
    /// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    /// Maximum value permitted.
    #[serde(default)]
    pub max_value: Option<CommandOptionValue>,
    /// Minimum value permitted.
    #[serde(default)]
    pub min_value: Option<CommandOptionValue>,
    /// Name of the option. It must be 32 characters or less.
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
    /// Whether or not the option is required to be completed by a user.
    #[serde(default)]
    pub required: bool,
}

/// Specifies an option that a user must choose from in a dropdown.
///
/// See [Discord Docs/Application Command Object].
///
/// [Discord Docs/Application Command Object]: https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-choice-structure
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CommandOptionChoice {
    String {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name_localizations: Option<HashMap<String, String>>,
        value: String,
    },
    Int {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name_localizations: Option<HashMap<String, String>>,
        value: i64,
    },
    Number {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name_localizations: Option<HashMap<String, String>>,
        value: f64,
    },
}

/// Type used in `max_value` and `min_value` command option field.
///
/// See [Discord Docs/Application Command Object].
///
/// [Discord Docs/Application Command Object]: https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CommandOptionValue {
    Integer(i64),
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

#[cfg(test)]
mod tests {
    use super::{
        super::{Command, CommandType},
        BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData, CommandOption,
        CommandOptionChoice, CommandOptionValue, NumberCommandOptionData, OptionsCommandOptionData,
    };
    use crate::{channel::ChannelType, guild::Permissions, id::Id};
    use serde_test::Token;
    use std::collections::HashMap;

    /// Test that when a subcommand or subcommand group's `options` field is
    /// missing during deserialization that the field is defaulted instead of
    /// returning a missing field error.
    #[test]
    fn issue_1150() {
        let value = CommandOption::SubCommand(OptionsCommandOptionData {
            description: "ponyville".to_owned(),
            description_localizations: None,
            name: "equestria".to_owned(),
            name_localizations: None,
            options: Vec::new(),
        });

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CommandOptionEnvelope",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("ponyville"),
                Token::Str("name"),
                Token::Str("equestria"),
                Token::Str("options"),
                Token::None,
                Token::Str("type"),
                Token::U8(1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_command_option_full() {
        let value = Command {
            application_id: Some(Id::new(100)),
            default_member_permissions: Some(Permissions::ADMINISTRATOR),
            dm_permission: Some(false),
            description: "this command is a test".into(),
            description_localizations: Some(HashMap::from([(
                "en-US".into(),
                "this command is a test".into(),
            )])),
            guild_id: Some(Id::new(300)),
            id: Some(Id::new(200)),
            kind: CommandType::ChatInput,
            name: "test command".into(),
            name_localizations: Some(HashMap::from([("en-US".into(), "test command".into())])),
            options: Vec::from([CommandOption::SubCommandGroup(OptionsCommandOptionData {
                description: "sub group desc".into(),
                description_localizations: None,
                name: "sub group name".into(),
                name_localizations: None,
                options: Vec::from([CommandOption::SubCommand(OptionsCommandOptionData {
                    description: "sub command desc".into(),
                    description_localizations: None,
                    name: "sub command name".into(),
                    name_localizations: None,
                    options: Vec::from([
                        CommandOption::String(ChoiceCommandOptionData {
                            autocomplete: true,
                            choices: Vec::new(),
                            description: "string manual desc".into(),
                            description_localizations: None,
                            max_length: None,
                            min_length: None,
                            name: "string_manual".into(),
                            name_localizations: None,
                            required: false,
                        }),
                        CommandOption::String(ChoiceCommandOptionData {
                            autocomplete: false,
                            choices: Vec::from([CommandOptionChoice::String {
                                name: "choicea".into(),
                                name_localizations: Some(HashMap::from([(
                                    "en-US".into(),
                                    "choicea".into(),
                                )])),
                                value: "choice_a".into(),
                            }]),
                            description: "string desc".into(),
                            description_localizations: None,
                            max_length: Some(6000),
                            min_length: Some(0),
                            name: "string".into(),
                            name_localizations: None,
                            required: false,
                        }),
                        CommandOption::Integer(NumberCommandOptionData {
                            autocomplete: false,
                            choices: Vec::from([CommandOptionChoice::Int {
                                name: "choice2".into(),
                                name_localizations: None,
                                value: 2,
                            }]),
                            description: "int desc".into(),
                            description_localizations: None,
                            max_value: Some(CommandOptionValue::Integer(20)),
                            min_value: Some(CommandOptionValue::Integer(10)),
                            name: "int".into(),
                            name_localizations: None,
                            required: false,
                        }),
                        CommandOption::Boolean(BaseCommandOptionData {
                            description: "bool desc".into(),
                            description_localizations: None,
                            name: "bool".into(),
                            name_localizations: None,
                            required: false,
                        }),
                        CommandOption::User(BaseCommandOptionData {
                            description: "user desc".into(),
                            description_localizations: None,
                            name: "user".into(),
                            name_localizations: None,
                            required: false,
                        }),
                        CommandOption::Channel(ChannelCommandOptionData {
                            channel_types: Vec::from([ChannelType::GuildText]),
                            description: "channel desc".into(),
                            description_localizations: None,
                            name: "channel".into(),
                            name_localizations: None,
                            required: false,
                        }),
                        CommandOption::Role(BaseCommandOptionData {
                            description: "role desc".into(),
                            description_localizations: None,
                            name: "role".into(),
                            name_localizations: None,
                            required: false,
                        }),
                        CommandOption::Mentionable(BaseCommandOptionData {
                            description: "mentionable desc".into(),
                            description_localizations: None,
                            name: "mentionable".into(),
                            name_localizations: None,
                            required: false,
                        }),
                        CommandOption::Number(NumberCommandOptionData {
                            autocomplete: false,
                            choices: Vec::from([CommandOptionChoice::Number {
                                name: "choice3".into(),
                                name_localizations: None,
                                value: 2.0,
                            }]),
                            description: "number desc".into(),
                            description_localizations: None,
                            max_value: Some(CommandOptionValue::Number(5.5)),
                            min_value: Some(CommandOptionValue::Number(10.0)),
                            name: "number".into(),
                            name_localizations: None,
                            required: false,
                        }),
                    ]),
                })]),
            })]),
            version: Id::new(1),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Command",
                    len: 12,
                },
                Token::Str("application_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100"),
                Token::Str("default_member_permissions"),
                Token::Some,
                Token::Str("8"),
                Token::Str("dm_permission"),
                Token::Some,
                Token::Bool(false),
                Token::Str("description"),
                Token::Str("this command is a test"),
                Token::Str("description_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::Str("en-US"),
                Token::Str("this command is a test"),
                Token::MapEnd,
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("300"),
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("200"),
                Token::Str("type"),
                Token::U8(1),
                Token::Str("name"),
                Token::Str("test command"),
                Token::Str("name_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::Str("en-US"),
                Token::Str("test command"),
                Token::MapEnd,
                Token::Str("options"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandOptionEnvelope",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("sub group desc"),
                Token::Str("name"),
                Token::Str("sub group name"),
                Token::Str("options"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandOptionEnvelope",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("sub command desc"),
                Token::Str("name"),
                Token::Str("sub command name"),
                Token::Str("options"),
                Token::Some,
                Token::Seq { len: Some(9) },
                Token::Struct {
                    name: "CommandOptionEnvelope",
                    len: 5,
                },
                Token::Str("autocomplete"),
                Token::Bool(true),
                Token::Str("choices"),
                Token::Some,
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("string manual desc"),
                Token::Str("name"),
                Token::Str("string_manual"),
                Token::Str("type"),
                Token::U8(3),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOptionEnvelope",
                    len: 6,
                },
                Token::Str("choices"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandOptionChoice",
                    len: 3,
                },
                Token::Str("name"),
                Token::Str("choicea"),
                Token::Str("name_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::Str("en-US"),
                Token::Str("choicea"),
                Token::MapEnd,
                Token::Str("value"),
                Token::Str("choice_a"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("string desc"),
                Token::Str("max_length"),
                Token::Some,
                Token::U16(6000),
                Token::Str("min_length"),
                Token::Some,
                Token::U16(0),
                Token::Str("name"),
                Token::Str("string"),
                Token::Str("type"),
                Token::U8(3),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOptionEnvelope",
                    len: 6,
                },
                Token::Str("choices"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandOptionChoice",
                    len: 2,
                },
                Token::Str("name"),
                Token::Str("choice2"),
                Token::Str("value"),
                Token::I64(2),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("int desc"),
                Token::Str("max_value"),
                Token::Some,
                Token::I64(20),
                Token::Str("min_value"),
                Token::Some,
                Token::I64(10),
                Token::Str("name"),
                Token::Str("int"),
                Token::Str("type"),
                Token::U8(4),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOptionEnvelope",
                    len: 3,
                },
                Token::Str("description"),
                Token::Str("bool desc"),
                Token::Str("name"),
                Token::Str("bool"),
                Token::Str("type"),
                Token::U8(5),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOptionEnvelope",
                    len: 3,
                },
                Token::Str("description"),
                Token::Str("user desc"),
                Token::Str("name"),
                Token::Str("user"),
                Token::Str("type"),
                Token::U8(6),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOptionEnvelope",
                    len: 4,
                },
                Token::Str("channel_types"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::U8(0),
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("channel desc"),
                Token::Str("name"),
                Token::Str("channel"),
                Token::Str("type"),
                Token::U8(7),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOptionEnvelope",
                    len: 3,
                },
                Token::Str("description"),
                Token::Str("role desc"),
                Token::Str("name"),
                Token::Str("role"),
                Token::Str("type"),
                Token::U8(8),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOptionEnvelope",
                    len: 3,
                },
                Token::Str("description"),
                Token::Str("mentionable desc"),
                Token::Str("name"),
                Token::Str("mentionable"),
                Token::Str("type"),
                Token::U8(9),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOptionEnvelope",
                    len: 6,
                },
                Token::Str("choices"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandOptionChoice",
                    len: 2,
                },
                Token::Str("name"),
                Token::Str("choice3"),
                Token::Str("value"),
                Token::F64(2.0),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("number desc"),
                Token::Str("max_value"),
                Token::Some,
                Token::F64(5.5),
                Token::Str("min_value"),
                Token::Some,
                Token::F64(10.0),
                Token::Str("name"),
                Token::Str("number"),
                Token::Str("type"),
                Token::U8(10),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("type"),
                Token::U8(1),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("type"),
                Token::U8(2),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("version"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::StructEnd,
            ],
        );
    }
}
