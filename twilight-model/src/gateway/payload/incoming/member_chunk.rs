use crate::{
    gateway::presence::{Presence, PresenceListDeserializer},
    guild::member::{Member, MemberListDeserializer},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};
use serde::{
    de::{Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MemberChunk {
    pub chunk_count: u32,
    pub chunk_index: u32,
    pub guild_id: Id<GuildMarker>,
    pub members: Vec<Member>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    pub not_found: Vec<Id<UserMarker>>,
    #[serde(default)]
    pub presences: Vec<Presence>,
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum Field {
    ChunkCount,
    ChunkIndex,
    GuildId,
    Members,
    Nonce,
    NotFound,
    Presences,
}

struct MemberChunkVisitor;

impl<'de> Visitor<'de> for MemberChunkVisitor {
    type Value = MemberChunk;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct MemberChunk")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut chunk_count = None;
        let mut chunk_index = None;
        let mut guild_id = None;
        let mut members = None;
        let mut nonce = None;
        let mut not_found = None;
        let mut presences = None;

        let span = tracing::trace_span!("deserializing member chunk");
        let _span_enter = span.enter();

        loop {
            let span_child = tracing::trace_span!("iterating over element");
            let _span_child_enter = span_child.enter();

            let key = match map.next_key() {
                Ok(Some(key)) => {
                    tracing::trace!(?key, "found key");

                    key
                }
                Ok(None) => break,
                Err(why) => {
                    // Encountered when we run into an unknown key.
                    map.next_value::<IgnoredAny>()?;

                    tracing::trace!("ran into an unknown key: {why:?}");

                    continue;
                }
            };

            match key {
                Field::ChunkCount => {
                    if chunk_count.is_some() {
                        return Err(DeError::duplicate_field("chunk_count"));
                    }

                    chunk_count = Some(map.next_value()?);
                }
                Field::ChunkIndex => {
                    if chunk_index.is_some() {
                        return Err(DeError::duplicate_field("chunk_index"));
                    }

                    chunk_index = Some(map.next_value()?);
                }
                Field::GuildId => {
                    if guild_id.is_some() {
                        return Err(DeError::duplicate_field("guild_id"));
                    }

                    guild_id = Some(map.next_value()?);
                }
                Field::Members => {
                    if members.is_some() {
                        return Err(DeError::duplicate_field("members"));
                    }

                    // Since the guild ID may not be deserialized yet we'll use
                    // a temporary placeholder value and update it with the real
                    // guild ID after all the fields have been deserialized.
                    let deserializer = MemberListDeserializer::new(Id::new(1));

                    members = Some(map.next_value_seed(deserializer)?);
                }
                Field::Nonce => {
                    if nonce.is_some() {
                        return Err(DeError::duplicate_field("nonce"));
                    }

                    nonce = Some(map.next_value()?);
                }
                Field::NotFound => {
                    if not_found.is_some() {
                        return Err(DeError::duplicate_field("not_found"));
                    }

                    not_found = Some(map.next_value()?);
                }
                Field::Presences => {
                    if presences.is_some() {
                        return Err(DeError::duplicate_field("presences"));
                    }

                    let deserializer = PresenceListDeserializer::new(Id::new(1));

                    presences = Some(map.next_value_seed(deserializer)?);
                }
            }
        }

        let chunk_count = chunk_count.ok_or_else(|| DeError::missing_field("chunk_count"))?;
        let chunk_index = chunk_index.ok_or_else(|| DeError::missing_field("chunk_index"))?;
        let guild_id = guild_id.ok_or_else(|| DeError::missing_field("guild_id"))?;
        let mut members = members.ok_or_else(|| DeError::missing_field("members"))?;
        let not_found = not_found.unwrap_or_default();
        let mut presences = presences.unwrap_or_default();

        tracing::trace!(
            %chunk_count,
            %chunk_index,
            ?guild_id,
            ?members,
            ?not_found,
            ?presences,
        );

        for member in &mut members {
            member.guild_id = guild_id;
        }

        for presence in &mut presences {
            presence.guild_id = guild_id;
        }

        Ok(MemberChunk {
            chunk_count,
            chunk_index,
            guild_id,
            members,
            nonce,
            not_found,
            presences,
        })
    }
}

impl<'de> Deserialize<'de> for MemberChunk {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        const FIELDS: &[&str] = &[
            "chunk_count",
            "chunk_index",
            "guild_id",
            "members",
            "nonce",
            "not_found",
            "presences",
        ];

        deserializer.deserialize_struct("MemberChunk", FIELDS, MemberChunkVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::MemberChunk;
    use crate::{
        gateway::presence::{ClientStatus, Presence, Status, UserOrId},
        guild::Member,
        id::Id,
        test::image_hash,
        user::{User, UserFlags},
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn simple_member_chunk() -> Result<(), TimestampParseError> {
        const JOINED_AT_INPUT: &str = "2020-04-04T04:04:04.000000+00:00";

        let joined_at = Timestamp::from_str(JOINED_AT_INPUT)?;

        let value = MemberChunk {
            chunk_count: 1,
            chunk_index: 0,
            guild_id: Id::new(1),
            members: Vec::from([
                Member {
                    avatar: None,
                    communication_disabled_until: None,
                    deaf: false,
                    guild_id: Id::new(1),
                    joined_at,
                    mute: false,
                    nick: Some("chunk".to_owned()),
                    pending: false,
                    premium_since: None,
                    roles: Vec::from([Id::new(6), Id::new(7)]),
                    user: User {
                        id: Id::new(2),
                        accent_color: None,
                        avatar: Some(image_hash::AVATAR),
                        banner: None,
                        bot: true,
                        discriminator: 1,
                        name: "test".to_owned(),
                        mfa_enabled: None,
                        locale: None,
                        verified: None,
                        email: None,
                        flags: None,
                        premium_type: None,
                        system: None,
                        public_flags: None,
                    },
                },
                Member {
                    avatar: None,
                    communication_disabled_until: None,
                    deaf: false,
                    guild_id: Id::new(1),
                    joined_at,
                    mute: false,
                    nick: Some("chunk".to_owned()),
                    pending: false,
                    premium_since: None,
                    roles: Vec::from([Id::new(6)]),
                    user: User {
                        id: Id::new(3),
                        accent_color: None,
                        avatar: Some(image_hash::AVATAR),
                        banner: None,
                        bot: true,
                        discriminator: 1,
                        name: "test".to_owned(),
                        mfa_enabled: None,
                        locale: None,
                        verified: None,
                        email: None,
                        flags: None,
                        premium_type: None,
                        system: None,
                        public_flags: None,
                    },
                },
                Member {
                    avatar: None,
                    communication_disabled_until: None,
                    deaf: false,
                    guild_id: Id::new(1),
                    joined_at,
                    mute: false,
                    nick: Some("chunk".to_owned()),
                    pending: true,
                    premium_since: None,
                    roles: Vec::from([Id::new(6)]),
                    user: User {
                        id: Id::new(5),
                        accent_color: None,
                        avatar: Some(image_hash::AVATAR),
                        banner: None,
                        bot: false,
                        discriminator: 1,
                        name: "test".to_owned(),
                        mfa_enabled: None,
                        locale: None,
                        verified: None,
                        email: None,
                        flags: None,
                        premium_type: None,
                        system: None,
                        public_flags: Some(UserFlags::VERIFIED_DEVELOPER),
                    },
                },
                Member {
                    avatar: None,
                    communication_disabled_until: None,
                    deaf: false,
                    guild_id: Id::new(1),
                    joined_at,
                    mute: false,
                    nick: Some("chunk".to_owned()),
                    pending: false,
                    premium_since: None,
                    roles: Vec::from([Id::new(6)]),
                    user: User {
                        id: Id::new(6),
                        accent_color: None,
                        avatar: Some(image_hash::AVATAR),
                        banner: None,
                        bot: false,
                        discriminator: 1,
                        name: "test".to_owned(),
                        mfa_enabled: None,
                        locale: None,
                        verified: None,
                        email: None,
                        flags: None,
                        premium_type: None,
                        system: None,
                        public_flags: None,
                    },
                },
            ]),
            nonce: None,
            not_found: Vec::new(),
            presences: Vec::from([
                Presence {
                    activities: Vec::new(),
                    client_status: ClientStatus {
                        desktop: None,
                        mobile: None,
                        web: Some(Status::Online),
                    },
                    guild_id: Id::new(1),
                    status: Status::Online,
                    user: UserOrId::UserId { id: Id::new(2) },
                },
                Presence {
                    activities: Vec::new(),
                    client_status: ClientStatus {
                        desktop: None,
                        mobile: None,
                        web: Some(Status::Online),
                    },
                    guild_id: Id::new(1),
                    status: Status::Online,
                    user: UserOrId::UserId { id: Id::new(3) },
                },
                Presence {
                    activities: Vec::new(),
                    client_status: ClientStatus {
                        desktop: Some(Status::DoNotDisturb),
                        mobile: None,
                        web: None,
                    },
                    guild_id: Id::new(1),
                    status: Status::DoNotDisturb,
                    user: UserOrId::UserId { id: Id::new(5) },
                },
            ]),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MemberChunk",
                    len: 6,
                },
                Token::Str("chunk_count"),
                Token::U32(1),
                Token::Str("chunk_index"),
                Token::U32(0),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("members"),
                Token::Seq { len: Some(4) },
                Token::Struct {
                    name: "Member",
                    len: 9,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("joined_at"),
                Token::Str(JOINED_AT_INPUT),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("chunk"),
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("roles"),
                Token::Seq { len: Some(2) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("7"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(true),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Struct {
                    name: "Member",
                    len: 9,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("joined_at"),
                Token::Str(JOINED_AT_INPUT),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("chunk"),
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(true),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Struct {
                    name: "Member",
                    len: 9,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("joined_at"),
                Token::Str(JOINED_AT_INPUT),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("chunk"),
                Token::Str("pending"),
                Token::Bool(true),
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 8,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::Str("username"),
                Token::Str("test"),
                Token::Str("public_flags"),
                Token::Some,
                Token::U64(131_072),
                Token::StructEnd,
                Token::StructEnd,
                Token::Struct {
                    name: "Member",
                    len: 9,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("joined_at"),
                Token::Str(JOINED_AT_INPUT),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("chunk"),
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("not_found"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("presences"),
                Token::Seq { len: Some(3) },
                Token::Struct {
                    name: "Presence",
                    len: 5,
                },
                Token::Str("activities"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("client_status"),
                Token::Struct {
                    name: "ClientStatus",
                    len: 1,
                },
                Token::Str("web"),
                Token::Some,
                Token::UnitVariant {
                    name: "Status",
                    variant: "online",
                },
                Token::StructEnd,
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("status"),
                Token::UnitVariant {
                    name: "Status",
                    variant: "online",
                },
                Token::Str("user"),
                Token::Struct {
                    name: "UserOrId",
                    len: 1,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Struct {
                    name: "Presence",
                    len: 5,
                },
                Token::Str("activities"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("client_status"),
                Token::Struct {
                    name: "ClientStatus",
                    len: 1,
                },
                Token::Str("web"),
                Token::Some,
                Token::UnitVariant {
                    name: "Status",
                    variant: "online",
                },
                Token::StructEnd,
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("status"),
                Token::UnitVariant {
                    name: "Status",
                    variant: "online",
                },
                Token::Str("user"),
                Token::Struct {
                    name: "UserOrId",
                    len: 1,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Struct {
                    name: "Presence",
                    len: 5,
                },
                Token::Str("activities"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("client_status"),
                Token::Struct {
                    name: "ClientStatus",
                    len: 1,
                },
                Token::Str("desktop"),
                Token::Some,
                Token::UnitVariant {
                    name: "Status",
                    variant: "dnd",
                },
                Token::StructEnd,
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("status"),
                Token::UnitVariant {
                    name: "Status",
                    variant: "dnd",
                },
                Token::Str("user"),
                Token::Struct {
                    name: "UserOrId",
                    len: 1,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::StructEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
