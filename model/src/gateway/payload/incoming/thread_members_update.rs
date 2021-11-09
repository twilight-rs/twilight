use std::fmt::{Formatter, Result as FmtResult};

use crate::{
    channel::thread::{ThreadMember, ThreadMemberIntermediary},
    id::{ChannelId, GuildId, UserId},
};
use serde::{
    de::{value::MapAccessDeserializer, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

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
