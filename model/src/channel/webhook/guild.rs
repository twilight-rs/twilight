use crate::id::GuildId;
use serde::{Deserialize, Serialize};

/// Partial guild object that a webhook is following.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct WebhookGuild {
    pub icon: Option<String>,
    pub id: GuildId,
    pub name: String,
}
