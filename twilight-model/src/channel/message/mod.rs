//! Textual user communication method.
#![warn(missing_docs)]

pub mod component;
pub mod embed;
pub mod sticker;

mod activity;
mod allowed_mentions;
mod application;
mod flags;
mod interaction;
mod kind;
mod mention;
mod reaction;
mod reference;
mod role_subscription_data;

pub use self::{
    activity::{MessageActivity, MessageActivityType},
    allowed_mentions::{AllowedMentions, MentionType},
    application::MessageApplication,
    component::Component,
    embed::Embed,
    flags::MessageFlags,
    interaction::MessageInteraction,
    kind::MessageType,
    mention::Mention,
    reaction::{Reaction, ReactionCountDetails, ReactionType},
    reference::MessageReference,
    role_subscription_data::RoleSubscriptionData,
    sticker::Sticker,
};

use self::sticker::MessageSticker;
use crate::{
    channel::{Attachment, Channel, ChannelMention},
    guild::PartialMember,
    id::{
        marker::{
            ApplicationMarker, ChannelMarker, GuildMarker, MessageMarker, RoleMarker, WebhookMarker,
        },
        Id,
    },
    user::User,
    util::Timestamp, application::interaction::InteractionMetadata,
};
use serde::{Deserialize, Serialize};

/// Text message sent in a [`Channel`].
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message {
    /// Present with Rich Presence-related chat embeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<MessageActivity>,
    /// Present with Rich Presence-related chat embeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<MessageApplication>,
    /// Associated application's ID.
    ///
    /// Present if the message is a response to an [`Interaction`].
    ///
    /// [`Interaction`]: crate::application::interaction::Interaction
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
    /// Author of the message.
    pub author: User,
    /// ID of the [`Channel`] the message was sent in.
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
    /// When the message was last edited.
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
    /// Flags of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    /// ID of the [`Guild`] the message was sent in.
    ///
    /// [`Guild`]: crate::guild::Guild
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Id of the message.
    pub id: Id<MessageMarker>,
    /// Interaction the message was sent as a response to.
    #[deprecated(note = "use interaction_metadata instead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction: Option<MessageInteraction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_metadata: Option<Box<InteractionMetadata>>,
    /// Type of message.
    #[serde(rename = "type")]
    pub kind: MessageType,
    /// Member properties of the [`author`].
    ///
    /// [`author`]: Message::author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialMember>,
    /// [`Channel`]s mentioned in the message.
    ///
    /// Note: only textual channels visible to everyone mentioned in crossposted
    /// messages (via channel following) will be included.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mention_channels: Vec<ChannelMention>,
    /// Whether the message mentions `@everyone`.
    pub mention_everyone: bool,
    /// [`Role`]s mentioned in the message.
    ///
    /// [`Role`]: crate::guild::Role
    pub mention_roles: Vec<Id<RoleMarker>>,
    /// Users mentioned in the message.
    pub mentions: Vec<Mention>,
    /// Whether the message is pinned.
    pub pinned: bool,
    /// List of reactions to the message.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reactions: Vec<Reaction>,
    /// Crosspost, channel follow add, pin and reply source message data.
    #[serde(rename = "message_reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<MessageReference>,
    /// The message associated with the [`reference`].
    ///
    /// [`reference`]: Self::reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_message: Option<Box<Message>>,
    /// Information about the role subscription purchase or renewal that
    /// prompted this message.
    ///
    /// Applies to [`RoleSubscriptionPurchase`] messages.
    ///
    /// [`RoleSubscriptionPurchase`]: MessageType::RoleSubscriptionPurchase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_subscription_data: Option<RoleSubscriptionData>,
    /// Stickers within the message.
    #[serde(default)]
    pub sticker_items: Vec<MessageSticker>,
    /// Timestamp of when the message was created.
    pub timestamp: Timestamp,
    /// Thread started from this message, includes [`Channel::member`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Channel>,
    /// Whether the message was a TTS message.
    pub tts: bool,
    /// ID of the webhook that generated the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<Id<WebhookMarker>>,
}

#[cfg(test)]
mod tests {
    use super::{
        reaction::ReactionCountDetails,
        sticker::{MessageSticker, StickerFormatType},
        Message, MessageActivity, MessageActivityType, MessageApplication, MessageFlags,
        MessageReference, MessageType, Reaction, ReactionType,
    };
    use crate::{
        channel::{ChannelMention, ChannelType},
        guild::{MemberFlags, PartialMember},
        id::Id,
        test::image_hash,
        user::User,
        util::{datetime::TimestampParseError, Timestamp},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn message_deserialization() {
        let joined_at = Some(Timestamp::from_str("2020-01-01T00:00:00.000000+00:00").unwrap());
        let timestamp = Timestamp::from_micros(1_580_608_922_020_000).expect("non zero");
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let value = Message {
            activity: None,
            application: None,
            application_id: None,
            attachments: Vec::new(),
            author: User {
                accent_color: None,
                avatar: Some(image_hash::AVATAR),
                avatar_decoration: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                global_name: Some("test".to_owned()),
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
                flags,
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
            role_subscription_data: None,
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
            interaction_metadata: None,
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
                    len: 9,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("avatar_decoration"),
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
                Token::Str("flags"),
                Token::U64(flags.bits()),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str("2020-01-01T00:00:00.000000+00:00"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("member nick"),
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
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn message_deserialization_complete() -> Result<(), TimestampParseError> {
        let edited_timestamp = Timestamp::from_str("2021-08-10T12:41:51.602000+00:00")?;
        let joined_at = Some(Timestamp::from_str("2020-01-01T00:00:00.000000+00:00")?);
        let timestamp = Timestamp::from_micros(1_580_608_922_020_000).expect("non zero");
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

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
                avatar_decoration: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                global_name: Some("test".to_owned()),
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
                flags,
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
            reactions: vec![Reaction {
                burst_colors: Vec::new(),
                count: 7,
                count_details: ReactionCountDetails {
                    burst: 0,
                    normal: 7,
                },
                emoji: ReactionType::Unicode {
                    name: "a".to_owned(),
                },
                me: true,
                me_burst: false,
            }],
            reference: Some(MessageReference {
                channel_id: Some(Id::new(1)),
                guild_id: None,
                message_id: None,
                fail_if_not_exists: None,
            }),
            role_subscription_data: None,
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
            interaction_metadata: None,
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
                    len: 9,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("avatar_decoration"),
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
                Token::Str("flags"),
                Token::U64(flags.bits()),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str("2020-01-01T00:00:00.000000+00:00"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("member nick"),
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
                    name: "Reaction",
                    len: 6,
                },
                Token::Str("burst_colors"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("count"),
                Token::U64(7),
                Token::Str("count_details"),
                Token::Struct {
                    name: "ReactionCountDetails",
                    len: 2,
                },
                Token::Str("burst"),
                Token::U64(0),
                Token::Str("normal"),
                Token::U64(7),
                Token::StructEnd,
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
                Token::Str("me_burst"),
                Token::Bool(false),
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
