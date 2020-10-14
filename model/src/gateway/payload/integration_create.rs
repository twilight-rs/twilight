use crate::guild::IntegrationAccount;
use crate::guild::IntegrationApplication;
use crate::id::{GuildId, IntegrationId};
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntegrationCreate {
    user: User,
    #[serde(rename = "type")]
    kind: String,
    name: String,
    id: IntegrationId,
    enabled: bool,
    application: IntegrationApplication,
    account: IntegrationAccount,
    guild_id: GuildId,
}
