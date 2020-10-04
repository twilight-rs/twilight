use crate::{guild::Permissions, id::RoleId};
use serde::{
    de::{DeserializeSeed, Deserializer, SeqAccess, Visitor},
    Deserialize, Serialize,
};
use serde_mappable_seq::Key;
use std::{
    collections::HashMap,
    fmt::{Formatter, Result as FmtResult},
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Role {
    pub color: u32,
    pub hoist: bool,
    pub id: RoleId,
    pub managed: bool,
    pub mentionable: bool,
    pub name: String,
    pub permissions: Permissions,
    pub position: i64,
}

impl Key<'_, RoleId> for Role {
    fn key(&self) -> RoleId {
        self.id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleMapDeserializer;

struct RoleMapDeserializerVisitor;

impl<'de> Visitor<'de> for RoleMapDeserializerVisitor {
    type Value = HashMap<RoleId, Role>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a sequence of roles")
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let mut map = seq
            .size_hint()
            .map_or_else(HashMap::new, HashMap::with_capacity);

        let span = tracing::trace_span!("adding elements to role map");
        let _span_enter = span.enter();

        while let Some(role) = seq.next_element::<Role>()? {
            tracing::trace!(%role.id, ?role);

            map.insert(role.id, role);
        }

        Ok(map)
    }
}

impl<'de> DeserializeSeed<'de> for RoleMapDeserializer {
    type Value = HashMap<RoleId, Role>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_seq(RoleMapDeserializerVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::{Permissions, Role, RoleId};
    use serde_test::Token;

    #[test]
    fn test_role() {
        let role = Role {
            color: 0,
            hoist: true,
            id: RoleId(123),
            managed: false,
            mentionable: true,
            name: "test".to_owned(),
            permissions: Permissions::ADMINISTRATOR,
            position: 12,
        };

        serde_test::assert_tokens(
            &role,
            &[
                Token::Struct {
                    name: "Role",
                    len: 8,
                },
                Token::Str("color"),
                Token::U32(0),
                Token::Str("hoist"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("123"),
                Token::Str("managed"),
                Token::Bool(false),
                Token::Str("mentionable"),
                Token::Bool(true),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("permissions"),
                Token::Str("8"),
                Token::Str("position"),
                Token::I64(12),
                Token::StructEnd,
            ],
        );
    }
}
