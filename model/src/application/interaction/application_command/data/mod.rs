mod resolved;

pub use self::resolved::{CommandInteractionDataResolved, InteractionChannel, InteractionMember};

use crate::{
    application::command::{CommandOptionType, Number},
    id::{ChannelId, CommandId, GenericId, RoleId, UserId},
};
use serde::{
    de::{Error as DeError, Unexpected},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::borrow::Cow;

/// Data received when an [`ApplicationCommand`] interaction is executed.
///
/// Refer to [the discord docs] for more information.
///
/// [`ApplicationCommand`]: crate::application::interaction::Interaction::ApplicationCommand
/// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#interaction-applicationcommandinteractiondata
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandData {
    /// ID of the command.
    pub id: CommandId,
    /// Name of the command.
    pub name: String,
    /// List of parsed options specified by the user.
    #[serde(default)]
    pub options: Vec<CommandDataOption>,
    /// Data sent if any of the options are discord types.
    pub resolved: Option<CommandInteractionDataResolved>,
}

/// Data received when a user fills in a command option.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-interaction-data-option-structure
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandDataOption {
    pub name: String,
    pub value: CommandOptionValue,
}

/// Value of a [`CommandDataOption`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandOptionValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    User(UserId),
    Channel(ChannelId),
    Role(RoleId),
    Mentionable(GenericId),
    SubCommand(Vec<CommandDataOption>),
    SubCommandGroup(Vec<CommandDataOption>),
    Number(Number),
}

#[derive(Debug, Deserialize)]
struct CommandDataOptionRaw<'a> {
    name: String,
    #[serde(rename = "type")]
    kind: CommandOptionType,
    value: Option<CommandOptionValueRaw<'a>>,
    #[serde(default)]
    options: Option<Vec<CommandDataOption>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum CommandOptionValueRaw<'a> {
    String(Cow<'a, str>),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}

impl Serialize for CommandDataOption {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let sub_command_is_empty = matches!(&self.value, CommandOptionValue::SubCommand(o) | CommandOptionValue::SubCommandGroup(o) if o.is_empty());

        let len = if sub_command_is_empty { 2 } else { 3 };

        let mut state = serializer.serialize_struct("CommandDataOption", len)?;

        state.serialize_field("name", &self.name)?;

        state.serialize_field("type", &self.value.kind())?;

        match self.value {
            CommandOptionValue::SubCommand(ref opts)
            | CommandOptionValue::SubCommandGroup(ref opts) => {
                if sub_command_is_empty {
                    state.skip_field("options")?
                } else {
                    state.serialize_field("options", &Some(opts))?
                }
            }
            CommandOptionValue::String(ref value) => state
                .serialize_field("value", &Some(CommandOptionValueRaw::String(value.into())))?,
            CommandOptionValue::Integer(value) => {
                state.serialize_field("value", &Some(CommandOptionValueRaw::Integer(value)))?
            }
            CommandOptionValue::Boolean(value) => {
                state.serialize_field("value", &Some(CommandOptionValueRaw::Boolean(value)))?
            }
            CommandOptionValue::User(UserId(id))
            | CommandOptionValue::Channel(ChannelId(id))
            | CommandOptionValue::Role(RoleId(id))
            | CommandOptionValue::Mentionable(GenericId(id)) => state.serialize_field(
                "value",
                &Some(CommandOptionValueRaw::String(id.to_string().into())),
            )?,
            CommandOptionValue::Number(value) => {
                state.serialize_field("value", &Some(CommandOptionValueRaw::Number(value.0)))?
            }
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for CommandDataOption {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = CommandDataOptionRaw::deserialize(deserializer)?;
        let value = if let Some(value) = raw.value {
            match (raw.kind, value) {
                (CommandOptionType::String, CommandOptionValueRaw::String(s)) => {
                    CommandOptionValue::String(s.into_owned())
                }
                (CommandOptionType::Integer, CommandOptionValueRaw::Integer(i)) => {
                    CommandOptionValue::Integer(i)
                }
                (CommandOptionType::Boolean, CommandOptionValueRaw::Boolean(b)) => {
                    CommandOptionValue::Boolean(b)
                }
                (CommandOptionType::User, CommandOptionValueRaw::String(s)) => {
                    let id =
                        UserId(s.parse().map_err(|_| {
                            DeError::invalid_value(Unexpected::Str(&s), &"user ID")
                        })?);

                    CommandOptionValue::User(id)
                }
                (CommandOptionType::Channel, CommandOptionValueRaw::String(s)) => {
                    let id =
                        ChannelId(s.parse().map_err(|_| {
                            DeError::invalid_value(Unexpected::Str(&s), &"channel ID")
                        })?);

                    CommandOptionValue::Channel(id)
                }
                (CommandOptionType::Role, CommandOptionValueRaw::String(s)) => {
                    let id =
                        RoleId(s.parse().map_err(|_| {
                            DeError::invalid_value(Unexpected::Str(&s), &"role ID")
                        })?);

                    CommandOptionValue::Role(id)
                }
                (CommandOptionType::Mentionable, CommandOptionValueRaw::String(s)) => {
                    let id = GenericId(s.parse().map_err(|_| {
                        DeError::invalid_value(Unexpected::Str(&s), &"snowflake ID")
                    })?);

                    CommandOptionValue::Mentionable(id)
                }
                (CommandOptionType::SubCommand | CommandOptionType::SubCommandGroup, _) => {
                    return Err(DeError::custom(format!(
                        "invalid option data: {:?} has value instead of options",
                        raw.kind
                    )));
                }
                (CommandOptionType::Number, CommandOptionValueRaw::String(s)) => {
                    let value = s
                        .parse::<f64>()
                        .map_err(|_| DeError::invalid_value(Unexpected::Str(&s), &"number"))?;

                    CommandOptionValue::Number(Number(value))
                }
                (kind, value) => {
                    return Err(DeError::custom(format!(
                        "invalid option value/type pair: value is {:?} but type is {:?}",
                        value, kind,
                    )));
                }
            }
        } else {
            let options = raw.options.unwrap_or_default();

            match raw.kind {
                CommandOptionType::SubCommand => CommandOptionValue::SubCommand(options),
                CommandOptionType::SubCommandGroup => CommandOptionValue::SubCommandGroup(options),
                kind => {
                    return Err(DeError::custom(format!(
                        "no `value` but type is {:?}",
                        kind
                    )))
                }
            }
        };
        Ok(CommandDataOption {
            name: raw.name,
            value,
        })
    }
}

impl CommandOptionValue {
    pub const fn kind(&self) -> CommandOptionType {
        match self {
            CommandOptionValue::String(_) => CommandOptionType::String,
            CommandOptionValue::Integer(_) => CommandOptionType::Integer,
            CommandOptionValue::Boolean(_) => CommandOptionType::Boolean,
            CommandOptionValue::User(_) => CommandOptionType::User,
            CommandOptionValue::Channel(_) => CommandOptionType::Channel,
            CommandOptionValue::Role(_) => CommandOptionType::Role,
            CommandOptionValue::Mentionable(_) => CommandOptionType::Mentionable,
            CommandOptionValue::SubCommand(_) => CommandOptionType::SubCommand,
            CommandOptionValue::SubCommandGroup(_) => CommandOptionType::SubCommandGroup,
            CommandOptionValue::Number(_) => CommandOptionType::Number,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_test::Token;

    use crate::{
        application::{
            command::CommandOptionType,
            interaction::application_command::{CommandDataOption, CommandOptionValue},
        },
        id::CommandId,
    };

    use super::CommandData;

    #[test]
    fn subcommand_without_option() {
        let value = CommandData {
            id: CommandId::new(1).expect("non zero"),
            name: "photo".to_owned(),
            options: vec![CommandDataOption {
                name: "cat".to_owned(),
                value: CommandOptionValue::SubCommand(vec![]),
            }],
            resolved: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CommandData",
                    len: 4,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "CommandId" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("photo"),
                Token::Str("options"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandDataOptionRaw",
                    len: 2,
                },
                Token::Str("name"),
                Token::Str("cat"),
                Token::Str("type"),
                Token::U8(CommandOptionType::SubCommand as u8),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("resolved"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }
}
