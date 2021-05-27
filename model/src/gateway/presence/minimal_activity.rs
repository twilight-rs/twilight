use super::{Activity, ActivityType};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MinimalActivity {
    pub kind: ActivityType,
    pub name: String,
    pub url: Option<String>,
}

impl From<MinimalActivity> for Activity {
    fn from(minimal_activity: MinimalActivity) -> Self {
        Self {
            application_id: None,
            assets: None,
            buttons: Vec::new(),
            created_at: None,
            details: None,
            emoji: None,
            flags: None,
            id: None,
            instance: None,
            kind: minimal_activity.kind,
            name: minimal_activity.name,
            party: None,
            secrets: None,
            state: None,
            timestamps: None,
            url: minimal_activity.url,
        }
    }
}
