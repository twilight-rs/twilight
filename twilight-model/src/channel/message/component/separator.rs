use serde::{Deserialize, Serialize};

/// A separator is a layout component that adds vertical padding and
/// visual division between components.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Separator {
    /// Optional identifier for the separator.
    pub id: Option<i32>,
    /// Whether a visual divider should be shown. Defaults to `true`.
    pub divider: Option<bool>,
    /// The size of the separator padding. Defaults to `Small`.
    pub spacing: Option<SeparatorSpacingSize>,
}

/// The size of the separator padding.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum SeparatorSpacingSize {
    /// A small separator padding.
    Small,
    /// A large separator padding.
    Large,
    /// An arbitrary separator padding.
    Other(u8),
}

impl From<u8> for SeparatorSpacingSize {
    fn from(value: u8) -> Self {
        match value {
            1 => SeparatorSpacingSize::Small,
            2 => SeparatorSpacingSize::Large,
            other => SeparatorSpacingSize::Other(other),
        }
    }
}

impl Into<u8> for SeparatorSpacingSize {
    fn into(self) -> u8 {
        match self {
            SeparatorSpacingSize::Small => 1,
            SeparatorSpacingSize::Large => 2,
            SeparatorSpacingSize::Other(other) => other,
        }
    }
}
