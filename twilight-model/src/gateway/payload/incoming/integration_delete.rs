use crate::id::{
    marker::{ApplicationMarker, GuildMarker, IntegrationMarker},
    Id,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntegrationDelete {
    /// ID of the Bot/OAuth2 application for this integration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    /// ID of the guild.
    pub guild_id: Id<GuildMarker>,
    /// ID of the integration.
    pub id: Id<IntegrationMarker>,
}
