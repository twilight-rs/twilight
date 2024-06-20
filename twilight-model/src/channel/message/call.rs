use crate::{
    id::{marker::UserMarker, Id},
    util::Timestamp,
};

use serde::{Deserialize, Serialize};

/// Information about the call in a private channel.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageCall {
    /// The timestamp of when the call ended.
    pub ended_timestamp: Option<Timestamp>,
    /// The IDs of the users that participated in the call.
    #[serde(default)]
    pub participants: Vec<Id<UserMarker>>,
}
