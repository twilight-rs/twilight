use super::builder::ShardScheme;
use crate::{
    queue::Queue,
    shard::{Config as ShardConfig, ResumeSession},
};
use std::{collections::HashMap, sync::Arc};
use twilight_http::Client;

/// Built configuration for a [`Cluster`].
///
/// [`Cluster`]: struct.Cluster.html
#[derive(Debug)]
pub struct Config {
    pub(super) http_client: Client,
    pub(super) shard_config: ShardConfig,
    pub(super) shard_scheme: ShardScheme,
    pub(super) queue: Arc<Box<dyn Queue>>,
    pub(super) resume_sessions: HashMap<u64, ResumeSession>,
}

impl Config {
    /// Returns the `twilight_http` client used by the cluster and shards to get the
    /// gateway information.
    ///
    /// Refer to [`ClusterConfigBuilder::http_client`] for the default value.
    ///
    /// [`ClusterConfigBuilder::http_client`]: struct.ClusterConfigBuilder.html#method.http_client
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }

    /// Returns the configuration used to create shards.
    ///
    /// Refer to [`shard::config::ClusterConfigBuilder`]'s methods for the default values.
    ///
    /// [`shard::config::ClusterConfigBuilder`]: ../../shard/config/struct.ShardConfigBuilder.html#methods
    pub fn shard_config(&self) -> &ShardConfig {
        &self.shard_config
    }

    /// Returns the shard scheme used to start shards.
    ///
    /// Refer to [`ClusterConfigBuilder::shard_scheme`] for the default value.
    ///
    /// [`ClusterConfigBuilder::shard_scheme`]: struct.ClusterConfigBuilder.html#method.shard_scheme
    pub fn shard_scheme(&self) -> ShardScheme {
        self.shard_scheme
    }

    /// Return an immutable reference to the queue used for initiating shard
    /// sessions.
    pub fn queue(&self) -> &Arc<Box<dyn Queue>> {
        &self.queue
    }
}
