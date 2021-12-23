use crate::{
    datetime::Timestamp,
    id::{GuildId, RoleId},
    user::User,
};

use serde::{
    de::{
        value::MapAccessDeserializer, DeserializeSeed, Deserializer, Error as DeError, MapAccess,
        SeqAccess, Visitor,
    },
    Deserialize, Serialize, Serializer,
};
use std::{
    fmt::{Formatter, Result as FmtResult},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Member {
    /// Member's guild avatar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub communication_disabled_until: MemberTimeoutState,
    pub deaf: bool,
    pub guild_id: GuildId,
    pub joined_at: Timestamp,
    pub mute: bool,
    pub nick: Option<String>,
    /// Whether the user has yet to pass the guild's [Membership Screening]
    /// requirements.
    pub pending: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<Timestamp>,
    pub roles: Vec<RoleId>,
    pub user: User,
}

/// Version of [`Member`] but without a guild ID, useful in some contexts.
///
/// The HTTP and Gateway APIs don't include guild IDs in their payloads, so this
/// can be useful when you're unable to use a deserialization seed like
/// [`MemberDeserializer`].
// Used in the guild deserializer.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MemberIntermediary {
    /// Member's guild avatar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    pub communication_disabled_until: MemberTimeoutState,
    pub deaf: bool,
    pub joined_at: Timestamp,
    pub mute: bool,
    pub nick: Option<String>,
    #[serde(default)]
    pub pending: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<Timestamp>,
    pub roles: Vec<RoleId>,
    pub user: User,
}

impl MemberIntermediary {
    /// Inject a guild ID to create a [`Member`].
    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn into_member(self, guild_id: GuildId) -> Member {
        Member {
            avatar: self.avatar,
            communication_disabled_until: self.communication_disabled_until,
            deaf: self.deaf,
            guild_id,
            joined_at: self.joined_at,
            mute: self.mute,
            nick: self.nick,
            pending: self.pending,
            premium_since: self.premium_since,
            roles: self.roles,
            user: self.user,
        }
    }
}

/// The state of a member's guild timeout.
///
/// This wraps an [`Option<Timestamp>`], as no corresponding `GUILD_MEMBER_UPDATE` events are
/// sent by Discord when the timeout of members have expired. Therefore, simply relying on whether
/// the `communication_disabled_until` field has a value does not facilitate the use case of
/// knowing whether a specified member is actually timed out (especially when using cached data);
/// and provides convenience methods to determine whether a member is timed out or not.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MemberTimeoutState(pub Option<Timestamp>);

impl MemberTimeoutState {
    /// Returns whether a member is currently timed out.
    #[allow(clippy::cast_possible_wrap)] // casting of a unix timestamp should never wrap
    #[allow(clippy::missing_panics_doc)] // this function never panics, false positive
    pub fn timed_out(&self) -> bool {
        if self.inner().is_none() {
            return false;
        }

        let until = self.inner().unwrap().as_secs();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        now < until
    }

    /// Returns the inner raw timestamp. `Some` if the timeout is still active,
    /// otherwise `None`.
    pub fn until(&self) -> Option<&Timestamp> {
        if !self.timed_out() {
            return None;
        }

        self.inner()
    }

    /// Returns the inner raw timestamp.
    ///
    /// Unlike [`until`](MemberTimeoutState::until), the timestamp is returned
    /// without checking if it is not expired.
    pub const fn inner(&self) -> Option<&Timestamp> {
        self.0.as_ref()
    }
}

impl<'de> Deserialize<'de> for MemberTimeoutState {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_option(MemberTimeoutStateVisitor)
    }
}

impl Serialize for MemberTimeoutState {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.inner().is_none() {
            return serializer.serialize_none();
        }

        serializer.serialize_some(self.inner().unwrap())
    }
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
    pub const fn new(guild_id: GuildId) -> Self {
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
            avatar: member.avatar,
            communication_disabled_until: member.communication_disabled_until,
            deaf: member.deaf,
            guild_id: self.0,
            joined_at: member.joined_at,
            mute: member.mute,
            nick: member.nick,
            pending: member.pending,
            premium_since: member.premium_since,
            roles: member.roles,
            user: member.user,
        })
    }
}

pub(crate) struct MemberTimeoutStateVisitor;

impl<'de> Visitor<'de> for MemberTimeoutStateVisitor {
    type Value = MemberTimeoutState;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a member timeout state")
    }

    fn visit_none<E: DeError>(self) -> Result<Self::Value, E> {
        Ok(MemberTimeoutState(None))
    }

    fn visit_some<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        Ok(MemberTimeoutState(Some(Timestamp::deserialize(
            deserializer,
        )?)))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OptionalMemberDeserializer(GuildId);

impl OptionalMemberDeserializer {
    /// Create a new deserializer for a member when you know the ID but the
    /// payload probably doesn't contain it.
    pub const fn new(guild_id: GuildId) -> Self {
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
pub struct MemberListDeserializer(GuildId);

impl MemberListDeserializer {
    /// Create a new deserializer for a map of members when you know the
    /// Guild ID but the payload probably doesn't contain it.
    pub const fn new(guild_id: GuildId) -> Self {
        Self(guild_id)
    }
}

struct MemberListVisitor(GuildId);

impl<'de> Visitor<'de> for MemberListVisitor {
    type Value = Vec<Member>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a sequence of members")
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let mut list = seq.size_hint().map_or_else(Vec::new, Vec::with_capacity);

        while let Some(member) = seq.next_element_seed(MemberDeserializer(self.0))? {
            list.push(member);
        }

        Ok(list)
    }
}

impl<'de> DeserializeSeed<'de> for MemberListDeserializer {
    type Value = Vec<Member>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(MemberListVisitor(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::{Member, MemberTimeoutState};
    use crate::{
        datetime::{Timestamp, TimestampParseError},
        id::{GuildId, UserId},
        user::User,
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn test_member_deserializer() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2015-04-26T06:26:56.936000+00:00")?;
        let premium_since = Timestamp::from_str("2021-03-16T14:29:19.046000+00:00")?;

        let value = Member {
            avatar: Some("guild avatar".to_owned()),
            communication_disabled_until: MemberTimeoutState(None),
            deaf: false,
            guild_id: GuildId::new(1).expect("non zero"),
            joined_at,
            mute: true,
            nick: Some("twilight".to_owned()),
            pending: false,
            premium_since: Some(premium_since),
            roles: Vec::new(),
            user: User {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: UserId::new(3).expect("non zero"),
                locale: None,
                mfa_enabled: None,
                name: "twilight".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Member",
                    len: 11,
                },
                Token::Str("avatar"),
                Token::Some,
                Token::Str("guild avatar"),
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("joined_at"),
                Token::Str("2015-04-26T06:26:56.936000+00:00"),
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("nick"),
                Token::Some,
                Token::Str("twilight"),
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("premium_since"),
                Token::Some,
                Token::Str("2021-03-16T14:29:19.046000+00:00"),
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("twilight"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );

        Ok(())
    }

    #[test]
    fn test_guild_member_communication_disabled_until() -> Result<(), TimestampParseError> {
        let communication_disabled_until = Timestamp::from_str("2021-12-23T14:29:19.046000+00:00")?;
        let joined_at = Timestamp::from_str("2015-04-26T06:26:56.936000+00:00")?;
        let premium_since = Timestamp::from_str("2021-03-16T14:29:19.046000+00:00")?;

        let value = Member {
            avatar: Some("guild avatar".to_owned()),
            communication_disabled_until: MemberTimeoutState(Some(communication_disabled_until)),
            deaf: false,
            guild_id: GuildId::new(1).expect("non zero"),
            joined_at,
            mute: true,
            nick: Some("twilight".to_owned()),
            pending: false,
            premium_since: Some(premium_since),
            roles: Vec::new(),
            user: User {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: UserId::new(3).expect("non zero"),
                locale: None,
                mfa_enabled: None,
                name: "twilight".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Member",
                    len: 11,
                },
                Token::Str("avatar"),
                Token::Some,
                Token::Str("guild avatar"),
                Token::Str("communication_disabled_until"),
                Token::Some,
                Token::Str("2021-12-23T14:29:19.046000+00:00"),
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("joined_at"),
                Token::Str("2015-04-26T06:26:56.936000+00:00"),
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("nick"),
                Token::Some,
                Token::Str("twilight"),
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("premium_since"),
                Token::Some,
                Token::Str("2021-03-16T14:29:19.046000+00:00"),
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("twilight"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
