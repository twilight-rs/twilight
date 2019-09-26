use crate::{
    guild::Permissions,
    id::{RoleId, UserId},
};
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, SerializeStruct, Serializer},
    Deserialize as DeserializeMacro,
    Serialize as SerializeMacro,
};

#[derive(DeserializeMacro, SerializeMacro)]
enum PermissionOverwriteTypeName {
    Member,
    Role,
}

#[derive(DeserializeMacro, SerializeMacro)]
struct PermissionOverwriteData {
    allow: Permissions,
    deny: Permissions,
    id: u64,
    #[serde(rename = "type")]
    kind: PermissionOverwriteTypeName,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PermissionOverwrite {
    pub allow: Permissions,
    pub deny: Permissions,
    pub kind: PermissionOverwriteType,
}

impl<'de> Deserialize<'de> for PermissionOverwrite {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = PermissionOverwriteData::deserialize(deserializer)?;

        let kind = match data.kind {
            PermissionOverwriteTypeName::Member => PermissionOverwriteType::Member(UserId(data.id)),
            PermissionOverwriteTypeName::Role => PermissionOverwriteType::Role(RoleId(data.id)),
        };

        Ok(Self {
            allow: data.allow,
            deny: data.deny,
            kind,
        })
    }
}

impl Serialize for PermissionOverwrite {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("PermissionOverwrite", 4)?;

        state.serialize_field("allow", &self.allow.bits())?;
        state.serialize_field("deny", &self.deny.bits())?;

        match &self.kind {
            PermissionOverwriteType::Member(id) => {
                state.serialize_field("id", &id)?;
                state.serialize_field("type", "member")?;
            },
            PermissionOverwriteType::Role(id) => {
                state.serialize_field("id", &id)?;
                state.serialize_field("type", "role")?;
            },
        }

        state.end()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PermissionOverwriteType {
    Member(UserId),
    Role(RoleId),
}
