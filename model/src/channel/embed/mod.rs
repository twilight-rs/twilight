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

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Embed {
    pub author: Option<EmbedAuthor>,
    #[serde(default)]
    pub color: Option<u32>,
    pub description: Option<String>,
    #[serde(default)]
    pub fields: Vec<EmbedField>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    #[serde(rename = "type")]
    pub kind: String,
    pub provider: Option<EmbedProvider>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub timestamp: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub video: Option<EmbedVideo>,
}

#[cfg(test)]
mod tests {
    use super::Embed;
    use serde_test::Token;

    #[test]
    fn test_embed() {
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
            timestamp: Some("a timestamp".to_owned()),
            title: Some("a title".to_owned()),
            url: Some("https://example.com".to_owned()),
            video: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Embed",
                    len: 13,
                },
                Token::Str("author"),
                Token::None,
                Token::Str("color"),
                Token::Some,
                Token::U32(123),
                Token::Str("description"),
                Token::Some,
                Token::Str("a description"),
                Token::Str("fields"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("footer"),
                Token::None,
                Token::Str("image"),
                Token::None,
                Token::Str("type"),
                Token::Str("rich"),
                Token::Str("provider"),
                Token::None,
                Token::Str("thumbnail"),
                Token::None,
                Token::Str("timestamp"),
                Token::Some,
                Token::Str("a timestamp"),
                Token::Str("title"),
                Token::Some,
                Token::Str("a title"),
                Token::Str("url"),
                Token::Some,
                Token::Str("https://example.com"),
                Token::Str("video"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }
}
