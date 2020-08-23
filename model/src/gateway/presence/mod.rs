mod activity;
mod activity_assets;
mod activity_emoji;
mod activity_flags;
mod activity_party;
mod activity_secrets;
mod activity_timestamps;
mod activity_type;
mod client_status;
mod status;

pub use self::{
    activity::Activity, activity_assets::ActivityAssets, activity_emoji::ActivityEmoji,
    activity_flags::ActivityFlags, activity_party::ActivityParty,
    activity_secrets::ActivitySecrets, activity_timestamps::ActivityTimestamps,
    activity_type::ActivityType, client_status::ClientStatus, status::Status,
};

use crate::{
    id::{GuildId, UserId},
    user::User,
};
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
pub struct Presence {
    #[serde(default)]
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
    pub game: Option<Activity>,
    pub guild_id: Option<GuildId>,
    pub nick: Option<String>,
    pub status: Status,
    pub user: UserOrId,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum UserOrId {
    User(User),
    UserId { id: UserId },
}

impl Key<'_, UserId> for Presence {
    fn key(&self) -> UserId {
        match self.user {
            UserOrId::User(ref u) => u.id,
            UserOrId::UserId { id } => id,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PresenceMapDeserializer(GuildId);

impl PresenceMapDeserializer {
    /// Create a new deserializer for a map of presences when you know the
    /// Guild ID but the payload probably doesn't contain it.
    pub fn new(guild_id: GuildId) -> Self {
        Self(guild_id)
    }
}

struct PresenceMapDeserializerVisitor(GuildId);

impl<'de> Visitor<'de> for PresenceMapDeserializerVisitor {
    type Value = HashMap<UserId, Presence>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a sequence of presences")
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let mut map = seq
            .size_hint()
            .map_or_else(HashMap::new, HashMap::with_capacity);

        while let Some(presence) = seq.next_element::<Presence>()? {
            let user_id = match presence.user {
                UserOrId::User(ref user) => user.id,
                UserOrId::UserId { id } => id,
            };

            map.insert(user_id, presence);
        }

        Ok(map)
    }
}

impl<'de> DeserializeSeed<'de> for PresenceMapDeserializer {
    type Value = HashMap<UserId, Presence>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(PresenceMapDeserializerVisitor(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::{Activity, ActivityEmoji, ActivityType, ClientStatus, Presence, Status, UserOrId};
    use crate::id::UserId;
    use serde_test::Token;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_custom() {
        let activity = Activity {
            application_id: None,
            assets: None,
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
        let value = Presence {
            activities: vec![activity.clone()],
            client_status: ClientStatus {
                desktop: Some(Status::Online),
                mobile: None,
                web: None,
            },
            game: Some(activity),
            guild_id: None,
            nick: None,
            status: Status::Online,
            user: UserOrId::UserId { id: UserId(1) },
        };

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Presence",
                    len: 4,
                },
                Token::Str("user"),
                Token::Struct {
                    name: "UserOrId",
                    len: 1,
                },
                Token::Str("id"),
                Token::Str("1"),
                Token::StructEnd,
                Token::Str("status"),
                Token::Enum { name: "Status" },
                Token::Str("online"),
                Token::Unit,
                Token::Str("game"),
                Token::Some,
                Token::Struct {
                    name: "Activity",
                    len: 4,
                },
                Token::Str("type"),
                Token::U8(4),
                Token::Str("name"),
                Token::Str("foo"),
                Token::Str("id"),
                Token::Some,
                Token::Str("aaaaaaaaaaaaaaaa"),
                Token::Str("emoji"),
                Token::Some,
                Token::Struct {
                    name: "ActivityEmoji",
                    len: 3,
                },
                Token::Str("name"),
                Token::Str("Test"),
                Token::Str("id"),
                Token::None,
                Token::Str("animated"),
                Token::None,
                Token::StructEnd,
                Token::Str("created_at"),
                Token::Some,
                Token::U64(1_571_048_061_237),
                Token::StructEnd,
                Token::Str("client_status"),
                Token::Struct {
                    name: "ClientStatus",
                    len: 3,
                },
                Token::Str("desktop"),
                Token::Some,
                Token::Enum { name: "Status" },
                Token::Str("online"),
                Token::Unit,
                Token::Str("mobile"),
                Token::None,
                Token::Str("web"),
                Token::None,
                Token::StructEnd,
                Token::Str("activities"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Activity",
                    len: 4,
                },
                Token::Str("type"),
                Token::U8(4),
                Token::Str("name"),
                Token::Str("foo"),
                Token::Str("emoji"),
                Token::Some,
                Token::Struct {
                    name: "ActivityEmoji",
                    len: 3,
                },
                Token::Str("name"),
                Token::Str("Test"),
                Token::Str("id"),
                Token::None,
                Token::Str("animated"),
                Token::None,
                Token::StructEnd,
                Token::Str("id"),
                Token::Some,
                Token::Str("aaaaaaaaaaaaaaaa"),
                Token::Str("created_at"),
                Token::Some,
                Token::U64(1_571_048_061_237),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
