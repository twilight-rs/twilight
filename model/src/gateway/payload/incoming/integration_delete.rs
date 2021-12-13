use crate::id::{marker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntegrationDelete {
    /// ID of the Bot/OAuth2 application for this integration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<marker::Application>>,
    /// ID of the guild.
    pub guild_id: Id<marker::Guild>,
    /// ID of the integration.
    pub id: Id<marker::Integration>,
}
