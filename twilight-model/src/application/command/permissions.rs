//! Limit who and where commands can be executed.

use crate::id::{
    marker::{
        ApplicationMarker, ChannelMarker, CommandMarker, GenericMarker, GuildMarker, RoleMarker,
        UserMarker,
    },
    Id,
};
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// List of [`CommandPermission`]s for a command in a guild.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildCommandPermissions {
    /// ID of the application the command belongs to.
    pub application_id: Id<ApplicationMarker>,
    /// ID of the guild.
    pub guild_id: Id<GuildMarker>,
    /// ID of the command.
    pub id: Id<CommandMarker>,
    /// Command permissions in the guild.
    ///
    /// Max 100.
    pub permissions: Vec<CommandPermission>,
}

/// Member, channel or role explicit permission to use a command.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CommandPermission {
    /// Affected resource.
    pub id: CommandPermissionType,
    /// Whether the resource is allowed or disallowed to use the command.
    pub permission: bool,
}

/// Resources commands can allow or disallow from executing them.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CommandPermissionType {
    /// Affected channel.
    ///
    /// Use `@everyone - 1` for all channels in the guild.
    Channel(Id<ChannelMarker>),
    /// Affected role.
    ///
    /// The `@everyone` role is permitted.
    Role(Id<RoleMarker>),
    /// Affected member.
    User(Id<UserMarker>),
}

impl CommandPermissionType {
    /// Get the inner ID.
    const fn id(self) -> Id<GenericMarker> {
        match self {
            Self::Channel(id) => id.cast(),
            Self::Role(id) => id.cast(),
            Self::User(id) => id.cast(),
        }
    }

    /// Get the associated resource type.
    const fn kind(self) -> CommandPermissionDataType {
        match self {
            Self::Channel(_) => CommandPermissionDataType::Channel,
            Self::Role(_) => CommandPermissionDataType::Role,
            Self::User(_) => CommandPermissionDataType::User,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct CommandPermissionData {
    /// Affected resource.
    id: Id<GenericMarker>,
    /// Resource type.
    #[serde(rename = "type")]
    kind: CommandPermissionDataType,
    /// Whether the resource is allowed or disallowed.
    permission: bool,
}

#[derive(Clone, Debug, Deserialize_repr, Eq, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u8)]
enum CommandPermissionDataType {
    Role = 1,
    User = 2,
    Channel = 3,
}

impl<'de> Deserialize<'de> for CommandPermission {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = CommandPermissionData::deserialize(deserializer)?;

        let span = tracing::trace_span!("deserializing command permission");
        let _span_enter = span.enter();

        let id = match data.kind {
            CommandPermissionDataType::Role => CommandPermissionType::Role(data.id.cast()),
            CommandPermissionDataType::User => CommandPermissionType::User(data.id.cast()),
            CommandPermissionDataType::Channel => CommandPermissionType::Channel(data.id.cast()),
        };

        tracing::trace!(id = %data.id, kind = ?data.kind);

        Ok(Self {
            id,
            permission: data.permission,
        })
    }
}

impl Serialize for CommandPermission {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let data = CommandPermissionData {
            id: self.id.id(),
            kind: self.id.kind(),
            permission: self.permission,
        };

        data.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CommandPermission, CommandPermissionDataType, CommandPermissionType,
        GuildCommandPermissions,
    };
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn serde_command_permission() {
        let value = CommandPermission {
            id: CommandPermissionType::Role(Id::new(100)),
            permission: true,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CommandPermissionData",
                    len: 3,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100"),
                Token::Str("type"),
                Token::U8(CommandPermissionDataType::Role as u8),
                Token::Str("permission"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn serde_guild_command_permission() {
        let value = GuildCommandPermissions {
            application_id: Id::new(1),
            guild_id: Id::new(2),
            id: Id::new(3),
            permissions: Vec::from([
                CommandPermission {
                    id: CommandPermissionType::Channel(Id::new(50)),
                    permission: false,
                },
                CommandPermission {
                    id: CommandPermissionType::User(Id::new(200)),
                    permission: true,
                },
            ]),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildCommandPermissions",
                    len: 4,
                },
                Token::Str("application_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("permissions"),
                Token::Seq { len: Some(2) },
                Token::Struct {
                    name: "CommandPermissionData",
                    len: 3,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("50"),
                Token::Str("type"),
                Token::U8(CommandPermissionDataType::Channel as u8),
                Token::Str("permission"),
                Token::Bool(false),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandPermissionData",
                    len: 3,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("200"),
                Token::Str("type"),
                Token::U8(CommandPermissionDataType::User as u8),
                Token::Str("permission"),
                Token::Bool(true),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
