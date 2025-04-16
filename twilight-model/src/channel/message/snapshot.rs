use crate::{
    channel::Attachment,
    id::{marker::GuildMarker, Id},
    util::Timestamp,
};

use super::{Embed, MessageFlags};

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
    /// Content of the message snapshot.
    pub content: String,
    /// When the message was last edited.
    pub edited_timestamp: Option<Timestamp>,
    /// List of embeds from the message snapshot.
    pub embeds: Vec<Embed>,
    /// Flags of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    /// Timestamp of when the message was created.
    pub timestamp: Timestamp,
}

#[cfg(test)]
mod tests {
    use super::{MessageSnapshot, MessageSnapshotFields};
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
                content: "test".to_owned(),
                edited_timestamp: Some(Timestamp::from_secs(1_571_573_184).unwrap()),
                embeds: Vec::new(),
                flags: None,
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
                    len: 5,
                },
                Token::Str("attachments"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Attachment",
                    len: 8,
                },
                Token::Str("content_type"),
                Token::None,
                // Token::Str("ephemeral"),
                // Token::Bool(false),
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
