//! Streaming utilities for initializing groups of shards.
//!
//! There are two groups of functionality to the stream module: initializers and
//! selectors. The initializers are functions like [`start_recommended`], which
//! initializes a group of shards based on Discord's recommendation. Once these
//! shards are initialized, the events or websocket messages of all of the
//! shards can be collected into an efficient stream via [`ShardEventStream`]
//! and [`ShardMessageStream`].

use crate::{
    error::{ReceiveMessageError, ShardInitializeError},
    message::Message,
    tls::TlsContainer,
    Config, Shard, ShardId,
};
use futures_util::stream::{FuturesUnordered, Stream, StreamExt};
use std::{
    cell::RefCell,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    future::Future,
    ops::{Bound, Deref, DerefMut, Range, RangeBounds},
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};
use twilight_http::Client;
use twilight_model::gateway::event::Event;

/// Generic list of unordered futures producing an item for each shard.
type FutureList<'a, Item> =
    FuturesUnordered<Pin<Box<dyn Future<Output = NextItemOutput<'a, Item>> + 'a>>>;

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

/// Stream selecting the next gateway event from a group of shards.
///
/// # Examples
///
/// Create the recommended number of shards and stream over their events:
///
/// ```no_run
/// use futures::StreamExt;
/// use std::{collections::HashMap, env, future};
/// use twilight_gateway::{
///     stream::{self, ShardEventStream},
///     Config, Intents,
/// };
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let token = env::var("DISCORD_TOKEN")?;
/// let client = Client::new(token.clone());
///
/// // callback to create a config for each shard, useful for when not all shards
/// // have the same configuration, such as for per-shard presences
/// let config_callback = |_| Config::new(token.clone(), Intents::GUILDS);
///
/// let mut shards = stream::start_recommended(&client, config_callback)
///     .await?
///     .filter_map(|shard_result| async move { shard_result.ok() })
///     .collect::<Vec<_>>()
///     .await;
///
/// let mut stream = ShardEventStream::new(shards.iter_mut());
///
/// loop {
///     let (shard, event) = match stream.next().await {
///         Some((shard, Ok(event))) => (shard, event),
///         Some((shard, Err(source))) => {
///             tracing::warn!(?source, "error receiving event");
///
///             if source.is_fatal() {
///                 break;
///             }
///
///             continue;
///         }
///         None => break,
///     };
///
///     println!("received event on shard {}: {event:?}", shard.id());
/// }
/// # Ok(()) }
/// ```
pub struct ShardEventStream<'a> {
    /// Set of futures resolving to the next event of each shard.
    futures: Rc<RefCell<FutureList<'a, Event>>>,
}

impl<'a> ShardEventStream<'a> {
    /// Create a new stream producing events from a set of shards.
    pub fn new(shards: impl Iterator<Item = &'a mut Shard>) -> Self {
        let mut this = Self {
            futures: Rc::new(RefCell::new(FuturesUnordered::new())),
        };

        for shard in shards {
            this.add_shard(shard);
        }

        this
    }

    /// Add a shard to the stream to produce a gateway event.
    fn add_shard(&mut self, shard: &'a mut Shard) {
        self.futures.borrow_mut().push(Box::pin(async {
            let result = shard.next_event().await;

            NextItemOutput { result, shard }
        }));
    }
}

impl<'a> Stream for ShardEventStream<'a> {
    type Item = (ShardRef<'a>, Result<Event, ReceiveMessageError>);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.as_mut();
        let poll = this.futures.borrow_mut().poll_next_unpin(cx);

        match poll {
            Poll::Ready(Some(output)) => Poll::Ready(Some((
                ShardRef {
                    list: ShardList::Events(Rc::clone(&this.futures)),
                    shard: Some(output.shard),
                },
                output.result,
            ))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Stream selecting the next websocket message from a group of shards.
///
/// # Examples
///
/// Create the recommended number of shards and stream over their messages:
///
/// ```no_run
/// use futures::StreamExt;
/// use std::{collections::HashMap, env, future};
/// use twilight_gateway::{
///     stream::{self, ShardMessageStream},
///     Config, Intents,
/// };
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let token = env::var("DISCORD_TOKEN")?;
/// let client = Client::new(token.clone());
///
/// // callback to create a config for each shard, useful for when not all shards
/// // have the same configuration, such as for per-shard presences
/// let config_callback = |_| Config::new(token.clone(), Intents::GUILDS);
///
/// let mut shards = stream::start_recommended(&client, config_callback)
///     .await?
///     .filter_map(|shard_result| async move { shard_result.ok() })
///     .collect::<Vec<_>>()
///     .await;
///
/// let mut stream = ShardMessageStream::new(shards.iter_mut());
///
/// loop {
///     let (shard, message) = match stream.next().await {
///         Some((shard, Ok(message))) => (shard, message),
///         Some((shard, Err(source))) => {
///             tracing::warn!(?source, "error receiving message");
///
///             if source.is_fatal() {
///                 break;
///             }
///
///             continue;
///         }
///         None => break,
///     };
///
///     println!("received message on shard {}: {message:?}", shard.id());
/// }
/// # Ok(()) }
/// ```
pub struct ShardMessageStream<'a> {
    /// Set of futures resolving to the next message of each shard.
    futures: Rc<RefCell<FutureList<'a, Message>>>,
}

impl<'a> ShardMessageStream<'a> {
    /// Create a new stream producing websocket messages from a set of shards.
    pub fn new(shards: impl Iterator<Item = &'a mut Shard>) -> Self {
        let mut this = Self {
            futures: Rc::new(RefCell::new(FuturesUnordered::new())),
        };

        for shard in shards {
            this.add_shard(shard);
        }

        this
    }

    /// Add a shard to the stream to produce a websocket message.
    fn add_shard(&mut self, shard: &'a mut Shard) {
        self.futures.borrow_mut().push(Box::pin(async {
            let result = shard.next_message().await;

            NextItemOutput { result, shard }
        }));
    }
}

impl<'a> Stream for ShardMessageStream<'a> {
    type Item = (ShardRef<'a>, Result<Message, ReceiveMessageError>);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.as_mut();
        let poll = this.futures.borrow_mut().poll_next_unpin(cx);

        match poll {
            Poll::Ready(Some(output)) => Poll::Ready(Some((
                ShardRef {
                    list: ShardList::Messages(Rc::clone(&this.futures)),
                    shard: Some(output.shard),
                },
                output.result,
            ))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Guard dereferencing to the shard that produced an event or message.
///
/// Note that manually causing the destructor to [not be called] will cause the
/// shard to not be re-inserted into the stream.
///
/// [not be called]: std::mem::forget
pub struct ShardRef<'a> {
    /// List of futures the shard will be re-inserted into when the reference is
    /// dropped.
    list: ShardList<'a>,
    /// Mutable reference to the shard that produced an event or message.
    shard: Option<&'a mut Shard>,
}

impl<'a> Deref for ShardRef<'a> {
    type Target = Shard;

    fn deref(&self) -> &Self::Target {
        self.shard.as_ref().unwrap()
    }
}

impl<'a> DerefMut for ShardRef<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.shard.as_mut().unwrap()
    }
}

impl Drop for ShardRef<'_> {
    fn drop(&mut self) {
        if let Some(shard) = self.shard.take() {
            match &mut self.list {
                ShardList::Events(event_list) => {
                    event_list.borrow_mut().push(Box::pin(async {
                        let result = shard.next_event().await;

                        NextItemOutput { result, shard }
                    }));
                }
                ShardList::Messages(message_list) => {
                    message_list.borrow_mut().push(Box::pin(async {
                        let result = shard.next_message().await;

                        NextItemOutput { result, shard }
                    }));
                }
            }
        }
    }
}

/// List of futures for receiving the next event or message of shards.
enum ShardList<'a> {
    /// List of futures for receiving the next event of shards.
    Events(Rc<RefCell<FutureList<'a, Event>>>),
    /// List of futures for receiving the next message of shards.
    Messages(Rc<RefCell<FutureList<'a, Message>>>),
}

/// Output of a stream, such as [`ShardMessageStream`].
struct NextItemOutput<'a, Item> {
    /// Result of the future.
    result: Result<Item, ReceiveMessageError>,
    /// Shard that produced the result.
    shard: &'a mut Shard,
}

/// Start a cluster with provided configuration for each shard.
///
/// Shards will all share the same TLS connector to reduce memory usage.
///
/// # Panics
///
/// Panics if the lower end of the range is equal to the higher end of the
/// range or the total isn't greater than the lower or higher end of the range.
///
/// Panics if the concurrency doesn't fit into a usize.
///
/// Panics if loading TLS certificates fails.
#[track_caller]
pub fn start_cluster<F: Fn(ShardId) -> Config>(
    bucket_id: u64,
    concurrency: u64,
    total: u64,
    per_shard_config: F,
) -> impl Stream<Item = Result<Shard, ShardInitializeError>> + Send + 'static {
    assert!(bucket_id < total, "bucket id must be less than the total");
    assert!(
        concurrency < total,
        "concurrency must be less than the total"
    );

    let concurrency = concurrency.try_into().unwrap();
    let tls = TlsContainer::new().unwrap();

    (bucket_id..total)
        .step_by(concurrency)
        .map(|index| {
            let id = ShardId::new(index, total);
            let mut config = per_shard_config(id);
            config.set_tls(tls.clone());

            Shard::with_config(id, config)
        })
        .collect::<FuturesUnordered<_>>()
}

/// Start a range of shards with provided configuration for each shard.
///
/// Lower end of the range must be less than the higher end.
///
/// Shards will all share the same TLS connector to reduce memory usage.
///
/// # Panics
///
/// Panics if the start is more than the end, the start is more than the total,
/// or the end is more than the total.
///
/// Panics if loading TLS certificates fails.
#[track_caller]
pub fn start_range<F: Fn(ShardId) -> Config>(
    range: impl RangeBounds<u64>,
    total: u64,
    per_shard_config: F,
) -> impl Stream<Item = Result<Shard, ShardInitializeError>> + Send + 'static {
    let range = calculate_range(range, total);
    let tls = TlsContainer::new().unwrap();

    range
        .map(|index| {
            let id = ShardId::new(index, total);
            let mut config = per_shard_config(id);
            config.set_tls(tls.clone());

            Shard::with_config(id, config)
        })
        .collect::<FuturesUnordered<_>>()
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
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let token = env::var("DISCORD_TOKEN")?;
/// let client = Client::new(token.clone());
///
/// // callback to create a config for each shard, useful for when not all shards
/// // have the same configuration, such as for per-shard presences
/// let config_callback = |_| Config::new(token.clone(), Intents::GUILDS);
///
/// let shards = stream::start_recommended(&client, config_callback)
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
///
/// # Panics
///
/// Panics if loading TLS certificates fails.
#[track_caller]
pub async fn start_recommended<F: Fn(ShardId) -> Config>(
    client: &Client,
    per_shard_config: F,
) -> Result<impl Stream<Item = Result<Shard, ShardInitializeError>> + Send, StartRecommendedError> {
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

    Ok(start_range(.., info.shards, per_shard_config))
}

/// Transform any range into a sized range based on the total.
///
/// # Panics
///
/// Panics if the start is more than the end, the start is more than the total,
/// or the end is more than the total.
fn calculate_range(range: impl RangeBounds<u64>, total: u64) -> Range<u64> {
    // 0, or the provided start bound (inclusive).
    let start = match range.start_bound() {
        Bound::Excluded(from) => *from + 1,
        Bound::Included(from) => *from,
        Bound::Unbounded => 0,
    };

    // Total, or the provided end bound (exclusive).
    let end = match range.end_bound() {
        Bound::Excluded(to) => *to,
        Bound::Included(to) => *to + 1,
        Bound::Unbounded => total,
    };

    assert!(start < end, "range start must be less than the end");
    assert!(start < total, "range start must be less than the total");
    assert!(end <= total, "range end must be less than the total");

    start..end
}
