use serde::{Deserialize, Serialize};

use super::EmbedMediaFlags;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    /// Image's [media type].
    ///
    /// [media type]: https://en.wikipedia.org/wiki/Media_type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// A [thumbhash](https://evanw.github.io/thumbhash) placeholder of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// Version of the placeholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder_version: Option<u64>,
    /// Media flags for this video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<EmbedMediaFlags>,
}

#[cfg(test)]
mod tests {
    use super::EmbedImage;
    use super::EmbedMediaFlags;
    use serde_test::Token;

    #[test]
    fn embed_image() {
        let value = EmbedImage {
            height: Some(1440),
            proxy_url: Some("https://cdn.example.com/1-hash.png".to_owned()),
            url: "https://example.com/1.png".to_owned(),
            width: Some(2560),
            content_type: Some("image/png".to_owned()),
            // Thumbhash of the twilight project logo.
            placeholder: Some("zSeKDQIoCLiHeIKeiLyfrsBqCIaYaHJ2Vg".to_owned()),
            placeholder_version: Some(1),
            flags: Some(EmbedMediaFlags::IS_ANIMATED),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "EmbedImage",
                    len: 8,
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
                Token::Str("content_type"),
                Token::Some,
                Token::Str("image/png"),
                Token::Str("placeholder"),
                Token::Some,
                Token::Str("zSeKDQIoCLiHeIKeiLyfrsBqCIaYaHJ2Vg"),
                Token::Str("placeholder_version"),
                Token::Some,
                Token::U64(1),
                Token::Str("flags"),
                Token::Some,
                Token::U64(32),
                Token::StructEnd,
            ],
        );
    }
}
