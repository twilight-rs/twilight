use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Well known voice server encryption modes.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "String", into = "Cow<'static, str>")]
pub enum EncryptionMode {
    /// `xsalsa20_poly1305_lite`.
    Lite,
    /// `xsalsa20_poly1305`.
    Normal,
    /// `xsalsa20_poly1305_suffix`.
    Suffix,
    /// Unknown encryption mode.
    Unknown(String),
}

impl From<EncryptionMode> for Cow<'static, str> {
    fn from(value: EncryptionMode) -> Self {
        match value {
            EncryptionMode::Lite => "xsalsa20_poly1305_lite".into(),
            EncryptionMode::Normal => "xsalsa20_poly1305".into(),
            EncryptionMode::Suffix => "xsalsa20_poly1305_suffix".into(),
            EncryptionMode::Unknown(unknown) => unknown.into(),
        }
    }
}

impl From<String> for EncryptionMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "xsalsa20_poly1305" => Self::Normal,
            "xsalsa20_poly1305_lite" => Self::Lite,
            "xsalsa20_poly1305_suffix" => Self::Suffix,
            _ => Self::Unknown(value),
        }
    }
}
