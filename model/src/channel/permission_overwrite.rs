use crate::{
    guild::Permissions,
    id::{RoleId, UserId},
};
use serde::{
    de::{Deserializer, Error as DeError},
    ser::{Error as SerError, SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::convert::TryInto;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PermissionOverwrite {
    pub allow_old: Permissions,
    pub allow: Permissions,
    pub deny_old: Permissions,
    pub deny: Permissions,
    pub kind: PermissionOverwriteType,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PermissionOverwriteType {
    Member(UserId),
    Role(RoleId),
}

#[derive(Deserialize)]
struct PermissionOverwriteData {
    #[serde(rename = "allow")]
    allow_old: Permissions,
    #[serde(rename = "allow_new")]
    allow: Permissions,
    #[serde(rename = "deny")]
    deny_old: Permissions,
    #[serde(rename = "deny_new")]
    deny: Permissions,
    id: String,
    #[serde(rename = "type")]
    kind: PermissionOverwriteTypeName,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum PermissionOverwriteTypeName {
    Member,
    Role,
}

impl<'de> Deserialize<'de> for PermissionOverwrite {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = PermissionOverwriteData::deserialize(deserializer)?;

        let span = tracing::trace_span!("deserializing permission overwrite");
        let _span_enter = span.enter();

        let kind = match data.kind {
            PermissionOverwriteTypeName::Member => {
                let id = UserId(data.id.parse().map_err(DeError::custom)?);
                tracing::trace!(id = %id.0, kind = ?data.kind);

                PermissionOverwriteType::Member(id)
            }
            PermissionOverwriteTypeName::Role => {
                let id = RoleId(data.id.parse().map_err(DeError::custom)?);
                tracing::trace!(id = %id.0, kind = ?data.kind);

                PermissionOverwriteType::Role(id)
            }
        };

        Ok(Self {
            allow_old: data.allow_old,
            allow: data.allow,
            deny_old: data.deny_old,
            deny: data.deny,
            kind,
        })
    }
}

impl Serialize for PermissionOverwrite {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("PermissionOverwrite", 4)?;

        state.serialize_field("allow_new", &self.allow)?;
        let allow_bits: u64 = self
            .allow_old
            .bits()
            .try_into()
            .map_err(|_| SerError::custom("allow_old bits can't be a u64"))?;
        state.serialize_field("allow", &allow_bits)?;
        state.serialize_field("deny_new", &self.deny)?;
        let deny_bits: u64 = self
            .deny_old
            .bits()
            .try_into()
            .map_err(|_| SerError::custom("deny_old bits can't be a u64"))?;
        state.serialize_field("deny", &deny_bits)?;

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

#[cfg(test)]
mod tests {
    use super::{PermissionOverwrite, PermissionOverwriteType, Permissions};
    use crate::id::UserId;

    #[test]
    fn test_overwrite() {
        let overwrite = PermissionOverwrite {
            allow_old: Permissions::CREATE_INVITE,
            allow: Permissions::CREATE_INVITE,
            deny_old: Permissions::KICK_MEMBERS,
            deny: Permissions::KICK_MEMBERS,
            kind: PermissionOverwriteType::Member(UserId(12_345_678)),
        };

        // We can't use serde_test because it doesn't support 128 bit integers.
        //
        // <https://github.com/serde-rs/serde/issues/1281>
        let input = r#"{
  "allow_new": "1",
  "allow": 1,
  "deny_new": "2",
  "deny": 2,
  "id": "12345678",
  "type": "member"
}"#;

        assert_eq!(
            serde_json::from_str::<PermissionOverwrite>(input).unwrap(),
            overwrite
        );
        assert_eq!(serde_json::to_string_pretty(&overwrite).unwrap(), input);
    }
}
