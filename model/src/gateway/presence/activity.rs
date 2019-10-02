use crate::{
    gateway::presence::{
        ActivityAssets,
        ActivityFlags,
        ActivityParty,
        ActivitySecrets,
        ActivityTimestamps,
        ActivityType,
    },
    id::ApplicationId,
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Activity {
    pub application_id: Option<ApplicationId>,
    pub assets: Option<ActivityAssets>,
    pub details: Option<String>,
    pub flags: Option<ActivityFlags>,
    pub instance: Option<bool>,
    #[cfg_attr(
        feature = "serde-support",
        serde(default = "ActivityType::default", rename = "type")
    )]
    pub kind: ActivityType,
    pub name: String,
    pub party: Option<ActivityParty>,
    pub secrets: Option<ActivitySecrets>,
    pub state: Option<String>,
    pub timestamps: Option<ActivityTimestamps>,
    pub url: Option<String>,
}
