use crate::guild::Guild;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildCreate(pub Guild);
