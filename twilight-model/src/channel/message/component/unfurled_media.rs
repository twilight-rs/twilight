use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Unfurled media item for use in components.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct UnfurledMediaItem {
    /// Results of Discord's content scanning, if the media item has been
    /// scanned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_scan_metadata: Option<ContentScanMetadata>,
    /// The media type of the content. This field is ignored and provided by the
    /// API as part of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The height of the media item. This field is ignored and provided by the
    /// API as part of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Option<u32>>,
    /// The proxied url of the media item. This field is ignored and provided
    /// by the API as part of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    /// Supports arbitrary urls and `attachment://<filename>` references.
    pub url: String,
    /// The width of the media item. This field is ignored and provided by the
    /// API as part of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Option<u32>>,
}

/// Results of Discord's content scanning, if the media item has been
/// scanned.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct ContentScanMetadata {
    /// Content scan metadata flags combined as a bitfield.
    pub flags: ContentScanMetadataFlags,
}

bitflags! {
    /// Content scan metadata flags.
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct ContentScanMetadataFlags: u64 {
        /// This media was flagged as explicit by Discord's content scanning.
        const IS_EXPLICIT_CONTENT = 1 << 0;
        /// This media was flagged as gore by Discord's content scanning.
        const IS_GORE_CONTENT = 1 << 1;
        /// This media was flagged as self-harm by Discord's content scanning.
        const IS_SELF_HARM_CONTENT = 1 << 2;
    }
}

impl<'de> Deserialize<'de> for ContentScanMetadataFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}

impl Serialize for ContentScanMetadataFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}
