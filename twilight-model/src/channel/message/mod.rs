pub mod allowed_mentions;
pub mod sticker;

mod activity;
mod activity_type;
mod application;
mod flags;
mod interaction;
mod kind;
mod mention;
mod reaction;
mod reference;

pub use self::{
    activity::MessageActivity, activity_type::MessageActivityType,
    allowed_mentions::AllowedMentions, application::MessageApplication, flags::MessageFlags,
    interaction::MessageInteraction, kind::MessageType, mention::Mention,
    reaction::MessageReaction, reference::MessageReference, sticker::Sticker,
};

use self::sticker::MessageSticker;
use crate::{
    application::component::Component,
    channel::{embed::Embed, Attachment, Channel, ChannelMention},
    guild::PartialMember,
    id::{
        marker::{
            ApplicationMarker, ChannelMarker, GuildMarker, MessageMarker, RoleMarker, WebhookMarker,
        },
        Id,
    },
    user::User,
    util::Timestamp,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<MessageActivity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<MessageApplication>,
    /// Associated application's ID.
    ///
    /// Sent if the message is a response to an Interaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    /// List of attachments.
    ///
    /// Receiving the attachments of messages requires that the
    /// [Message Content Intent] be enabled for the application. In the case of
    /// receiving messages over the Gateway, the intent must also be enabled for
    /// the session.
    ///
    /// Message attachments will be empty unless the [Message Content Intent] is
    /// enabled, the message was sent by the current user, or the message is in
    /// a direct message channel.
    ///
    /// [Message Content Intent]: crate::gateway::Intents::MESSAGE_CONTENT
    pub attachments: Vec<Attachment>,
    pub author: User,
    pub channel_id: Id<ChannelMarker>,
    /// List of provided components, such as buttons.
    ///
    /// Receiving the components of messages requires that the
    /// [Message Content Intent] be enabled for the application. In the case of
    /// receiving messages over the Gateway, the intent must also be enabled for
    /// the session.
    ///
    /// Message components will be empty unless the [Message Content Intent] is
    /// enabled, the message was sent by the current user, or the message is in
    /// a direct message channel.
    ///
    /// [Message Content Intent]: crate::gateway::Intents::MESSAGE_CONTENT
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,
    /// Content of the message.
    ///
    /// Receiving the content of messages requires that the
    /// [Message Content Intent] be enabled for the application. In the case of
    /// receiving messages over the Gateway, the intent must also be enabled for
    /// the session.
    ///
    /// Message content will be empty unless the [Message Content Intent] is
    /// enabled, the message was sent by the current user, or the message is in
    /// a direct message channel.
    ///
    /// [Message Content Intent]: crate::gateway::Intents::MESSAGE_CONTENT
    pub content: String,
    pub edited_timestamp: Option<Timestamp>,
    /// List of embeds.
    ///
    /// Receiving the embeds of messages requires that the
    /// [Message Content Intent] be enabled for the application. In the case of
    /// receiving messages over the Gateway, the intent must also be enabled for
    /// the session.
    ///
    /// Message embeds will be empty unless the [Message Content Intent] is
    /// enabled, the message was sent by the current user, or the message is in
    /// a direct message channel.
    ///
    /// [Message Content Intent]: crate::gateway::Intents::MESSAGE_CONTENT
    pub embeds: Vec<Embed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<MessageMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction: Option<MessageInteraction>,
    #[serde(rename = "type")]
    pub kind: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialMember>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mention_channels: Vec<ChannelMention>,
    pub mention_everyone: bool,
    pub mention_roles: Vec<Id<RoleMarker>>,
    pub mentions: Vec<Mention>,
    pub pinned: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reactions: Vec<MessageReaction>,
    /// Reference data sent with crossposted messages and replies.
    #[serde(rename = "message_reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<MessageReference>,
    /// The message associated with the [reference].
    ///
    /// [reference]: Self::reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_message: Option<Box<Message>>,
    /// Stickers within the message.
    #[serde(default)]
    pub sticker_items: Vec<MessageSticker>,
    /// Timestamp of when the message was created.
    pub timestamp: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Channel>,
    pub tts: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<Id<WebhookMarker>>,
}

#[cfg(test)]
mod tests {
    use super::{
        sticker::{MessageSticker, StickerFormatType},
        ChannelMention, Message, MessageActivity, MessageActivityType, MessageApplication,
        MessageFlags, MessageReaction, MessageReference, MessageType,
    };
    use crate::{
        channel::{ChannelType, ReactionType},
        guild::PartialMember,
        id::Id,
        test::image_hash,
        user::User,
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn message_deserialization() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2020-01-01T00:00:00.000000+00:00")?;
        let timestamp = Timestamp::from_micros(1_580_608_922_020_000).expect("non zero");

        let value = Message {
            activity: None,
            application: None,
            application_id: None,
            attachments: Vec::new(),
            author: User {
                accent_color: None,
                avatar: Some(image_hash::AVATAR),
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: Id::new(3),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
            channel_id: Id::new(2),
            components: Vec::new(),
            content: "ping".to_owned(),
            edited_timestamp: None,
            embeds: Vec::new(),
            flags: Some(MessageFlags::empty()),
            guild_id: Some(Id::new(1)),
            id: Id::new(4),
            interaction: None,
            kind: MessageType::Regular,
            member: Some(PartialMember {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                joined_at,
                mute: false,
                nick: Some("member nick".to_owned()),
                permissions: None,
                premium_since: None,
                roles: Vec::new(),
                user: None,
            }),
            mention_channels: Vec::new(),
            mention_everyone: false,
            mention_roles: Vec::new(),
            mentions: Vec::new(),
            pinned: false,
            reactions: Vec::new(),
            reference: None,
            sticker_items: vec![MessageSticker {
                format_type: StickerFormatType::Png,
                id: Id::new(1),
                name: "sticker name".to_owned(),
            }],
            referenced_message: None,
            timestamp,
            thread: None,
            tts: false,
            webhook_id: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Message",
                    len: 18,
                },
                Token::Str("attachments"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("author"),
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
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("content"),
                Token::Str("ping"),
                Token::Str("edited_timestamp"),
                Token::None,
                Token::Str("embeds"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("flags"),
                Token::Some,
                Token::U64(0),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("type"),
                Token::U8(0),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "PartialMember",
                    len: 8,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("joined_at"),
                Token::Str("2020-01-01T00:00:00.000000+00:00"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("member nick"),
                Token::Str("permissions"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("user"),
                Token::None,
                Token::StructEnd,
                Token::Str("mention_everyone"),
                Token::Bool(false),
                Token::Str("mention_roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("mentions"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("pinned"),
                Token::Bool(false),
                Token::Str("sticker_items"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "MessageSticker",
                    len: 3,
                },
                Token::Str("format_type"),
                Token::U8(1),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("sticker name"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("timestamp"),
                Token::Str("2020-02-02T02:02:02.020000+00:00"),
                Token::Str("tts"),
                Token::Bool(false),
                Token::StructEnd,
            ],
        );

        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn message_deserialization_complete() -> Result<(), TimestampParseError> {
        let edited_timestamp = Timestamp::from_str("2021-08-10T12:41:51.602000+00:00")?;
        let joined_at = Timestamp::from_str("2020-01-01T00:00:00.000000+00:00")?;
        let timestamp = Timestamp::from_micros(1_580_608_922_020_000).expect("non zero");

        let value = Message {
            activity: Some(MessageActivity {
                kind: MessageActivityType::Join,
                party_id: None,
            }),
            application: Some(MessageApplication {
                cover_image: Some(image_hash::COVER),
                description: "a description".to_owned(),
                icon: Some(image_hash::ICON),
                id: Id::new(1),
                name: "application".to_owned(),
            }),
            application_id: Some(Id::new(1)),
            attachments: Vec::new(),
            author: User {
                accent_color: None,
                avatar: Some(image_hash::AVATAR),
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: Id::new(3),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
            channel_id: Id::new(2),
            components: Vec::new(),
            content: "ping".to_owned(),
            edited_timestamp: Some(edited_timestamp),
            embeds: Vec::new(),
            flags: Some(MessageFlags::empty()),
            guild_id: Some(Id::new(1)),
            id: Id::new(4),
            interaction: None,
            kind: MessageType::Regular,
            member: Some(PartialMember {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                joined_at,
                mute: false,
                nick: Some("member nick".to_owned()),
                permissions: None,
                premium_since: None,
                roles: Vec::new(),
                user: None,
            }),
            mention_channels: vec![ChannelMention {
                guild_id: Id::new(1),
                id: Id::new(2),
                kind: ChannelType::GuildText,
                name: "channel".to_owned(),
            }],
            mention_everyone: false,
            mention_roles: Vec::new(),
            mentions: Vec::new(),
            pinned: false,
            reactions: vec![MessageReaction {
                count: 7,
                emoji: ReactionType::Unicode {
                    name: "a".to_owned(),
                },
                me: true,
            }],
            reference: Some(MessageReference {
                channel_id: Some(Id::new(1)),
                guild_id: None,
                message_id: None,
                fail_if_not_exists: None,
            }),
            sticker_items: vec![MessageSticker {
                format_type: StickerFormatType::Png,
                id: Id::new(1),
                name: "sticker name".to_owned(),
            }],
            referenced_message: None,
            timestamp,
            thread: None,
            tts: false,
            webhook_id: Some(Id::new(1)),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Message",
                    len: 25,
                },
                Token::Str("activity"),
                Token::Some,
                Token::Struct {
                    name: "MessageActivity",
                    len: 1,
                },
                Token::Str("type"),
                Token::U8(1),
                Token::StructEnd,
                Token::Str("application"),
                Token::Some,
                Token::Struct {
                    name: "MessageApplication",
                    len: 5,
                },
                Token::Str("cover_image"),
                Token::Some,
                Token::Str(image_hash::COVER_INPUT),
                Token::Str("description"),
                Token::Str("a description"),
                Token::Str("icon"),
                Token::Some,
                Token::Str(image_hash::ICON_INPUT),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("application"),
                Token::StructEnd,
                Token::Str("application_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("attachments"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("author"),
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
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("content"),
                Token::Str("ping"),
                Token::Str("edited_timestamp"),
                Token::Some,
                Token::Str("2021-08-10T12:41:51.602000+00:00"),
                Token::Str("embeds"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("flags"),
                Token::Some,
                Token::U64(0),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("type"),
                Token::U8(0),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "PartialMember",
                    len: 8,
                },
                Token::Str("communication_disabled_until"),
                Token::None,
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("joined_at"),
                Token::Str("2020-01-01T00:00:00.000000+00:00"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("member nick"),
                Token::Str("permissions"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("user"),
                Token::None,
                Token::StructEnd,
                Token::Str("mention_channels"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "ChannelMention",
                    len: 4,
                },
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(0),
                Token::Str("name"),
                Token::Str("channel"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("mention_everyone"),
                Token::Bool(false),
                Token::Str("mention_roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("mentions"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("pinned"),
                Token::Bool(false),
                Token::Str("reactions"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "MessageReaction",
                    len: 3,
                },
                Token::Str("count"),
                Token::U64(7),
                Token::Str("emoji"),
                Token::Struct {
                    name: "ReactionType",
                    len: 1,
                },
                Token::Str("name"),
                Token::Str("a"),
                Token::StructEnd,
                Token::Str("me"),
                Token::Bool(true),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("message_reference"),
                Token::Some,
                Token::Struct {
                    name: "MessageReference",
                    len: 1,
                },
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::StructEnd,
                Token::Str("sticker_items"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "MessageSticker",
                    len: 3,
                },
                Token::Str("format_type"),
                Token::U8(1),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("sticker name"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("timestamp"),
                Token::Str("2020-02-02T02:02:02.020000+00:00"),
                Token::Str("tts"),
                Token::Bool(false),
                Token::Str("webhook_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
