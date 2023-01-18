pub mod forum;
pub mod message;
pub mod permission_overwrite;
pub mod stage_instance;
pub mod thread;
pub mod webhook;

mod attachment;
mod channel_mention;
mod channel_type;
mod flags;
mod followed_channel;
mod video_quality_mode;

pub use self::{
    attachment::Attachment,
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
    use serde_test::Token;

    // The deserializer for GuildChannel should skip over fields names that
    // it couldn't deserialize.
    #[allow(clippy::too_many_lines)]
    #[test]
    fn guild_channel_unknown_field_deserialization() {
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
            thread_metadata: None,
            topic: Some("a".to_owned()),
            user_limit: None,
            video_quality_mode: None,
        };

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Channel",
                    len: 17,
                },
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(u8::from(ChannelType::GuildText)),
                Token::Str("last_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("name"),
                Token::Some,
                Token::Str("hey"),
                Token::Str("nsfw"),
                Token::Some,
                Token::Bool(false),
                Token::Str("permission_overwrites"),
                Token::Some,
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("position"),
                Token::Some,
                Token::I32(0),
                Token::Str("rate_limit_per_user"),
                Token::Some,
                Token::U16(0),
                Token::Str("topic"),
                Token::Some,
                Token::Str("a"),
                Token::Str("unknown_field"),
                Token::Str("unknown value"),
                Token::StructEnd,
            ],
        );
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
            thread_metadata: None,
            topic: None,
            user_limit: None,
            video_quality_mode: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Channel",
                    len: 6,
                },
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(u8::from(ChannelType::GuildCategory)),
                Token::Str("name"),
                Token::Some,
                Token::Str("foo"),
                Token::Str("permission_overwrites"),
                Token::Some,
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("position"),
                Token::Some,
                Token::I32(3),
                Token::StructEnd,
            ],
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
            thread_metadata: None,
            topic: Some("a news channel".to_owned()),
            user_limit: None,
            video_quality_mode: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Channel",
                    len: 10,
                },
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(u8::from(ChannelType::GuildAnnouncement)),
                Token::Str("last_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("name"),
                Token::Some,
                Token::Str("news"),
                Token::Str("nsfw"),
                Token::Some,
                Token::Bool(true),
                Token::Str("parent_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::Str("permission_overwrites"),
                Token::Some,
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("position"),
                Token::Some,
                Token::I32(3),
                Token::Str("topic"),
                Token::Some,
                Token::Str("a news channel"),
                Token::StructEnd,
            ],
        );
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn guild_announcement_thread_deserialization() {
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
            kind: ChannelType::AnnouncementThread,
            last_message_id: Some(Id::new(3)),
            last_pin_timestamp: None,
            member: Some(ThreadMember {
                flags: 0_u64,
                id: Some(Id::new(4)),
                join_timestamp: timestamp,
                member: None,
                presence: None,
                user_id: Some(Id::new(5)),
            }),
            member_count: Some(50_u8),
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

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Channel",
                    len: 14,
                },
                Token::Str("default_auto_archive_duration"),
                Token::Some,
                Token::U16(AutoArchiveDuration::Hour.number()),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::Str("type"),
                Token::U8(u8::from(ChannelType::AnnouncementThread)),
                Token::Str("last_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "ThreadMember",
                    len: 4,
                },
                Token::Str("flags"),
                Token::U64(0),
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("join_timestamp"),
                Token::Str("2021-09-19T18:06:32.000000+00:00"),
                Token::Str("user_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::StructEnd,
                Token::Str("member_count"),
                Token::Some,
                Token::U8(50),
                Token::Str("message_count"),
                Token::Some,
                Token::U32(50),
                Token::Str("name"),
                Token::Some,
                Token::Str("newsthread"),
                Token::Str("newly_created"),
                Token::Some,
                Token::Bool(true),
                Token::Str("owner_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::Str("parent_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("rate_limit_per_user"),
                Token::Some,
                Token::U16(1000),
                Token::Str("thread_metadata"),
                Token::Some,
                Token::Struct {
                    name: "ThreadMetadata",
                    len: 5,
                },
                Token::Str("archived"),
                Token::Bool(false),
                Token::Str("auto_archive_duration"),
                Token::U16(AutoArchiveDuration::Day.number()),
                Token::Str("archive_timestamp"),
                Token::Str("2021-09-19T18:06:32.000000+00:00"),
                Token::Str("create_timestamp"),
                Token::Some,
                Token::Str("2021-09-19T18:06:32.000000+00:00"),
                Token::Str("locked"),
                Token::Bool(false),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[allow(clippy::too_many_lines)]
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
            member: Some(ThreadMember {
                flags: 0_u64,
                id: Some(Id::new(4)),
                join_timestamp: timestamp,
                member: None,
                presence: None,
                user_id: Some(Id::new(5)),
            }),
            member_count: Some(50_u8),
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

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Channel",
                    len: 14,
                },
                Token::Str("default_auto_archive_duration"),
                Token::Some,
                Token::U16(AutoArchiveDuration::Hour.number()),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::Str("type"),
                Token::U8(u8::from(ChannelType::PublicThread)),
                Token::Str("last_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "ThreadMember",
                    len: 4,
                },
                Token::Str("flags"),
                Token::U64(0),
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("join_timestamp"),
                Token::Str("2021-09-19T18:06:32.000000+00:00"),
                Token::Str("user_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::StructEnd,
                Token::Str("member_count"),
                Token::Some,
                Token::U8(50),
                Token::Str("message_count"),
                Token::Some,
                Token::U32(50),
                Token::Str("name"),
                Token::Some,
                Token::Str("publicthread"),
                Token::Str("newly_created"),
                Token::Some,
                Token::Bool(true),
                Token::Str("owner_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::Str("parent_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("rate_limit_per_user"),
                Token::Some,
                Token::U16(1000),
                Token::Str("thread_metadata"),
                Token::Some,
                Token::Struct {
                    name: "ThreadMetadata",
                    len: 5,
                },
                Token::Str("archived"),
                Token::Bool(false),
                Token::Str("auto_archive_duration"),
                Token::U16(AutoArchiveDuration::Day.number()),
                Token::Str("archive_timestamp"),
                Token::Str("2021-09-19T18:06:32.000000+00:00"),
                Token::Str("create_timestamp"),
                Token::Some,
                Token::Str("2021-09-19T18:06:32.000000+00:00"),
                Token::Str("locked"),
                Token::Bool(false),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn private_thread_deserialization() {
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
            invitable: Some(true),
            kind: ChannelType::PrivateThread,
            last_message_id: Some(Id::new(3)),
            last_pin_timestamp: None,
            member: Some(ThreadMember {
                flags: 0_u64,
                id: Some(Id::new(4)),
                join_timestamp: timestamp,
                member: None,
                presence: None,
                user_id: Some(Id::new(5)),
            }),
            member_count: Some(50_u8),
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

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Channel",
                    len: 16,
                },
                Token::Str("default_auto_archive_duration"),
                Token::Some,
                Token::U16(AutoArchiveDuration::Hour.number()),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::Str("invitable"),
                Token::Some,
                Token::Bool(true),
                Token::Str("type"),
                Token::U8(u8::from(ChannelType::PrivateThread)),
                Token::Str("last_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "ThreadMember",
                    len: 4,
                },
                Token::Str("flags"),
                Token::U64(0),
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("join_timestamp"),
                Token::Str("2021-09-19T18:06:32.000000+00:00"),
                Token::Str("user_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::StructEnd,
                Token::Str("member_count"),
                Token::Some,
                Token::U8(50),
                Token::Str("message_count"),
                Token::Some,
                Token::U32(50),
                Token::Str("name"),
                Token::Some,
                Token::Str("privatethread"),
                Token::Str("newly_created"),
                Token::Some,
                Token::Bool(true),
                Token::Str("owner_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::Str("parent_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("permission_overwrites"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "PermissionOverwrite",
                    len: 4,
                },
                Token::Str("allow"),
                Token::Str("0"),
                Token::Str("deny"),
                Token::Str("0"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::Str("type"),
                Token::U8(u8::from(PermissionOverwriteType::Member)),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("rate_limit_per_user"),
                Token::Some,
                Token::U16(1000),
                Token::Str("thread_metadata"),
                Token::Some,
                Token::Struct {
                    name: "ThreadMetadata",
                    len: 5,
                },
                Token::Str("archived"),
                Token::Bool(false),
                Token::Str("auto_archive_duration"),
                Token::U16(AutoArchiveDuration::Day.number()),
                Token::Str("archive_timestamp"),
                Token::Str("2021-09-19T18:06:32.000000+00:00"),
                Token::Str("create_timestamp"),
                Token::Some,
                Token::Str("2021-09-19T18:06:32.000000+00:00"),
                Token::Str("locked"),
                Token::Bool(false),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
