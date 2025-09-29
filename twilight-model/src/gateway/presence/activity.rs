use crate::{
    gateway::presence::{
        ActivityAssets, ActivityButton, ActivityEmoji, ActivityFlags, ActivityParty,
        ActivitySecrets, ActivityTimestamps, ActivityType,
    },
    id::{Id, marker::ApplicationMarker},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Activity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssets>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub buttons: Vec<ActivityButton>,
    /// Unix timestamp of when the activity was added to the user's session, in
    /// milliseconds.
    pub created_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<ActivityEmoji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ActivityFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<bool>,
    #[serde(default = "ActivityType::default", rename = "type")]
    pub kind: ActivityType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<ActivityParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ActivitySecrets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<ActivityTimestamps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(test)]
mod tests {
    // Custom activities is tested by the custom presence test.
}
