use super::scheme::ShardScheme;
use crate::shard::{Config as ShardConfig, ResumeSession};
use std::{collections::HashMap, sync::Arc};
use twilight_gateway_queue::Queue;
use twilight_http::Client;

/// Built configuration for a [`Cluster`].
///
/// [`Cluster`]: crate::Cluster
#[derive(Debug)]
pub struct Config {
    pub(super) http_client: Client,
    pub(super) shard_config: ShardConfig,
    pub(super) shard_scheme: ShardScheme,
    pub(super) queue: Arc<Box<dyn Queue>>,
    pub(super) resume_sessions: HashMap<u64, ResumeSession>,
}

impl Config {
    /// Return an immutable reference to the `twilight_http` client used by the
    /// cluster and shards to get the gateway information.
    ///
    /// Refer to [`ClusterBuilder::http_client`] for the default value.
    ///
    /// [`ClusterBuilder::http_client`]: super::ClusterBuilder::http_client
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }

    /// Return an immutable reference to the configuration used to create
    /// shards.
    ///
    /// Refer to [`ShardBuilder`]'s methods for the default values.
    ///
    /// [`ShardBuilder`]: crate::shard::ShardBuilder#impl
    pub fn shard_config(&self) -> &ShardConfig {
        &self.shard_config
    }

    /// Return an immutable reference to the shard scheme used to start shards.
    ///
    /// Refer to [`ClusterBuilder::shard_scheme`] for the default value.
    ///
    /// [`ClusterBuilder::shard_scheme`]: super::ClusterBuilder::shard_scheme
    pub fn shard_scheme(&self) -> &ShardScheme {
        &self.shard_scheme
    }

    /// Return an immutable reference to the queue used for initiating shard
    /// sessions.
    pub fn queue(&self) -> &Arc<Box<dyn Queue>> {
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
