use crate::{
    id::{marker::AttachmentMarker, Id},
    util::is_false,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    pub id: Id<AttachmentMarker>,
    pub proxy_url: String,
    pub size: u64,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::Attachment;
    use crate::id::Id;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

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
        Eq,
        Hash,
        PartialEq,
        Serialize
    );

    #[test]
    fn test_attachment() {
        let value = Attachment {
            content_type: Some("image/png".to_owned()),
            ephemeral: false,
            filename: "a.png".to_owned(),
            description: Some("a image".to_owned()),
            height: Some(184),
            id: Id::new(700_000_000_000_000_000),
            proxy_url: "https://cdn.example.com/1.png".to_owned(),
            size: 13_593,
            url: "https://example.com/1.png".to_owned(),
            width: Some(184),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Attachment",
                    len: 9,
                },
                Token::Str("content_type"),
                Token::Some,
                Token::Str("image/png"),
                Token::Str("filename"),
                Token::Str("a.png"),
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
                Token::Str("url"),
                Token::Str("https://example.com/1.png"),
                Token::Str("width"),
                Token::Some,
                Token::U64(184),
                Token::StructEnd,
            ],
        );
    }
}
