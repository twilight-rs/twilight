mod resolved;

pub use self::resolved::{CommandInteractionDataResolved, InteractionChannel, InteractionMember};

use crate::{
    application::command::{CommandOptionType, CommandType, Number},
    id::{
        marker::{ChannelMarker, CommandMarker, GenericMarker, RoleMarker, UserMarker},
        Id,
    },
};
use serde::{
    de::{Error as DeError, IgnoredAny, MapAccess, Unexpected, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt::{Formatter, Result as FmtResult};

/// Data received when an [`ApplicationCommand`] interaction is executed.
///
/// See [Discord Docs/Interaction Object].
///
/// [`ApplicationCommand`]: crate::application::interaction::Interaction::ApplicationCommand
/// [Discord Docs/Interaction Object]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data-structure
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandData {
    /// ID of the command.
    pub id: Id<CommandMarker>,
    /// Name of the command.
    pub name: String,
    /// Type of the command.
    #[serde(rename = "type")]
    pub kind: CommandType,
    /// List of parsed options specified by the user.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CommandDataOption>,
    /// Data sent if any of the options are Discord types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<CommandInteractionDataResolved>,
    /// If this is a user or message command, the ID of the targeted user/message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<Id<GenericMarker>>,
}

/// Data received when a user fills in a command option.
///
/// See [Discord Docs/Application Command Object].
///
/// [Discord Docs/Application Command Object]: https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-interaction-data-option-structure
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandDataOption {
    /// [`true`] if this autocomplete option is currently highlighted.
    pub focused: bool,
    pub name: String,
    pub value: CommandOptionValue,
}

impl Serialize for CommandDataOption {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let subcommand_is_empty = matches!(
            &self.value,
            CommandOptionValue::SubCommand(o)
            | CommandOptionValue::SubCommandGroup(o)
                if o.is_empty()
        );

        let len = 2 + !subcommand_is_empty as usize + self.focused as usize;

        let mut state = serializer.serialize_struct("CommandDataOption", len)?;

        if self.focused {
            state.serialize_field("focused", &self.focused)?;
        }

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

impl<'de> Deserialize<'de> for CommandDataOption {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Debug, Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Fields {
            Name,
            Type,
            Value,
            Options,
            Focused,
        }

        // Id before string such that IDs will always be interpreted
        // as such, this does mean that string inputs that looks like
        // IDs will have to be caught if it is a string.
        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        enum ValueEnvelope {
            Boolean(bool),
            Integer(i64),
            Number(f64),
            Id(Id<GenericMarker>),
            String(String),
        }

        fn make_unexpected(unexpected: &ValueEnvelope) -> Unexpected<'_> {
            match unexpected {
                ValueEnvelope::Boolean(b) => Unexpected::Bool(*b),
                ValueEnvelope::Integer(i) => Unexpected::Signed(*i),
                ValueEnvelope::Number(f) => Unexpected::Float(*f),
                ValueEnvelope::Id(_id) => Unexpected::Other("ID"),
                ValueEnvelope::String(s) => Unexpected::Str(s),
            }
        }

        struct CommandDataOptionVisitor;

        impl<'de> Visitor<'de> for CommandDataOptionVisitor {
            type Value = CommandDataOption;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
                formatter.write_str("CommandDataOption")
            }

            #[allow(clippy::too_many_lines)]
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name_opt = None;
                let mut kind_opt = None;
                let mut options = Vec::new();
                let mut value_opt = None;
                let mut focused = None;

                loop {
                    let key = match map.next_key() {
                        Ok(Some(key)) => key,
                        Ok(None) => break,
                        #[cfg(feature = "tracing")]
                        Err(why) => {
                            map.next_value::<IgnoredAny>()?;

                            tracing::trace!("ran into an unknown key: {:?}", why);

                            continue;
                        }
                        #[cfg(not(feature = "tracing"))]
                        Err(_) => {
                            map.next_value::<IgnoredAny>()?;

                            continue;
                        }
                    };

                    match key {
                        Fields::Name => {
                            if name_opt.is_some() {
                                return Err(DeError::duplicate_field("name"));
                            }

                            name_opt = Some(map.next_value()?);
                        }
                        Fields::Type => {
                            if kind_opt.is_some() {
                                return Err(DeError::duplicate_field("type"));
                            }

                            kind_opt = Some(map.next_value()?);
                        }
                        Fields::Value => {
                            if value_opt.is_some() {
                                return Err(DeError::duplicate_field("value"));
                            }

                            value_opt = Some(map.next_value()?);
                        }
                        Fields::Options => {
                            if !options.is_empty() {
                                return Err(DeError::duplicate_field("options"));
                            }

                            options = map.next_value()?;
                        }
                        Fields::Focused => {
                            if focused.is_some() {
                                return Err(DeError::duplicate_field("focused"));
                            }

                            focused = map.next_value()?;
                        }
                    }
                }

                let name = name_opt.ok_or_else(|| DeError::missing_field("name"))?;
                let kind = kind_opt.ok_or_else(|| DeError::missing_field("type"))?;

                let value = match kind {
                    CommandOptionType::Boolean => {
                        let val = value_opt.ok_or_else(|| DeError::missing_field("value"))?;

                        if let ValueEnvelope::Boolean(b) = val {
                            CommandOptionValue::Boolean(b)
                        } else {
                            return Err(DeError::invalid_type(make_unexpected(&val), &"boolean"));
                        }
                    }
                    CommandOptionType::Channel => {
                        let val = value_opt.ok_or_else(|| DeError::missing_field("value"))?;

                        if let ValueEnvelope::Id(id) = val {
                            CommandOptionValue::Channel(id.cast())
                        } else {
                            return Err(DeError::invalid_type(
                                make_unexpected(&val),
                                &"channel id",
                            ));
                        }
                    }
                    CommandOptionType::Integer => {
                        let val = value_opt.ok_or_else(|| DeError::missing_field("value"))?;

                        if let ValueEnvelope::Integer(i) = val {
                            CommandOptionValue::Integer(i)
                        } else {
                            return Err(DeError::invalid_type(make_unexpected(&val), &"integer"));
                        }
                    }
                    CommandOptionType::Mentionable => {
                        let val = value_opt.ok_or_else(|| DeError::missing_field("value"))?;

                        if let ValueEnvelope::Id(id) = val {
                            CommandOptionValue::Mentionable(id)
                        } else {
                            return Err(DeError::invalid_type(
                                make_unexpected(&val),
                                &"mentionable id",
                            ));
                        }
                    }
                    CommandOptionType::Number => {
                        let val = value_opt.ok_or_else(|| DeError::missing_field("value"))?;

                        match val {
                            ValueEnvelope::Integer(i) => {
                                // As json allows sending floating
                                // points without the tailing decimals
                                // it may be interpreted as a integer
                                // but it is safe to cast as there can
                                // not occur any loss.
                                #[allow(clippy::cast_precision_loss)]
                                CommandOptionValue::Number(Number(i as f64))
                            }
                            ValueEnvelope::Number(f) => CommandOptionValue::Number(Number(f)),
                            other => {
                                return Err(DeError::invalid_type(
                                    make_unexpected(&other),
                                    &"number",
                                ));
                            }
                        }
                    }
                    CommandOptionType::Role => {
                        let val = value_opt.ok_or_else(|| DeError::missing_field("value"))?;

                        if let ValueEnvelope::Id(id) = val {
                            CommandOptionValue::Role(id.cast())
                        } else {
                            return Err(DeError::invalid_type(make_unexpected(&val), &"role id"));
                        }
                    }

                    CommandOptionType::String => {
                        let val = value_opt.ok_or_else(|| DeError::missing_field("value"))?;

                        match val {
                            ValueEnvelope::String(s) => CommandOptionValue::String(s),
                            ValueEnvelope::Id(id) => {
                                CommandOptionValue::String(id.get().to_string())
                            }
                            other => {
                                return Err(DeError::invalid_type(
                                    make_unexpected(&other),
                                    &"string",
                                ));
                            }
                        }
                    }
                    CommandOptionType::SubCommand => CommandOptionValue::SubCommand(options),
                    CommandOptionType::SubCommandGroup => {
                        CommandOptionValue::SubCommandGroup(options)
                    }
                    CommandOptionType::User => {
                        let val = value_opt.ok_or_else(|| DeError::missing_field("value"))?;

                        if let ValueEnvelope::Id(id) = val {
                            CommandOptionValue::User(id.cast())
                        } else {
                            return Err(DeError::invalid_type(make_unexpected(&val), &"user id"));
                        }
                    }
                };

                Ok(CommandDataOption {
                    name,
                    value,
                    focused: focused.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_map(CommandDataOptionVisitor)
    }
}

/// Value of a [`CommandDataOption`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandOptionValue {
    Boolean(bool),
    Channel(Id<ChannelMarker>),
    Integer(i64),
    Mentionable(Id<GenericMarker>),
    Number(Number),
    Role(Id<RoleMarker>),
    String(String),
    SubCommand(Vec<CommandDataOption>),
    SubCommandGroup(Vec<CommandDataOption>),
    User(Id<UserMarker>),
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
            command::{CommandOptionType, CommandType, Number},
            interaction::application_command::{CommandDataOption, CommandOptionValue},
        },
        id::Id,
    };
    use serde_test::Token;

    #[test]
    fn no_options() {
        let value = CommandData {
            id: Id::new(1),
            name: "permissions".to_owned(),
            kind: CommandType::ChatInput,
            options: Vec::new(),
            resolved: None,
            target_id: None,
        };
        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CommandData",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("permissions"),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    fn subcommand_without_option() {
        let value = CommandData {
            id: Id::new(1),
            name: "photo".to_owned(),
            kind: CommandType::ChatInput,
            options: Vec::from([CommandDataOption {
                focused: false,
                name: "cat".to_owned(),
                value: CommandOptionValue::SubCommand(Vec::new()),
            }]),
            resolved: None,
            target_id: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CommandData",
                    len: 3,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("photo"),
                Token::Str("options"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandDataOption",
                    len: 2,
                },
                Token::Str("name"),
                Token::Str("cat"),
                Token::Str("type"),
                Token::U8(CommandOptionType::SubCommand as u8),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn numbers() {
        let value = CommandDataOption {
            focused: false,
            name: "opt".to_string(),
            value: CommandOptionValue::Number(Number(5.0)),
        };

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CommandDataOption",
                    len: 3,
                },
                Token::Str("name"),
                Token::Str("opt"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Number as u8),
                Token::Str("value"),
                Token::I64(5),
                Token::StructEnd,
            ],
        );
    }
}
