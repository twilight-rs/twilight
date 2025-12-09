use serde::{Deserialize, Serialize};

/// Status indicating whether event webhooks are enabled or disabled for an application.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum EventWebhookStatus {
    /// Event webhooks are disabled.
    Disabled,
    /// Event webhooks are enabled.
    Enabled,
    /// Event webhooks have been disabled by Discord due to inactivity.
    DisabledByDiscord,
    Unknown(u8),
}

impl From<u8> for EventWebhookStatus {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Disabled,
            2 => Self::Enabled,
            3 => Self::DisabledByDiscord,
            _ => Self::Unknown(value),
        }
    }
}
