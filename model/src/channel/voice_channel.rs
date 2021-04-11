use crate::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType},
    id::{ChannelId, GuildId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceChannel {
    pub bitrate: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub position: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::{ChannelId, ChannelType, GuildId, VoiceChannel};
    use serde_test::Token;

    #[test]
    fn test_voice_channel() {
        let value = VoiceChannel {
            id: ChannelId(1),
            bitrate: 124_000,
            guild_id: Some(GuildId(2)),
            kind: ChannelType::GuildVoice,
            name: "foo".to_owned(),
            permission_overwrites: Vec::new(),
            parent_id: None,
            position: 3,
            user_limit: Some(7),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "VoiceChannel",
                    len: 8,
                },
                Token::Str("bitrate"),
                Token::U64(124_000),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(2),
                Token::Str("name"),
                Token::Str("foo"),
                Token::Str("permission_overwrites"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("position"),
                Token::I64(3),
                Token::Str("user_limit"),
                Token::Some,
                Token::U64(7),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_voice_channel_complete() {
        fn channel(kind: ChannelType) -> VoiceChannel {
            VoiceChannel {
                id: ChannelId(1),
                bitrate: 124_000,
                guild_id: Some(GuildId(2)),
                kind,
                name: "foo".to_owned(),
                permission_overwrites: Vec::new(),
                parent_id: Some(ChannelId(3)),
                position: 3,
                user_limit: Some(7),
            }
        }

        fn tokens(kind: ChannelType) -> [Token; 27] {
            [
                Token::Struct {
                    name: "VoiceChannel",
                    len: 9,
                },
                Token::Str("bitrate"),
                Token::U64(124_000),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(kind as u8),
                Token::Str("name"),
                Token::Str("foo"),
                Token::Str("parent_id"),
                Token::Some,
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("3"),
                Token::Str("permission_overwrites"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("position"),
                Token::I64(3),
                Token::Str("user_limit"),
                Token::Some,
                Token::U64(7),
                Token::StructEnd,
            ]
        }

        serde_test::assert_tokens(
            &channel(ChannelType::GuildVoice),
            &tokens(ChannelType::GuildVoice),
        );
        serde_test::assert_tokens(
            &channel(ChannelType::GuildStageVoice),
            &tokens(ChannelType::GuildStageVoice),
        );
    }
}
