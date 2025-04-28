use serde::{Deserialize, Serialize};

/// Unfurled media item for use in components.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct UnfurledMediaItem {
    /// Supports arbitrary urls and `attachment://<filename>` references.
    pub url: String,
    /// The proxied url of the media item. This field is ignored and provided
    /// by the API as part of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    /// The height of the media item. This field is ignored and provided by the
    /// API as part of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Option<u32>>,
    /// The width of the media item. This field is ignored and provided by the
    /// API as part of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Option<u32>>,
    /// The media type of the content. This field is ignored and provided by the
    /// API as part of the response.
    pub content_type: Option<String>,
}
