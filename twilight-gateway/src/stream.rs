//! Utilities for managing collections of shards.
//!
//! Multiple shards may easily be created at once, with a per shard config
//! created from a `Fn(ShardId) -> Config` closure, with the help of the
//! `create_` set of functions. These functions will also reuse shards' TLS
//! context, something otherwise achieved by cloning an existing [`Config`], but
//! will not by default set a shared [session queue] (see
//! [`ConfigBuilder::queue`]).
//!
//! # Concurrency
//!
//! Multiple shards' events or websocket messages may be concurrently streamed
//! via [`ShardEventStream`] or [`ShardMessageStream`] respectively, returning a
//! mutable reference to the yielded shard and its item. The yielded shard is
//! later returned to the stream in its [`Drop`] implementation.
//!
//! This is the recommended way to run bots with multiple shards.
//!
//! # Parallelism
//!
//! A multi-threaded executor is able to run tasks in parallel, but splitting
//! shards across tasks requires channels to communicate between them, for
//! example, to coordinate shutdowns or to collect all events to a single event
//! handler task and is therefore **not** recommended unless it's required for
//! performance reasons (a single core should, on a reasonably performant CPU,
//! be capable of handling tens of thousands of Discord events per second).
//!
//! ## Example
//!
//! Run the recommended number of shards in `available_parallelism` tasks. Note
//! that neither [`ShardEventStream`] nor [`ShardMessageStream`] is required for
//! parallelism; each shard can run on an independent task.
//!
//! ```no_run
//! use futures::{future::join_all, StreamExt};
//! use std::{env, iter, sync::Arc, thread, time::Duration};
//! use tokio::{signal, sync::watch, time};
//! use twilight_gateway::{
//!     message::CloseFrame,
//!     queue::LocalQueue,
//!     stream::{self, ShardEventStream},
//!     Config, Intents, Shard,
//! };
//! use twilight_http::Client;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let token = env::var("DISCORD_TOKEN")?;
//!     let client = Client::new(token.clone());
//!
//!     let queue = Arc::new(LocalQueue::new());
//!     // callback to create a config for each shard, useful for when not all
//!     // shards have the same configuration, such as for per-shard presences
//!     let config_callback = |_| {
//!         Config::builder(token.clone(), Intents::GUILDS)
//!             .queue(queue.clone())
//!             .build()
//!     };
//!
//!     let tasks = thread::available_parallelism()?.get();
//!
//!     // split shards into a vec of `tasks` vecs of shards
//!     let init = iter::repeat_with(|| Vec::new())
//!         .take(tasks)
//!         .collect::<Vec<Vec<_>>>();
//!     let shards = stream::create_recommended(&client, config_callback)
//!         .await?
//!         .enumerate()
//!         .fold(init, |mut fold, (idx, shard)| {
//!             fold[idx % tasks].push(shard);
//!             fold
//!         });
//!
//!     let (tx, rx) = watch::channel(false);
//!
//!     for mut shards in shards {
//!         let mut rx = rx.clone();
//!         tokio::spawn(async move {
//!             // run `process` and `rx.changed()` concurrently, returning when
//!             // the first branch completes, cancelling the other one
//!             tokio::select! {
//!                 _ = process(shards.iter_mut()) => {},
//!                 _ = rx.changed() => {
//!                     join_all(shards.iter_mut().map(|shard| async move {
//!                         shard.close(CloseFrame::NORMAL).await
//!                     })).await;
//!                 }
//!             }
//!         });
//!     }
//!
//!     signal::ctrl_c().await?;
//!
//!     // instruct the tasks to shutdown
//!     tx.send(true)?;
//!
//!     // delay a bit to let all shards shutdown
//!     // a more realistic option would be to have another channel each task sends
//!     // on when completed or use the (currently unstable) `tokio::task::JoinSet`
//!     time::sleep(Duration::from_secs(5)).await;
//!
//!     Ok(())
//! }
//!
//! async fn process(shards: impl Iterator<Item = &mut Shard>) {
//!     let mut stream = ShardEventStream::new(shards);
//!     loop {
//!         let (shard, event) = match stream.next().await {
//!             Some((shard, Ok(event))) => (shard, event),
//!             Some((shard, Err(source))) => {
//!                 tracing::warn!(?source, "error receiving event");
//!
//!                 if source.is_fatal() {
//!                     break;
//!                 }
//!
//!                 continue;
//!             }
//!             None => break,
//!         };
//!
//!         println!("received event on shard {}: {event:?}", shard.id());
//!     }
//! }
//! ```
//!
//! [`ConfigBuilder::queue`]: crate::ConfigBuilder::queue
//! [session queue]: crate::queue

use crate::{
    error::ReceiveMessageError, message::Message, tls::TlsContainer, Config, Shard, ShardId,
};
use futures_util::{
    future::BoxFuture,
    stream::{FuturesUnordered, Stream, StreamExt},
};
#[cfg(feature = "twilight-http")]
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use std::{
    ops::{Bound, Deref, DerefMut, Range, RangeBounds},
    pin::Pin,
    sync::mpsc,
    task::{Context, Poll},
};
#[cfg(feature = "twilight-http")]
use twilight_http::Client;
use twilight_model::gateway::event::Event;

/// Generic list of unordered futures producing an item for each shard.
type FutureList<'a, Item> = FuturesUnordered<BoxFuture<'a, NextItemOutput<'a, Item>>>;

/// Failure when fetching the recommended number of shards to use from Discord's
/// REST API.
#[cfg(feature = "twilight-http")]
#[derive(Debug)]
pub struct StartRecommendedError {
    /// Type of error.
    pub(crate) kind: StartRecommendedErrorType,
    /// Source error if available.
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

#[cfg(feature = "twilight-http")]
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

#[cfg(feature = "twilight-http")]
impl Error for StartRecommendedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`StartRecommendedError`] that occurred.
#[cfg(feature = "twilight-http")]
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

/// Stream selecting the next gateway event from a collection of shards.
///
/// # Examples
///
/// Create the recommended number of shards and stream over their events:
///
/// ```no_run
/// use futures::StreamExt;
/// use std::{env, sync::Arc};
/// use twilight_gateway::{
///     queue::LocalQueue,
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
/// let queue = Arc::new(LocalQueue::new());
/// // callback to create a config for each shard, useful for when not all shards
/// // have the same configuration, such as for per-shard presences
/// let config_callback = |_| {
///     Config::builder(token.clone(), Intents::GUILDS)
///         .queue(queue.clone())
///         .build()
/// };
///
/// let mut shards = stream::create_recommended(&client, config_callback)
///     .await?
///     .collect::<Vec<_>>();
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
    futures: FutureList<'a, Event>,
    /// Sender to include in [`ShardRef`].
    sender: mpsc::Sender<&'a mut Shard>,
    /// Receiver to re-insert shards into to the stream.
    receiver: mpsc::Receiver<&'a mut Shard>,
}

impl<'a> ShardEventStream<'a> {
    /// Create a new stream producing events from a set of shards.
    pub fn new(shards: impl Iterator<Item = &'a mut Shard>) -> Self {
        let (sender, receiver) = mpsc::channel();
        let mut this = Self {
            futures: FuturesUnordered::new(),
            sender,
            receiver,
        };

        for shard in shards {
            this.add_shard(shard);
        }

        this
    }

    /// Add a shard to the stream to produce a gateway event.
    fn add_shard(&mut self, shard: &'a mut Shard) {
        self.futures.push(Box::pin(async {
            let result = shard.next_event().await;

            NextItemOutput { result, shard }
        }));
    }
}

impl<'a> Stream for ShardEventStream<'a> {
    type Item = (ShardRef<'a>, Result<Event, ReceiveMessageError>);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        while let Some(shard) = self.receiver.try_iter().next() {
            self.add_shard(shard);
        }

        match self.futures.poll_next_unpin(cx) {
            Poll::Ready(Some(output)) => Poll::Ready(Some((
                ShardRef {
                    channel: self.sender.clone(),
                    shard: Some(output.shard),
                },
                output.result,
            ))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Stream selecting the next websocket message from a collection of shards.
///
/// # Examples
///
/// Create the recommended number of shards and stream over their messages:
///
/// ```no_run
/// use futures::StreamExt;
/// use std::{env, sync::Arc};
/// use twilight_gateway::{
///     queue::LocalQueue,
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
/// let queue = Arc::new(LocalQueue::new());
/// // callback to create a config for each shard, useful for when not all shards
/// // have the same configuration, such as for per-shard presences
/// let config_callback = |_| {
///     Config::builder(token.clone(), Intents::GUILDS)
///         .queue(queue.clone())
///         .build()
/// };
///
/// let mut shards = stream::create_recommended(&client, config_callback)
///     .await?
///     .collect::<Vec<_>>();
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
    futures: FutureList<'a, Message>,
    /// Sender to include in [`ShardRef`].
    sender: mpsc::Sender<&'a mut Shard>,
    /// Receiver to re-insert shards into the stream.
    receiver: mpsc::Receiver<&'a mut Shard>,
}

impl<'a> ShardMessageStream<'a> {
    /// Create a new stream producing websocket messages from a set of shards.
    pub fn new(shards: impl Iterator<Item = &'a mut Shard>) -> Self {
        let (sender, receiver) = mpsc::channel();
        let mut this = Self {
            futures: FuturesUnordered::new(),
            sender,
            receiver,
        };

        for shard in shards {
            this.add_shard(shard);
        }

        this
    }

    /// Add a shard to the stream to produce a websocket message.
    fn add_shard(&mut self, shard: &'a mut Shard) {
        self.futures.push(Box::pin(async {
            let result = shard.next_message().await;

            NextItemOutput { result, shard }
        }));
    }
}

impl<'a> Stream for ShardMessageStream<'a> {
    type Item = (ShardRef<'a>, Result<Message, ReceiveMessageError>);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        while let Some(shard) = self.receiver.try_iter().next() {
            self.add_shard(shard);
        }

        match self.futures.poll_next_unpin(cx) {
            Poll::Ready(Some(output)) => Poll::Ready(Some((
                ShardRef {
                    channel: self.sender.clone(),
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
    /// Sender pointing back to the parent stream.
    channel: mpsc::Sender<&'a mut Shard>,
    /// Mutable reference to the shard that produced an event or message.
    shard: Option<&'a mut Shard>,
}

impl Deref for ShardRef<'_> {
    type Target = Shard;

    fn deref(&self) -> &Self::Target {
        self.shard.as_ref().unwrap()
    }
}

impl DerefMut for ShardRef<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.shard.as_mut().unwrap()
    }
}

impl Drop for ShardRef<'_> {
    fn drop(&mut self) {
        if let Some(shard) = self.shard.take() {
            self.channel.send(shard).unwrap();
        }
    }
}

/// Output of a stream, such as [`ShardMessageStream`].
struct NextItemOutput<'a, Item> {
    /// Result of the future.
    result: Result<Item, ReceiveMessageError>,
    /// Shard that produced the result.
    shard: &'a mut Shard,
}

/// Create a single bucket's worth of shards with provided configuration for
/// each shard.
///
/// # Examples
///
/// Start bucket 2 out of 10 with 100 shards in total and collect them into a
/// list:
///
/// ```no_run
/// use std::{env, sync::Arc};
/// use twilight_gateway::{queue::LocalQueue, stream, Config, Intents};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let token = env::var("DISCORD_TOKEN")?;
///
/// let queue = Arc::new(LocalQueue::new());
/// // callback to create a config for each shard, useful for when not all shards
/// // have the same configuration, such as for per-shard presences
/// let config_callback = |_| {
///     Config::builder(token.clone(), Intents::GUILDS)
///         .queue(queue.clone())
///         .build()
/// };
///
/// let shards = stream::create_bucket(2, 10, 100, config_callback)
///     .map(|shard| (shard.id().number(), shard))
///     .collect::<Vec<_>>();
///
/// assert_eq!(shards.len(), 10);
/// # Ok(()) }
/// ```
///
/// # Panics
///
/// Panics if `bucket_id >= total` or if `concurrency >= total`.
///
/// Panics if `concurrency` doesn't fit into a usize.
///
/// Panics if loading TLS certificates fails.
#[track_caller]
pub fn create_bucket<F: Fn(ShardId) -> Config>(
    bucket_id: u64,
    concurrency: u64,
    total: u64,
    per_shard_config: F,
) -> impl Iterator<Item = Shard> {
    assert!(bucket_id < total, "bucket id must be less than the total");
    assert!(
        concurrency < total,
        "concurrency must be less than the total"
    );

    let concurrency = concurrency.try_into().unwrap();
    let tls = TlsContainer::new().unwrap();

    (bucket_id..total).step_by(concurrency).map(move |index| {
        let id = ShardId::new(index, total);
        let mut config = per_shard_config(id);
        config.set_tls(tls.clone());

        Shard::with_config(id, config)
    })
}

/// Create a range of shards with provided configuration for each shard.
///
/// # Examples
///
/// Start 10 out of 10 shards and collect them into a map:
///
/// ```no_run
/// use std::{collections::HashMap, env, sync::Arc};
/// use twilight_gateway::{queue::LocalQueue, stream, Config, Intents};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let token = env::var("DISCORD_TOKEN")?;
///
/// let queue = Arc::new(LocalQueue::new());
/// // callback to create a config for each shard, useful for when not all shards
/// // have the same configuration, such as for per-shard presences
/// let config_callback = |_| {
///     Config::builder(token.clone(), Intents::GUILDS)
///         .queue(queue.clone())
///         .build()
/// };
///
/// let shards = stream::create_range(0..10, 10, config_callback)
///     .map(|shard| (shard.id().number(), shard))
///     .collect::<HashMap<_, _>>();
///
/// assert_eq!(shards.len(), 10);
/// # Ok(()) }
/// ```
///
/// # Panics
///
/// Panics if `start >= total` or if `end > total`, where `start` and `end`
/// refer to `range`'s start and end.
///
/// Panics if loading TLS certificates fails.
#[track_caller]
pub fn create_range<F: Fn(ShardId) -> Config>(
    range: impl RangeBounds<u64>,
    total: u64,
    per_shard_config: F,
) -> impl Iterator<Item = Shard> {
    let range = calculate_range(range, total);
    let tls = TlsContainer::new().unwrap();

    range.map(move |index| {
        let id = ShardId::new(index, total);
        let mut config = per_shard_config(id);
        config.set_tls(tls.clone());

        Shard::with_config(id, config)
    })
}

/// Create a range of shards from Discord's recommendation with configuration
/// for each shard.
///
/// Internally calls [`create_range`] with the values from [`GetGatewayAuthed`].
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
///
/// [`GetGatewayAuthed`]: twilight_http::request::GetGatewayAuthed
#[cfg(feature = "twilight-http")]
#[track_caller]
pub async fn create_recommended<F: Fn(ShardId) -> Config>(
    client: &Client,
    per_shard_config: F,
) -> Result<impl Iterator<Item = Shard>, StartRecommendedError> {
    let request = client.gateway().authed();
    let response = request.await.map_err(|source| StartRecommendedError {
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

    Ok(create_range(.., info.shards, per_shard_config))
}

/// Transform any range into a sized range based on the total.
///
/// # Panics
///
/// Panics if `start >= total` or if `end > total`, where `start` and `end`
/// refer to `range`'s start and end.
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

    assert!(start < total, "range start must be less than the total");
    assert!(end <= total, "range end must be less than the total");

    start..end
}

#[cfg(test)]
mod tests {
    use super::{ShardEventStream, ShardMessageStream};
    use futures_util::Stream;
    use static_assertions::assert_impl_all;

    assert_impl_all!(ShardEventStream<'_>: Send, Stream);
    assert_impl_all!(ShardMessageStream<'_>: Send, Stream);
}
