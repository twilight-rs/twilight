pub mod activity_button;

mod activity;
mod activity_assets;
mod activity_emoji;
mod activity_flags;
mod activity_party;
mod activity_secrets;
mod activity_timestamps;
mod activity_type;
mod client_status;
mod minimal_activity;
mod status;

pub use self::{
    activity::Activity, activity_assets::ActivityAssets, activity_button::ActivityButton,
    activity_emoji::ActivityEmoji, activity_flags::ActivityFlags, activity_party::ActivityParty,
    activity_secrets::ActivitySecrets, activity_timestamps::ActivityTimestamps,
    activity_type::ActivityType, client_status::ClientStatus, minimal_activity::MinimalActivity,
    status::Status,
};

use crate::{
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Presence {
    #[serde(default)]
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
    pub guild_id: Option<Id<GuildMarker>>,
    pub status: Status,
    pub user: UserOrId,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum UserOrId {
    User(User),
    UserId { id: Id<UserMarker> },
}

impl UserOrId {
    /// ID of the inner object.
    pub const fn id(&self) -> Id<UserMarker> {
        match self {
            UserOrId::User(u) => u.id,
            UserOrId::UserId { id } => *id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Activity, ActivityEmoji, ActivityType, ClientStatus, Presence, Status, UserOrId};
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn custom() {
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
        let value = Presence {
            activities: vec![activity],
            client_status: ClientStatus {
                desktop: Some(Status::Online),
                mobile: None,
                web: None,
            },
            guild_id: Some(Id::new(2)),
            status: Status::Online,
            user: UserOrId::UserId { id: Id::new(1) },
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
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
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
}
