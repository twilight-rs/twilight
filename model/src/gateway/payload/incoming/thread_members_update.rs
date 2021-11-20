use crate::{
    channel::thread::{ThreadMember, ThreadMemberIntermediary},
    id::{ChannelId, GuildId, UserId},
};
use serde::{
    de::{value::MapAccessDeserializer, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMembersUpdate {
    /// List of thread members.
    ///
    /// Includes the [`member`] key.
    ///
    /// [`member`]: ThreadMember::member
    #[serde(default)]
    pub added_members: Vec<ThreadMember>,
    pub guild_id: GuildId,
    pub id: ChannelId,
    /// Max value of 50.
    pub member_count: u8,
    #[serde(default)]
    pub removed_member_ids: Vec<UserId>,
}

impl<'de> Deserialize<'de> for ThreadMembersUpdate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ThreadMembersUpdateVisitor)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
struct ThreadMembersUpdateIntermediary {
    /// ThreadMembers without the guild ID.
    #[serde(default)]
    pub added_members: Vec<ThreadMemberIntermediary>,
    pub guild_id: GuildId,
    pub id: ChannelId,
    /// Max value of 50.
    pub member_count: u8,
    #[serde(default)]
    pub removed_member_ids: Vec<UserId>,
}

impl ThreadMembersUpdateIntermediary {
    fn into_thread_members_update(self) -> ThreadMembersUpdate {
        let guild_id = self.guild_id;
        let added_members = self
            .added_members
            .into_iter()
            .map(|tm| tm.into_thread_member(guild_id))
            .collect();

        ThreadMembersUpdate {
            added_members,
            guild_id,
            id: self.id,
            member_count: self.member_count,
            removed_member_ids: self.removed_member_ids,
        }
    }
}

struct ThreadMembersUpdateVisitor;

impl<'de> Visitor<'de> for ThreadMembersUpdateVisitor {
    type Value = ThreadMembersUpdate;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct ThreadMembersUpdate")
    }

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        let deser = MapAccessDeserializer::new(map);
        let update = ThreadMembersUpdateIntermediary::deserialize(deser)?;

        Ok(update.into_thread_members_update())
    }
}

#[cfg(test)]
mod tests {
    use super::ThreadMembersUpdate;
    use crate::{
        channel::thread::ThreadMember,
        datetime::Timestamp,
        gateway::presence::{
            Activity, ActivityEmoji, ActivityType, ClientStatus, Presence, Status, UserOrId,
        },
        guild::Member,
        id::{ChannelId, GuildId, UserId},
        user::User,
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_thread_members_update() {
        const JOIN_TIMESTAMP: &str = "2015-04-26T06:26:56.936000+00:00";
        const PREMIUM_SINCE: &str = "2021-03-16T14:29:19.046000+00:00";

        let joined_at = Timestamp::from_str(JOIN_TIMESTAMP).expect("timestamp error");
        let premium_since = Timestamp::from_str(PREMIUM_SINCE).expect("timestamp error");

        let member = Member {
            avatar: Some("guild avatar".to_owned()),
            deaf: false,
            guild_id: GuildId::new(2).expect("non zero"),
            joined_at: Some(joined_at),
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

        let activity = Activity {
            application_id: None,
            assets: None,
            buttons: Vec::new(),
            created_at: Some(1_571_048_061_237),
            details: None,
            flags: None,
            id: Some("aaaaaaaaaaaaaaaa".to_owned()),
            instance: None,
            kind: ActivityType::Custom,
            name: "foo".to_owned(),
            emoji: Some(ActivityEmoji {
                name: "Test".to_string(),
                id: None,
                animated: None,
            }),
            party: None,
            secrets: None,
            state: None,
            timestamps: None,
            url: None,
        };
        let presence = Presence {
            activities: vec![activity],
            client_status: ClientStatus {
                desktop: Some(Status::Online),
                mobile: None,
                web: None,
            },
            guild_id: GuildId::new(2).expect("non zero"),
            status: Status::Online,
            user: UserOrId::UserId {
                id: UserId::new(3).expect("non zero"),
            },
        };

        let join_timestamp = Timestamp::from_str(JOIN_TIMESTAMP).expect("timestamp error");

        let value = ThreadMembersUpdate {
            added_members: vec![ThreadMember {
                flags: 1,
                id: Some(ChannelId::new(123).expect("non zero")),
                join_timestamp,
                member: Some(member),
                presence: Some(presence),
                user_id: Some(UserId::new(3).expect("non zero")),
            }],
            guild_id: GuildId::new(2).expect("non zero"),
            id: ChannelId::new(4).expect("non zero"),
            member_count: 8,
            removed_member_ids: vec![],
        };

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ThreadMemberUpdate",
                    len: 6,
                },
                Token::Str("added_members"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "ThreadMemberIntermediary",
                    len: 6,
                },
                Token::Str("flags"),
                Token::U64(1),
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("123"),
                Token::Str("join_timestamp"),
                Token::Str(JOIN_TIMESTAMP),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "MemberIntermediary",
                    len: 10,
                },
                Token::Str("avatar"),
                Token::Some,
                Token::Str("guild avatar"),
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str(JOIN_TIMESTAMP),
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("nick"),
                Token::Some,
                Token::Str("twilight"),
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("premium_since"),
                Token::Some,
                Token::Str(PREMIUM_SINCE),
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
                Token::Str("presence"),
                Token::Some,
                Token::Struct {
                    name: "PresenceIntermediary",
                    len: 5,
                },
                Token::Str("activities"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Activity",
                    len: 5,
                },
                Token::Str("created_at"),
                Token::Some,
                Token::U64(1_571_048_061_237),
                Token::Str("emoji"),
                Token::Some,
                Token::Struct {
                    name: "ActivityEmoji",
                    len: 1,
                },
                Token::Str("name"),
                Token::Str("Test"),
                Token::StructEnd,
                Token::Str("id"),
                Token::Some,
                Token::Str("aaaaaaaaaaaaaaaa"),
                Token::Str("type"),
                Token::U8(4),
                Token::Str("name"),
                Token::Str("foo"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("client_status"),
                Token::Struct {
                    name: "ClientStatus",
                    len: 1,
                },
                Token::Str("desktop"),
                Token::Some,
                Token::Enum { name: "Status" },
                Token::Str("online"),
                Token::Unit,
                Token::StructEnd,
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("status"),
                Token::Enum { name: "Status" },
                Token::Str("online"),
                Token::Unit,
                Token::Str("user"),
                Token::Struct {
                    name: "UserOrId",
                    len: 1,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("user_id"),
                Token::Some,
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("4"),
                Token::Str("member_count"),
                Token::U8(8),
                Token::Str("removed_member_ids"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
