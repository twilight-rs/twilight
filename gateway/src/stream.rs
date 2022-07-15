//! Streaming utilities for initializing clusters of shards.

use crate::{error::ShardInitializeError, Config, Shard, ShardId};
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
/// # Panics
///
/// Panics if the lower end of the range is equal to the higher end of the
/// range or the total isn't greater than the lower or higher end of the range.
pub fn start_range(
    from: u64,
    to: u64,
    total: u64,
    config: Config,
) -> impl Stream<Item = Result<Shard, ShardInitializeError>> {
    assert!(from < to, "range start must be less than the end");
    assert!(from < total, "range start must be less than the total");
    assert!(to < total, "range end must be less than the total");

    let mut futures = Vec::new();

    for index in from..to {
        let id = ShardId::new(index, total);

        if index < to - 1 {
            futures.push(Shard::with_config(id, config.clone()));
        } else {
            futures.push(Shard::with_config(id, config));

            break;
        }
    }

    FuturesUnordered::from_iter(futures)
}

/// Start all of the shards recommended for Discord in a single cluster.
///
/// # Examples
///
/// Start all of the shards recommended by Discord and collect them into a map:
///
/// ```no_run
/// use futures::StreamExt;
/// use std::{collections::HashMap, env, future};
/// use twilight_gateway::{Config, Intents};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let token = env::var("DISCORD_TOKEN")?;
/// let config = Config::new(token, Intents::GUILDS);
///
/// let shards = twilight_gateway::stream::start_recommended(config)
///     .await?
///     .filter_map(|shard_result| async move {
///         shard_result.ok().map(|shard| (shard.id().current(), shard))
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
pub async fn start_recommended(
    config: Config,
) -> Result<impl Stream<Item = Result<Shard, ShardInitializeError>>, StartRecommendedError> {
    let info = Client::new(config.token().to_owned())
        .gateway()
        .authed()
        .exec()
        .await
        .map_err(|source| StartRecommendedError {
            kind: StartRecommendedErrorType::Request,
            source: Some(Box::new(source)),
        })?
        .model()
        .await
        .map_err(|source| StartRecommendedError {
            kind: StartRecommendedErrorType::Deserializing,
            source: Some(Box::new(source)),
        })?;

    Ok(start_range(0, info.shards - 1, info.shards, config))
}
