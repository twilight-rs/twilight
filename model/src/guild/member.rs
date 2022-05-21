use crate::{
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
    user::User,
    util::{ImageHash, Timestamp},
};

use serde::{
    de::{
        value::MapAccessDeserializer, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor,
    },
    Deserialize, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Member {
    /// Member's guild avatar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageHash>,
    pub communication_disabled_until: Option<Timestamp>,
    pub deaf: bool,
    pub guild_id: Id<GuildMarker>,
    pub joined_at: Timestamp,
    pub mute: bool,
    pub nick: Option<String>,
    /// Whether the user has yet to pass the guild's [Membership Screening]
    /// requirements.
    pub pending: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<Timestamp>,
    pub roles: Vec<Id<RoleMarker>>,
    pub user: User,
}

/// Version of [`Member`] but without a guild ID, useful in some contexts.
///
/// The HTTP and Gateway APIs don't include guild IDs in their payloads, so this
/// can be useful when you're unable to use a deserialization seed like
/// [`MemberDeserializer`].
// Used in the guild deserializer.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename(deserialize = "Member"))]
pub struct MemberIntermediary {
    /// Member's guild avatar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageHash>,
    pub communication_disabled_until: Option<Timestamp>,
    pub deaf: bool,
    pub joined_at: Timestamp,
    pub mute: bool,
    pub nick: Option<String>,
    #[serde(default)]
    pub pending: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<Timestamp>,
    pub roles: Vec<Id<RoleMarker>>,
    pub user: User,
}

impl MemberIntermediary {
    /// Inject a guild ID to create a [`Member`].
    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn into_member(self, guild_id: Id<GuildMarker>) -> Member {
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

/// Deserialize a member when the payload doesn't have the guild ID but
/// you already know the guild ID.
///
/// Member payloads from the HTTP API, for example, don't have the guild
/// ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberDeserializer(Id<GuildMarker>);

impl MemberDeserializer {
    /// Create a new deserializer for a member when you know the ID but the
    /// payload probably doesn't contain it.
    pub const fn new(guild_id: Id<GuildMarker>) -> Self {
        Self(guild_id)
    }
}

impl<'de> DeserializeSeed<'de> for MemberDeserializer {
    type Value = Member;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_map(MemberVisitor(self.0))
    }
}

struct MemberVisitor(Id<GuildMarker>);

impl<'de> Visitor<'de> for MemberVisitor {
    type Value = Member;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a map of member fields")
    }

    fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
        let deser = MapAccessDeserializer::new(map);
        let member = MemberIntermediary::deserialize(deser)?;

        Ok(member.into_member(self.0))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberListDeserializer(Id<GuildMarker>);

impl MemberListDeserializer {
    /// Create a new deserializer for a map of members when you know the
    /// Guild ID but the payload probably doesn't contain it.
    pub const fn new(guild_id: Id<GuildMarker>) -> Self {
        Self(guild_id)
    }
}

impl<'de> DeserializeSeed<'de> for MemberListDeserializer {
    type Value = Vec<Member>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(MemberListVisitor(self.0))
    }
}

struct MemberListVisitor(Id<GuildMarker>);

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

#[cfg(test)]
mod tests {
    use super::Member;
    use crate::{
        id::Id,
        test::image_hash,
        user::User,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn test_member_deserializer() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2015-04-26T06:26:56.936000+00:00")?;
        let premium_since = Timestamp::from_str("2021-03-16T14:29:19.046000+00:00")?;

        let value = Member {
            avatar: Some(image_hash::AVATAR),
            communication_disabled_until: None,
            deaf: false,
            guild_id: Id::new(1),
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
                id: Id::new(3),
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
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
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
                Token::NewtypeStruct { name: "Id" },
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
            avatar: Some(image_hash::AVATAR),
            communication_disabled_until: Some(communication_disabled_until),
            deaf: false,
            guild_id: Id::new(1),
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
                id: Id::new(3),
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
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("communication_disabled_until"),
                Token::Some,
                Token::Str("2021-12-23T14:29:19.046000+00:00"),
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
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
                Token::NewtypeStruct { name: "Id" },
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
