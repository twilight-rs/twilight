//! Constants, error types, and functions for validating [`Command`]s.

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::application::command::{
    Command, CommandOption, CommandOptionChoice, CommandOptionChoiceValue, CommandOptionType,
    CommandType,
};

/// Maximum number of choices an option can have.
pub const CHOICES_LIMIT: usize = 25;

/// The maximum combined command length in codepoints.
pub const COMMAND_TOTAL_LENGTH: usize = 4000;

/// Maximum length of a command's description.
pub const DESCRIPTION_LENGTH_MAX: usize = 100;

/// Minimum length of a command's description.
pub const DESCRIPTION_LENGTH_MIN: usize = 1;

/// Maximum length of a command's name.
pub const NAME_LENGTH_MAX: usize = 32;

/// Minimum length of a command's name.
pub const NAME_LENGTH_MIN: usize = 1;

/// Maximum amount of options a command may have.
pub const OPTIONS_LIMIT: usize = 25;

/// Maximum length of an option choice name.
pub const OPTION_CHOICE_NAME_LENGTH_MAX: usize = 100;

/// Minimum length of an option choice name.
pub const OPTION_CHOICE_NAME_LENGTH_MIN: usize = 1;

/// Maximum length of an option choice string value.
pub const OPTION_CHOICE_STRING_VALUE_LENGTH_MAX: usize = 100;

/// Minimum length of an option choice string value.
pub const OPTION_CHOICE_STRING_VALUE_LENGTH_MIN: usize = 1;

/// Maximum length of a command's description.
pub const OPTION_DESCRIPTION_LENGTH_MAX: usize = 100;

/// Minimum length of a command's description.
pub const OPTION_DESCRIPTION_LENGTH_MIN: usize = 1;

/// Maximum length of a command's name.
pub const OPTION_NAME_LENGTH_MAX: usize = 32;

/// Minimum length of a command's name.
pub const OPTION_NAME_LENGTH_MIN: usize = 1;

/// Maximum number of commands an application may have in an individual
/// guild.
pub const GUILD_COMMAND_LIMIT: usize = 100;

/// Maximum number of permission overwrites an application may have in an
/// individual guild command.
pub const GUILD_COMMAND_PERMISSION_LIMIT: usize = 10;

/// Error created when a [`Command`] is invalid.
#[derive(Debug)]
pub struct CommandValidationError {
    /// Type of error that occurred.
    kind: CommandValidationErrorType,
}

impl CommandValidationError {
    /// Constant instance of a [`CommandValidationError`] with type
    /// [`CountInvalid`].
    ///
    /// [`CountInvalid`]: CommandValidationErrorType::CountInvalid
    pub const COMMAND_COUNT_INVALID: CommandValidationError = CommandValidationError {
        kind: CommandValidationErrorType::CountInvalid,
    };

    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &CommandValidationErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        CommandValidationErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }

    /// Create an error of type [`OptionNameNotUnique`] with a provided index of
    /// the duplicated option name.
    ///
    /// [`OptionNameNotUnique`]: CommandValidationErrorType::OptionNameNotUnique
    #[must_use = "creating an error has no effect if left unused"]
    pub const fn option_name_not_unique(option_index: usize) -> Self {
        Self {
            kind: CommandValidationErrorType::OptionNameNotUnique { option_index },
        }
    }

    /// Create an error of type [`OptionsRequiredFirst`] with a provided index.
    ///
    /// [`OptionsRequiredFirst`]: CommandValidationErrorType::OptionsRequiredFirst
    #[must_use = "creating an error has no effect if left unused"]
    pub const fn option_required_first(index: usize) -> Self {
        Self {
            kind: CommandValidationErrorType::OptionsRequiredFirst { index },
        }
    }
}

impl Display for CommandValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            CommandValidationErrorType::CountInvalid => {
                f.write_str("more than ")?;
                Display::fmt(&GUILD_COMMAND_LIMIT, f)?;

                f.write_str(" commands were set")
            }
            CommandValidationErrorType::CommandTooLarge { characters } => {
                f.write_str("the combined total length of the command is ")?;
                Display::fmt(characters, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&COMMAND_TOTAL_LENGTH, f)
            }
            CommandValidationErrorType::DescriptionInvalid => {
                f.write_str("command description must be between ")?;
                Display::fmt(&DESCRIPTION_LENGTH_MIN, f)?;
                f.write_str(" and ")?;
                Display::fmt(&DESCRIPTION_LENGTH_MAX, f)?;

                f.write_str(" characters")
            }
            CommandValidationErrorType::DescriptionNotAllowed => f.write_str(
                "command description must be a empty string on message and user commands",
            ),
            CommandValidationErrorType::NameLengthInvalid => {
                f.write_str("command name must be between ")?;
                Display::fmt(&NAME_LENGTH_MIN, f)?;
                f.write_str(" and ")?;

                Display::fmt(&NAME_LENGTH_MAX, f)
            }
            CommandValidationErrorType::NameCharacterInvalid { character } => {
                f.write_str(
                    "command name must only contain lowercase alphanumeric characters, found `",
                )?;
                Display::fmt(character, f)?;

                f.write_str("`")
            }
            CommandValidationErrorType::OptionDescriptionInvalid => {
                f.write_str("command option description must be between ")?;
                Display::fmt(&OPTION_DESCRIPTION_LENGTH_MIN, f)?;
                f.write_str(" and ")?;
                Display::fmt(&OPTION_DESCRIPTION_LENGTH_MAX, f)?;

                f.write_str(" characters")
            }
            CommandValidationErrorType::OptionNameNotUnique { option_index } => {
                f.write_str("command option at index ")?;
                Display::fmt(option_index, f)?;

                f.write_str(" has the same name as another option")
            }
            CommandValidationErrorType::OptionNameLengthInvalid => {
                f.write_str("command option name must be between ")?;
                Display::fmt(&OPTION_NAME_LENGTH_MIN, f)?;
                f.write_str(" and ")?;

                Display::fmt(&OPTION_NAME_LENGTH_MAX, f)
            }
            CommandValidationErrorType::OptionNameCharacterInvalid { character } => {
                f.write_str("command option name must only contain lowercase alphanumeric characters, found `")?;
                Display::fmt(character, f)?;

                f.write_str("`")
            }
            CommandValidationErrorType::OptionChoiceNameLengthInvalid => {
                f.write_str("command option choice name must be between ")?;
                Display::fmt(&OPTION_CHOICE_NAME_LENGTH_MIN, f)?;
                f.write_str(" and ")?;
                Display::fmt(&OPTION_CHOICE_NAME_LENGTH_MAX, f)?;

                f.write_str(" characters")
            }
            CommandValidationErrorType::OptionChoiceStringValueLengthInvalid => {
                f.write_str("command option choice string value must be between ")?;
                Display::fmt(&OPTION_CHOICE_STRING_VALUE_LENGTH_MIN, f)?;
                f.write_str(" and ")?;
                Display::fmt(&OPTION_CHOICE_STRING_VALUE_LENGTH_MAX, f)?;

                f.write_str(" characters")
            }
            CommandValidationErrorType::OptionsCountInvalid => {
                f.write_str("more than ")?;
                Display::fmt(&OPTIONS_LIMIT, f)?;

                f.write_str(" options were set")
            }
            CommandValidationErrorType::OptionsRequiredFirst { .. } => {
                f.write_str("optional command options must be added after required")
            }
            CommandValidationErrorType::PermissionsCountInvalid => {
                f.write_str("more than ")?;
                Display::fmt(&GUILD_COMMAND_PERMISSION_LIMIT, f)?;

                f.write_str(" permission overwrites were set")
            }
        }
    }
}

impl Error for CommandValidationError {}

/// Type of [`CommandValidationError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum CommandValidationErrorType {
    /// Too many commands have been provided.
    ///
    /// The maximum number of commands is defined by
    /// [`GUILD_COMMAND_LIMIT`].
    CountInvalid,
    /// Combined values of the command are larger than
    /// [`COMMAND_TOTAL_LENGTH`].
    ///
    /// This includes name or the longest name localization,
    /// description or the longest description localization
    /// of the command and its options and the choice names
    /// or the longest name localization and the choice value
    /// if it is a string choice.
    CommandTooLarge {
        /// Provided number of codepoints.
        characters: usize,
    },
    /// Command description is invalid.
    DescriptionInvalid,
    /// Command description must be a empty string.
    DescriptionNotAllowed,
    /// Command name length is invalid.
    NameLengthInvalid,
    /// Command name contain an invalid character.
    NameCharacterInvalid {
        /// Invalid character.
        character: char,
    },
    /// Command option description is invalid.
    OptionDescriptionInvalid,
    /// Command option name length is invalid.
    OptionNameLengthInvalid,
    /// Command option name is non-unique.
    OptionNameNotUnique {
        /// Index of the option that has a duplicated name.
        option_index: usize,
    },
    /// Command option name contain an invalid character.
    OptionNameCharacterInvalid {
        /// Invalid character.
        character: char,
    },
    /// Command option choice name length is invalid.
    OptionChoiceNameLengthInvalid,
    /// String command option choice value length is invalid.
    OptionChoiceStringValueLengthInvalid,
    /// Command options count invalid.
    OptionsCountInvalid,
    /// Required command options have to be passed before optional ones.
    OptionsRequiredFirst {
        /// Index of the option that failed validation.
        index: usize,
    },
    /// More than 10 permission overwrites were set.
    PermissionsCountInvalid,
}

/// Validate a [`Command`].
///
/// # Errors
///
/// Returns an error of type [`DescriptionInvalid`] if the description is
/// invalid.
///
/// Returns an error of type [`NameLengthInvalid`] or [`NameCharacterInvalid`]
/// if the name is invalid.
///
/// [`DescriptionInvalid`]: CommandValidationErrorType::DescriptionInvalid
/// [`NameLengthInvalid`]: CommandValidationErrorType::NameLengthInvalid
/// [`NameCharacterInvalid`]: CommandValidationErrorType::NameCharacterInvalid
pub fn command(value: &Command) -> Result<(), CommandValidationError> {
    let characters = self::command_characters(value);

    if characters > COMMAND_TOTAL_LENGTH {
        return Err(CommandValidationError {
            kind: CommandValidationErrorType::CommandTooLarge { characters },
        });
    }

    let Command {
        description,
        description_localizations,
        name,
        name_localizations,
        kind,
        ..
    } = value;

    if *kind == CommandType::ChatInput {
        self::description(description)?;
        if let Some(description_localizations) = description_localizations {
            for description in description_localizations.values() {
                self::description(description)?;
            }
        }
    } else if !description.is_empty() {
        return Err(CommandValidationError {
            kind: CommandValidationErrorType::DescriptionNotAllowed,
        });
    };

    if let Some(name_localizations) = name_localizations {
        for name in name_localizations.values() {
            match kind {
                CommandType::ChatInput => self::chat_input_name(name)?,
                CommandType::User | CommandType::Message => {
                    self::name(name)?;
                }
                CommandType::Unknown(_) => (),
                _ => unimplemented!(),
            }
        }
    }

    match kind {
        CommandType::ChatInput => self::chat_input_name(name),
        CommandType::User | CommandType::Message => self::name(name),
        CommandType::Unknown(_) => Ok(()),
        _ => unimplemented!(),
    }
}

/// Calculate the total character count of a command.
pub fn command_characters(command: &Command) -> usize {
    let mut characters =
        longest_localization_characters(&command.name, &command.name_localizations)
            + longest_localization_characters(
                &command.description,
                &command.description_localizations,
            );

    for option in &command.options {
        characters += option_characters(option);
    }

    characters
}

/// Calculate the total character count of a command option.
pub fn option_characters(option: &CommandOption) -> usize {
    let mut characters = 0;

    characters += longest_localization_characters(&option.name, &option.name_localizations);
    characters +=
        longest_localization_characters(&option.description, &option.description_localizations);

    match option.kind {
        CommandOptionType::String => {
            if let Some(choices) = option.choices.as_ref() {
                for choice in choices {
                    if let CommandOptionChoiceValue::String(string_choice) = &choice.value {
                        characters += longest_localization_characters(
                            &choice.name,
                            &choice.name_localizations,
                        ) + string_choice.len();
                    }
                }
            }
        }
        CommandOptionType::SubCommandGroup | CommandOptionType::SubCommand => {
            if let Some(options) = option.options.as_ref() {
                for option in options {
                    characters += option_characters(option);
                }
            }
        }
        _ => {}
    }

    characters
}

/// Calculate the characters for the longest name/description.
///
/// Discord only counts the longest localization to the character
/// limit. If the default value is longer than any of the
/// localizations, the length of the default value will be used
/// instead.
fn longest_localization_characters(
    default: &str,
    localizations: &Option<HashMap<String, String>>,
) -> usize {
    let mut characters = default.len();

    if let Some(localizations) = localizations {
        for localization in localizations.values() {
            if localization.len() > characters {
                characters = localization.len();
            }
        }
    }

    characters
}

/// Validate the description of a [`Command`].
///
/// The length of the description must be more than [`DESCRIPTION_LENGTH_MIN`]
/// and less than or equal to [`DESCRIPTION_LENGTH_MAX`].
///
/// # Errors
///
/// Returns an error of type [`DescriptionInvalid`] if the description is
/// invalid.
///
/// [`DescriptionInvalid`]: CommandValidationErrorType::DescriptionInvalid
pub fn description(value: impl AsRef<str>) -> Result<(), CommandValidationError> {
    let len = value.as_ref().chars().count();

    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    if (DESCRIPTION_LENGTH_MIN..=DESCRIPTION_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(CommandValidationError {
            kind: CommandValidationErrorType::DescriptionInvalid,
        })
    }
}

/// Validate the name of a [`User`] or [`Message`] command.
///
/// The length of the name must be more than [`NAME_LENGTH_MIN`] and less than
/// or equal to [`NAME_LENGTH_MAX`].
///
/// Use [`chat_input_name`] to validate name of a [`ChatInput`] command.
///
/// # Errors
///
/// Returns an error of type [`NameLengthInvalid`] if the name is invalid.
///
/// [`User`]: CommandType::User
/// [`Message`]: CommandType::Message
/// [`ChatInput`]: CommandType::ChatInput
/// [`NameLengthInvalid`]: CommandValidationErrorType::NameLengthInvalid
pub fn name(value: impl AsRef<str>) -> Result<(), CommandValidationError> {
    let len = value.as_ref().chars().count();

    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    if (NAME_LENGTH_MIN..=NAME_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(CommandValidationError {
            kind: CommandValidationErrorType::NameLengthInvalid,
        })
    }
}

/// Validate the name of a [`ChatInput`] command.
///
/// The length of the name must be more than [`NAME_LENGTH_MIN`] and less than
/// or equal to [`NAME_LENGTH_MAX`]. It can only contain alphanumeric characters
/// and lowercase variants must be used where possible. Special characters `-`
/// and `_` are allowed.
///
/// # Errors
///
/// Returns an error of type [`NameLengthInvalid`] if the length is invalid.
///
/// Returns an error of type [`NameCharacterInvalid`] if the name contains a
/// non-alphanumeric character or an uppercase character for which a lowercase
/// variant exists.
///
/// [`ChatInput`]: CommandType::ChatInput
/// [`NameLengthInvalid`]: CommandValidationErrorType::NameLengthInvalid
/// [`NameCharacterInvalid`]: CommandValidationErrorType::NameCharacterInvalid
pub fn chat_input_name(value: impl AsRef<str>) -> Result<(), CommandValidationError> {
    self::name(&value)?;

    self::name_characters(value)?;

    Ok(())
}

/// Validate the name of a [`CommandOption`].
///
/// The length of the name must be more than [`NAME_LENGTH_MIN`] and less than
/// or equal to [`NAME_LENGTH_MAX`]. It can only contain alphanumeric characters
/// and lowercase variants must be used where possible. Special characters `-`
/// and `_` are allowed.
///
/// # Errors
///
/// Returns an error of type [`NameLengthInvalid`] if the length is invalid.
///
/// Returns an error of type [`NameCharacterInvalid`] if the name contains a
/// non-alphanumeric character or an uppercase character for which a lowercase
/// variant exists.
///
/// [`NameLengthInvalid`]: CommandValidationErrorType::NameLengthInvalid
/// [`NameCharacterInvalid`]: CommandValidationErrorType::NameCharacterInvalid
pub fn option_name(value: impl AsRef<str>) -> Result<(), CommandValidationError> {
    let len = value.as_ref().chars().count();

    if !(OPTION_NAME_LENGTH_MIN..=OPTION_NAME_LENGTH_MAX).contains(&len) {
        return Err(CommandValidationError {
            kind: CommandValidationErrorType::NameLengthInvalid,
        });
    }

    self::name_characters(value)?;

    Ok(())
}

/// Validate the characters of a [`ChatInput`] command name or a
/// [`CommandOption`] name.
///
/// The name can only contain alphanumeric characters and lowercase variants
/// must be used where possible. Special characters `-` and `_` are allowed.
///
/// # Errors
///
/// Returns an error of type [`NameCharacterInvalid`] if the name contains a
/// non-alphanumeric character or an uppercase character for which a lowercase
/// variant exists.
///
/// [`ChatInput`]: CommandType::ChatInput
/// [`NameCharacterInvalid`]: CommandValidationErrorType::NameCharacterInvalid
fn name_characters(value: impl AsRef<str>) -> Result<(), CommandValidationError> {
    let chars = value.as_ref().chars();

    for char in chars {
        if !char.is_alphanumeric() && char != '_' && char != '-' {
            return Err(CommandValidationError {
                kind: CommandValidationErrorType::NameCharacterInvalid { character: char },
            });
        }

        if char.to_lowercase().next() != Some(char) {
            return Err(CommandValidationError {
                kind: CommandValidationErrorType::NameCharacterInvalid { character: char },
            });
        }
    }

    Ok(())
}

/// Validate a single name localization in a [`CommandOptionChoice`].
///
/// # Errors
///
/// Returns an error of type [`OptionChoiceNameLengthInvalid`] if the name is
/// less than [`OPTION_CHOICE_NAME_LENGTH_MIN`] or more than [`OPTION_CHOICE_NAME_LENGTH_MAX`].
///
/// [`OptionChoiceNameLengthInvalid`]: CommandValidationErrorType::OptionChoiceNameLengthInvalid
pub fn choice_name(name: &str) -> Result<(), CommandValidationError> {
    let len = name.chars().count();

    if (OPTION_CHOICE_NAME_LENGTH_MIN..=OPTION_CHOICE_NAME_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(CommandValidationError {
            kind: CommandValidationErrorType::OptionChoiceNameLengthInvalid,
        })
    }
}

/// Validate a single [`CommandOptionChoice`].
///
/// # Errors
///
/// Returns an error of type [`OptionChoiceNameLengthInvalid`] if the name is
/// less than [`OPTION_CHOICE_NAME_LENGTH_MIN`] or more than [`OPTION_CHOICE_NAME_LENGTH_MAX`].
///
/// [`OptionChoiceNameLengthInvalid`]: CommandValidationErrorType::OptionChoiceNameLengthInvalid
pub fn choice(choice: &CommandOptionChoice) -> Result<(), CommandValidationError> {
    self::choice_name(&choice.name)?;

    if let CommandOptionChoiceValue::String(value) = &choice.value {
        let value_len = value.chars().count();

        if !(OPTION_CHOICE_STRING_VALUE_LENGTH_MIN..=OPTION_CHOICE_STRING_VALUE_LENGTH_MAX)
            .contains(&value_len)
        {
            return Err(CommandValidationError {
                kind: CommandValidationErrorType::OptionChoiceStringValueLengthInvalid,
            });
        }
    }

    if let Some(name_localizations) = &choice.name_localizations {
        name_localizations
            .values()
            .try_for_each(|name| self::choice_name(name))?;
    }

    Ok(())
}

/// Validate a single [`CommandOption`].
///
/// # Errors
///
/// Returns an error of type [`OptionDescriptionInvalid`] if the description is
/// invalid.
///
/// Returns an error of type [`OptionNameLengthInvalid`] or [`OptionNameCharacterInvalid`]
/// if the name is invalid.
///
/// [`OptionDescriptionInvalid`]: CommandValidationErrorType::OptionDescriptionInvalid
/// [`OptionNameLengthInvalid`]: CommandValidationErrorType::OptionNameLengthInvalid
/// [`OptionNameCharacterInvalid`]: CommandValidationErrorType::OptionNameCharacterInvalid
pub fn option(option: &CommandOption) -> Result<(), CommandValidationError> {
    let description_len = option.description.chars().count();
    if !(OPTION_DESCRIPTION_LENGTH_MIN..=OPTION_DESCRIPTION_LENGTH_MAX).contains(&description_len) {
        return Err(CommandValidationError {
            kind: CommandValidationErrorType::OptionDescriptionInvalid,
        });
    }

    if let Some(choices) = &option.choices {
        choices.iter().try_for_each(self::choice)?;
    }

    self::option_name(&option.name)
}

/// Validate a list of command options for count, order, and internal validity.
///
/// # Errors
///
/// Returns an error of type [`OptionsRequiredFirst`] if a required option is
/// listed before an optional option.
///
/// Returns an error of type [`OptionsCountInvalid`] if the list of options or
/// any sub-list of options is too long.
///
/// [`OptionsRequiredFirst`]: CommandValidationErrorType::OptionsRequiredFirst
/// [`OptionsCountInvalid`]: CommandValidationErrorType::OptionsCountInvalid
pub fn options(options: &[CommandOption]) -> Result<(), CommandValidationError> {
    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-structure
    if options.len() > OPTIONS_LIMIT {
        return Err(CommandValidationError {
            kind: CommandValidationErrorType::OptionsCountInvalid,
        });
    }

    let mut names = HashSet::with_capacity(options.len());

    for (option_index, option) in options.iter().enumerate() {
        if !names.insert(&option.name) {
            return Err(CommandValidationError::option_name_not_unique(option_index));
        }
    }

    // Validate that there are no required options listed after optional ones.
    options
        .iter()
        .zip(options.iter().skip(1))
        .enumerate()
        .try_for_each(|(index, (first, second))| {
            if !first.required.unwrap_or_default() && second.required.unwrap_or_default() {
                Err(CommandValidationError::option_required_first(index))
            } else {
                Ok(())
            }
        })?;

    // Validate that each option is correct.
    options.iter().try_for_each(|option| {
        if let Some(options) = &option.options {
            self::options(options)
        } else {
            self::option(option)
        }
    })?;

    Ok(())
}

/// Validate the number of guild command permission overwrites.
///
/// The maximum number of commands allowed in a guild is defined by
/// [`GUILD_COMMAND_PERMISSION_LIMIT`].
///
/// # Errors
///
/// Returns an error of type [`PermissionsCountInvalid`] if the permissions are
/// invalid.
///
/// [`PermissionsCountInvalid`]: CommandValidationErrorType::PermissionsCountInvalid
pub const fn guild_permissions(count: usize) -> Result<(), CommandValidationError> {
    // https://discord.com/developers/docs/interactions/application-commands#registering-a-command
    if count <= GUILD_COMMAND_PERMISSION_LIMIT {
        Ok(())
    } else {
        Err(CommandValidationError {
            kind: CommandValidationErrorType::PermissionsCountInvalid,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use twilight_model::id::Id;

    #[test]
    fn choice_name_limit() {
        let valid_choice = CommandOptionChoice {
            name: "a".repeat(100),
            name_localizations: None,
            value: CommandOptionChoiceValue::String("a".to_string()),
        };

        assert!(choice(&valid_choice).is_ok());

        let invalid_choice = CommandOptionChoice {
            name: "a".repeat(101),
            name_localizations: None,
            value: CommandOptionChoiceValue::String("b".to_string()),
        };

        assert!(choice(&invalid_choice).is_err());

        let invalid_choice = CommandOptionChoice {
            name: String::new(),
            name_localizations: None,
            value: CommandOptionChoiceValue::String("c".to_string()),
        };

        assert!(choice(&invalid_choice).is_err());
    }

    #[test]
    fn choice_name_localizations() {
        let mut name_localizations = HashMap::new();
        name_localizations.insert("en-US".to_string(), "a".repeat(100));

        let valid_choice = CommandOptionChoice {
            name: "a".to_string(),
            name_localizations: Some(name_localizations),
            value: CommandOptionChoiceValue::String("a".to_string()),
        };

        assert!(choice(&valid_choice).is_ok());

        let mut name_localizations = HashMap::new();
        name_localizations.insert("en-US".to_string(), "a".repeat(101));

        let invalid_choice = CommandOptionChoice {
            name: "a".to_string(),
            name_localizations: Some(name_localizations),
            value: CommandOptionChoiceValue::String("b".to_string()),
        };

        assert!(choice(&invalid_choice).is_err());

        let mut name_localizations = HashMap::new();
        name_localizations.insert("en-US".to_string(), String::new());

        let invalid_choice = CommandOptionChoice {
            name: "a".to_string(),
            name_localizations: Some(name_localizations),
            value: CommandOptionChoiceValue::String("c".to_string()),
        };

        assert!(choice(&invalid_choice).is_err());

        let mut name_localizations = HashMap::new();
        name_localizations.insert("en-US".to_string(), String::from("a"));
        name_localizations.insert("en-GB".to_string(), "a".repeat(101));
        name_localizations.insert("es-ES".to_string(), "a".repeat(100));

        let invalid_choice = CommandOptionChoice {
            name: "a".to_string(),
            name_localizations: Some(name_localizations),
            value: CommandOptionChoiceValue::String("c".to_string()),
        };

        assert!(choice(&invalid_choice).is_err());
    }

    #[test]
    fn choice_string_value() {
        let valid_choice = CommandOptionChoice {
            name: "a".to_string(),
            name_localizations: None,
            value: CommandOptionChoiceValue::String("a".to_string()),
        };

        assert!(choice(&valid_choice).is_ok());

        let invalid_choice = CommandOptionChoice {
            name: "b".to_string(),
            name_localizations: None,
            value: CommandOptionChoiceValue::String("b".repeat(101)),
        };

        assert!(choice(&invalid_choice).is_err());

        let invalid_choice = CommandOptionChoice {
            name: "c".to_string(),
            name_localizations: None,
            value: CommandOptionChoiceValue::String(String::new()),
        };

        assert!(choice(&invalid_choice).is_err());
    }

    // This tests [`description`] and [`name`] by proxy.
    #[test]
    #[allow(deprecated)]
    fn command_length() {
        let valid_command = Command {
            application_id: Some(Id::new(1)),
            contexts: None,
            default_member_permissions: None,
            dm_permission: None,
            description: "a".repeat(100),
            description_localizations: Some(HashMap::from([(
                "en-US".to_string(),
                "a".repeat(100),
            )])),
            guild_id: Some(Id::new(2)),
            id: Some(Id::new(3)),
            integration_types: None,
            kind: CommandType::ChatInput,
            name: "b".repeat(32),
            name_localizations: Some(HashMap::from([("en-US".to_string(), "b".repeat(32))])),
            nsfw: None,
            options: Vec::new(),
            version: Id::new(4),
        };

        assert!(command(&valid_command).is_ok());

        let invalid_message_command = Command {
            description: "c".repeat(101),
            name: "d".repeat(33),
            ..valid_command.clone()
        };
        assert!(command(&invalid_message_command).is_err());

        let valid_context_menu_command = Command {
            description: String::new(),
            kind: CommandType::Message,
            ..valid_command.clone()
        };

        assert!(command(&valid_context_menu_command).is_ok());

        let invalid_context_menu_command = Command {
            description: "example description".to_string(),
            kind: CommandType::Message,
            ..valid_command
        };

        assert!(command(&invalid_context_menu_command).is_err());
    }

    #[test]
    fn name_allowed_characters() {
        assert!(name_characters("hello-command").is_ok()); // Latin language
        assert!(name_characters("Hello").is_err()); // Latin language with uppercase
        assert!(name_characters("hello!").is_err()); // Latin language with non-alphanumeric

        assert!(name_characters("здрасти").is_ok()); // Russian
        assert!(name_characters("Здрасти").is_err()); // Russian with uppercase
        assert!(name_characters("здрасти!").is_err()); // Russian with non-alphanumeric

        assert!(name_characters("你好").is_ok()); // Chinese (no upper and lowercase variants)
        assert!(name_characters("你好。").is_err()); // Chinese with non-alphanumeric
    }

    #[test]
    fn guild_permissions_count() {
        assert!(guild_permissions(0).is_ok());
        assert!(guild_permissions(1).is_ok());
        assert!(guild_permissions(10).is_ok());

        assert!(guild_permissions(11).is_err());
    }

    #[test]
    #[allow(deprecated)]
    fn command_combined_limit() {
        let mut command = Command {
            application_id: Some(Id::new(1)),
            default_member_permissions: None,
            dm_permission: None,
            description: "a".repeat(10),
            description_localizations: Some(HashMap::from([(
                "en-US".to_string(),
                "a".repeat(100),
            )])),
            guild_id: Some(Id::new(2)),
            id: Some(Id::new(3)),
            kind: CommandType::ChatInput,
            name: "b".repeat(10),
            name_localizations: Some(HashMap::from([("en-US".to_string(), "b".repeat(32))])),
            nsfw: None,
            options: Vec::from([CommandOption {
                autocomplete: None,
                channel_types: None,
                choices: None,
                description: "a".repeat(10),
                description_localizations: Some(HashMap::from([(
                    "en-US".to_string(),
                    "a".repeat(100),
                )])),
                kind: CommandOptionType::SubCommandGroup,
                max_length: None,
                max_value: None,
                min_length: None,
                min_value: None,
                name: "b".repeat(10),
                name_localizations: Some(HashMap::from([("en-US".to_string(), "b".repeat(32))])),
                options: Some(Vec::from([CommandOption {
                    autocomplete: None,
                    channel_types: None,
                    choices: None,
                    description: "a".repeat(100),
                    description_localizations: Some(HashMap::from([(
                        "en-US".to_string(),
                        "a".repeat(10),
                    )])),
                    kind: CommandOptionType::SubCommand,
                    max_length: None,
                    max_value: None,
                    min_length: None,
                    min_value: None,
                    name: "b".repeat(32),
                    name_localizations: Some(HashMap::from([(
                        "en-US".to_string(),
                        "b".repeat(10),
                    )])),
                    options: Some(Vec::from([CommandOption {
                        autocomplete: Some(false),
                        channel_types: None,
                        choices: Some(Vec::from([CommandOptionChoice {
                            name: "b".repeat(32),
                            name_localizations: Some(HashMap::from([(
                                "en-US".to_string(),
                                "b".repeat(10),
                            )])),
                            value: CommandOptionChoiceValue::String("c".repeat(100)),
                        }])),
                        description: "a".repeat(100),
                        description_localizations: Some(HashMap::from([(
                            "en-US".to_string(),
                            "a".repeat(10),
                        )])),
                        kind: CommandOptionType::String,
                        max_length: None,
                        max_value: None,
                        min_length: None,
                        min_value: None,
                        name: "b".repeat(32),
                        name_localizations: Some(HashMap::from([(
                            "en-US".to_string(),
                            "b".repeat(10),
                        )])),
                        options: None,
                        required: Some(false),
                    }])),
                    required: None,
                }])),
                required: None,
            }]),
            version: Id::new(4),
            contexts: None,
            integration_types: None,
        };

        assert_eq!(command_characters(&command), 660);
        assert!(super::command(&command).is_ok());

        command.description = "a".repeat(3441);
        assert_eq!(command_characters(&command), 4001);

        assert!(matches!(
            super::command(&command).unwrap_err().kind(),
            CommandValidationErrorType::CommandTooLarge { characters: 4001 }
        ));
    }

    /// Assert that a list of options can't contain the same name.
    #[test]
    fn option_name_uniqueness() {
        let option = CommandOption {
            autocomplete: None,
            channel_types: None,
            choices: None,
            description: "a description".to_owned(),
            description_localizations: None,
            kind: CommandOptionType::String,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "name".to_owned(),
            name_localizations: None,
            options: None,
            required: None,
        };
        let mut options = Vec::from([option.clone()]);
        assert!(super::options(&options).is_ok());
        options.push(option);
        assert!(matches!(super::options(&options).unwrap_err().kind(),
            CommandValidationErrorType::OptionNameNotUnique { option_index } if *option_index == 1));
    }

    /// Test if option description length is checked properly
    #[test]
    fn option_description_length() {
        let base = CommandOption {
            autocomplete: None,
            channel_types: None,
            choices: None,
            description: String::new(),
            description_localizations: None,
            kind: CommandOptionType::Boolean,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "testcommand".to_string(),
            name_localizations: None,
            options: None,
            required: None,
        };
        let toolong = CommandOption {
            description: "e".repeat(OPTION_DESCRIPTION_LENGTH_MAX + 1),
            ..base.clone()
        };
        let tooshort = CommandOption {
            description: "e".repeat(OPTION_DESCRIPTION_LENGTH_MIN - 1),
            ..base.clone()
        };
        let maxlen = CommandOption {
            description: "e".repeat(OPTION_DESCRIPTION_LENGTH_MAX),
            ..base.clone()
        };
        // clippy yells at us if this value is 1, but just using to_string would be incorrect
        #[allow(clippy::repeat_once)]
        let minlen = CommandOption {
            description: "e".repeat(OPTION_DESCRIPTION_LENGTH_MIN),
            ..base
        };
        assert!(option(&toolong).is_err());
        assert!(option(&tooshort).is_err());
        assert!(option(&maxlen).is_ok());
        assert!(option(&minlen).is_ok());
    }
}
