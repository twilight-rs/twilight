mod resolved;

pub use self::resolved::{CommandInteractionDataResolved, InteractionChannel, InteractionMember};

use crate::application::command::CommandOptionType;
use crate::id::{ChannelId, CommandId, RoleId, UserId};
use serde::de;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
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
/// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#interaction-applicationcommandinteractiondataoption
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandDataOption {
    pub name: String,
    pub value: CommandOptionValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandOptionValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    User(UserId),
    Channel(ChannelId),
    Role(RoleId),
    Mentionable(u64),
    SubCommand(Vec<CommandDataOption>),
    SubCommandGroup(Vec<CommandDataOption>),
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
enum CommandOptionValueRaw<'a> {
    String(Cow<'a, str>),
    Integer(i64),
    Boolean(bool),
}

impl Serialize for CommandDataOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("CommandDataOptionRaw", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("type", &self.value.kind())?;
        match &self.value {
            CommandOptionValue::SubCommand(opts) | CommandOptionValue::SubCommandGroup(opts) => {
                state.serialize_field("options", &Some(opts))?
            }
            _ => state.serialize_field("value", &Some(self.value.raw_value().unwrap()))?,
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for CommandDataOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
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
                    let id = UserId(s.parse().map_err(|_| {
                        de::Error::invalid_value(de::Unexpected::Str(&s), &"user ID")
                    })?);
                    CommandOptionValue::User(id)
                }
                (CommandOptionType::Channel, CommandOptionValueRaw::String(s)) => {
                    let id = ChannelId(s.parse().map_err(|_| {
                        de::Error::invalid_value(de::Unexpected::Str(&s), &"channel ID")
                    })?);
                    CommandOptionValue::Channel(id)
                }
                (CommandOptionType::Role, CommandOptionValueRaw::String(s)) => {
                    let id = RoleId(s.parse().map_err(|_| {
                        de::Error::invalid_value(de::Unexpected::Str(&s), &"role ID")
                    })?);
                    CommandOptionValue::Role(id)
                }
                (CommandOptionType::Mentionable, CommandOptionValueRaw::String(s)) => {
                    let id = s.parse().map_err(|_| {
                        de::Error::invalid_value(de::Unexpected::Str(&s), &"snowflake ID")
                    })?;
                    CommandOptionValue::Mentionable(id)
                }
                (CommandOptionType::SubCommand, _) | (CommandOptionType::SubCommandGroup, _) => {
                    return Err(de::Error::custom(format!(
                        "invalid option data: {:?} has value instead of options",
                        raw.kind
                    )));
                }
                (kind, value) => {
                    return Err(de::Error::custom(format!(
                        "invalid option value/type pair: value is {:?} but type is {:?}",
                        value, kind,
                    )));
                }
            }
        } else {
            let options = raw.options.unwrap();
            match raw.kind {
                CommandOptionType::SubCommand => CommandOptionValue::SubCommand(options),
                CommandOptionType::SubCommandGroup => CommandOptionValue::SubCommandGroup(options),
                kind => {
                    return Err(de::Error::custom(format!(
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
    const fn kind(&self) -> CommandOptionType {
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
        }
    }

    fn raw_value(&self) -> Option<CommandOptionValueRaw<'_>> {
        Some(match *self {
            CommandOptionValue::String(ref s) => CommandOptionValueRaw::String(s.into()),
            CommandOptionValue::Integer(i) => CommandOptionValueRaw::Integer(i),
            CommandOptionValue::Boolean(b) => CommandOptionValueRaw::Boolean(b),
            CommandOptionValue::User(u) => CommandOptionValueRaw::String(u.to_string().into()),
            CommandOptionValue::Channel(c) => CommandOptionValueRaw::String(c.to_string().into()),
            CommandOptionValue::Role(r) => CommandOptionValueRaw::String(r.to_string().into()),
            CommandOptionValue::Mentionable(m) => {
                CommandOptionValueRaw::String(m.to_string().into())
            }
            CommandOptionValue::SubCommand(_) | CommandOptionValue::SubCommandGroup(_) => {
                return None;
            }
        })
    }
}
