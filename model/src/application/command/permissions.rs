use crate::id::{ApplicationId, CommandId, GenericId, GuildId, RoleId, UserId};
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuildCommandPermissions {
    pub application_id: ApplicationId,
    pub guild_id: GuildId,
    pub id: CommandId,
    pub permissions: Vec<CommandPermissions>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandPermissions {
    pub id: CommandPermissionsType,
    pub permission: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandPermissionsType {
    Role(RoleId),
    User(UserId),
}

#[derive(Deserialize, Serialize)]
struct CommandPermissionsData {
    id: GenericId,
    #[serde(rename = "type")]
    kind: CommandPermissionsDataType,
    permission: bool,
}

#[derive(Clone, Debug, Deserialize_repr, Eq, PartialEq, Serialize_repr)]
#[repr(u8)]
enum CommandPermissionsDataType {
    Role = 1,
    User = 2,
}

impl<'de> Deserialize<'de> for CommandPermissions {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = CommandPermissionsData::deserialize(deserializer)?;

        let span = tracing::trace_span!("deserializing command permission");
        let _span_enter = span.enter();

        let id = match data.kind {
            CommandPermissionsDataType::Role => {
                let id = RoleId(data.id.0);
                tracing::trace!(id = %id.0, kind = ?data.kind);

                CommandPermissionsType::Role(id)
            }
            CommandPermissionsDataType::User => {
                let id = UserId(data.id.0);
                tracing::trace!(id = %id.0, kind = ?data.kind);

                CommandPermissionsType::User(id)
            }
        };

        Ok(Self {
            id,
            permission: data.permission,
        })
    }
}

impl Serialize for CommandPermissions {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let data = CommandPermissionsData {
            id: match self.id {
                CommandPermissionsType::Role(role_id) => GenericId(role_id.0),
                CommandPermissionsType::User(user_id) => GenericId(user_id.0),
            },
            kind: match self.id {
                CommandPermissionsType::Role(_) => CommandPermissionsDataType::Role,
                CommandPermissionsType::User(_) => CommandPermissionsDataType::User,
            },
            permission: self.permission,
        };

        data.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::{CommandPermissions, CommandPermissionsType};
    use crate::id::RoleId;
    use serde_test::Token;

    #[test]
    fn test_command_permissions() {
        let value = CommandPermissions {
            id: CommandPermissionsType::Role(RoleId::new(100).expect("non zero")),
            permission: true,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CommandPermissionsData",
                    len: 3,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "GenericId" },
                Token::Str("100"),
                Token::Str("type"),
                Token::U8(1),
                Token::Str("permission"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
