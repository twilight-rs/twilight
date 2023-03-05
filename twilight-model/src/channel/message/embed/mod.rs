//! Rich message content.

#![allow(missing_docs)]
mod author;
mod field;
mod footer;
mod image;
mod provider;
mod thumbnail;
mod video;

pub use self::{
    author::EmbedAuthor, field::EmbedField, footer::EmbedFooter, image::EmbedImage,
    provider::EmbedProvider, thumbnail::EmbedThumbnail, video::EmbedVideo,
};

use crate::util::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<EmbedField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedThumbnail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedVideo>,
}

#[cfg(test)]
mod tests {
    use super::{
        Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedProvider, EmbedThumbnail,
        EmbedVideo,
    };
    use crate::util::datetime::{Timestamp, TimestampParseError};
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn embed() -> Result<(), TimestampParseError> {
        let timestamp = Timestamp::from_str("2021-08-02T16:56:43.772000+00:00")?;

        let value = Embed {
            author: None,
            color: Some(123),
            description: Some("a description".to_owned()),
            fields: Vec::new(),
            footer: None,
            image: None,
            kind: "rich".to_owned(),
            provider: None,
            thumbnail: None,
            timestamp: Some(timestamp),
            title: Some("a title".to_owned()),
            url: Some("https://example.com".to_owned()),
            video: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Embed",
                    len: 6,
                },
                Token::Str("color"),
                Token::Some,
                Token::U32(123),
                Token::Str("description"),
                Token::Some,
                Token::Str("a description"),
                Token::Str("type"),
                Token::Str("rich"),
                Token::Str("timestamp"),
                Token::Some,
                Token::Str("2021-08-02T16:56:43.772000+00:00"),
                Token::Str("title"),
                Token::Some,
                Token::Str("a title"),
                Token::Str("url"),
                Token::Some,
                Token::Str("https://example.com"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn embed_complete() -> Result<(), TimestampParseError> {
        let timestamp = Timestamp::from_str("2021-08-02T16:56:43.772000+00:00")?;

        let value = Embed {
            author: Some(EmbedAuthor {
                icon_url: Some("https://example.com/1.png".to_owned()),
                name: "test".to_owned(),
                proxy_icon_url: Some("https://example.com".to_owned()),
                url: Some("https://example.com".to_owned()),
            }),
            color: Some(123),
            description: Some("a description".to_owned()),
            fields: vec![EmbedField {
                inline: true,
                name: "name".to_owned(),
                value: "value".to_owned(),
            }],
            footer: Some(EmbedFooter {
                icon_url: Some("https://example.com/1.png".to_owned()),
                proxy_icon_url: Some("https://cdn.example.com/1-hash.png".to_owned()),
                text: "a footer".to_owned(),
            }),
            image: Some(EmbedImage {
                height: Some(1440),
                proxy_url: Some("https://cdn.example.com/1-hash.png".to_owned()),
                url: "https://example.com/1.png".to_owned(),
                width: Some(2560),
            }),
            kind: "rich".to_owned(),
            provider: Some(EmbedProvider {
                name: Some("Example".to_owned()),
                url: Some("https://example.com".to_owned()),
            }),
            thumbnail: Some(EmbedThumbnail {
                height: Some(1440),
                proxy_url: Some("https://cdn.example.com/1-hash.png".to_owned()),
                url: "https://example.com/1.png".to_owned(),
                width: Some(2560),
            }),
            timestamp: Some(timestamp),
            title: Some("a title".to_owned()),
            url: Some("https://example.com".to_owned()),
            video: Some(EmbedVideo {
                height: Some(1440),
                proxy_url: Some("https://proxy.cdn.example.com/1-hash.mp4".to_owned()),
                url: Some("https://cdn.example.com/1-hash.mp4".to_owned()),
                width: Some(2560),
            }),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Embed",
                    len: 13,
                },
                Token::Str("author"),
                Token::Some,
                Token::Struct {
                    name: "EmbedAuthor",
                    len: 4,
                },
                Token::Str("icon_url"),
                Token::Some,
                Token::Str("https://example.com/1.png"),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("proxy_icon_url"),
                Token::Some,
                Token::Str("https://example.com"),
                Token::Str("url"),
                Token::Some,
                Token::Str("https://example.com"),
                Token::StructEnd,
                Token::Str("color"),
                Token::Some,
                Token::U32(123),
                Token::Str("description"),
                Token::Some,
                Token::Str("a description"),
                Token::Str("fields"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "EmbedField",
                    len: 3,
                },
                Token::Str("inline"),
                Token::Bool(true),
                Token::Str("name"),
                Token::Str("name"),
                Token::Str("value"),
                Token::Str("value"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("footer"),
                Token::Some,
                Token::Struct {
                    name: "EmbedFooter",
                    len: 3,
                },
                Token::Str("icon_url"),
                Token::Some,
                Token::Str("https://example.com/1.png"),
                Token::Str("proxy_icon_url"),
                Token::Some,
                Token::Str("https://cdn.example.com/1-hash.png"),
                Token::Str("text"),
                Token::Str("a footer"),
                Token::StructEnd,
                Token::Str("image"),
                Token::Some,
                Token::Struct {
                    name: "EmbedImage",
                    len: 4,
                },
                Token::Str("height"),
                Token::Some,
                Token::U64(1440),
                Token::Str("proxy_url"),
                Token::Some,
                Token::Str("https://cdn.example.com/1-hash.png"),
                Token::Str("url"),
                Token::Str("https://example.com/1.png"),
                Token::Str("width"),
                Token::Some,
                Token::U64(2560),
                Token::StructEnd,
                Token::Str("type"),
                Token::Str("rich"),
                Token::Str("provider"),
                Token::Some,
                Token::Struct {
                    name: "EmbedProvider",
                    len: 2,
                },
                Token::Str("name"),
                Token::Some,
                Token::Str("Example"),
                Token::Str("url"),
                Token::Some,
                Token::Str("https://example.com"),
                Token::StructEnd,
                Token::Str("thumbnail"),
                Token::Some,
                Token::Struct {
                    name: "EmbedThumbnail",
                    len: 4,
                },
                Token::Str("height"),
                Token::Some,
                Token::U64(1440),
                Token::Str("proxy_url"),
                Token::Some,
                Token::Str("https://cdn.example.com/1-hash.png"),
                Token::Str("url"),
                Token::Str("https://example.com/1.png"),
                Token::Str("width"),
                Token::Some,
                Token::U64(2560),
                Token::StructEnd,
                Token::Str("timestamp"),
                Token::Some,
                Token::Str("2021-08-02T16:56:43.772000+00:00"),
                Token::Str("title"),
                Token::Some,
                Token::Str("a title"),
                Token::Str("url"),
                Token::Some,
                Token::Str("https://example.com"),
                Token::Str("video"),
                Token::Some,
                Token::Struct {
                    name: "EmbedVideo",
                    len: 4,
                },
                Token::Str("height"),
                Token::Some,
                Token::U64(1440),
                Token::Str("proxy_url"),
                Token::Some,
                Token::Str("https://proxy.cdn.example.com/1-hash.mp4"),
                Token::Str("url"),
                Token::Some,
                Token::Str("https://cdn.example.com/1-hash.mp4"),
                Token::Str("width"),
                Token::Some,
                Token::U64(2560),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
