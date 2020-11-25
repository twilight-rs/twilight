pub mod sticker;

mod activity;
mod activity_type;
mod application;
mod flags;
mod kind;
mod mention;
mod reaction;
mod reference;

pub use self::{
    activity::MessageActivity, activity_type::MessageActivityType, application::MessageApplication,
    flags::MessageFlags, kind::MessageType, mention::Mention, reaction::MessageReaction,
    reference::MessageReference, sticker::Sticker,
};

use crate::{
    channel::{embed::Embed, Attachment, ChannelMention},
    guild::PartialMember,
    id::{ChannelId, GuildId, MessageId, RoleId, UserId, WebhookId},
    user::User,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Message {
    pub activity: Option<MessageActivity>,
    pub application: Option<MessageApplication>,
    pub attachments: Vec<Attachment>,
    pub author: User,
    pub channel_id: ChannelId,
    pub content: String,
    pub edited_timestamp: Option<String>,
    pub embeds: Vec<Embed>,
    pub flags: Option<MessageFlags>,
    pub guild_id: Option<GuildId>,
    pub id: MessageId,
    #[serde(rename = "type")]
    pub kind: MessageType,
    pub member: Option<PartialMember>,
    #[serde(default)]
    pub mention_channels: Vec<ChannelMention>,
    pub mention_everyone: bool,
    pub mention_roles: Vec<RoleId>,
    #[serde(with = "serde_mappable_seq")]
    pub mentions: HashMap<UserId, Mention>,
    pub pinned: bool,
    #[serde(default)]
    pub reactions: Vec<MessageReaction>,
    /// Reference data sent with crossposted messages and replies.
    #[serde(rename = "message_reference")]
    pub reference: Option<MessageReference>,
    /// The message associated with the [reference].
    ///
    /// [reference]: #structfield.reference
    pub referenced_message: Option<Box<Message>>,
    /// Stickers within the message.
    #[serde(default)]
    pub stickers: Vec<Sticker>,
    pub timestamp: String,
    pub tts: bool,
    pub webhook_id: Option<WebhookId>,
}

#[cfg(test)]
mod tests {
    use super::{
        sticker::{Sticker, StickerFormatType, StickerId, StickerPackId},
        Message, MessageFlags, MessageType,
    };
    use crate::{
        guild::PartialMember,
        id::{ChannelId, GuildId, MessageId, UserId},
        user::User,
    };
    use serde_test::Token;
    use std::collections::HashMap;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_message_deserialization() {
        let value = Message {
            activity: None,
            application: None,
            attachments: Vec::new(),
            author: User {
                avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
                bot: false,
                discriminator: "0001".to_owned(),
                email: None,
                flags: None,
                id: UserId(3),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
            channel_id: ChannelId(2),
            content: "ping".to_owned(),
            edited_timestamp: None,
            embeds: Vec::new(),
            flags: Some(MessageFlags::empty()),
            guild_id: Some(GuildId(1)),
            id: MessageId(4),
            kind: MessageType::Regular,
            member: Some(PartialMember {
                deaf: false,
                joined_at: Some("2020-01-01T00:00:00.000000+00:00".to_owned()),
                mute: false,
                nick: Some("member nick".to_owned()),
                premium_since: None,
                roles: Vec::new(),
            }),
            mention_channels: Vec::new(),
            mention_everyone: false,
            mention_roles: Vec::new(),
            mentions: HashMap::new(),
            pinned: false,
            reactions: Vec::new(),
            reference: None,
            stickers: vec![Sticker {
                asset: "foo1".to_owned(),
                description: "foo2".to_owned(),
                format_type: StickerFormatType::Png,
                id: StickerId(1),
                name: "sticker name".to_owned(),
                pack_id: StickerPackId(2),
                preview_asset: None,
                tags: Some("foo,bar,baz".to_owned()),
            }],
            referenced_message: None,
            timestamp: "2020-02-02T02:02:02.020000+00:00".to_owned(),
            tts: false,
            webhook_id: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Message",
                    len: 25,
                },
                Token::Str("activity"),
                Token::None,
                Token::Str("application"),
                Token::None,
                Token::Str("attachments"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("author"),
                Token::Struct {
                    name: "User",
                    len: 13,
                },
                Token::Str("avatar"),
                Token::Some,
                Token::Str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("email"),
                Token::None,
                Token::Str("flags"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::Str("locale"),
                Token::None,
                Token::Str("mfa_enabled"),
                Token::None,
                Token::Str("username"),
                Token::Str("test"),
                Token::Str("premium_type"),
                Token::None,
                Token::Str("public_flags"),
                Token::None,
                Token::Str("system"),
                Token::None,
                Token::Str("verified"),
                Token::None,
                Token::StructEnd,
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
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
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("4"),
                Token::Str("type"),
                Token::U8(0),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "PartialMember",
                    len: 6,
                },
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str("2020-01-01T00:00:00.000000+00:00"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("member nick"),
                Token::Str("premium_since"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
                Token::Str("mention_channels"),
                Token::Seq { len: Some(0) },
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
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("message_reference"),
                Token::None,
                Token::Str("referenced_message"),
                Token::None,
                Token::Str("stickers"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Sticker",
                    len: 8,
                },
                Token::Str("asset"),
                Token::Str("foo1"),
                Token::Str("description"),
                Token::Str("foo2"),
                Token::Str("format_type"),
                Token::U8(1),
                Token::Str("id"),
                Token::NewtypeStruct { name: "StickerId" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("sticker name"),
                Token::Str("pack_id"),
                Token::NewtypeStruct {
                    name: "StickerPackId",
                },
                Token::Str("2"),
                Token::Str("preview_asset"),
                Token::None,
                Token::Str("tags"),
                Token::Some,
                Token::Str("foo,bar,baz"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("timestamp"),
                Token::Str("2020-02-02T02:02:02.020000+00:00"),
                Token::Str("tts"),
                Token::Bool(false),
                Token::Str("webhook_id"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }
}
