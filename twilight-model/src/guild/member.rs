//! Mostly internal custom serde deserializers.

use super::MemberFlags;
use crate::{
    id::{Id, marker::RoleMarker},
    user::User,
    util::{ImageHash, Timestamp},
};

use serde::{Deserialize, Serialize};

/// [`User`] that is in a [`Guild`].
///
/// [`Guild`]: super::Guild
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Member {
    /// Member's guild avatar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageHash>,
    pub communication_disabled_until: Option<Timestamp>,
    pub deaf: bool,
    /// Flags for the member.
    ///
    /// Defaults to an empty bitfield.
    pub flags: MemberFlags,
    pub joined_at: Option<Timestamp>,
    pub mute: bool,
    pub nick: Option<String>,
    /// Whether the user has yet to pass the guild's [Membership Screening]
    /// requirements.
    #[serde(default)]
    pub pending: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<Timestamp>,
    pub roles: Vec<Id<RoleMarker>>,
    pub user: User,
}

#[cfg(test)]
mod tests {
    use super::Member;
    use crate::{
        guild::MemberFlags,
        id::Id,
        test::image_hash,
        user::User,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn member_deserializer() -> Result<(), TimestampParseError> {
        let joined_at = Some(Timestamp::from_str("2015-04-26T06:26:56.936000+00:00")?);
        let premium_since = Timestamp::from_str("2021-03-16T14:29:19.046000+00:00")?;
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let value = Member {
            avatar: Some(image_hash::AVATAR),
            communication_disabled_until: None,
            deaf: false,
            flags,
            joined_at,
            mute: true,
            nick: Some("twilight".to_owned()),
            pending: false,
            premium_since: Some(premium_since),
            roles: Vec::new(),
            user: User {
                accent_color: None,
                avatar: None,
                avatar_decoration: None,
                avatar_decoration_data: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                global_name: Some("test".to_owned()),
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
                Token::Str("flags"),
                Token::U64(flags.bits()),
                Token::Str("joined_at"),
                Token::Some,
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
                    len: 10,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::None,
                Token::Str("avatar_decoration"),
                Token::None,
                Token::Str("avatar_decoration_data"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("global_name"),
                Token::Some,
                Token::Str("test"),
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
    #[allow(clippy::too_many_lines)]
    fn guild_member_communication_disabled_until() -> Result<(), TimestampParseError> {
        let communication_disabled_until = Timestamp::from_str("2021-12-23T14:29:19.046000+00:00")?;
        let joined_at = Some(Timestamp::from_str("2015-04-26T06:26:56.936000+00:00")?);
        let premium_since = Timestamp::from_str("2021-03-16T14:29:19.046000+00:00")?;
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let value = Member {
            avatar: Some(image_hash::AVATAR),
            communication_disabled_until: Some(communication_disabled_until),
            deaf: false,
            flags,
            joined_at,
            mute: true,
            nick: Some("twilight".to_owned()),
            pending: false,
            premium_since: Some(premium_since),
            roles: Vec::new(),
            user: User {
                accent_color: None,
                avatar: None,
                avatar_decoration: None,
                avatar_decoration_data: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                global_name: Some("test".to_owned()),
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
                Token::Str("flags"),
                Token::U64(flags.bits()),
                Token::Str("joined_at"),
                Token::Some,
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
                    len: 10,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::None,
                Token::Str("avatar_decoration"),
                Token::None,
                Token::Str("avatar_decoration_data"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("global_name"),
                Token::Some,
                Token::Str("test"),
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
