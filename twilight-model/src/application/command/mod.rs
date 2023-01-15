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
        CommandOption, CommandOptionChoice, CommandOptionChoiceData, CommandOptionType,
        CommandOptionValue,
    },
};

use crate::{
    guild::Permissions,
    id::{
        marker::{ApplicationMarker, CommandMarker, CommandVersionMarker, GuildMarker},
        Id,
    },
    user::Locale,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    /// Default permissions required for a member to run the command.
    ///
    /// Setting this [`Permissions::empty()`] will prohibit anyone from running
    /// the command, except for guild administrators.
    pub default_member_permissions: Option<Permissions>,
    /// Whether the command is available in DMs.
    ///
    /// This is only relevant for globally-scoped commands. By default, commands
    /// are visible in DMs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
    /// Description of the command.
    ///
    /// For [`USER`] and [`MESSAGE`] commands, this will be an empty string.
    ///
    /// [`USER`]: CommandType::USER
    /// [`MESSAGE`]: CommandType::MESSAGE
    pub description: String,
    /// Localization dictionary for the `description` field.
    ///
    /// See [Discord Docs/Localization].
    ///
    /// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<Locale, String>>,
    /// Guild ID of the command, if not global.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<CommandMarker>>,
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
    pub name_localizations: Option<HashMap<Locale, String>>,
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
        Command, CommandOption, CommandOptionChoice, CommandOptionChoiceData, CommandOptionType,
        CommandOptionValue, CommandType,
    };
    use crate::{channel::ChannelType, guild::Permissions, id::Id, user::Locale};
    use serde_test::Token;
    use std::collections::HashMap;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn command_option_full() {
        let value = Command {
            application_id: Some(Id::new(100)),
            default_member_permissions: Some(Permissions::ADMINISTRATOR),
            dm_permission: Some(false),
            description: "this command is a test".into(),
            description_localizations: Some(HashMap::from([(
                Locale::ENGLISH_US,
                "this command is a test".into(),
            )])),
            guild_id: Some(Id::new(300)),
            id: Some(Id::new(200)),
            kind: CommandType::CHAT_INPUT,
            name: "test command".into(),
            name_localizations: Some(HashMap::from([(Locale::ENGLISH_US, "test command".into())])),
            nsfw: None,
            options: Vec::from([CommandOption {
                autocomplete: None,
                channel_types: None,
                choices: None,
                description: "sub command group desc".to_owned(),
                description_localizations: None,
                kind: CommandOptionType::SUB_COMMAND_GROUP,
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
                    kind: CommandOptionType::SUB_COMMAND,
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
                            kind: CommandOptionType::ATTACHMENT,
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
                            kind: CommandOptionType::BOOLEAN,
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
                            kind: CommandOptionType::CHANNEL,
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
                            channel_types: Some(Vec::from([ChannelType::GUILD_TEXT])),
                            choices: None,
                            description: "channel desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::CHANNEL,
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
                            kind: CommandOptionType::INTEGER,
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
                                Locale::ENGLISH_UK,
                                "mentionable desc (but british)".to_owned(),
                            )])),
                            kind: CommandOptionType::MENTIONABLE,
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
                            choices: Some(Vec::from([CommandOptionChoice::Number(
                                CommandOptionChoiceData {
                                    name: "number choice".to_owned(),
                                    name_localizations: Some(HashMap::from([(
                                        Locale::ENGLISH_US,
                                        "number choice (but american)".to_owned(),
                                    )])),
                                    value: 10.0,
                                },
                            )])),
                            description: "number desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::NUMBER,
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
                            kind: CommandOptionType::ROLE,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "role name".to_owned(),
                            name_localizations: Some(HashMap::from([(
                                Locale::GERMAN,
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
                            kind: CommandOptionType::STRING,
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
                Token::NewtypeStruct { name: "Locale" },
                Token::Str(Locale::ENGLISH_US.get()),
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
                Token::NewtypeStruct {
                    name: "CommandType",
                },
                Token::U8(CommandType::CHAT_INPUT.get()),
                Token::Str("name"),
                Token::Str("test command"),
                Token::Str("name_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "Locale" },
                Token::Str(Locale::ENGLISH_US.get()),
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
                Token::NewtypeStruct {
                    name: "CommandOptionType",
                },
                Token::U8(CommandOptionType::SUB_COMMAND_GROUP.get()),
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
                Token::NewtypeStruct {
                    name: "CommandOptionType",
                },
                Token::U8(CommandOptionType::SUB_COMMAND.get()),
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
                Token::NewtypeStruct {
                    name: "CommandOptionType",
                },
                Token::U8(CommandOptionType::ATTACHMENT.get()),
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
                Token::NewtypeStruct {
                    name: "CommandOptionType",
                },
                Token::U8(CommandOptionType::BOOLEAN.get()),
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
                Token::NewtypeStruct {
                    name: "CommandOptionType",
                },
                Token::U8(CommandOptionType::CHANNEL.get()),
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
                Token::NewtypeStruct {
                    name: "ChannelType",
                },
                Token::U8(ChannelType::GUILD_TEXT.get()),
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("channel desc"),
                Token::Str("type"),
                Token::NewtypeStruct {
                    name: "CommandOptionType",
                },
                Token::U8(CommandOptionType::CHANNEL.get()),
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
                Token::NewtypeStruct {
                    name: "CommandOptionType",
                },
                Token::U8(CommandOptionType::INTEGER.get()),
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
                Token::NewtypeStruct { name: "Locale" },
                Token::Str(Locale::ENGLISH_UK.get()),
                Token::Str("mentionable desc (but british)"),
                Token::MapEnd,
                Token::Str("type"),
                Token::NewtypeStruct {
                    name: "CommandOptionType",
                },
                Token::U8(CommandOptionType::MENTIONABLE.get()),
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
                    name: "CommandOptionChoiceData",
                    len: 3,
                },
                Token::Str("name"),
                Token::Str("number choice"),
                Token::Str("name_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "Locale" },
                Token::Str(Locale::ENGLISH_US.get()),
                Token::Str("number choice (but american)"),
                Token::MapEnd,
                Token::Str("value"),
                Token::F64(10.0),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("number desc"),
                Token::Str("type"),
                Token::NewtypeStruct {
                    name: "CommandOptionType",
                },
                Token::U8(CommandOptionType::NUMBER.get()),
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
                Token::NewtypeStruct {
                    name: "CommandOptionType",
                },
                Token::U8(CommandOptionType::ROLE.get()),
                Token::Str("name"),
                Token::Str("role name"),
                Token::Str("name_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "Locale" },
                Token::Str(Locale::GERMAN.get()),
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
                Token::NewtypeStruct {
                    name: "CommandOptionType",
                },
                Token::U8(CommandOptionType::STRING.get()),
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
