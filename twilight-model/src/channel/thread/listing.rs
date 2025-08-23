use crate::channel::{Channel, thread::ThreadMember};
use serde::{Deserialize, Serialize};

/// Response body returned in thread listing methods.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadsListing {
    /// Whether there are potentially more threads that could be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// A thread member object for each returned thread the current user has joined.
    pub members: Vec<ThreadMember>,
    /// List of threads.
    pub threads: Vec<Channel>,
}
