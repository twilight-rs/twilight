use crate::id::ChannelId;
use serde::{Deserialize, Serialize};

/// Partial channel object that a webhook is following.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct WebhookChannel {
    pub id: ChannelId,
    pub name: String,
}
