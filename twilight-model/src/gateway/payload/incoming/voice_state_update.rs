use crate::voice::VoiceState;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceStateUpdate(pub VoiceState);

impl Deref for VoiceStateUpdate {
    type Target = VoiceState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VoiceStateUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{VoiceState, VoiceStateUpdate};
    use crate::{
        guild::Member,
        id::Id,
        test::image_hash,
        user::User,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_voice_state_update() {
        let joined_at = Timestamp::from_secs(1_632_072_645).expect("non zero");

        let value = VoiceStateUpdate(VoiceState {
            channel_id: None,
            deaf: false,
            guild_id: Some(Id::new(1)),
            member: Some(Member {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                joined_at,
                mute: false,
                nick: None,
                pending: false,
                premium_since: None,
                roles: vec![Id::new(4)],
                user: User {
                    id: Id::new(1),
                    accent_color: None,
                    avatar: None,
                    banner: None,
                    bot: false,
                    discriminator: 909,
                    name: "foo".to_string(),
                    mfa_enabled: None,
                    locale: None,
                    verified: None,
                    email: None,
                    flags: None,
                    premium_type: None,
                    system: None,
                    public_flags: None,
                },
            }),
            mute: false,
            self_deaf: false,
            self_mute: false,
            self_stream: false,
            self_video: false,
            session_id: "a".to_owned(),
            suppress: false,
            user_id: Id::new(1),
            request_to_speak_timestamp: None,
        });

        serde_test::assert_tokens(
            &value,
            &[
                Token::NewtypeStruct {
                    name: "VoiceStateUpdate",
                },
                Token::Struct {
                    name: "VoiceState",
                    len: 13,
                },
                Token::Str("channel_id"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "Member",
                    len: 9,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("joined_at"),
                Token::Str("2021-09-19T17:30:45.000000+00:00"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::None,
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
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
                Token::Str("0909"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("username"),
                Token::Str("foo"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("self_deaf"),
                Token::Bool(false),
                Token::Str("self_mute"),
                Token::Bool(false),
                Token::Str("self_stream"),
                Token::Bool(false),
                Token::Str("self_video"),
                Token::Bool(false),
                Token::Str("session_id"),
                Token::Str("a"),
                Token::Str("suppress"),
                Token::Bool(false),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("request_to_speak_timestamp"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn voice_state_update_deser_tokens() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2016-12-08T18:41:21.954000+00:00")?;
        let request_to_speak_timestamp = Timestamp::from_str("2021-03-31T18:45:31.297561+00:00")?;

        let value = VoiceStateUpdate(VoiceState {
            channel_id: None,
            deaf: false,
            guild_id: Some(Id::new(999_999)),
            member: Some(Member {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                joined_at,
                mute: false,
                nick: Some("Twilight".to_string()),
                pending: false,
                premium_since: None,
                roles: vec![Id::new(123), Id::new(124)],
                user: User {
                    id: Id::new(1_234_123_123_123),
                    accent_color: None,
                    avatar: Some(image_hash::AVATAR),
                    banner: None,
                    bot: false,
                    discriminator: 4242,
                    name: "Twilight Sparkle".to_string(),
                    mfa_enabled: None,
                    locale: None,
                    verified: None,
                    email: None,
                    flags: None,
                    premium_type: None,
                    system: None,
                    public_flags: None,
                },
            }),
            mute: false,
            self_deaf: false,
            self_mute: false,
            self_stream: false,
            self_video: false,
            session_id: "asdasdas1da98da2b3ab3a".to_owned(),
            suppress: false,
            user_id: Id::new(123_213),
            request_to_speak_timestamp: Some(request_to_speak_timestamp),
        });

        // Token stream here's `Member` has no `guild_id`, which deserializer
        // must add.
        // Lack of "guild_id" in real "member" means that de+ser does not
        // reproduce original input (assert only `de`).
        serde_test::assert_de_tokens(
            &value,
            &[
                Token::NewtypeStruct {
                    name: "VoiceStateUpdate",
                },
                Token::Struct {
                    name: "VoiceState",
                    len: 12,
                },
                Token::Str("channel_id"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("999999"),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "Member",
                    len: 9,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("joined_at"),
                Token::Str("2016-12-08T18:41:21.954000+00:00"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("Twilight"),
                Token::Str("pending"),
                Token::Bool(false),
                Token::Str("roles"),
                Token::Seq { len: Some(2) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("124"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("4242"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1234123123123"),
                Token::Str("username"),
                Token::Str("Twilight Sparkle"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("self_deaf"),
                Token::Bool(false),
                Token::Str("self_mute"),
                Token::Bool(false),
                Token::Str("self_stream"),
                Token::Bool(false),
                Token::Str("self_video"),
                Token::Bool(false),
                Token::Str("session_id"),
                Token::Str("asdasdas1da98da2b3ab3a"),
                Token::Str("suppress"),
                Token::Bool(false),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123213"),
                Token::Str("request_to_speak_timestamp"),
                Token::Some,
                Token::Str("2021-03-31T18:45:31.297561+00:00"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
