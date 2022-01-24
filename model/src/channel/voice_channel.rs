use crate::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType, VideoQualityMode},
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceChannel {
    pub bitrate: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<ChannelMarker>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id<ChannelMarker>>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub position: i64,
    /// ID of the voice channel's region.
    ///
    /// Automatic when not present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality_mode: Option<VideoQualityMode>,
}

#[cfg(test)]
mod tests {
    use super::{ChannelType, VideoQualityMode, VoiceChannel};
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn test_voice_channel() {
        let value = VoiceChannel {
            id: Id::new(1),
            bitrate: 124_000,
            guild_id: Some(Id::new(2)),
            kind: ChannelType::GuildVoice,
            name: "foo".to_owned(),
            permission_overwrites: Vec::new(),
            parent_id: None,
            position: 3,
            rtc_region: None,
            user_limit: Some(7),
            video_quality_mode: None,
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
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
                id: Id::new(1),
                bitrate: 124_000,
                guild_id: Some(Id::new(2)),
                kind,
                name: "foo".to_owned(),
                permission_overwrites: Vec::new(),
                parent_id: Some(Id::new(3)),
                position: 3,
                rtc_region: Some("a".to_owned()),
                user_limit: Some(7),
                video_quality_mode: Some(VideoQualityMode::Auto),
            }
        }

        const fn tokens(kind: ChannelType) -> [Token; 33] {
            [
                Token::Struct {
                    name: "VoiceChannel",
                    len: 11,
                },
                Token::Str("bitrate"),
                Token::U64(124_000),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(kind as u8),
                Token::Str("name"),
                Token::Str("foo"),
                Token::Str("parent_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("permission_overwrites"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("position"),
                Token::I64(3),
                Token::Str("rtc_region"),
                Token::Some,
                Token::Str("a"),
                Token::Str("user_limit"),
                Token::Some,
                Token::U64(7),
                Token::Str("video_quality_mode"),
                Token::Some,
                Token::U8(1),
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
