//! Streaming utilities for initializing clusters of shards.

use crate::{
    config::{Config, ShardId},
    error::ShardInitializeError,
    Shard,
};
use futures_util::stream::{FuturesUnordered, Stream};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_http::Client;

#[derive(Debug)]
pub struct StartRecommendedError {
    pub(crate) kind: StartRecommendedErrorType,
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl Display for StartRecommendedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
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
    Deserializing,
    Request,
}

pub fn start_range(
    from: u64,
    to: u64,
    total: u64,
    config: Config,
) -> impl Stream<Item = Result<Shard, ShardInitializeError>> {
    // todo document panics
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

    #[allow(clippy::from_iter_instead_of_collect)]
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
/// use std::env;
/// use twilight_gateway::{Config, Intents};
///
/// let token = env::var("DISCORD_TOKEN")?;
/// let config = Config::new(token, Intents::GUILDS);
///
/// let shards = twilight_gateway::stream::start_recommended(config)
///     .await?
///     .filter_map(|shard_result| {
///         let shard = shard_result?;
///
///         (shard.id(), shard)
///     })
///     .collect::<HashMap<_, _>>()
///     .await;
///
/// println!("total shards: {}", shards);
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
    // todo drop twilight_http dependency
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
