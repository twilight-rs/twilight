//! Streaming utilities for initializing groups of shards.

use crate::{error::ShardInitializeError, tls::TlsContainer, Config, Shard, ShardId};
use futures_util::stream::{FuturesUnordered, Stream};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_http::Client;

/// Failure when fetching the recommended number of shards to use from Discord's
/// REST API.
#[derive(Debug)]
pub struct StartRecommendedError {
    /// Type of error.
    pub(crate) kind: StartRecommendedErrorType,
    /// Source error if available.
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl Display for StartRecommendedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            StartRecommendedErrorType::Deserializing => {
                f.write_str("payload isn't a recognized type")
            }
            StartRecommendedErrorType::Request => f.write_str("request failed to complete"),
        }
    }
}

impl Error for StartRecommendedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`StartRecommendedError`] that occurred.
#[derive(Debug)]
pub enum StartRecommendedErrorType {
    /// Received gateway event failed to be deserialized.
    ///
    /// The message payload is likely an unrecognized type that is not yet
    /// supported.
    Deserializing,
    /// Requesting recommended shards from Discord's REST API failed.
    ///
    /// May be due to something such as a network or authentication issue.
    Request,
}

/// Start a range of shards with provided configuration for each shard.
///
/// Lower end of the range must be less than the higher end. The higher end of
/// the range is exclusive.
///
/// Shards will all share the same TLS connector to reduce memory usage.
///
/// # Panics
///
/// Panics if the lower end of the range is equal to the higher end of the
/// range or the total isn't greater than the lower or higher end of the range.
///
/// Panics if loading TLS certificates fails.
pub fn start_range<F: Fn(ShardId) -> Config>(
    from: u64,
    to: u64,
    total: u64,
    per_shard_config: F,
) -> impl Stream<Item = Result<Shard, ShardInitializeError>> + Send + 'static {
    assert!(from < to, "range start must be less than the end");
    assert!(from < total, "range start must be less than the total");
    assert!(to < total, "range end must be less than the total");

    let capacity = (to - from).try_into().unwrap_or_default();
    let mut futures = Vec::with_capacity(capacity);
    let tls = TlsContainer::new().unwrap();

    for index in from..to {
        let id = ShardId::new(index, total);
        let mut config = per_shard_config(id);
        config.set_tls(tls.clone());
        futures.push(Shard::with_config(id, config));

        if index < to - 1 {
            break;
        }
    }

    FuturesUnordered::from_iter(futures)
}

/// Start all of the shards recommended for Discord in a single group.
///
/// Shards will all share the same TLS connector to reduce memory usage.
///
/// # Examples
///
/// Start all of the shards recommended by Discord and collect them into a map:
///
/// ```no_run
/// use futures::StreamExt;
/// use std::{collections::HashMap, env, future};
/// use twilight_gateway::{stream, Config, Intents};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let token = env::var("DISCORD_TOKEN")?;
///
/// // callback to create a config for each shard, useful for when not all shards
/// // have the same configuration, such as for per-shard presences
/// let config_callback = |_| Config::new(token.clone(), Intents::GUILDS);
///
/// let shards = stream::start_recommended(token.clone(), config_callback)
///     .await?
///     .filter_map(|shard_result| async move {
///         shard_result.ok().map(|shard| (shard.id().number(), shard))
///     })
///     .collect::<HashMap<_, _>>()
///     .await;
///
/// println!("total shards: {}", shards.len());
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns a [`StartRecommendedErrorType::Deserializing`] error type if the
/// response body failed to deserialize.
///
/// Returns a [`StartRecommendedErrorType::Request`] error type if the request
/// failed to complete.
pub async fn start_recommended<F: Fn(ShardId) -> Config>(
    token: String,
    per_shard_config: F,
) -> Result<impl Stream<Item = Result<Shard, ShardInitializeError>> + Send, StartRecommendedError> {
    let client = Client::new(token);
    let request = client.gateway().authed();
    let response = request
        .exec()
        .await
        .map_err(|source| StartRecommendedError {
            kind: StartRecommendedErrorType::Request,
            source: Some(Box::new(source)),
        })?;
    let info = response
        .model()
        .await
        .map_err(|source| StartRecommendedError {
            kind: StartRecommendedErrorType::Deserializing,
            source: Some(Box::new(source)),
        })?;

    Ok(start_range(
        0,
        info.shards - 1,
        info.shards,
        per_shard_config,
    ))
}
