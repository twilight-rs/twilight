use crate::guild::GuildIntegration;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntegrationUpdate(pub GuildIntegration);
