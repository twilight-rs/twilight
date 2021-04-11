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
    de::{
        value::MapAccessDeserializer, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor,
    },
    Deserialize, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Presence {
    #[serde(default)]
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
    pub guild_id: GuildId,
    pub status: Status,
    pub user: UserOrId,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum UserOrId {
    User(User),
    UserId { id: UserId },
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct PresenceIntermediary {
    #[serde(default)]
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
    pub guild_id: Option<GuildId>,
    pub nick: Option<String>,
    pub status: Status,
    pub user: UserOrId,
}

struct PresenceVisitor(GuildId);

impl<'de> Visitor<'de> for PresenceVisitor {
    type Value = Presence;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("Presence struct")
    }

    fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
        let deser = MapAccessDeserializer::new(map);
        let presence = PresenceIntermediary::deserialize(deser)?;

        Ok(Presence {
            activities: presence.activities,
            client_status: presence.client_status,
            guild_id: presence.guild_id.unwrap_or(self.0),
            status: presence.status,
            user: presence.user,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PresenceDeserializer(GuildId);

impl PresenceDeserializer {
    /// Create a new deserializer for a presence when you know the guild ID but
    /// the payload probably doesn't contain it.
    pub fn new(guild_id: GuildId) -> Self {
        Self(guild_id)
    }
}

impl<'de> DeserializeSeed<'de> for PresenceDeserializer {
    type Value = Presence;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_map(PresenceVisitor(self.0))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PresenceListDeserializer(GuildId);

impl PresenceListDeserializer {
    /// Create a new deserializer for a map of presences when you know the
    /// Guild ID but the payload probably doesn't contain it.
    pub fn new(guild_id: GuildId) -> Self {
        Self(guild_id)
    }
}

struct PresenceListDeserializerVisitor(GuildId);

impl<'de> Visitor<'de> for PresenceListDeserializerVisitor {
    type Value = Vec<Presence>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a sequence of presences")
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let mut list = seq.size_hint().map_or_else(Vec::new, Vec::with_capacity);

        while let Some(presence) = seq.next_element_seed(PresenceDeserializer(self.0))? {
            list.push(presence);
        }

        Ok(list)
    }
}

impl<'de> DeserializeSeed<'de> for PresenceListDeserializer {
    type Value = Vec<Presence>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_any(PresenceListDeserializerVisitor(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Activity, ActivityEmoji, ActivityType, ClientStatus, Presence, PresenceListDeserializer,
        Status, UserOrId,
    };
    use crate::id::{GuildId, UserId};
    use serde::de::DeserializeSeed;
    use serde_json::Deserializer;
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
            activities: vec![activity],
            client_status: ClientStatus {
                desktop: Some(Status::Online),
                mobile: None,
                web: None,
            },
            guild_id: GuildId(2),
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
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("status"),
                Token::Enum { name: "Status" },
                Token::Str("online"),
                Token::Unit,
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

    // Test that presences through the deserializer are given a default guild ID
    // if they have none.
    //
    // Can't test seeded deserializers with serde_test.
    #[test]
    fn test_presence_map_guild_id_default() {
        let input = r#"[{
            "user": {
                "id": "1"
            },
            "status": "online",
            "client_status": {
                "desktop": "online"
            },
            "activities": []
        }]"#;

        let expected = Vec::from([Presence {
            activities: vec![],
            client_status: ClientStatus {
                desktop: Some(Status::Online),
                mobile: None,
                web: None,
            },
            guild_id: GuildId(2),
            status: Status::Online,
            user: UserOrId::UserId { id: UserId(1) },
        }]);

        let mut json_deserializer = Deserializer::from_str(input);
        let deserializer = PresenceListDeserializer::new(GuildId(2));
        let actual = deserializer.deserialize(&mut json_deserializer).unwrap();

        assert_eq!(actual, expected);
    }
}
