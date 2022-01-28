pub mod embed;
pub mod message;
pub mod permission_overwrite;
pub mod stage_instance;
pub mod thread;
pub mod webhook;

mod attachment;
mod channel_mention;
mod channel_type;
mod followed_channel;
mod reaction;
mod reaction_type;
mod video_quality_mode;

use self::permission_overwrite::PermissionOverwrite;
pub use self::{
    attachment::Attachment,
    channel_mention::ChannelMention,
    channel_type::ChannelType,
    followed_channel::FollowedChannel,
    message::Message,
    reaction::Reaction,
    reaction_type::ReactionType,
    stage_instance::StageInstance,
    video_quality_mode::VideoQualityMode,
    webhook::{Webhook, WebhookType},
};

use crate::{
    channel::thread::{AutoArchiveDuration, ThreadMember, ThreadMetadata},
    datetime::Timestamp,
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, MessageMarker, UserMarker},
        Id,
    },
    user::User,
    util::ImageHash,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ConversionError {
    MessageType(u8),
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ConversionError::MessageType(num) => {
                f.write_str("Could not convert ")?;
                Display::fmt(num, f)?;

                f.write_str(" into a valid MessageType!")
            }
        }
    }
}

/// Channel to send messages in, call with other users, organize groups, and
/// more.
///
/// The `Channel` type is one overarching type for all types of channels: there
/// is no distinction between audio channels, textual channels, guild channels,
/// groups, threads, and so on. The type of channel can be determined by
/// checking [`Channel::kind`], which can be used to determine what fields you
/// might expect to be present.
///
/// For Discord's documentation on channels, refer to [Discord Docs/Channel].
///
/// [Discord Docs/Channel]: https://discord.com/developers/docs/resources/channel
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Channel {
    /// ID of the application that created the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    /// Bitrate setting of audio channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u64>,
    /// Default duration before the channel's threads archive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<AutoArchiveDuration>,
    /// ID of the guild the channel is in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Hash of the channel's icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,
    /// ID of the channel.
    pub id: Id<ChannelMarker>,
    /// Type of the channel.
    ///
    /// This can be used to determine what fields *might* be available.
    #[serde(rename = "type")]
    pub kind: ChannelType,
    /// Name of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether users can be invited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
    /// ID of the last message sent in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<Id<MessageMarker>>,
    /// ID of the last message pinned in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<Timestamp>,
    /// Whether the channel has been configured to be NSFW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// Member that created the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<ThreadMember>,
    /// Number of members in the channel.
    ///
    /// At most a value of 50 is provided although the real number may be
    /// higher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u8>,
    /// Number of messages in the channel.
    ///
    /// At most a value of 50 is provided although the real number may be
    /// higher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<u8>,
    /// ID of the creator of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Id<UserMarker>>,
    /// ID of the parent channel.
    ///
    /// For guild channels this is the ID of the parent category channel.
    ///
    /// For threads this is the ID of the channel the thread was created in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Id<ChannelMarker>>,
    /// Explicit permission overwrites for members and roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    /// Sorting position of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    /// Amount of seconds a user has to wait before sending another message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u16>,
    /// Recipients of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<User>>,
    /// ID of the voice region for the channel.
    ///
    /// Defaults to automatic for applicable channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<String>,
    /// Topic of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// Metadata about a thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_metadata: Option<ThreadMetadata>,
    /// Number of users that may be in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u64>,
    /// Camera video quality mode of the channel.
    ///
    /// Defaults to [`VideoQualityMode::Auto`] for applicable channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality_mode: Option<VideoQualityMode>,
}

#[cfg(test)]
mod tests {
    use super::{AutoArchiveDuration, Channel, ChannelType, ThreadMember, ThreadMetadata};
    use crate::{
        channel::permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
        datetime::Timestamp,
        guild::Permissions,
        id::Id,
    };

    // The deserializer for GuildChannel should skip over fields names that
    // it couldn't deserialize.
    #[test]
    fn test_guild_channel_unknown_field_deserialization() {
        let input = serde_json::json!({
            "type": 0,
            "topic": "a",
            "rate_limit_per_user": 0,
            "position": 0,
            "permission_overwrites": [],
            "parent_id": null,
            "nsfw": false,
            "name": "hey",
            "last_message_id": "3",
            "id": "2",
            "guild_id": "1",
            "guild_hashes": {
                "version": 1,
                "roles": {
                    "hash": "aaaaaaaaaaa"
                },
                "metadata": {
                    "hash": "bbbbbbbbbbb"
                },
                "channels": {
                    "hash": "ccccccccccc"
                }
            },
            "unknown_field": "the deserializer should skip unknown field names",
        });

        let value = Channel {
            application_id: None,
            bitrate: None,
            default_auto_archive_duration: None,
            guild_id: Some(Id::new(1)),
            icon: None,
            id: Id::new(2),
            kind: ChannelType::GuildText,
            name: Some("hey".to_owned()),
            invitable: None,
            last_message_id: Some(Id::new(3)),
            last_pin_timestamp: None,
            nsfw: Some(false),
            member: None,
            member_count: None,
            message_count: None,
            owner_id: None,
            parent_id: None,
            permission_overwrites: Some(Vec::new()),
            position: Some(0),
            rate_limit_per_user: Some(0),
            recipients: None,
            rtc_region: None,
            topic: Some("a".to_owned()),
            thread_metadata: None,
            user_limit: None,
            video_quality_mode: None,
        };

        assert_eq!(value, serde_json::from_value(input).unwrap());
    }

    #[test]
    fn test_guild_category_channel_deserialization() {
        let value = Channel {
            application_id: None,
            bitrate: None,
            default_auto_archive_duration: None,
            guild_id: Some(Id::new(2)),
            icon: None,
            id: Id::new(1),
            kind: ChannelType::GuildCategory,
            name: Some("foo".to_owned()),
            invitable: None,
            last_message_id: None,
            last_pin_timestamp: None,
            nsfw: None,
            member: None,
            member_count: None,
            message_count: None,
            owner_id: None,
            parent_id: None,
            permission_overwrites: Some(Vec::new()),
            position: Some(3),
            rate_limit_per_user: None,
            recipients: None,
            rtc_region: None,
            topic: None,
            thread_metadata: None,
            user_limit: None,
            video_quality_mode: None,
        };
        let permission_overwrites: Vec<PermissionOverwrite> = Vec::new();

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "1",
                "guild_id": Some("2"),
                "name": "foo",
                "permission_overwrites": permission_overwrites,
                "position": 3,
                "type": 4,
            }))
            .unwrap()
        );
    }

    #[test]
    fn test_guild_news_channel_deserialization() {
        let value = Channel {
            application_id: None,
            bitrate: None,
            default_auto_archive_duration: None,
            guild_id: Some(Id::new(2)),
            icon: None,
            id: Id::new(1),
            kind: ChannelType::GuildNews,
            name: Some("news".to_owned()),
            invitable: None,
            last_message_id: Some(Id::new(4)),
            last_pin_timestamp: None,
            nsfw: Some(true),
            member: None,
            member_count: None,
            message_count: None,
            owner_id: None,
            parent_id: Some(Id::new(5)),
            permission_overwrites: Some(Vec::new()),
            position: Some(3),
            rate_limit_per_user: None,
            recipients: None,
            rtc_region: None,
            topic: Some("a news channel".to_owned()),
            thread_metadata: None,
            user_limit: None,
            video_quality_mode: None,
        };
        let permission_overwrites: Vec<PermissionOverwrite> = Vec::new();

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "1",
                "guild_id": "2",
                "name": "news",
                "nsfw": true,
                "last_message_id": "4",
                "parent_id": "5",
                "permission_overwrites": permission_overwrites,
                "position": 3,
                "topic": "a news channel",
                "type": ChannelType::GuildNews,
            }))
            .unwrap()
        );
    }

    #[test]
    fn test_guild_store_channel_deserialization() {
        let value = Channel {
            application_id: None,
            bitrate: None,
            default_auto_archive_duration: None,
            guild_id: Some(Id::new(2)),
            icon: None,
            id: Id::new(1),
            kind: ChannelType::GuildStore,
            name: Some("store".to_owned()),
            invitable: None,
            last_message_id: None,
            last_pin_timestamp: None,
            nsfw: Some(false),
            member: None,
            member_count: None,
            message_count: None,
            owner_id: None,
            parent_id: None,
            permission_overwrites: Some(Vec::new()),
            position: Some(2),
            rate_limit_per_user: None,
            recipients: None,
            rtc_region: None,
            topic: Some("a store channel".to_owned()),
            thread_metadata: None,
            user_limit: None,
            video_quality_mode: None,
        };
        let permission_overwrites: Vec<PermissionOverwrite> = Vec::new();

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "1",
                "guild_id": "2",
                "name": "store",
                "nsfw": false,
                "permission_overwrites": permission_overwrites,
                "position": 2,
                "topic": "a store channel",
                "type": ChannelType::GuildStore,
            }))
            .unwrap()
        );
    }

    #[test]
    fn test_guild_news_thread_deserialization() {
        let timestamp = Timestamp::from_secs(1_632_074_792).expect("non zero");
        let formatted = timestamp.iso_8601().to_string();

        let value = Channel {
            application_id: None,
            bitrate: None,
            default_auto_archive_duration: Some(AutoArchiveDuration::Hour),
            guild_id: Some(Id::new(1)),
            icon: None,
            id: Id::new(6),
            kind: ChannelType::GuildNewsThread,
            name: Some("newsthread".into()),
            invitable: None,
            last_message_id: Some(Id::new(3)),
            last_pin_timestamp: None,
            nsfw: None,
            member: Some(ThreadMember {
                flags: 0_u64,
                id: Some(Id::new(4)),
                join_timestamp: timestamp,
                member: None,
                presence: None,
                user_id: Some(Id::new(5)),
            }),
            member_count: Some(50_u8),
            message_count: Some(50_u8),
            owner_id: Some(Id::new(5)),
            parent_id: Some(Id::new(2)),
            permission_overwrites: None,
            position: None,
            rate_limit_per_user: Some(1000_u16),
            recipients: None,
            rtc_region: None,
            topic: None,
            thread_metadata: Some(ThreadMetadata {
                archived: false,
                auto_archive_duration: AutoArchiveDuration::Day,
                archive_timestamp: timestamp,
                create_timestamp: Some(timestamp),
                invitable: None,
                locked: false,
            }),
            user_limit: None,
            video_quality_mode: None,
        };

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "6",
                "guild_id": "1",
                "type": ChannelType::GuildNewsThread,
                "last_message_id": "3",
                "member": {
                    "flags": 0,
                    "id": "4",
                    "join_timestamp": formatted,
                    "user_id": "5",
                },
                "default_auto_archive_duration": 60,
                "member_count": 50,
                "message_count": 50,
                "name": "newsthread",
                "owner_id": "5",
                "parent_id": "2",
                "rate_limit_per_user": 1000,
                "thread_metadata": {
                    "archive_timestamp": formatted,
                    "archived": false,
                    "auto_archive_duration": AutoArchiveDuration::Day,
                    "create_timestamp": formatted,
                    "locked": false
                }
            }))
            .unwrap()
        )
    }

    #[test]
    fn test_guild_public_thread_deserialization() {
        let timestamp = Timestamp::from_secs(1_632_074_792).expect("non zero");

        let value = Channel {
            application_id: None,
            bitrate: None,
            default_auto_archive_duration: Some(AutoArchiveDuration::Hour),
            guild_id: Some(Id::new(1)),
            icon: None,
            id: Id::new(6),
            kind: ChannelType::GuildPublicThread,
            name: Some("publicthread".into()),
            invitable: None,
            last_message_id: Some(Id::new(3)),
            last_pin_timestamp: None,
            nsfw: None,
            member: Some(ThreadMember {
                flags: 0_u64,
                id: Some(Id::new(4)),
                join_timestamp: timestamp,
                member: None,
                presence: None,
                user_id: Some(Id::new(5)),
            }),
            member_count: Some(50_u8),
            message_count: Some(50_u8),
            owner_id: Some(Id::new(5)),
            parent_id: Some(Id::new(2)),
            permission_overwrites: None,
            position: None,
            rate_limit_per_user: Some(1000_u16),
            recipients: None,
            rtc_region: None,
            topic: None,
            thread_metadata: Some(ThreadMetadata {
                archived: false,
                auto_archive_duration: AutoArchiveDuration::Day,
                archive_timestamp: timestamp,
                create_timestamp: Some(timestamp),
                invitable: None,
                locked: false,
            }),
            user_limit: None,
            video_quality_mode: None,
        };

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "6",
                "guild_id": "1",
                "type": ChannelType::GuildPublicThread,
                "last_message_id": "3",
                "member": {
                    "flags": 0,
                    "id": "4",
                    "join_timestamp": timestamp,
                    "user_id": "5",
                },
                "default_auto_archive_duration": 60,
                "member_count": 50,
                "message_count": 50,
                "name": "publicthread",
                "owner_id": "5",
                "parent_id": "2",
                "rate_limit_per_user": 1000,
                "thread_metadata": {
                    "archive_timestamp": timestamp,
                    "archived": false,
                    "auto_archive_duration": AutoArchiveDuration::Day,
                    "create_timestamp": timestamp,
                    "locked": false
                }
            }))
            .unwrap()
        )
    }

    #[test]
    fn test_guild_private_thread_deserialization() {
        let timestamp = Timestamp::from_secs(1_632_074_792).expect("non zero");
        let formatted = timestamp.iso_8601().to_string();

        let value = Channel {
            application_id: None,
            bitrate: None,
            default_auto_archive_duration: Some(AutoArchiveDuration::Hour),
            guild_id: Some(Id::new(1)),
            icon: None,
            id: Id::new(6),
            kind: ChannelType::GuildPrivateThread,
            name: Some("privatethread".into()),
            invitable: Some(true),
            last_message_id: Some(Id::new(3)),
            last_pin_timestamp: None,
            nsfw: None,
            member: Some(ThreadMember {
                flags: 0_u64,
                id: Some(Id::new(4)),
                join_timestamp: timestamp,
                member: None,
                presence: None,
                user_id: Some(Id::new(5)),
            }),
            member_count: Some(50_u8),
            message_count: Some(50_u8),
            owner_id: Some(Id::new(5)),
            parent_id: Some(Id::new(2)),
            permission_overwrites: Some(Vec::from([PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(Id::new(5)),
            }])),
            position: None,
            rate_limit_per_user: Some(1000_u16),
            recipients: None,
            rtc_region: None,
            topic: None,
            thread_metadata: Some(ThreadMetadata {
                archived: false,
                auto_archive_duration: AutoArchiveDuration::Day,
                archive_timestamp: timestamp,
                create_timestamp: Some(timestamp),
                invitable: None,
                locked: false,
            }),
            user_limit: None,
            video_quality_mode: None,
        };

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "6",
                "guild_id": "1",
                "type": ChannelType::GuildPrivateThread,
                "last_message_id": "3",
                "member": {
                    "flags": 0,
                    "id": "4",
                    "join_timestamp": formatted,
                    "user_id": "5",
                },
                "default_auto_archive_duration": 60,
                "invitable": true,
                "member_count": 50,
                "message_count": 50,
                "name": "privatethread",
                "owner_id": "5",
                "parent_id": "2",
                "rate_limit_per_user": 1000,
                "thread_metadata": {
                    "archive_timestamp": formatted,
                    "archived": false,
                    "auto_archive_duration": AutoArchiveDuration::Day,
                    "create_timestamp": formatted,
                    "locked": false
                },
                "permission_overwrites": [
                    {
                        "allow": "0",
                        "deny": "0",
                        "type": 1,
                        "id": "5"
                    }
                ]
            }))
            .unwrap()
        )
    }
}
