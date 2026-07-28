use serde::{Deserialize, Serialize};

use super::EmbedMediaFlags;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    /// Video's [media type].
    ///
    /// [media type]: https://en.wikipedia.org/wiki/Media_type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// A [thumbhash](https://evanw.github.io/thumbhash) placeholder of the video.
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
    use super::EmbedMediaFlags;
    use super::EmbedVideo;
    use serde_test::Token;

    #[test]
    fn embed_video() {
        let value = EmbedVideo {
            height: Some(1440),
            proxy_url: Some("https://proxy.cdn.example.com/1-hash.mp4".to_owned()),
            url: Some("https://cdn.example.com/1-hash.mp4".to_owned()),
            width: Some(2560),
            content_type: Some("video/mp4".to_owned()),
            // The example value on the thumbhash website - picture of a green field.
            placeholder: Some("DCE71125807877787F888787784877788870FA3DC0".to_owned()),
            placeholder_version: Some(1),
            flags: Some(EmbedMediaFlags::IS_ANIMATED),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "EmbedVideo",
                    len: 8,
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
                Token::Str("content_type"),
                Token::Some,
                Token::Str("video/mp4"),
                Token::Str("placeholder"),
                Token::Some,
                Token::Str("DCE71125807877787F888787784877788870FA3DC0"),
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
