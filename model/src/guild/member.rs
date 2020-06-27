use crate::{
    id::{GuildId, RoleId, UserId},
    user::User,
};

use serde::{
    de::{
        value::MapAccessDeserializer, DeserializeSeed, Deserializer, Error as DeError, MapAccess,
        SeqAccess, Visitor,
    },
    Deserialize, Serialize,
};
use serde_mappable_seq::Key;
use std::{
    collections::HashMap,
    fmt::{Formatter, Result as FmtResult},
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Member {
    pub deaf: bool,
    pub guild_id: GuildId,
    pub hoisted_role: Option<RoleId>,
    pub joined_at: Option<String>,
    pub mute: bool,
    pub nick: Option<String>,
    pub premium_since: Option<String>,
    pub roles: Vec<RoleId>,
    pub user: User,
}

impl Key<'_, UserId> for Member {
    fn key(&self) -> UserId {
        self.user.id
    }
}

// Used in the guild deserializer.
#[derive(Deserialize, Serialize)]
pub(crate) struct MemberIntermediary {
    pub deaf: bool,
    pub hoisted_role: Option<RoleId>,
    pub joined_at: Option<String>,
    pub mute: bool,
    pub nick: Option<String>,
    pub premium_since: Option<String>,
    pub roles: Vec<RoleId>,
    pub user: User,
}

/// Deserialize a member when the payload doesn't have the guild ID but
/// you already know the guild ID.
///
/// Member payloads from the HTTP API, for example, don't have the guild
/// ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberDeserializer(GuildId);

impl MemberDeserializer {
    /// Create a new deserializer for a member when you know the ID but the
    /// payload probably doesn't contain it.
    pub fn new(guild_id: GuildId) -> Self {
        Self(guild_id)
    }
}

impl<'de> DeserializeSeed<'de> for MemberDeserializer {
    type Value = Member;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_map(MemberVisitor(self.0))
    }
}

pub(crate) struct MemberVisitor(GuildId);

impl<'de> Visitor<'de> for MemberVisitor {
    type Value = Member;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a map of member fields")
    }

    fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
        let deser = MapAccessDeserializer::new(map);
        let member = MemberIntermediary::deserialize(deser)?;

        Ok(Member {
            deaf: member.deaf,
            guild_id: self.0,
            hoisted_role: member.hoisted_role,
            joined_at: member.joined_at,
            mute: member.mute,
            nick: member.nick,
            premium_since: member.premium_since,
            roles: member.roles,
            user: member.user,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OptionalMemberDeserializer(GuildId);

impl OptionalMemberDeserializer {
    /// Create a new deserializer for a member when you know the ID but the
    /// payload probably doesn't contain it.
    pub fn new(guild_id: GuildId) -> Self {
        Self(guild_id)
    }
}

impl<'de> DeserializeSeed<'de> for OptionalMemberDeserializer {
    type Value = Option<Member>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_option(OptionalMemberVisitor(self.0))
    }
}

struct OptionalMemberVisitor(GuildId);

impl<'de> Visitor<'de> for OptionalMemberVisitor {
    type Value = Option<Member>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("an optional member")
    }

    fn visit_none<E: DeError>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        Ok(Some(deserializer.deserialize_map(MemberVisitor(self.0))?))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberMapDeserializer(GuildId);

impl MemberMapDeserializer {
    /// Create a new deserializer for a map of members when you know the
    /// Guild ID but the payload probably doesn't contain it.
    pub fn new(guild_id: GuildId) -> Self {
        Self(guild_id)
    }
}

struct MemberMapVisitor(GuildId);

impl<'de> Visitor<'de> for MemberMapVisitor {
    type Value = HashMap<UserId, Member>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a sequence of members")
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let mut map = seq
            .size_hint()
            .map_or_else(HashMap::new, HashMap::with_capacity);

        while let Some(member) = seq.next_element_seed(MemberDeserializer(self.0))? {
            map.insert(member.user.id, member);
        }

        Ok(map)
    }
}

impl<'de> DeserializeSeed<'de> for MemberMapDeserializer {
    type Value = HashMap<UserId, Member>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(MemberMapVisitor(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::{Member, MemberDeserializer};
    use crate::{
        id::{GuildId, UserId},
        user::User,
    };
    use serde::de::DeserializeSeed;
    use serde_value::Value;
    use std::collections::BTreeMap;

    #[test]
    fn test_member_deserializer() {
        let mut user = BTreeMap::new();
        user.insert(
            Value::String("discriminator".to_owned()),
            Value::String("0001".to_owned()),
        );
        user.insert(
            Value::String("id".to_owned()),
            Value::String("2".to_owned()),
        );
        user.insert(
            Value::String("username".to_owned()),
            Value::String("twilight".to_owned()),
        );

        let mut map = BTreeMap::new();
        map.insert(Value::String("deaf".to_owned()), Value::Bool(false));
        map.insert(
            Value::String("hoisted_role".to_owned()),
            Value::Option(None),
        );
        map.insert(
            Value::String("joined_at".to_owned()),
            Value::String(String::new()),
        );
        map.insert(Value::String("mute".to_owned()), Value::Bool(true));
        map.insert(
            Value::String("nick".to_owned()),
            Value::Option(Some(Box::new(Value::String("twilight".to_owned())))),
        );
        map.insert(
            Value::String("premium_since".to_owned()),
            Value::Option(None),
        );
        map.insert(Value::String("roles".to_owned()), Value::Seq(Vec::new()));
        map.insert(Value::String("user".to_owned()), Value::Map(user));
        let value = Value::Map(map);

        let expected = Member {
            deaf: false,
            guild_id: GuildId(1),
            hoisted_role: None,
            joined_at: Some(String::new()),
            mute: true,
            nick: Some("twilight".to_owned()),
            premium_since: None,
            roles: Vec::new(),
            user: User {
                avatar: None,
                bot: false,
                discriminator: "0001".to_owned(),
                locale: None,
                email: None,
                flags: None,
                id: UserId(2),
                mfa_enabled: None,
                name: "twilight".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
        };

        let deserializer = MemberDeserializer::new(GuildId(1));

        assert_eq!(expected, deserializer.deserialize(value).unwrap());
    }
}
