//! Gateway event payload when an invite is created.

use crate::{
    datetime::Timestamp,
    id::{
        marker::{ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
    invite::TargetType,
    user::{self, DiscriminatorDisplay, User},
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Serialize};

/// A new [`Invite`] has been created.
///
/// [`Invite`]: crate::invite::Invite
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteCreate {
    /// ID of the channel invited users will first see.
    pub channel_id: Id<ChannelMarker>,
    /// Unique code.
    pub code: String,
    /// When the invite was created.
    pub created_at: Timestamp,
    /// ID of the guild being invited to.
    pub guild_id: Id<GuildMarker>,
    /// Information about the user who created the invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter: Option<User>,
    /// Maximum age before the invite expires.
    ///
    /// This is in seconds.
    pub max_age: u64,
    /// Maximum number of uses before the invite expires.
    pub max_uses: u64,
    /// Target of the invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_type: Option<TargetType>,
    /// User whose stream to display for this voice channel stream invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user: Option<PartialUser>,
    /// Whether the invite is temporary.
    ///
    /// Invited users will be kicked when they are disconnected from an audio
    /// channel unless they're assigned a role.
    pub temporary: bool,
    /// Number of times the invite has been used.
    ///
    /// This will always be zero.
    pub uses: u8,
}

/// Information about the user whose stream to display for a voice channel
/// stream invite.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialUser {
    /// Hash of the user's avatar.
    pub avatar: Option<ImageHash>,
    /// Discriminator used to differentiate people with the same [`username`].
    ///
    /// [`username`]: Self::username
    ///
    /// # serde
    ///
    /// The discriminator field can be deserialized from either a string or an
    /// integer. The field will always serialize into a string due to that being
    /// the type Discord's API uses.
    #[serde(with = "user::discriminator")]
    pub discriminator: u16,
    /// ID of the user.
    pub id: Id<UserMarker>,
    /// Username of the user.
    pub username: String,
}

impl PartialUser {
    /// Create a [`Display`] formatter for a user discriminator.
    ///
    /// [`Display`]: core::fmt::Display
    pub const fn discriminator(&self) -> DiscriminatorDisplay {
        DiscriminatorDisplay::new(self.discriminator)
    }
}

#[cfg(test)]
mod tests {
    use super::{InviteCreate, PartialUser};
    use crate::{datetime::Timestamp, id::Id, test::image_hash};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        InviteCreate: channel_id,
        code,
        created_at,
        guild_id,
        inviter,
        max_age,
        max_uses,
        target_user_type,
        target_user,
        temporary,
        uses
    );
    assert_fields!(PartialUser: avatar, discriminator, id, username);
    assert_impl_all!(
        InviteCreate: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
    assert_impl_all!(
        PartialUser: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn test_invite_create() {
        let created_at = Timestamp::from_secs(1_609_459_200).expect("non zero");

        let value = InviteCreate {
            channel_id: Id::new(1),
            code: "a".repeat(7),
            created_at,
            guild_id: Id::new(2),
            inviter: None,
            max_age: 3600,
            max_uses: 5,
            target_user_type: None,
            target_user: None,
            temporary: false,
            uses: 0,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InviteCreate",
                    len: 8,
                },
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("code"),
                Token::Str("aaaaaaa"),
                Token::Str("created_at"),
                Token::Str("2021-01-01T00:00:00.000000+00:00"),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("max_age"),
                Token::U64(3600),
                Token::Str("max_uses"),
                Token::U64(5),
                Token::Str("temporary"),
                Token::Bool(false),
                Token::Str("uses"),
                Token::U8(0),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_partial_user() {
        let value = PartialUser {
            avatar: Some(image_hash::AVATAR),
            discriminator: 123,
            id: Id::new(1),
            username: "twilight".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PartialUser",
                    len: 4,
                },
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("discriminator"),
                Token::Str("0123"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("username"),
                Token::Str("twilight"),
                Token::StructEnd,
            ],
        );
    }
}
