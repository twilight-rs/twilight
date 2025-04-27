use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Separator {
    pub id: Option<i32>,
    pub divider: Option<bool>,
    pub spacing: Option<SeparatorSpacingSize>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum SeparatorSpacingSize {
    Small,
    Large,
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
