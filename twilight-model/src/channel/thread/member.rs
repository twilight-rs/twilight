use crate::{
    gateway::presence::{Presence, PresenceIntermediary},
    guild::Member,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
    util::Timestamp,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMember {
    // Values currently unknown and undocumented.
    pub flags: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<ChannelMarker>>,
    pub join_timestamp: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<Presence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Id<UserMarker>>,
}

/// Version of [`ThreadMember`], but without a guild ID in the
/// [`Self::member`] field.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub(crate) struct ThreadMemberIntermediary {
    // Values currently unknown and undocumented.
    pub flags: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<ChannelMarker>>,
    pub join_timestamp: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<PresenceIntermediary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Id<UserMarker>>,
}

impl ThreadMemberIntermediary {
    /// Inject a guild ID into a thread member intermediary
    pub fn into_thread_member(self, guild_id: Id<GuildMarker>) -> ThreadMember {
        let presence = self.presence.map(|p| p.into_presence(guild_id));
        ThreadMember {
            flags: self.flags,
            id: self.id,
            join_timestamp: self.join_timestamp,
            member: self.member,
            presence,
            user_id: self.user_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ThreadMember;
    use crate::{
        id::Id,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn thread_member() -> Result<(), TimestampParseError> {
        const DATETIME: &str = "2021-09-19T14:17:32.000000+00:00";

        let join_timestamp = Timestamp::from_str(DATETIME)?;

        let value = ThreadMember {
            flags: 3,
            id: Some(Id::new(1)),
            member: None,
            presence: None,
            join_timestamp,
            user_id: Some(Id::new(2)),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ThreadMember",
                    len: 4,
                },
                Token::Str("flags"),
                Token::U64(3),
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("join_timestamp"),
                Token::Str(DATETIME),
                Token::Str("user_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
