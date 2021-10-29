mod resolved;

pub use self::resolved::{CommandInteractionDataResolved, InteractionChannel, InteractionMember};

use crate::{
    application::command::{CommandOptionType, Number},
    id::{ChannelId, CommandId, GenericId, RoleId, UserId},
};
use serde::{
    de::Error as DeError, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
};

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

impl<'de> Deserialize<'de> for CommandDataOption {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = CommandDataOptionEnvelope::deserialize(deserializer)?;

        let value = match (raw.kind, raw.value) {
            (CommandOptionType::Boolean, Some(CommandOptionValueEnvelope::Boolean(b))) => {
                CommandOptionValue::Boolean(b)
            }
            (CommandOptionType::Channel, Some(CommandOptionValueEnvelope::Id(i))) => {
                CommandOptionValue::Channel(i.0.into())
            }
            (CommandOptionType::Integer, Some(CommandOptionValueEnvelope::Integer(i))) => {
                CommandOptionValue::Integer(i)
            }
            (CommandOptionType::Mentionable, Some(CommandOptionValueEnvelope::Id(i))) => {
                CommandOptionValue::Mentionable(i.0.into())
            }
            (CommandOptionType::Number, Some(CommandOptionValueEnvelope::Number(n))) => {
                CommandOptionValue::Number(n)
            }
            (CommandOptionType::Role, Some(CommandOptionValueEnvelope::Id(i))) => {
                CommandOptionValue::Role(i.0.into())
            }
            (CommandOptionType::String, Some(CommandOptionValueEnvelope::String(s))) => {
                CommandOptionValue::String(s)
            }
            (CommandOptionType::SubCommand, _) => CommandOptionValue::SubCommand(raw.options),
            (CommandOptionType::SubCommandGroup, _) => {
                CommandOptionValue::SubCommandGroup(raw.options)
            }
            (CommandOptionType::User, Some(CommandOptionValueEnvelope::Id(i))) => {
                CommandOptionValue::User(i.0.into())
            }
            (t, v) => {
                return Err(DeError::custom(format!(
                    "invalid value/type pair: value={:?} type={:?}",
                    v, t
                )));
            }
        };

        Ok(CommandDataOption {
            name: raw.name,
            value,
        })
    }
}

impl Serialize for CommandDataOption {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let subcommand_is_empty = matches!(
            &self.value,
            CommandOptionValue::SubCommand(o)
            | CommandOptionValue::SubCommandGroup(o)
                if o.is_empty()
        );

        let len = 2 + !subcommand_is_empty as usize;

        let mut state = serializer.serialize_struct("CommandDataOptionEnvelope", len)?;

        state.serialize_field("name", &self.name)?;

        state.serialize_field("type", &self.value.kind())?;

        match &self.value {
            CommandOptionValue::Boolean(b) => state.serialize_field("value", b)?,
            CommandOptionValue::Channel(c) => state.serialize_field("value", c)?,
            CommandOptionValue::Integer(i) => state.serialize_field("value", i)?,
            CommandOptionValue::Mentionable(m) => state.serialize_field("value", m)?,
            CommandOptionValue::Number(n) => state.serialize_field("value", n)?,
            CommandOptionValue::Role(r) => state.serialize_field("value", r)?,
            CommandOptionValue::String(s) => state.serialize_field("value", s)?,
            CommandOptionValue::User(u) => state.serialize_field("value", u)?,
            CommandOptionValue::SubCommand(s) | CommandOptionValue::SubCommandGroup(s) => {
                if !subcommand_is_empty {
                    state.serialize_field("options", s)?
                }
            }
        }

        state.end()
    }
}

#[derive(Deserialize)]
struct CommandDataOptionEnvelope {
    name: String,
    #[serde(rename = "type")]
    kind: CommandOptionType,
    #[serde(default)]
    options: Vec<CommandDataOption>,
    value: Option<CommandOptionValueEnvelope>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CommandOptionValueEnvelope {
    Boolean(bool),
    Id(GenericId),
    Integer(i64),
    Number(Number),
    String(String),
}

/// Value of a [`CommandDataOption`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandOptionValue {
    Boolean(bool),
    Channel(ChannelId),
    Integer(i64),
    Mentionable(GenericId),
    Number(Number),
    Role(RoleId),
    String(String),
    SubCommand(Vec<CommandDataOption>),
    SubCommandGroup(Vec<CommandDataOption>),
    User(UserId),
}

impl CommandOptionValue {
    pub const fn kind(&self) -> CommandOptionType {
        match self {
            CommandOptionValue::Boolean(_) => CommandOptionType::Boolean,
            CommandOptionValue::Channel(_) => CommandOptionType::Channel,
            CommandOptionValue::Integer(_) => CommandOptionType::Integer,
            CommandOptionValue::Mentionable(_) => CommandOptionType::Mentionable,
            CommandOptionValue::Number(_) => CommandOptionType::Number,
            CommandOptionValue::Role(_) => CommandOptionType::Role,
            CommandOptionValue::String(_) => CommandOptionType::String,
            CommandOptionValue::SubCommand(_) => CommandOptionType::SubCommand,
            CommandOptionValue::SubCommandGroup(_) => CommandOptionType::SubCommandGroup,
            CommandOptionValue::User(_) => CommandOptionType::User,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CommandData;
    use crate::{
        application::{
            command::CommandOptionType,
            interaction::application_command::{CommandDataOption, CommandOptionValue},
        },
        id::CommandId,
    };
    use serde_test::Token;

    #[test]
    fn subcommand_without_option() {
        let value = CommandData {
            id: CommandId::new(1).expect("non zero"),
            name: "photo".to_owned(),
            options: Vec::from([CommandDataOption {
                name: "cat".to_owned(),
                value: CommandOptionValue::SubCommand(Vec::new()),
            }]),
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
                    name: "CommandDataOptionEnvelope",
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
