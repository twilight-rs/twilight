use crate::id::{marker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageDelete {
    pub channel_id: Id<marker::Channel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<marker::Guild>>,
    pub id: Id<marker::Message>,
}
