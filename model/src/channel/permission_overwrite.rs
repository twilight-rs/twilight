use crate::{
    guild::Permissions,
    id::{RoleId, UserId},
};
use serde::{
    de::{Deserialize, Deserializer, Error as DeError},
    ser::{Serialize, SerializeStruct, Serializer},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PermissionOverwrite {
    pub allow: Permissions,
    pub deny: Permissions,
    pub kind: PermissionOverwriteType,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PermissionOverwriteType {
    Member(UserId),
    Role(RoleId),
}

#[derive(serde::Deserialize, serde::Serialize)]
struct PermissionOverwriteData {
    allow: Permissions,
    deny: Permissions,
    id: String,
    #[serde(rename = "type")]
    kind: PermissionOverwriteTypeName,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum PermissionOverwriteTypeName {
    Member,
    Role,
}

impl<'de> Deserialize<'de> for PermissionOverwrite {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = PermissionOverwriteData::deserialize(deserializer)?;

        let kind = match data.kind {
            PermissionOverwriteTypeName::Member => {
                let id = UserId(data.id.parse().map_err(DeError::custom)?);

                PermissionOverwriteType::Member(id)
            }
            PermissionOverwriteTypeName::Role => {
                let id = RoleId(data.id.parse().map_err(DeError::custom)?);

                PermissionOverwriteType::Role(id)
            }
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
            }
            PermissionOverwriteType::Role(id) => {
                state.serialize_field("id", &id)?;
                state.serialize_field("type", "role")?;
            }
        }

        state.end()
    }
}
