use crate::{
    gateway::presence::{
        ActivityAssets, ActivityEmoji, ActivityFlags, ActivityParty, ActivitySecrets,
        ActivityTimestamps, ActivityType,
    },
    id::ApplicationId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Activity {
    pub application_id: Option<ApplicationId>,
    pub assets: Option<ActivityAssets>,
    // Introduced with custom statuses.
    pub created_at: Option<u64>,
    pub details: Option<String>,
    pub flags: Option<ActivityFlags>,
    // Introduced with custom statuses.
    pub id: Option<String>,
    pub instance: Option<bool>,
    #[serde(default = "ActivityType::default", rename = "type")]
    pub kind: ActivityType,
    pub name: String,
    pub emoji: Option<ActivityEmoji>,
    pub party: Option<ActivityParty>,
    pub secrets: Option<ActivitySecrets>,
    pub state: Option<String>,
    pub timestamps: Option<ActivityTimestamps>,
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    // Custom activities is tested by the custom presence test.
}
