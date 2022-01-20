use super::scheme::ShardScheme;
use crate::shard::ResumeSession;
use std::{collections::HashMap, sync::Arc};
use twilight_gateway_queue::Queue;
use twilight_model::gateway::payload::outgoing::update_presence::UpdatePresencePayload;

/// Built configuration for a [`Cluster`].
///
/// [`Cluster`]: crate::Cluster
#[derive(Debug)]
pub struct Config {
    pub(super) queue: Arc<dyn Queue>,
    pub(super) resume_sessions: HashMap<u64, ResumeSession>,
    pub(super) shard_presences: HashMap<u64, UpdatePresencePayload>,
    pub(super) shard_scheme: ShardScheme,
}

impl Config {
    /// Return an immutable reference to the shard scheme used to start shards.
    ///
    /// Refer to [`ClusterBuilder::shard_scheme`] for the default value.
    ///
    /// [`ClusterBuilder::shard_scheme`]: super::ClusterBuilder::shard_scheme
    pub const fn shard_scheme(&self) -> &ShardScheme {
        &self.shard_scheme
    }

    /// Return an immutable reference to the queue used for initiating shard
    /// sessions.
    pub fn queue(&self) -> &Arc<dyn Queue> {
        &self.queue
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(Config: Debug, Send, Sync);
}
