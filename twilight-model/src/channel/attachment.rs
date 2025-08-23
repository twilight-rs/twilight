use super::AttachmentFlags;
use crate::{
    id::{Id, marker::AttachmentMarker},
    util::is_false,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Attachment {
    /// Attachment's [media type].
    ///
    /// [media type]: https://en.wikipedia.org/wiki/Media_type
    pub content_type: Option<String>,
    /// Whether this attachment is ephemeral.
    ///
    /// Ephemeral attachments will automatically be removed after a set period
    /// of time. Ephemeral attachments on messages are guaranteed to be
    /// available as long as the message itself exists.
    #[serde(default, skip_serializing_if = "is_false")]
    pub ephemeral: bool,
    /// Duration of the audio file (currently for voice messages).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_secs: Option<f64>,
    pub filename: String,
    // Flags for this attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<AttachmentFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    pub id: Id<AttachmentMarker>,
    pub proxy_url: String,
    pub size: u64,
    /// The title of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub url: String,
    /// Base64 encoded bytearray representing a sampled waveform (currently for voice messages).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waveform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::Attachment;
    use crate::{channel::AttachmentFlags, id::Id};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(
        Attachment: content_type,
        ephemeral,
        filename,
        height,
        id,
        proxy_url,
        size,
        url,
        width
    );

    assert_impl_all!(
        Attachment: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Serialize
    );

    #[test]
    fn attachment() {
        let value = Attachment {
            content_type: Some("image/png".to_owned()),
            ephemeral: false,
            filename: "a.png".to_owned(),
            flags: Some(AttachmentFlags::IS_REMIX),
            description: Some("a image".to_owned()),
            duration_secs: Some(3.2),
            height: Some(184),
            id: Id::new(700_000_000_000_000_000),
            proxy_url: "https://cdn.example.com/1.png".to_owned(),
            size: 13_593,
            title: Some("a title".to_owned()),
            url: "https://example.com/1.png".to_owned(),
            waveform: Some(String::from("waveform")),
            width: Some(184),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Attachment",
                    len: 13,
                },
                Token::Str("content_type"),
                Token::Some,
                Token::Str("image/png"),
                Token::Str("duration_secs"),
                Token::Some,
                Token::F64(3.2),
                Token::Str("filename"),
                Token::Str("a.png"),
                Token::Str("flags"),
                Token::Some,
                Token::U64(4),
                Token::Str("description"),
                Token::Some,
                Token::Str("a image"),
                Token::Str("height"),
                Token::Some,
                Token::U64(184),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("700000000000000000"),
                Token::Str("proxy_url"),
                Token::Str("https://cdn.example.com/1.png"),
                Token::Str("size"),
                Token::U64(13_593),
                Token::Str("title"),
                Token::Some,
                Token::Str("a title"),
                Token::Str("url"),
                Token::Str("https://example.com/1.png"),
                Token::Str("waveform"),
                Token::Some,
                Token::Str("waveform"),
                Token::Str("width"),
                Token::Some,
                Token::U64(184),
                Token::StructEnd,
            ],
        );
    }
}
