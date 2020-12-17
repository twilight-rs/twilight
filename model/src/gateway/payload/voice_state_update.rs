use crate::voice::VoiceState;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceStateUpdate(pub VoiceState);

#[cfg(test)]
mod tests {
    use super::{VoiceState, VoiceStateUpdate};
    use crate::id::{ChannelId, GuildId, UserId};
    use serde_test::Token;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_voice_state_update() {
        let value = VoiceStateUpdate(VoiceState {
            channel_id: Some(ChannelId(1)),
            deaf: false,
            guild_id: Some(GuildId(1)),
            member: None,
            mute: false,
            self_deaf: false,
            self_mute: false,
            self_stream: false,
            session_id: "a".to_owned(),
            suppress: false,
            user_id: UserId(1),
        });

        serde_test::assert_tokens(
            &value,
            &[
                Token::NewtypeStruct {
                    name: "VoiceStateUpdate",
                },
                Token::Struct {
                    name: "VoiceState",
                    len: 10,
                },
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("self_deaf"),
                Token::Bool(false),
                Token::Str("self_mute"),
                Token::Bool(false),
                Token::Str("self_stream"),
                Token::Bool(false),
                Token::Str("session_id"),
                Token::Str("a"),
                Token::Str("suppress"),
                Token::Bool(false),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("1"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn voice_state_update_deser_tokens() {
        let value = VoiceStateUpdate(VoiceState {
            channel_id: None,
            deaf: false,
            guild_id: Some(GuildId(999_999)),
            member: None,
            mute: false,
            self_deaf: false,
            self_mute: false,
            self_stream: false,
            session_id: "asdasdas1da98da2b3ab3a".to_owned(),
            suppress: false,
            user_id: UserId(123_213),
        });

        // Token stream here's `Member` has no `guild_id`, which deserialiser
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
                    len: 11,
                },
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("999999"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("self_deaf"),
                Token::Bool(false),
                Token::Str("self_mute"),
                Token::Bool(false),
                Token::Str("self_stream"),
                Token::Bool(false),
                Token::Str("session_id"),
                Token::Str("asdasdas1da98da2b3ab3a"),
                Token::Str("suppress"),
                Token::Bool(false),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("123213"),
                Token::StructEnd,
            ],
        );
    }
}
