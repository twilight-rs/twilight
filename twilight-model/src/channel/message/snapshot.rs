use crate::{
    channel::Attachment,
    id::{marker::GuildMarker, Id},
    util::Timestamp,
};

use super::{Component, Embed, Mention, MessageFlags, MessageSticker, MessageType};

use crate::id::marker::RoleMarker;
use serde::{Deserialize, Serialize};

/// The snap-shot of a message.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageSnapshot {
    /// Subset of fields in the message object.
    pub message: MessageSnapshotFields,
    /// ID of the origin message's guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
}

/// A subset of the fields for a message that has been snap-shotted.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MessageSnapshotFields {
    /// List of attachments from the message snapshot.
    pub attachments: Vec<Attachment>,
    /// Components in the message snapshot.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,
    /// Content of the message snapshot.
    pub content: String,
    /// When the message was last edited.
    pub edited_timestamp: Option<Timestamp>,
    /// List of embeds from the message snapshot.
    pub embeds: Vec<Embed>,
    /// Flags of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    /// Type of message.
    #[serde(rename = "type")]
    pub kind: MessageType,
    /// Users mentioned in the message snapshot.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mentions: Vec<Mention>,
    /// Roles mentioned in the message snapshot.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mention_roles: Vec<Id<RoleMarker>>,
    /// Stickers within the message snapshot.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sticker_items: Vec<MessageSticker>,
    /// Timestamp of when the message was created.
    pub timestamp: Timestamp,
}

#[cfg(test)]
mod tests {
    use super::{MessageSnapshot, MessageSnapshotFields};
    use crate::channel::message::component::{ActionRow, Button, ButtonStyle, ComponentType};
    use crate::channel::message::sticker::StickerFormatType;
    use crate::channel::message::{Component, MessageSticker, MessageType};
    use crate::{channel::Attachment, id::Id, util::Timestamp};
    use serde_test::Token;

    #[test]
    fn test_message_snapshot() {
        let value = MessageSnapshot {
            message: MessageSnapshotFields {
                attachments: vec![Attachment {
                    content_type: None,
                    description: None,
                    duration_secs: None,
                    ephemeral: false,
                    filename: "file.jpg".to_owned(),
                    flags: None,
                    height: Some(100),
                    id: Id::new(1),
                    proxy_url: "https://example.com".to_owned(),
                    size: 1000,
                    title: None,
                    url: "https://example.com".to_owned(),
                    waveform: None,
                    width: Some(100),
                }],
                components: vec![],
                content: "test".to_owned(),
                edited_timestamp: Some(Timestamp::from_secs(1_571_573_184).unwrap()),
                embeds: Vec::new(),
                kind: MessageType::Regular,
                flags: None,
                mentions: Vec::new(),
                mention_roles: Vec::new(),
                sticker_items: Vec::new(),
                timestamp: Timestamp::from_secs(1_571_573_184).unwrap(),
            },
            guild_id: Some(Id::new(1)),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageSnapshot",
                    len: 2,
                },
                Token::Str("message"),
                Token::Struct {
                    name: "MessageSnapshotFields",
                    len: 6,
                },
                Token::Str("attachments"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Attachment",
                    len: 8,
                },
                Token::Str("content_type"),
                Token::None,
                Token::Str("filename"),
                Token::Str("file.jpg"),
                Token::Str("height"),
                Token::Some,
                Token::U64(100),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("proxy_url"),
                Token::Str("https://example.com"),
                Token::Str("size"),
                Token::U64(1000),
                Token::Str("url"),
                Token::Str("https://example.com"),
                Token::Str("width"),
                Token::Some,
                Token::U64(100),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("content"),
                Token::Str("test"),
                Token::Str("edited_timestamp"),
                Token::Some,
                Token::Str("2019-10-20T12:06:24.000000+00:00"),
                Token::Str("embeds"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("type"),
                Token::U8(0),
                Token::Str("timestamp"),
                Token::Str("2019-10-20T12:06:24.000000+00:00"),
                Token::StructEnd,
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::StructEnd,
            ],
        );
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_message_snapshot_with_sticker_and_components() {
        let value = MessageSnapshot {
            message: MessageSnapshotFields {
                attachments: vec![Attachment {
                    content_type: None,
                    description: None,
                    duration_secs: None,
                    ephemeral: false,
                    filename: "file.jpg".to_owned(),
                    flags: None,
                    height: Some(100),
                    id: Id::new(1),
                    proxy_url: "https://example.com".to_owned(),
                    size: 1000,
                    title: None,
                    url: "https://example.com".to_owned(),
                    waveform: None,
                    width: Some(100),
                }],
                components: vec![Component::ActionRow(ActionRow {
                    components: Vec::from([Component::Button(Button {
                        custom_id: Some("button-1".to_owned()),
                        disabled: false,
                        emoji: None,
                        style: ButtonStyle::Primary,
                        label: Some("Button".to_owned()),
                        url: None,
                        sku_id: None,
                    })]),
                })],
                content: "test".to_owned(),
                edited_timestamp: Some(Timestamp::from_secs(1_571_573_184).unwrap()),
                embeds: Vec::new(),
                kind: MessageType::Regular,
                flags: None,
                mentions: Vec::new(),
                mention_roles: Vec::new(),
                sticker_items: vec![MessageSticker {
                    format_type: StickerFormatType::Png,
                    id: Id::new(1),
                    name: "sticker name".to_owned(),
                }],
                timestamp: Timestamp::from_secs(1_571_573_184).unwrap(),
            },
            guild_id: Some(Id::new(1)),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageSnapshot",
                    len: 2,
                },
                Token::Str("message"),
                Token::Struct {
                    name: "MessageSnapshotFields",
                    len: 8,
                },
                Token::Str("attachments"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Attachment",
                    len: 8,
                },
                Token::Str("content_type"),
                Token::None,
                Token::Str("filename"),
                Token::Str("file.jpg"),
                Token::Str("height"),
                Token::Some,
                Token::U64(100),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("proxy_url"),
                Token::Str("https://example.com"),
                Token::Str("size"),
                Token::U64(1000),
                Token::Str("url"),
                Token::Str("https://example.com"),
                Token::Str("width"),
                Token::Some,
                Token::U64(100),
                Token::StructEnd,
                Token::SeqEnd,
                Token::String("components"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Component",
                    len: 2,
                },
                Token::String("type"),
                Token::U8(ComponentType::ActionRow.into()),
                Token::String("components"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Component",
                    len: 4,
                },
                Token::String("type"),
                Token::U8(2),
                Token::String("custom_id"),
                Token::Some,
                Token::String("button-1"),
                Token::String("label"),
                Token::Some,
                Token::String("Button"),
                Token::String("style"),
                Token::U8(1),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("content"),
                Token::Str("test"),
                Token::Str("edited_timestamp"),
                Token::Some,
                Token::Str("2019-10-20T12:06:24.000000+00:00"),
                Token::Str("embeds"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("type"),
                Token::U8(0),
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
                Token::Str("2019-10-20T12:06:24.000000+00:00"),
                Token::StructEnd,
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::StructEnd,
            ],
        );
    }
}
