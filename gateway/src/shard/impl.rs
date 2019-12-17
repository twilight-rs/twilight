use super::{
    config::Config,
    error::Result,
    event::{Event, EventType, Events},
    processor::{Latency, Session, ShardProcessor},
    sink::ShardSink,
    stage::Stage,
};
use crate::listener::Listeners;
use futures::future::{self, AbortHandle};
use log::debug;
use std::sync::Arc;
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug)]
pub struct ShardRef {
    config: Arc<Config>,
    listeners: Listeners<Event>,
    processor_handle: AbortHandle,
    session: Arc<Session>,
}

/// Information about a shard, including its latency, current session sequence,
/// and connection stage.
#[derive(Clone, Debug)]
pub struct Information {
    id: u64,
    latency: Latency,
    seq: u64,
    stage: Stage,
}

impl Information {
    /// Returns the ID of the shard.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns the latency information for the shard.
    ///
    /// This includes the average latency over all time, and the latency
    /// information for the 5 most recent heartbeats.
    pub fn latency(&self) -> &Latency {
        &self.latency
    }

    /// The current sequence of the connection.
    ///
    /// This is the number of the event that was received this session (without
    /// reconnecting). A larger number typically correlates that the shard has
    /// been connected for a longer time, while a smaller number typically
    /// correlates to meaning that it's been connected for a less amount of
    /// time.
    pub fn seq(&self) -> u64 {
        self.seq
    }

    /// The current stage of the shard.
    ///
    /// For example, once a shard is fully booted then it will be
    /// [`Connected`].
    ///
    /// [`Connected`]: enum.Stage.html
    pub fn stage(&self) -> Stage {
        self.stage
    }
}

#[derive(Clone, Debug)]
pub struct Shard(Arc<ShardRef>);

impl Shard {
    /// Creates a new shard, which will automatically connect to the gateway.
    ///
    /// # Examples
    ///
    /// Create a new shard, wait a second, and then print its current connection
    /// stage:
    ///
    /// ```no_run
    /// use dawn_gateway::Shard;
    /// use std::{env, time::Duration};
    /// use tokio::time as tokio_time;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let shard = Shard::new(env::var("DISCORD_TOKEN")?).await?;
    ///
    /// tokio_time::delay_for(Duration::from_secs(1)).await;
    ///
    /// let info = shard.info().await;
    /// println!("Shard stage: {}", info.stage());
    /// # Ok(()) }
    /// ```
    pub async fn new(config: impl Into<Config>) -> Result<Self> {
        Self::_new(config.into()).await
    }

    async fn _new(config: Config) -> Result<Self> {
        let config = Arc::new(config);
        let processor = ShardProcessor::new(Arc::clone(&config)).await?;
        let listeners = processor.listeners.clone();
        let session = Arc::clone(&processor.session);
        let (fut, handle) = future::abortable(processor.run());

        tokio::spawn(async move {
            let _ = fut.await;

            debug!("[Shard] Shard processor future ended");
        });

        Ok(Self(Arc::new(ShardRef {
            config,
            listeners,
            processor_handle: handle,
            session,
        })))
    }

    /// Returns an immutable reference to the configuration used for this client.
    pub fn config(&self) -> &Config {
        &self.0.config
    }

    /// Returns information about the running of the shard, such as the current
    /// connection stage.
    pub async fn info(&self) -> Information {
        Information {
            id: self.config().shard()[0],
            latency: self.0.session.heartbeats.latency().await,
            seq: self.0.session.seq(),
            stage: self.0.session.stage(),
        }
    }

    /// Creates a new stream of events from the shard.
    ///
    /// There can be multiple streams of events. All events will be broadcast to
    /// all streams of events.
    ///
    /// All event types except for [`EventType::SHARD_PAYLOAD`] are enabled.
    /// If you need to enable it, consider calling [`some_events`] instead.
    ///
    /// [`EventType::SHARD_PAYLOAD`]: events/struct.EventType.html#const.SHARD_PAYLOAD
    /// [`some_events`]: #method.some_events
    pub async fn events(&self) -> Events {
        let rx = self.0.listeners.add(EventType::default()).await;

        Events::new(EventType::default(), rx)
    }

    /// Creates a new filtered stream of events from the shard.
    ///
    /// Only the events specified in the bitflags will be sent over the stream.
    ///
    /// # Examples
    ///
    /// Filter the events so that you only receive the [`Event::ShardConnected`]
    /// and [`Event::ShardDisconnected`] events:
    ///
    /// ```no_run
    /// use dawn_gateway::shard::{Event, EventType, Shard};
    /// use futures::StreamExt;
    /// use std::env;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let shard = Shard::new(env::var("DISCORD_TOKEN")?).await?;
    ///
    /// let event_types = EventType::SHARD_CONNECTED | EventType::SHARD_DISCONNECTED;
    /// let mut events = shard.some_events(event_types).await;
    ///
    /// while let Some(event) = events.next().await {
    ///     match event {
    ///         Event::ShardConnected(_) => println!("Shard is now connected"),
    ///         Event::ShardDisconnected(_) => println!("Shard is now disconnected"),
    ///         // No other event will come in through the stream.
    ///         _ => {},
    ///     }
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn some_events(&self, event_types: EventType) -> Events {
        let rx = self.0.listeners.add(event_types).await;

        Events::new(event_types, rx)
    }

    /// Returns an interface implementing the `Sink` trait which can be used to
    /// send messages.
    pub fn sink(&self) -> ShardSink {
        ShardSink(self.0.session.tx.clone())
    }

    /// Shuts down the shard.
    ///
    /// If `wait` is true, then this will wait until a close message has been
    /// sent to Discord, which will immediately show the shard as offline. If
    /// `wait` is false, then the connection will be immediately dropped. This
    /// may continue to show your bot as being online for some time when it's
    /// not.
    pub async fn shutdown(&self) {
        // Since we're shutting down now, we don't care if it sends or not.
        let _ = self.0.session.tx.unbounded_send(Message::Close(None));

        self.0.processor_handle.abort();
        self.0.listeners.remove_all().await;
        self.0.session.stop_heartbeater().await;
    }
}
