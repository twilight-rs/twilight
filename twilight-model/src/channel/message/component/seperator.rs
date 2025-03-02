use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Separator {
    pub id: Option<i32>,
    pub divider: Option<bool>,
    pub spacing: Option<SeperatorSpacingSize>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum SeperatorSpacingSize {
    Small,
    Large,
    Other(u8),
}

impl From<u8> for SeperatorSpacingSize {
    fn from(value: u8) -> Self {
        match value {
            1 => SeperatorSpacingSize::Small,
            2 => SeperatorSpacingSize::Large,
            other => SeperatorSpacingSize::Other(other),
        }
    }
}

impl Into<u8> for SeperatorSpacingSize {
    fn into(self) -> u8 {
        match self {
            SeperatorSpacingSize::Small => 1,
            SeperatorSpacingSize::Large => 2,
            SeperatorSpacingSize::Other(other) => other,
        }
    }
}
