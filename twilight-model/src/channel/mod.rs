pub mod forum;
pub mod message;
pub mod permission_overwrite;
pub mod stage_instance;
pub mod thread;
pub mod webhook;

mod attachment;
mod attachment_flags;
mod channel_mention;
mod channel_type;
mod flags;
mod followed_channel;
mod video_quality_mode;

pub use self::{
    attachment::Attachment,
    attachment_flags::AttachmentFlags,
    channel_mention::ChannelMention,
    channel_type::ChannelType,
    flags::ChannelFlags,
    followed_channel::FollowedChannel,
    message::Message,
    stage_instance::StageInstance,
    video_quality_mode::VideoQualityMode,
    webhook::{Webhook, WebhookType},
};

use crate::{
    channel::{
        forum::{DefaultReaction, ForumLayout, ForumSortOrder, ForumTag},
        permission_overwrite::PermissionOverwrite,
        thread::{AutoArchiveDuration, ThreadMember, ThreadMetadata},
    },
    id::{
        marker::{
            ApplicationMarker, ChannelMarker, GenericMarker, GuildMarker, TagMarker, UserMarker,
        },
        Id,
    },
    user::User,
    util::{ImageHash, Timestamp},
};
use serde::{Deserialize, Serialize};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<Id<TagMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_tags: Option<Vec<ForumTag>>,
    /// Bitrate (in bits) setting of audio channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    /// Default duration without messages before the channel's threads
    /// automatically archive.
    ///
    /// Automatic archive durations are not locked behind the guild's boost
    /// level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<AutoArchiveDuration>,
    /// Default forum layout view used to display posts in forum channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<ForumLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_reaction_emoji: Option<DefaultReaction>,
    /// Default sort order used to display posts in forum channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<ForumSortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<u16>,
    /// Flags of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ChannelFlags>,
    /// ID of the guild the channel is in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Hash of the channel's icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,
    /// ID of the channel.
    pub id: Id<ChannelMarker>,
    /// Whether users can be invited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
    /// Type of the channel.
    ///
    /// This can be used to determine what fields *might* be available.
    #[serde(rename = "type")]
    pub kind: ChannelType,
    /// For text channels, this is the ID of the last message sent in the
    /// channel.
    ///
    /// For forum channels, this is the ID of the last created thread in the
    /// forum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<Id<GenericMarker>>,
    /// ID of the last message pinned in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<Timestamp>,
    /// Whether the channel is managed by an application via the [`gdm.join`]
    /// oauth scope.
    ///
    /// This is only applicable to [group channels].
    ///
    /// [`gdm.join`]: crate::oauth::scope::GDM_JOIN
    /// [group channels]: ChannelType::Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    /// Member that created the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<ThreadMember>,
    /// Number of members in the channel.
    ///
    /// At most a value of 50 is provided although the real number may be
    /// higher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i8>,
    /// Number of messages in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<u32>,
    /// Name of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether a thread was newly created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newly_created: Option<bool>,
    /// Whether the channel has been configured to be NSFW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
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
    pub position: Option<i32>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Metadata about a thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_metadata: Option<ThreadMetadata>,
    /// Topic of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// Number of users that may be in the channel.
    ///
    /// Zero refers to no limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u32>,
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
        guild::Permissions,
        id::Id,
        util::Timestamp,
    };

    // The deserializer for GuildChannel should skip over fields names that
    // it couldn't deserialize.
    #[test]
    fn guild_channel_unknown_field_deserialization() {
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
            applied_tags: None,
            available_tags: None,
            bitrate: None,
            default_auto_archive_duration: None,
            default_forum_layout: None,
            default_reaction_emoji: None,
            default_sort_order: None,
            default_thread_rate_limit_per_user: None,
            flags: None,
            guild_id: Some(Id::new(1)),
            icon: None,
            id: Id::new(2),
            invitable: None,
            kind: ChannelType::GuildText,
            last_message_id: Some(Id::new(3)),
            last_pin_timestamp: None,
            managed: None,
            member: None,
            member_count: None,
            message_count: None,
            name: Some("hey".to_owned()),
            newly_created: None,
            nsfw: Some(false),
            owner_id: None,
            parent_id: None,
            permission_overwrites: Some(Vec::new()),
            position: Some(0),
            rate_limit_per_user: Some(0),
            recipients: None,
            rtc_region: None,
            status: None,
            thread_metadata: None,
            topic: Some("a".to_owned()),
            user_limit: None,
            video_quality_mode: None,
        };

        assert_eq!(value, serde_json::from_value(input).unwrap());
    }

    #[test]
    fn guild_category_channel_deserialization() {
        let value = Channel {
            application_id: None,
            applied_tags: None,
            available_tags: None,
            bitrate: None,
            default_auto_archive_duration: None,
            default_forum_layout: None,
            default_reaction_emoji: None,
            default_sort_order: None,
            default_thread_rate_limit_per_user: None,
            flags: None,
            guild_id: Some(Id::new(2)),
            icon: None,
            id: Id::new(1),
            invitable: None,
            kind: ChannelType::GuildCategory,
            last_message_id: None,
            last_pin_timestamp: None,
            managed: None,
            member: None,
            member_count: None,
            message_count: None,
            name: Some("foo".to_owned()),
            newly_created: None,
            nsfw: None,
            owner_id: None,
            parent_id: None,
            permission_overwrites: Some(Vec::new()),
            position: Some(3),
            rate_limit_per_user: None,
            recipients: None,
            rtc_region: None,
            status: None,
            thread_metadata: None,
            topic: None,
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
    fn guild_announcement_channel_deserialization() {
        let value = Channel {
            application_id: None,
            applied_tags: None,
            available_tags: None,
            bitrate: None,
            default_auto_archive_duration: None,
            default_forum_layout: None,
            default_reaction_emoji: None,
            default_sort_order: None,
            default_thread_rate_limit_per_user: None,
            flags: None,
            guild_id: Some(Id::new(2)),
            icon: None,
            id: Id::new(1),
            invitable: None,
            kind: ChannelType::GuildAnnouncement,
            last_message_id: Some(Id::new(4)),
            last_pin_timestamp: None,
            managed: None,
            member: None,
            member_count: None,
            message_count: None,
            name: Some("news".to_owned()),
            newly_created: None,
            nsfw: Some(true),
            owner_id: None,
            parent_id: Some(Id::new(5)),
            permission_overwrites: Some(Vec::new()),
            position: Some(3),
            rate_limit_per_user: None,
            recipients: None,
            rtc_region: None,
            status: None,
            thread_metadata: None,
            topic: Some("a news channel".to_owned()),
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
                "type": ChannelType::GuildAnnouncement,
            }))
            .unwrap()
        );
    }

    #[test]
    fn guild_announcement_thread_deserialization() {
        let timestamp = Timestamp::from_secs(1_632_074_792).expect("non zero");
        let formatted = timestamp.iso_8601().to_string();

        let value = Channel {
            application_id: None,
            applied_tags: None,
            available_tags: None,
            bitrate: None,
            default_auto_archive_duration: Some(AutoArchiveDuration::Hour),
            default_forum_layout: None,
            default_reaction_emoji: None,
            default_sort_order: None,
            default_thread_rate_limit_per_user: None,
            flags: None,
            guild_id: Some(Id::new(1)),
            icon: None,
            id: Id::new(6),
            invitable: None,
            kind: ChannelType::AnnouncementThread,
            last_message_id: Some(Id::new(3)),
            last_pin_timestamp: None,
            managed: Some(true),
            member: Some(ThreadMember {
                flags: 0_u64,
                id: Some(Id::new(4)),
                join_timestamp: timestamp,
                member: None,
                presence: None,
                user_id: Some(Id::new(5)),
            }),
            member_count: Some(50),
            message_count: Some(50),
            name: Some("newsthread".into()),
            newly_created: Some(true),
            nsfw: None,
            owner_id: Some(Id::new(5)),
            parent_id: Some(Id::new(2)),
            permission_overwrites: None,
            position: None,
            rate_limit_per_user: Some(1000),
            recipients: None,
            rtc_region: None,
            status: None,
            thread_metadata: Some(ThreadMetadata {
                archived: false,
                auto_archive_duration: AutoArchiveDuration::Day,
                archive_timestamp: timestamp,
                create_timestamp: Some(timestamp),
                invitable: None,
                locked: false,
            }),
            topic: None,
            user_limit: None,
            video_quality_mode: None,
        };

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "6",
                "guild_id": "1",
                "type": ChannelType::AnnouncementThread,
                "last_message_id": "3",
                "member": {
                    "flags": 0,
                    "id": "4",
                    "join_timestamp": formatted,
                    "user_id": "5",
                },
                "default_auto_archive_duration": 60,
                "managed": true,
                "member_count": 50,
                "message_count": 50,
                "name": "newsthread",
                "newly_created": true,
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
    fn public_thread_deserialization() {
        let timestamp = Timestamp::from_secs(1_632_074_792).expect("non zero");

        let value = Channel {
            application_id: None,
            applied_tags: None,
            available_tags: None,
            bitrate: None,
            default_auto_archive_duration: Some(AutoArchiveDuration::Hour),
            default_forum_layout: None,
            default_reaction_emoji: None,
            default_sort_order: None,
            default_thread_rate_limit_per_user: None,
            flags: None,
            guild_id: Some(Id::new(1)),
            icon: None,
            id: Id::new(6),
            invitable: None,
            kind: ChannelType::PublicThread,
            last_message_id: Some(Id::new(3)),
            last_pin_timestamp: None,
            managed: Some(true),
            member: Some(ThreadMember {
                flags: 0_u64,
                id: Some(Id::new(4)),
                join_timestamp: timestamp,
                member: None,
                presence: None,
                user_id: Some(Id::new(5)),
            }),
            member_count: Some(50),
            message_count: Some(50),
            name: Some("publicthread".into()),
            newly_created: Some(true),
            nsfw: None,
            owner_id: Some(Id::new(5)),
            parent_id: Some(Id::new(2)),
            permission_overwrites: None,
            position: None,
            rate_limit_per_user: Some(1000),
            recipients: None,
            rtc_region: None,
            status: None,
            thread_metadata: Some(ThreadMetadata {
                archived: false,
                auto_archive_duration: AutoArchiveDuration::Day,
                archive_timestamp: timestamp,
                create_timestamp: Some(timestamp),
                invitable: None,
                locked: false,
            }),
            topic: None,
            user_limit: None,
            video_quality_mode: None,
        };

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "6",
                "guild_id": "1",
                "type": ChannelType::PublicThread,
                "last_message_id": "3",
                "member": {
                    "flags": 0,
                    "id": "4",
                    "join_timestamp": timestamp,
                    "user_id": "5",
                },
                "default_auto_archive_duration": 60,
                "managed": true,
                "member_count": 50,
                "message_count": 50,
                "name": "publicthread",
                "newly_created": true,
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
    fn private_thread_deserialization() {
        let timestamp = Timestamp::from_secs(1_632_074_792).expect("non zero");
        let formatted = timestamp.iso_8601().to_string();

        let value = Channel {
            application_id: None,
            applied_tags: None,
            available_tags: None,
            bitrate: None,
            default_auto_archive_duration: Some(AutoArchiveDuration::Hour),
            default_forum_layout: None,
            default_reaction_emoji: None,
            default_sort_order: None,
            default_thread_rate_limit_per_user: None,
            flags: None,
            guild_id: Some(Id::new(1)),
            icon: None,
            id: Id::new(6),
            invitable: Some(true),
            kind: ChannelType::PrivateThread,
            last_message_id: Some(Id::new(3)),
            last_pin_timestamp: None,
            managed: Some(true),
            member: Some(ThreadMember {
                flags: 0_u64,
                id: Some(Id::new(4)),
                join_timestamp: timestamp,
                member: None,
                presence: None,
                user_id: Some(Id::new(5)),
            }),
            // Old threads can have negative member counts, so we need to ensure
            // we keep negative member counts around.
            member_count: Some(-1),
            message_count: Some(50),
            name: Some("privatethread".into()),
            newly_created: Some(true),
            nsfw: None,
            owner_id: Some(Id::new(5)),
            parent_id: Some(Id::new(2)),
            permission_overwrites: Some(Vec::from([PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::empty(),
                id: Id::new(5),
                kind: PermissionOverwriteType::Member,
            }])),
            position: None,
            rate_limit_per_user: Some(1000),
            recipients: None,
            rtc_region: None,
            status: None,
            thread_metadata: Some(ThreadMetadata {
                archived: false,
                auto_archive_duration: AutoArchiveDuration::Day,
                archive_timestamp: timestamp,
                create_timestamp: Some(timestamp),
                invitable: None,
                locked: false,
            }),
            topic: None,
            user_limit: None,
            video_quality_mode: None,
        };

        assert_eq!(
            value,
            serde_json::from_value(serde_json::json!({
                "id": "6",
                "guild_id": "1",
                "type": ChannelType::PrivateThread,
                "last_message_id": "3",
                "member": {
                    "flags": 0,
                    "id": "4",
                    "join_timestamp": formatted,
                    "user_id": "5",
                },
                "default_auto_archive_duration": 60,
                "invitable": true,
                "managed": true,
                "member_count": -1,
                "message_count": 50,
                "name": "privatethread",
                "newly_created": true,
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
