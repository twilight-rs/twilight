mod settings;

pub use self::settings::GuildWidgetSettings;

use crate::{
    gateway::presence::Status,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
    user::{discriminator, DiscriminatorDisplay},
    util::ImageHash,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildWidget {
    pub channels: Vec<GuildWidgetChannel>,
    pub id: Id<GuildMarker>,
    pub instant_invite: Option<String>,
    pub members: Vec<GuildWidgetMember>,
    pub name: String,
    pub presence_count: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildWidgetChannel {
    pub id: Id<ChannelMarker>,
    pub name: String,
    pub position: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildWidgetMember {
    pub avatar: Option<ImageHash>,
    pub avatar_url: Option<String>,
    #[serde(with = "discriminator")]
    pub discriminator: u16,
    /// Member's ID.
    ///
    /// This is a string because this is often anonymized to zero, and our
    /// current ID implementation does not support this.
    pub id: String,
    #[serde(rename = "username")]
    pub name: String,
    pub status: Status,
}

impl GuildWidgetMember {
    /// Create a [`Display`] formatter for a user discriminator.
    ///
    /// [`Display`]: core::fmt::Display
    pub const fn discriminator(&self) -> DiscriminatorDisplay {
        DiscriminatorDisplay::new(self.discriminator)
    }
}

#[cfg(test)]
mod tests {
    use super::{GuildWidget, GuildWidgetChannel, GuildWidgetMember, Id, Status};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        GuildWidget: channels,
        id,
        instant_invite,
        members,
        name,
        presence_count
    );
    assert_fields!(GuildWidgetChannel: id, name, position);
    assert_fields!(
        GuildWidgetMember: avatar,
        avatar_url,
        discriminator,
        id,
        name,
        status
    );
    assert_impl_all!(
        GuildWidget: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );
    assert_impl_all!(
        GuildWidgetChannel: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );
    assert_impl_all!(
        GuildWidgetMember: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );

    #[test]
    fn guild_widget() {
        let value = GuildWidget {
            channels: Vec::from([GuildWidgetChannel {
                id: Id::new(2),
                name: "General".to_string(),
                position: 1,
            }]),
            id: Id::new(1),
            instant_invite: Some("https://discord.com/invite/P8PkgN2".to_string()),
            members: Vec::from([GuildWidgetMember {
                avatar: None,
                avatar_url: Some("widget avatar link".to_string()),
                discriminator: 1,
                id: "3".to_string(),
                name: "Foo".to_string(),
                status: Status::Online,
            }]),
            name: "Twilight".to_string(),
            presence_count: 15,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildWidget",
                    len: 6,
                },
                Token::Str("channels"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "GuildWidgetChannel",
                    len: 3,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("name"),
                Token::Str("General"),
                Token::Str("position"),
                Token::I64(1),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("instant_invite"),
                Token::Some,
                Token::Str("https://discord.com/invite/P8PkgN2"),
                Token::Str("members"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "GuildWidgetMember",
                    len: 6,
                },
                Token::Str("avatar"),
                Token::None,
                Token::Str("avatar_url"),
                Token::Some,
                Token::Str("widget avatar link"),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("Foo"),
                Token::Str("status"),
                Token::UnitVariant {
                    name: "Status",
                    variant: "online",
                },
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("name"),
                Token::Str("Twilight"),
                Token::Str("presence_count"),
                Token::U64(15),
                Token::StructEnd,
            ],
        );
    }
}
