use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum InteractionContextType {
    Guild,
    BotDm,
    PrivateChannel,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl InteractionContextType {
    pub const fn kind(self) -> &'static str {
        match self {
            Self::Guild => "GUILD",
            Self::BotDm => "BOT_DM",
            Self::PrivateChannel => "PRIVATE_CHANNEL",
            Self::Unknown(_) => "Unknown",
        }
    }
}

impl From<u8> for InteractionContextType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Guild,
            1 => Self::BotDm,
            2 => Self::PrivateChannel,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<InteractionContextType> for u8 {
    fn from(value: InteractionContextType) -> Self {
        match value {
            InteractionContextType::Guild => 0,
            InteractionContextType::BotDm => 1,
            InteractionContextType::PrivateChannel => 2,
            InteractionContextType::Unknown(unknown) => unknown,
        }
    }
}
