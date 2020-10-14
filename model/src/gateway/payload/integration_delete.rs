use crate::id::{ApplicationId, GuildId, IntegrationId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntegrationDelete {
    application_id: ApplicationId,
    guild_id: GuildId,
    id: IntegrationId,
}
