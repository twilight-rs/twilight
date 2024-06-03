//! Used for building commands to send to Discord.
//!
//! It is highly recommended to use the associated [`CommandBuilder`] in the
//! [`twilight-util`] to create [`Command`]s; [`CommandOption`] is especially
//! verbose.
//!
//! [`CommandBuilder`]: https://docs.rs/twilight-util/latest/twilight_util/builder/command/index.html
//! [`twilight-util`]: https://docs.rs/twilight-util

pub mod permissions;

mod command_type;
mod option;

pub use self::{
    command_type::CommandType,
    option::{
        CommandOption, CommandOptionChoice, CommandOptionChoiceValue, CommandOptionType,
        CommandOptionValue,
    },
};

use crate::{
    guild::Permissions,
    id::{
        marker::{ApplicationMarker, CommandMarker, CommandVersionMarker, GuildMarker},
        Id,
    },
    oauth::ApplicationIntegrationType,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::interaction::InteractionContextType;

/// Data sent to Discord to create a command.
///
/// [`CommandOption`]s that are required must be listed before optional ones.
/// Command names must be lower case, matching the Regex `^[\w-]{1,32}$`. See
/// [Discord Docs/Application Command Object].
///
/// [Discord Docs/Application Command Object]: https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-structure
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Command {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<InteractionContextType>>,
    /// Default permissions required for a member to run the command.
    ///
    /// Setting this [`Permissions::empty()`] will prohibit anyone from running
    /// the command, except for guild administrators.
    pub default_member_permissions: Option<Permissions>,
    /// Whether the command is available in DMs.
    ///
    /// This is only relevant for globally-scoped commands. By default, commands
    /// are visible in DMs.
    #[deprecated(note = "use contexts instead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
    /// Description of the command.
    ///
    /// For [`User`] and [`Message`] commands, this will be an empty string.
    ///
    /// [`User`]: CommandType::User
    /// [`Message`]: CommandType::Message
    pub description: String,
    /// Localization dictionary for the `description` field.
    ///
    /// See [Discord Docs/Localization].
    ///
    /// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    /// Guild ID of the command, if not global.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<CommandMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_types: Option<Vec<ApplicationIntegrationType>>,
    #[serde(rename = "type")]
    pub kind: CommandType,
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
    /// Whether the command is age-restricted.
    ///
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(default)]
    pub options: Vec<CommandOption>,
    /// Autoincrementing version identifier.
    pub version: Id<CommandVersionMarker>,
}

#[cfg(test)]
mod tests {
    use super::{
        Command, CommandOption, CommandOptionChoice, CommandOptionChoiceValue, CommandOptionType,
        CommandOptionValue, CommandType,
    };
    use crate::{channel::ChannelType, guild::Permissions, id::Id};
    use serde_test::Token;
    use std::collections::HashMap;

    #[test]
    #[allow(clippy::too_many_lines, deprecated)]
    fn command_option_full() {
        let value = Command {
            application_id: Some(Id::new(100)),
            contexts: None,
            default_member_permissions: Some(Permissions::ADMINISTRATOR),
            dm_permission: Some(false),
            description: "this command is a test".into(),
            description_localizations: Some(HashMap::from([(
                "en-US".into(),
                "this command is a test".into(),
            )])),
            guild_id: Some(Id::new(300)),
            id: Some(Id::new(200)),
            integration_types: None,
            kind: CommandType::ChatInput,
            name: "test command".into(),
            name_localizations: Some(HashMap::from([("en-US".into(), "test command".into())])),
            nsfw: None,
            options: Vec::from([CommandOption {
                autocomplete: None,
                channel_types: None,
                choices: None,
                description: "sub command group desc".to_owned(),
                description_localizations: None,
                kind: CommandOptionType::SubCommandGroup,
                max_length: None,
                max_value: None,
                min_length: None,
                min_value: None,
                name: "sub command group name".to_owned(),
                name_localizations: None,
                options: Some(Vec::from([CommandOption {
                    autocomplete: None,
                    channel_types: None,
                    choices: None,
                    description: "sub command desc".to_owned(),
                    description_localizations: None,
                    kind: CommandOptionType::SubCommand,
                    max_length: None,
                    max_value: None,
                    min_length: None,
                    min_value: None,
                    name: "sub command name".to_owned(),
                    name_localizations: None,
                    options: Some(Vec::from([
                        CommandOption {
                            autocomplete: None,
                            channel_types: None,
                            choices: None,
                            description: "attachment desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Attachment,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "attachment name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: None,
                            channel_types: None,
                            choices: None,
                            description: "boolean desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Boolean,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "boolean name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: Some(true),
                        },
                        CommandOption {
                            autocomplete: None,
                            channel_types: Some(Vec::new()),
                            choices: None,
                            description: "channel desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Channel,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "channel name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: None,
                            channel_types: Some(Vec::from([ChannelType::GuildText])),
                            choices: None,
                            description: "channel desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Channel,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "channel name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: Some(true),
                            channel_types: None,
                            choices: Some(Vec::new()),
                            description: "integer desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Integer,
                            max_length: None,
                            max_value: Some(CommandOptionValue::Integer(100)),
                            min_length: None,
                            min_value: Some(CommandOptionValue::Integer(0)),
                            name: "integer name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: None,
                            channel_types: None,
                            choices: None,
                            description: "mentionable desc".to_owned(),
                            description_localizations: Some(HashMap::from([(
                                "en-GB".to_owned(),
                                "mentionable desc (but british)".to_owned(),
                            )])),
                            kind: CommandOptionType::Mentionable,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "mentionable name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: Some(false),
                            channel_types: None,
                            choices: Some(Vec::from([CommandOptionChoice {
                                name: "number choice".to_owned(),
                                name_localizations: Some(HashMap::from([(
                                    "en-US".to_owned(),
                                    "number choice (but american)".to_owned(),
                                )])),
                                value: CommandOptionChoiceValue::Number(10.0),
                            }])),
                            description: "number desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Number,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "number name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: None,
                            channel_types: None,
                            choices: None,
                            description: "role desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Role,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "role name".to_owned(),
                            name_localizations: Some(HashMap::from([(
                                "de-DE".to_owned(),
                                "role name (but german)".to_owned(),
                            )])),
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: None,
                            channel_types: None,
                            choices: None,
                            description: "string desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::String,
                            max_length: Some(6000),
                            max_value: None,
                            min_length: Some(0),
                            min_value: None,
                            name: "string name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                    ])),
                    required: None,
                }])),
                required: None,
            }]),
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
                Token::U8(CommandType::ChatInput.into()),
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
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("sub command group desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::SubCommandGroup as u8),
                Token::Str("name"),
                Token::Str("sub command group name"),
                Token::Str("options"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("sub command desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::SubCommand as u8),
                Token::Str("name"),
                Token::Str("sub command name"),
                Token::Str("options"),
                Token::Some,
                Token::Seq { len: Some(9) },
                Token::Struct {
                    name: "CommandOption",
                    len: 3,
                },
                Token::Str("description"),
                Token::Str("attachment desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Attachment as u8),
                Token::Str("name"),
                Token::Str("attachment name"),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("boolean desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Boolean as u8),
                Token::Str("name"),
                Token::Str("boolean name"),
                Token::Str("required"),
                Token::Some,
                Token::Bool(true),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("channel_types"),
                Token::Some,
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("channel desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Channel as u8),
                Token::Str("name"),
                Token::Str("channel name"),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("channel_types"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::U8(ChannelType::GuildText.into()),
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("channel desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Channel as u8),
                Token::Str("name"),
                Token::Str("channel name"),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 7,
                },
                Token::Str("autocomplete"),
                Token::Some,
                Token::Bool(true),
                Token::Str("choices"),
                Token::Some,
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("integer desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Integer as u8),
                Token::Str("max_value"),
                Token::Some,
                Token::I64(100),
                Token::Str("min_value"),
                Token::Some,
                Token::I64(0),
                Token::Str("name"),
                Token::Str("integer name"),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("mentionable desc"),
                Token::Str("description_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::Str("en-GB"),
                Token::Str("mentionable desc (but british)"),
                Token::MapEnd,
                Token::Str("type"),
                Token::U8(CommandOptionType::Mentionable as u8),
                Token::Str("name"),
                Token::Str("mentionable name"),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 5,
                },
                Token::Str("autocomplete"),
                Token::Some,
                Token::Bool(false),
                Token::Str("choices"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandOptionChoice",
                    len: 3,
                },
                Token::Str("name"),
                Token::Str("number choice"),
                Token::Str("name_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::Str("en-US"),
                Token::Str("number choice (but american)"),
                Token::MapEnd,
                Token::Str("value"),
                Token::F64(10.0),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("number desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Number as u8),
                Token::Str("name"),
                Token::Str("number name"),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("role desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Role as u8),
                Token::Str("name"),
                Token::Str("role name"),
                Token::Str("name_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::Str("de-DE"),
                Token::Str("role name (but german)"),
                Token::MapEnd,
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 5,
                },
                Token::Str("description"),
                Token::Str("string desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::String as u8),
                Token::Str("max_length"),
                Token::Some,
                Token::U16(6000),
                Token::Str("min_length"),
                Token::Some,
                Token::U16(0),
                Token::Str("name"),
                Token::Str("string name"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
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
