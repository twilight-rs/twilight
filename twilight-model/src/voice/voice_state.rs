use crate::{
    guild::Member,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
    util::Timestamp,
};
use serde::{Deserialize, Serialize};

/// User's voice connection status.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceState {
    /// Channel this user is connected to.
    ///
    /// [`None`] corresponds to being disconnected.
    pub channel_id: Option<Id<ChannelMarker>>,
    /// Whether this user is server deafened.
    pub deaf: bool,
    /// Guild this voice state is for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Member this voice state is for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    /// Whether this user is server muted.
    pub mute: bool,
    /// Whether this user is locally deafened.
    pub self_deaf: bool,
    /// Whether this user is locally muted.
    pub self_mute: bool,
    /// Whether this user is streaming using "Go Live".
    #[serde(default)]
    pub self_stream: bool,
    /// Whether this user's camera is enabled.
    pub self_video: bool,
    /// Session ID for this voice state.
    ///
    /// Used to establish a voice websocket connection.
    pub session_id: String,
    /// Whether the user's permission to speak is denied.
    ///
    /// Only applies to stage channels.
    pub suppress: bool,
    /// User this voice state is for.
    pub user_id: Id<UserMarker>,
    /// When the user requested to speak.
    ///
    /// # serde
    ///
    /// This is serialized as an ISO 8601 timestamp in the format of
    /// "2021-01-01T01-01-01.010000+00:00".
    pub request_to_speak_timestamp: Option<Timestamp>,
}

#[cfg(test)]
mod tests {
    use super::VoiceState;
    use crate::{
        guild::{Member, MemberFlags},
        id::Id,
        user::User,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn voice_state() {
        let value = VoiceState {
            channel_id: Some(Id::new(1)),
            deaf: false,
            guild_id: Some(Id::new(2)),
            member: None,
            mute: true,
            self_deaf: false,
            self_mute: true,
            self_stream: false,
            self_video: false,
            session_id: "a".to_owned(),
            suppress: true,
            user_id: Id::new(3),
            request_to_speak_timestamp: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "VoiceState",
                    len: 12,
                },
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("self_deaf"),
                Token::Bool(false),
                Token::Str("self_mute"),
                Token::Bool(true),
                Token::Str("self_stream"),
                Token::Bool(false),
                Token::Str("self_video"),
                Token::Bool(false),
                Token::Str("session_id"),
                Token::Str("a"),
                Token::Str("suppress"),
                Token::Bool(true),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("request_to_speak_timestamp"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn voice_state_complete() -> Result<(), TimestampParseError> {
        let joined_at = Some(Timestamp::from_str("2015-04-26T06:26:56.936000+00:00")?);
        let premium_since = Timestamp::from_str("2021-03-16T14:29:19.046000+00:00")?;
        let request_to_speak_timestamp = Timestamp::from_str("2021-04-21T22:16:50.000000+00:00")?;
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let value = VoiceState {
            channel_id: Some(Id::new(1)),
            deaf: false,
            guild_id: Some(Id::new(2)),
            member: Some(Member {
                avatar: None,
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
            }),
            mute: true,
            self_deaf: false,
            self_mute: true,
            self_stream: false,
            self_video: false,
            session_id: "a".to_owned(),
            suppress: true,
            user_id: Id::new(3),
            request_to_speak_timestamp: Some(request_to_speak_timestamp),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "VoiceState",
                    len: 13,
                },
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "Member",
                    len: 10,
                },
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
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("self_deaf"),
                Token::Bool(false),
                Token::Str("self_mute"),
                Token::Bool(true),
                Token::Str("self_stream"),
                Token::Bool(false),
                Token::Str("self_video"),
                Token::Bool(false),
                Token::Str("session_id"),
                Token::Str("a"),
                Token::Str("suppress"),
                Token::Bool(true),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("request_to_speak_timestamp"),
                Token::Some,
                Token::Str("2021-04-21T22:16:50.000000+00:00"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
