#![doc = include_str!("../README.md")]
#![warn(
    clippy::missing_const_for_fn,
    clippy::missing_docs_in_private_items,
    clippy::pedantic,
    missing_docs,
    unsafe_code
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::unnecessary_wraps
)]

pub mod future;

use self::future::{
    WaitForComponentFuture, WaitForComponentStream, WaitForEventFuture, WaitForEventStream,
    WaitForGuildEventFuture, WaitForGuildEventStream, WaitForMessageFuture, WaitForMessageStream,
    WaitForReactionFuture, WaitForReactionStream,
};
use dashmap::DashMap;
use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::Hash,
    sync::atomic::{AtomicU64, Ordering},
};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender as MpscSender},
    oneshot::{self, Receiver, Sender as OneshotSender},
};
use twilight_model::{
    application::interaction::{Interaction, InteractionType},
    gateway::{
        event::Event,
        payload::incoming::{MessageCreate, ReactionAdd},
    },
    id::{
        marker::{ChannelMarker, GuildMarker, MessageMarker},
        Id,
    },
};

/// Map keyed by an ID - such as a channel ID or message ID - storing a list of
/// bystanders.
type BystanderMap<K, V> = DashMap<K, Vec<Bystander<V>>>;

/// Sender to a caller that may be for a future bystander or a stream bystander.
#[derive(Debug)]
enum Sender<E> {
    /// Bystander is a future and the sender is a oneshot.
    Future(OneshotSender<E>),
    /// Bystander is a stream and the sender is an MPSC.
    Stream(MpscSender<E>),
}

impl<E> Sender<E> {
    /// Whether the channel is closed.
    fn is_closed(&self) -> bool {
        match self {
            Self::Future(sender) => sender.is_closed(),
            Self::Stream(sender) => sender.is_closed(),
        }
    }
}

/// Registration for a caller to wait for an event based on a predicate
/// function.
struct Bystander<T> {
    /// Predicate check to perform on an event.
    func: Box<dyn Fn(&T) -> bool + Send + Sync>,
    /// [`Sender::Future`]s consume themselves once upon sending so the sender
    /// needs to be able to be taken out separately.
    sender: Option<Sender<T>>,
}

impl<T: Debug> Debug for Bystander<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Bystander")
            .field("func", &"<dyn Fn(&T) -> bool>")
            .field("sender", &self.sender)
            .finish()
    }
}

/// The `Standby` struct, used by the main event loop to process events and by
/// tasks to wait for an event.
///
/// Refer to the crate-level documentation for more information.
///
/// # Using Standby in multiple tasks
///
/// To use a Standby instance in multiple tasks, consider wrapping it in an
/// [`std::sync::Arc`] or [`std::rc::Rc`].
///
/// # Examples
///
/// ## Timeouts
///
/// Futures can be timed out by passing the future returned by Standby to
/// functions such as [`tokio::time::timeout`]:
///
/// ```rust,no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::time::Duration;
/// use twilight_model::gateway::event::{Event, EventType};
/// use twilight_standby::Standby;
///
/// let standby = Standby::new();
/// let future = standby.wait_for_event(|event: &Event| event.kind() == EventType::Ready);
/// let event = tokio::time::timeout(Duration::from_secs(1), future).await?;
/// # Ok(()) }
/// ```
///
/// [`tokio::time::timeout`]: https://docs.rs/tokio/latest/tokio/time/fn.timeout.html
#[derive(Debug, Default)]
pub struct Standby {
    /// List of component bystanders where the ID of the message is known
    /// beforehand.
    components: DashMap<Id<MessageMarker>, Vec<Bystander<Interaction>>>,
    /// Bystanders for any event that may not be in any particular guild.
    ///
    /// The key is generated via [`event_counter`].
    ///
    /// [`event_counter`]: Self::event_counter
    events: DashMap<u64, Bystander<Event>>,
    /// Event counter to be used as the key of [`events`].
    ///
    /// [`events`]: Self::events
    event_counter: AtomicU64,
    /// List of bystanders where the ID of the guild is known beforehand.
    guilds: DashMap<Id<GuildMarker>, Vec<Bystander<Event>>>,
    /// List of message bystanders where the ID of the channel is known
    /// beforehand.
    messages: DashMap<Id<ChannelMarker>, Vec<Bystander<MessageCreate>>>,
    /// List of reaction bystanders where the ID of the message is known
    /// beforehand.
    reactions: DashMap<Id<MessageMarker>, Vec<Bystander<ReactionAdd>>>,
}

impl Standby {
    /// Create a new instance of `Standby`.
    ///
    /// Once a `Standby` has been created it must process gateway events via
    /// [`process`]. Awaiting an event can start via methods such as
    /// [`wait_for`] and [`wait_for_message_stream`].
    ///
    /// [`process`]: Self::process
    /// [`wait_for`]: Self::wait_for
    /// [`wait_for_message_stream`]: Self::wait_for_message_stream
    #[must_use = "must process events to be useful"]
    pub fn new() -> Self {
        Self::default()
    }

    /// Process an event, calling any bystanders that might be waiting on it.
    ///
    /// Returns statistics about matched [`Standby`] calls and how they were
    /// processed. For example, by using [`ProcessResults::matched`] you can
    /// determine how many calls were sent an event.
    ///
    /// When a bystander checks to see if an event is what it's waiting for, it
    /// will receive the event by cloning it.
    ///
    /// This function must be called when events are received in order for
    /// futures returned by methods to fulfill.
    pub fn process(&self, event: &Event) -> ProcessResults {
        tracing::trace!(event_type = ?event.kind(), ?event, "processing event");

        let mut completions = ProcessResults::new();

        match event {
            Event::InteractionCreate(e) => {
                if e.kind == InteractionType::MessageComponent {
                    if let Some(message) = &e.message {
                        completions.add_with(&Self::process_specific_event(
                            &self.components,
                            message.id,
                            e,
                        ));
                    }
                }
            }
            Event::MessageCreate(e) => {
                completions.add_with(&Self::process_specific_event(
                    &self.messages,
                    e.0.channel_id,
                    e,
                ));
            }
            Event::ReactionAdd(e) => {
                completions.add_with(&Self::process_specific_event(
                    &self.reactions,
                    e.0.message_id,
                    e,
                ));
            }
            _ => {}
        }

        if let Some(guild_id) = event.guild_id() {
            completions.add_with(&Self::process_specific_event(&self.guilds, guild_id, event));
        }

        completions.add_with(&Self::process_event(&self.events, event));

        completions
    }

    /// Wait for an event in a certain guild.
    ///
    /// To wait for multiple guild events matching the given predicate use
    /// [`wait_for_stream`].
    ///
    /// # Examples
    ///
    /// Wait for a [`BanAdd`] event in guild 123:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_model::{
    ///     gateway::event::{Event, EventType},
    ///     id::Id,
    /// };
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let guild_id = Id::new(123);
    ///
    /// let reaction = standby
    ///     .wait_for(guild_id, |event: &Event| event.kind() == EventType::BanAdd)
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// The returned future resolves to a [`Canceled`] error if the associated
    /// [`Standby`] instance is dropped.
    ///
    /// [`BanAdd`]: twilight_model::gateway::payload::incoming::BanAdd
    /// [`Canceled`]: future::Canceled
    /// [`wait_for_stream`]: Self::wait_for_stream
    pub fn wait_for<F: Fn(&Event) -> bool + Send + Sync + 'static>(
        &self,
        guild_id: Id<GuildMarker>,
        check: impl Into<Box<F>>,
    ) -> WaitForGuildEventFuture {
        tracing::trace!(%guild_id, "waiting for event in guild");

        WaitForGuildEventFuture {
            rx: Self::insert_future(&self.guilds, guild_id, check),
        }
    }

    /// Wait for a stream of events in a certain guild.
    ///
    /// To wait for only one guild event matching the given predicate use
    /// [`wait_for`].
    ///
    /// # Examples
    ///
    /// Wait for multiple [`BanAdd`] events in guild 123:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use tokio_stream::StreamExt;
    /// use twilight_model::{
    ///     gateway::event::{Event, EventType},
    ///     id::Id,
    /// };
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let guild_id = Id::new(123);
    ///
    /// let mut stream =
    ///     standby.wait_for_stream(guild_id, |event: &Event| event.kind() == EventType::BanAdd);
    ///
    /// while let Some(event) = stream.next().await {
    ///     if let Event::BanAdd(ban) = event {
    ///         println!("user {} was banned in guild {}", ban.user.id, ban.guild_id);
    ///     }
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// The returned stream ends when the associated [`Standby`] instance is
    /// dropped.
    ///
    /// [`BanAdd`]: twilight_model::gateway::payload::incoming::BanAdd
    /// [`wait_for`]: Self::wait_for
    pub fn wait_for_stream<F: Fn(&Event) -> bool + Send + Sync + 'static>(
        &self,
        guild_id: Id<GuildMarker>,
        check: impl Into<Box<F>>,
    ) -> WaitForGuildEventStream {
        tracing::trace!(%guild_id, "waiting for event in guild");

        WaitForGuildEventStream {
            rx: Self::insert_stream(&self.guilds, guild_id, check),
        }
    }

    /// Wait for an event not in a certain guild. This must be filtered by an
    /// event type.
    ///
    /// To wait for multiple events matching the given predicate use
    /// [`wait_for_event_stream`].
    ///
    /// # Examples
    ///
    /// Wait for a [`Ready`] event for shard 5:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_model::gateway::event::{Event, EventType};
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let ready = standby
    ///     .wait_for_event(|event: &Event| {
    ///         if let Event::Ready(ready) = event {
    ///             ready.shard.map_or(false, |id| id.number() == 5)
    ///         } else {
    ///             false
    ///         }
    ///     })
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// The returned future resolves to a [`Canceled`] error if the associated
    /// [`Standby`] instance is dropped.
    ///
    /// [`Canceled`]: future::Canceled
    /// [`Ready`]: twilight_model::gateway::payload::incoming::Ready
    /// [`wait_for_event_stream`]: Self::wait_for_event_stream
    pub fn wait_for_event<F: Fn(&Event) -> bool + Send + Sync + 'static>(
        &self,
        check: impl Into<Box<F>>,
    ) -> WaitForEventFuture {
        tracing::trace!("waiting for event");

        let (tx, rx) = oneshot::channel();

        self.events.insert(
            self.next_event_id(),
            Bystander {
                func: check.into(),
                sender: Some(Sender::Future(tx)),
            },
        );

        WaitForEventFuture { rx }
    }

    /// Wait for a stream of events not in a certain guild. This must be
    /// filtered by an event type.
    ///
    /// To wait for only one event matching the given predicate use
    /// [`wait_for_event`].
    ///
    /// # Examples
    ///
    /// Wait for multiple [`Ready`] events on shard 5:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use tokio_stream::StreamExt;
    /// use twilight_model::gateway::event::{Event, EventType};
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let mut events = standby.wait_for_event_stream(|event: &Event| {
    ///     if let Event::Ready(ready) = event {
    ///         ready.shard.map_or(false, |id| id.number() == 5)
    ///     } else {
    ///         false
    ///     }
    /// });
    ///
    /// while let Some(event) = events.next().await {
    ///     println!("got event with type {:?}", event.kind());
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// The returned stream ends when the associated [`Standby`] instance is
    /// dropped.
    ///
    /// [`Ready`]: twilight_model::gateway::payload::incoming::Ready
    /// [`wait_for_event`]: Self::wait_for_event
    pub fn wait_for_event_stream<F: Fn(&Event) -> bool + Send + Sync + 'static>(
        &self,
        check: impl Into<Box<F>>,
    ) -> WaitForEventStream {
        tracing::trace!("waiting for event");

        let (tx, rx) = mpsc::unbounded_channel();

        self.events.insert(
            self.next_event_id(),
            Bystander {
                func: check.into(),
                sender: Some(Sender::Stream(tx)),
            },
        );

        WaitForEventStream { rx }
    }

    /// Wait for a message in a certain channel.
    ///
    /// To wait for multiple messages matching the given predicate use
    /// [`wait_for_message_stream`].
    ///
    /// # Examples
    ///
    /// Wait for a message in channel 123 by user 456 with the content "test":
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_model::{gateway::payload::incoming::MessageCreate, id::Id};
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let author_id = Id::new(456);
    /// let channel_id = Id::new(123);
    ///
    /// let message = standby
    ///     .wait_for_message(channel_id, move |event: &MessageCreate| {
    ///         event.author.id == author_id && event.content == "test"
    ///     })
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// The returned future resolves to a [`Canceled`] error if the associated
    /// [`Standby`] instance is dropped.
    ///
    /// [`Canceled`]: future::Canceled
    /// [`wait_for_message_stream`]: Self::wait_for_message_stream
    pub fn wait_for_message<F: Fn(&MessageCreate) -> bool + Send + Sync + 'static>(
        &self,
        channel_id: Id<ChannelMarker>,
        check: impl Into<Box<F>>,
    ) -> WaitForMessageFuture {
        tracing::trace!(%channel_id, "waiting for message in channel");

        WaitForMessageFuture {
            rx: Self::insert_future(&self.messages, channel_id, check),
        }
    }

    /// Wait for a stream of message in a certain channel.
    ///
    /// To wait for only one message matching the given predicate use
    /// [`wait_for_message`].
    ///
    /// # Examples
    ///
    /// Wait for multiple messages in channel 123 by user 456 with the content
    /// "test":
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use tokio_stream::StreamExt;
    /// use twilight_model::{gateway::payload::incoming::MessageCreate, id::Id};
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let author_id = Id::new(456);
    /// let channel_id = Id::new(123);
    ///
    /// let mut messages = standby.wait_for_message_stream(channel_id, move |event: &MessageCreate| {
    ///     event.author.id == author_id && event.content == "test"
    /// });
    ///
    /// while let Some(message) = messages.next().await {
    ///     println!("got message by {}", message.author.id);
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// The returned stream ends when the associated [`Standby`] instance is
    /// dropped.
    ///
    /// [`wait_for_message`]: Self::wait_for_message
    pub fn wait_for_message_stream<F: Fn(&MessageCreate) -> bool + Send + Sync + 'static>(
        &self,
        channel_id: Id<ChannelMarker>,
        check: impl Into<Box<F>>,
    ) -> WaitForMessageStream {
        tracing::trace!(%channel_id, "waiting for message in channel");

        WaitForMessageStream {
            rx: Self::insert_stream(&self.messages, channel_id, check),
        }
    }

    /// Wait for a reaction on a certain message.
    ///
    /// To wait for multiple reactions matching the given predicate use
    /// [`wait_for_reaction_stream`].
    ///
    /// # Examples
    ///
    /// Wait for a reaction on message 123 by user 456:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_model::{gateway::payload::incoming::ReactionAdd, id::Id};
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let message_id = Id::new(123);
    /// let user_id = Id::new(456);
    ///
    /// let reaction = standby
    ///     .wait_for_reaction(message_id, move |event: &ReactionAdd| {
    ///         event.user_id == user_id
    ///     })
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// The returned future resolves to a [`Canceled`] error if the associated
    /// [`Standby`] instance is dropped.
    ///
    /// [`Canceled`]: future::Canceled
    /// [`wait_for_reaction_stream`]: Self::wait_for_reaction_stream
    pub fn wait_for_reaction<F: Fn(&ReactionAdd) -> bool + Send + Sync + 'static>(
        &self,
        message_id: Id<MessageMarker>,
        check: impl Into<Box<F>>,
    ) -> WaitForReactionFuture {
        tracing::trace!(%message_id, "waiting for reaction on message");

        WaitForReactionFuture {
            rx: Self::insert_future(&self.reactions, message_id, check),
        }
    }

    /// Wait for a stream of reactions on a certain message.
    ///
    /// To wait for only one reaction matching the given predicate use
    /// [`wait_for_reaction`].
    ///
    /// # Examples
    ///
    /// Wait for multiple reactions on message 123 with unicode reaction "🤠":
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use tokio_stream::StreamExt;
    /// use twilight_model::{
    ///     channel::message::ReactionType,
    ///     gateway::payload::incoming::ReactionAdd,
    ///     id::Id,
    /// };
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let message_id = Id::new(123);
    ///
    /// let mut reactions = standby.wait_for_reaction_stream(message_id, |event: &ReactionAdd| {
    ///     matches!(&event.emoji, ReactionType::Unicode { name } if name == "🤠")
    /// });
    ///
    /// while let Some(reaction) = reactions.next().await {
    ///     println!("got a reaction by {}", reaction.user_id);
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// The returned stream ends when the associated [`Standby`] instance is
    /// dropped.
    ///
    /// [`wait_for_reaction`]: Self::wait_for_reaction
    pub fn wait_for_reaction_stream<F: Fn(&ReactionAdd) -> bool + Send + Sync + 'static>(
        &self,
        message_id: Id<MessageMarker>,
        check: impl Into<Box<F>>,
    ) -> WaitForReactionStream {
        tracing::trace!(%message_id, "waiting for reaction on message");

        WaitForReactionStream {
            rx: Self::insert_stream(&self.reactions, message_id, check),
        }
    }

    /// Wait for a component on a certain message.
    ///
    /// Returns a `Canceled` error if the `Standby` struct was dropped.
    ///
    /// If you need to wait for multiple components matching the given predicate,
    /// use [`wait_for_component_stream`].
    ///
    /// # Examples
    ///
    /// Wait for a component on message 123 by user 456:
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_model::{application::interaction::Interaction, id::Id};
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    /// let message_id = Id::new(123);
    ///
    /// let component = standby
    ///     .wait_for_component(message_id, |event: &Interaction| {
    ///         event.author_id() == Some(Id::new(456))
    ///     })
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`wait_for_component_stream`]: Self::wait_for_component_stream
    pub fn wait_for_component<F: Fn(&Interaction) -> bool + Send + Sync + 'static>(
        &self,
        message_id: Id<MessageMarker>,
        check: impl Into<Box<F>>,
    ) -> WaitForComponentFuture {
        tracing::trace!(%message_id, "waiting for component on message");

        WaitForComponentFuture {
            rx: Self::insert_future(&self.components, message_id, check),
        }
    }

    /// Wait for a stream of components on a certain message.
    ///
    /// Returns a `Canceled` error if the `Standby` struct was dropped.
    ///
    /// If you need to wait for only one component matching the given predicate,
    /// use [`wait_for_component`].
    ///
    /// # Examples
    ///
    /// Wait for multiple button components on message 123 with a `custom_id` of
    /// "Click":
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use tokio_stream::StreamExt;
    /// use twilight_model::{
    ///     application::interaction::{Interaction, InteractionData},
    ///     id::Id,
    /// };
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    /// let message_id = Id::new(123);
    ///
    /// let mut components = standby.wait_for_component_stream(message_id, |event: &Interaction| {
    ///     if let Some(InteractionData::MessageComponent(data)) = &event.data {
    ///         data.custom_id == "Click".to_string()
    ///     } else {
    ///         false
    ///     }
    /// });
    ///
    /// while let Some(component) = components.next().await {
    ///     println!("got a component by {}", component.author_id().unwrap());
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// [`wait_for_component`]: Self::wait_for_component
    pub fn wait_for_component_stream<F: Fn(&Interaction) -> bool + Send + Sync + 'static>(
        &self,
        message_id: Id<MessageMarker>,
        check: impl Into<Box<F>>,
    ) -> WaitForComponentStream {
        tracing::trace!(%message_id, "waiting for component on message");

        WaitForComponentStream {
            rx: Self::insert_stream(&self.components, message_id, check),
        }
    }

    /// Next event ID in [`Standby::event_counter`].
    fn next_event_id(&self) -> u64 {
        self.event_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Append a new future bystander into a map according to the ID.
    fn insert_future<F: Fn(&V) -> bool + Send + Sync + 'static, K: Eq + Hash, V>(
        map: &BystanderMap<K, V>,
        id: K,
        check: impl Into<Box<F>>,
    ) -> Receiver<V> {
        let (tx, rx) = oneshot::channel();

        let mut entry = map.entry(id).or_default();
        entry.push(Bystander {
            func: check.into(),
            sender: Some(Sender::Future(tx)),
        });

        rx
    }

    /// Append a new stream bystander into a map according to the ID.
    fn insert_stream<F: Fn(&V) -> bool + Send + Sync + 'static, K: Eq + Hash, V>(
        map: &BystanderMap<K, V>,
        id: K,
        check: impl Into<Box<F>>,
    ) -> UnboundedReceiver<V> {
        let (tx, rx) = mpsc::unbounded_channel();

        let mut entry = map.entry(id).or_default();
        entry.push(Bystander {
            func: check.into(),
            sender: Some(Sender::Stream(tx)),
        });

        rx
    }

    /// Process a general event that is not of any particular type or in any
    /// particular guild.
    #[tracing::instrument(level = "trace")]
    fn process_event<K: Debug + Display + Eq + Hash + PartialEq + 'static, V: Clone + Debug>(
        map: &DashMap<K, Bystander<V>>,
        event: &V,
    ) -> ProcessResults {
        tracing::trace!(?event, "processing event");

        let mut results = ProcessResults::new();

        map.retain(|id, bystander| {
            let result = Self::bystander_process(bystander, event);
            results.handle(result);

            tracing::trace!(bystander_id = %id, ?result, "event bystander processed");

            // We want to retain bystanders that are *incomplete* and remove
            // bystanders that are *complete*.
            !result.is_complete()
        });

        results
    }

    /// Process a general event that is either of a particular type or in a
    /// particular guild.
    #[tracing::instrument(level = "trace")]
    fn process_specific_event<
        K: Debug + Display + Eq + Hash + PartialEq + 'static,
        V: Clone + Debug,
    >(
        map: &DashMap<K, Vec<Bystander<V>>>,
        guild_id: K,
        event: &V,
    ) -> ProcessResults {
        // Iterate over a guild's bystanders and mark it for removal if there
        // are no bystanders remaining.
        let (remove_guild, results) = if let Some(mut bystanders) = map.get_mut(&guild_id) {
            let results = Self::bystander_iter(&mut bystanders, event);

            (bystanders.is_empty(), results)
        } else {
            tracing::trace!(%guild_id, "guild has no event bystanders");

            return ProcessResults::new();
        };

        if remove_guild {
            tracing::trace!(%guild_id, "removing guild from map");

            map.remove(&guild_id);
        }

        results
    }

    /// Iterate over bystanders and remove the ones that match the predicate.
    #[tracing::instrument(level = "trace")]
    fn bystander_iter<E: Clone + Debug>(
        bystanders: &mut Vec<Bystander<E>>,
        event: &E,
    ) -> ProcessResults {
        tracing::trace!(?bystanders, "iterating over bystanders");

        // Iterate over the list of bystanders by using an index and manually
        // indexing in to the list.
        //
        // # Logic
        //
        // In each iteration we decide whether to retain a bystander: if we do
        // then we can increment our index and move on, but if we opt to instead
        // remove it then we do so and don't increment the index. The reason we
        // don't increment the index is because when we remove an element the
        // index does not become empty and instead everything to the right is
        // shifted to the left, illustrated as such:
        //
        //     |---|
        //     v   |
        // A - B - C - D
        //     |   ^   |
        //     |   |---|
        //     |
        //  Remove B
        //
        // After: A - C - D
        //
        // # Reasons not to use alternatives
        //
        // **`Vec::retain`** we need to mutate the entries in order to take out
        // the sender and `Vec::retain` only gives us immutable references.
        //
        // A form of enumeration can't be used because sometimes the index
        // doesn't advance; iterators would continue to provide incrementing
        // enumeration indexes while we sometimes want to reuse an index.
        let mut index = 0;
        let mut results = ProcessResults::new();

        while index < bystanders.len() {
            tracing::trace!(%index, "checking bystander");

            let status = Self::bystander_process(&mut bystanders[index], event);
            results.handle(status);

            tracing::trace!(%index, ?status, "checked bystander");

            if status.is_complete() {
                bystanders.remove(index);
            } else {
                index += 1;
            }
        }

        results
    }

    /// Process a bystander, sending the event if the sender is active and the
    /// predicate matches. Returns whether the bystander has fulfilled.
    ///
    /// Returns whether the bystander is fulfilled; if the bystander has been
    /// fulfilled then the channel is now closed.
    #[tracing::instrument(level = "trace")]
    fn bystander_process<T: Clone + Debug>(
        bystander: &mut Bystander<T>,
        event: &T,
    ) -> ProcessStatus {
        // We need to take the sender out because `OneshotSender`s consume
        // themselves when calling `OneshotSender::send`.
        let Some(sender) = bystander.sender.take() else {
            tracing::trace!("bystander has no sender, indicating for removal");

            return ProcessStatus::AlreadyComplete;
        };

        // The channel may have closed due to the receiver dropping their end,
        // in which case we can say we're done.
        if sender.is_closed() {
            tracing::trace!("bystander's rx dropped, indicating for removal");

            return ProcessStatus::Dropped;
        }

        // Lastly check to see if the predicate matches the event. If it doesn't
        // then we can short-circuit.
        if !(bystander.func)(event) {
            tracing::trace!("bystander check doesn't match, not removing");

            // Put the sender back into its bystander since we'll still need it
            // next time around.
            bystander.sender.replace(sender);

            return ProcessStatus::Skip;
        }

        match sender {
            Sender::Future(tx) => {
                // We don't care if the event successfully sends or not since
                // we're going to be tossing out the bystander anyway.
                drop(tx.send(event.clone()));

                tracing::trace!("bystander matched event, indicating for removal");

                ProcessStatus::SentFuture
            }
            Sender::Stream(tx) => {
                // If we can send an event to the receiver and the channel is
                // still open then we need to retain the bystander, otherwise we
                // need to mark it for removal.
                if tx.send(event.clone()).is_ok() {
                    tracing::trace!("bystander is a stream, retaining in map");

                    bystander.sender.replace(Sender::Stream(tx));

                    ProcessStatus::SentStream
                } else {
                    ProcessStatus::Dropped
                }
            }
        }
    }
}
/// Number of [`Standby`] calls that were completed.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ProcessResults {
    /// Number of bystanders that were dropped due to the receiving end
    /// dropping.
    dropped: usize,
    /// Number of future bystanders that were open and were sent an event.
    fulfilled: usize,
    /// Number of stream bystanders that were open and were sent an event.
    sent: usize,
}

impl ProcessResults {
    /// Create a new set of zeroed out results.
    const fn new() -> Self {
        Self {
            dropped: 0,
            fulfilled: 0,
            sent: 0,
        }
    }

    /// Number of [`Standby`] calls where the receiver had already dropped their
    /// end.
    ///
    /// This may happen when a caller calls into [`Standby`] but then times out
    /// or otherwise cancels their request.
    pub const fn dropped(&self) -> usize {
        self.dropped
    }

    /// Number of [`Standby`] calls that were fulfilled.
    ///
    /// Calls for futures via methods such as [`Standby::wait_for`] will be
    /// marked as fulfilled once matched and an event is sent over the channel.
    ///
    /// **Caveat**: although an event has been sent over the channel to the
    /// receiver it is not guaranteed whether the receiver end actually received
    /// it; the receiver end may drop shortly after an event is sent. In this
    /// case the call is considered to be fulfilled.
    pub const fn fulfilled(&self) -> usize {
        self.fulfilled
    }

    /// Number of calls that were matched and were sent an event.
    ///
    /// This is the sum of [`fulfilled`] and [`sent`].
    ///
    /// See the caveats for both methods.
    ///
    /// [`fulfilled`]: Self::fulfilled
    /// [`sent`]: Self::sent
    pub const fn matched(&self) -> usize {
        self.fulfilled() + self.sent()
    }

    /// Number of [`Standby`] streaming calls that were matched and had an event
    /// sent to them.
    ///
    /// **Caveat**: although an event has been sent over the channel to the
    /// receiver it is not guaranteed whether the receiver end actually received
    /// it; the receiver end may drop shortly after an event is sent. In this
    /// case the call is considered to be sent. Checks over this call will in
    /// the future be considered [`dropped`].
    ///
    /// [`dropped`]: Self::dropped
    pub const fn sent(&self) -> usize {
        self.sent
    }

    /// Add another set of results to this set.
    fn add_with(&mut self, other: &Self) {
        self.dropped = self.dropped.saturating_add(other.dropped);
        self.fulfilled = self.fulfilled.saturating_add(other.fulfilled);
        self.sent = self.sent.saturating_add(other.sent);
    }

    /// Handle a process status.
    fn handle(&mut self, status: ProcessStatus) {
        match status {
            ProcessStatus::Dropped => {
                self.dropped += 1;
            }
            ProcessStatus::SentFuture => {
                self.fulfilled += 1;
            }
            ProcessStatus::SentStream => {
                self.sent += 1;
            }
            ProcessStatus::AlreadyComplete | ProcessStatus::Skip => {}
        }
    }
}

/// Status result of processing a bystander via [`Standby::bystander_process`].
#[derive(Clone, Copy, Debug)]
enum ProcessStatus {
    /// Call matched but already matched previously and was not removed, so the
    /// subject must be removed and not counted towards results.
    AlreadyComplete,
    /// Call matched but the receiver dropped their end.
    Dropped,
    /// Call matched a oneshot.
    SentFuture,
    /// Call matched a stream.
    SentStream,
    /// Call was not matched.
    Skip,
}

impl ProcessStatus {
    /// Whether the call is complete.
    const fn is_complete(self) -> bool {
        matches!(
            self,
            Self::AlreadyComplete | Self::Dropped | Self::SentFuture
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::non_ascii_literal)]

    use crate::Standby;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;
    use tokio_stream::StreamExt;
    use twilight_gateway::{Event, EventType};
    use twilight_model::{
        application::interaction::{
            message_component::MessageComponentInteractionData, Interaction, InteractionData,
            InteractionType,
        },
        channel::{
            message::{component::ComponentType, Message, MessageType, ReactionType},
            Channel, ChannelType,
        },
        gateway::{
            payload::incoming::{InteractionCreate, MessageCreate, ReactionAdd, Ready, RoleDelete},
            GatewayReaction, ShardId,
        },
        guild::Permissions,
        id::{marker::GuildMarker, Id},
        oauth::{ApplicationFlags, PartialApplication, ApplicationIntegrationMap},
        user::{CurrentUser, User},
        util::Timestamp,
    };

    assert_impl_all!(Standby: Debug, Default, Send, Sync);

    fn message() -> Message {
        Message {
            activity: None,
            application: None,
            application_id: None,
            attachments: Vec::new(),
            author: User {
                accent_color: None,
                avatar: None,
                avatar_decoration: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                global_name: Some("test".to_owned()),
                id: Id::new(2),
                locale: None,
                mfa_enabled: None,
                name: "twilight".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
            channel_id: Id::new(1),
            components: Vec::new(),
            content: "test".to_owned(),
            edited_timestamp: None,
            embeds: Vec::new(),
            flags: None,
            guild_id: Some(Id::new(4)),
            id: Id::new(3),
            interaction: None,
            interaction_metadata: None,
            kind: MessageType::Regular,
            member: None,
            mention_channels: Vec::new(),
            mention_everyone: false,
            mention_roles: Vec::new(),
            mentions: Vec::new(),
            pinned: false,
            reactions: Vec::new(),
            reference: None,
            referenced_message: None,
            role_subscription_data: None,
            sticker_items: Vec::new(),
            timestamp: Timestamp::from_secs(1_632_072_645).expect("non zero"),
            thread: None,
            tts: false,
            webhook_id: None,
        }
    }

    fn reaction() -> GatewayReaction {
        GatewayReaction {
            channel_id: Id::new(2),
            emoji: ReactionType::Unicode {
                name: "🍎".to_owned(),
            },
            guild_id: Some(Id::new(1)),
            member: None,
            message_author_id: None,
            message_id: Id::new(4),
            user_id: Id::new(3),
        }
    }

    #[allow(deprecated)]
    fn button() -> Interaction {
        Interaction {
            app_permissions: Permissions::SEND_MESSAGES,
            application_id: Id::new(1),
            authorizing_integration_owners: ApplicationIntegrationMap {
                guild: None,
                user: None,
            },
            channel: Some(Channel {
                bitrate: None,
                guild_id: None,
                id: Id::new(400),
                kind: ChannelType::GuildText,
                last_message_id: None,
                last_pin_timestamp: None,
                name: None,
                nsfw: None,
                owner_id: None,
                parent_id: None,
                permission_overwrites: None,
                position: None,
                rate_limit_per_user: None,
                recipients: None,
                rtc_region: None,
                topic: None,
                user_limit: None,
                application_id: None,
                applied_tags: None,
                available_tags: None,
                default_auto_archive_duration: None,
                default_forum_layout: None,
                default_reaction_emoji: None,
                default_sort_order: None,
                default_thread_rate_limit_per_user: None,
                flags: None,
                icon: None,
                invitable: None,
                managed: None,
                member: None,
                member_count: None,
                message_count: None,
                newly_created: None,
                thread_metadata: None,
                video_quality_mode: None,
            }),
            channel_id: None,
            data: Some(InteractionData::MessageComponent(Box::new(
                MessageComponentInteractionData {
                    custom_id: String::from("Click"),
                    component_type: ComponentType::Button,
                    resolved: None,
                    values: Vec::new(),
                },
            ))),
            guild_id: Some(Id::new(3)),
            guild_locale: None,
            id: Id::new(4),
            kind: InteractionType::MessageComponent,
            locale: Some("en-GB".to_owned()),
            member: None,
            message: Some(message()),
            token: String::from("token"),
            user: Some(User {
                accent_color: None,
                avatar: None,
                avatar_decoration: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                global_name: Some("test".to_owned()),
                id: Id::new(2),
                locale: None,
                mfa_enabled: None,
                name: "twilight".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            }),
        }
    }

    /// Test that if a receiver drops their end, the result properly counts the
    /// statistic.
    #[tokio::test]
    async fn test_dropped() {
        let standby = Standby::new();
        let guild_id = Id::new(1);

        {
            let _rx = standby.wait_for(guild_id, move |_: &Event| false);
        }

        let results = standby.process(&Event::RoleDelete(RoleDelete {
            guild_id,
            role_id: Id::new(2),
        }));

        assert_eq!(1, results.dropped());
        assert_eq!(0, results.fulfilled());
        assert_eq!(0, results.sent());
    }

    /// Test that both events in guild 1 is matched but the event in guild 2 is
    /// not matched by testing the returned matched amount.
    #[tokio::test]
    async fn test_matched() {
        fn check(event: &Event, guild_id: Id<GuildMarker>) -> bool {
            matches!(event, Event::RoleDelete(e) if e.guild_id == guild_id)
        }

        let standby = Standby::new();
        let guild_id_one = Id::new(1);
        let guild_id_two = Id::new(2);
        let _one = standby.wait_for(guild_id_one, move |event: &Event| {
            check(event, guild_id_one)
        });
        let _two = standby.wait_for(guild_id_one, move |event: &Event| {
            check(event, guild_id_one)
        });
        let _three = standby.wait_for(guild_id_two, move |event: &Event| {
            check(event, guild_id_two)
        });

        let results = standby.process(&Event::RoleDelete(RoleDelete {
            guild_id: Id::new(1),
            role_id: Id::new(2),
        }));

        assert_eq!(0, results.dropped());
        assert_eq!(2, results.fulfilled());
        assert_eq!(0, results.sent());
    }

    /// Test that the [`ProcessResults::sent`] counter increments if a match is
    /// sent to it.
    #[tokio::test]
    async fn test_sent() {
        let standby = Standby::new();
        let guild_id = Id::new(1);

        let _rx = standby.wait_for_stream(guild_id, move |_: &Event| true);

        let results = standby.process(&Event::RoleDelete(RoleDelete {
            guild_id,
            role_id: Id::new(2),
        }));

        assert_eq!(0, results.dropped());
        assert_eq!(0, results.fulfilled());
        assert_eq!(1, results.sent());
    }

    /// Test basic functionality of the [`Standby::wait_for`] method.
    #[tokio::test]
    async fn test_wait_for() {
        let standby = Standby::new();
        let wait = standby.wait_for(
            Id::new(1),
            |event: &Event| matches!(event, Event::RoleDelete(e) if e.guild_id.get() == 1),
        );
        standby.process(&Event::RoleDelete(RoleDelete {
            guild_id: Id::new(1),
            role_id: Id::new(2),
        }));

        assert_eq!(
            wait.await.unwrap(),
            Event::RoleDelete(RoleDelete {
                guild_id: Id::new(1),
                role_id: Id::new(2),
            })
        );
        assert!(standby.guilds.is_empty());
    }

    /// Test basic functionality of the [`Standby::wait_for_stream`] method.
    #[tokio::test]
    async fn test_wait_for_stream() {
        let standby = Standby::new();
        let mut stream = standby.wait_for_stream(
            Id::new(1),
            |event: &Event| matches!(event, Event::RoleDelete(e) if e.guild_id.get() == 1),
        );
        standby.process(&Event::RoleDelete(RoleDelete {
            guild_id: Id::new(1),
            role_id: Id::new(2),
        }));
        standby.process(&Event::RoleDelete(RoleDelete {
            guild_id: Id::new(1),
            role_id: Id::new(3),
        }));

        assert_eq!(
            stream.next().await,
            Some(Event::RoleDelete(RoleDelete {
                guild_id: Id::new(1),
                role_id: Id::new(2)
            }))
        );
        assert_eq!(
            stream.next().await,
            Some(Event::RoleDelete(RoleDelete {
                guild_id: Id::new(1),
                role_id: Id::new(3)
            }))
        );
        assert!(!standby.guilds.is_empty());
        drop(stream);
        standby.process(&Event::RoleDelete(RoleDelete {
            guild_id: Id::new(1),
            role_id: Id::new(4),
        }));
        assert!(standby.guilds.is_empty());
    }

    /// Test basic functionality of the [`Standby::wait_for_event`] method.
    #[tokio::test]
    async fn test_wait_for_event() {
        let ready = Ready {
            application: PartialApplication {
                flags: ApplicationFlags::empty(),
                id: Id::new(1),
            },
            guilds: Vec::new(),
            resume_gateway_url: "wss://gateway.discord.gg".into(),
            session_id: String::new(),
            shard: Some(ShardId::new(5, 7)),
            user: CurrentUser {
                accent_color: None,
                avatar: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                id: Id::new(1),
                mfa_enabled: true,
                name: "twilight".to_owned(),
                verified: Some(false),
                premium_type: None,
                public_flags: None,
                flags: None,
                locale: None,
            },
            version: 6,
        };
        let event = Event::Ready(Box::new(ready));

        let standby = Standby::new();
        let wait = standby.wait_for_event(|event: &Event| match event {
            Event::Ready(ready) => ready.shard.map_or(false, |id| id.number() == 5),
            _ => false,
        });
        assert!(!standby.events.is_empty());
        standby.process(&event);

        assert_eq!(event, wait.await.unwrap());
        assert!(standby.events.is_empty());
    }

    /// Test basic functionality of the [`Standby::wait_for_event_stream`]
    /// method.
    #[tokio::test]
    async fn test_wait_for_event_stream() {
        let standby = Standby::new();
        let mut stream =
            standby.wait_for_event_stream(|event: &Event| event.kind() == EventType::Resumed);
        standby.process(&Event::Resumed);
        assert_eq!(stream.next().await, Some(Event::Resumed));
        assert!(!standby.events.is_empty());
        drop(stream);
        standby.process(&Event::Resumed);
        assert!(standby.events.is_empty());
    }

    /// Test basic functionality of the [`Standby::wait_for_message`] method.
    #[tokio::test]
    async fn test_wait_for_message() {
        let message = message();
        let event = Event::MessageCreate(Box::new(MessageCreate(message)));

        let standby = Standby::new();
        let wait = standby.wait_for_message(Id::new(1), |message: &MessageCreate| {
            message.author.id.get() == 2
        });
        standby.process(&event);

        assert_eq!(3, wait.await.map(|msg| msg.id.get()).unwrap());
        assert!(standby.messages.is_empty());
    }

    /// Test basic functionality of the [`Standby::wait_for_message_stream`]
    /// method.
    #[tokio::test]
    async fn test_wait_for_message_stream() {
        let standby = Standby::new();
        let mut stream = standby.wait_for_message_stream(Id::new(1), |_: &MessageCreate| true);
        standby.process(&Event::MessageCreate(Box::new(MessageCreate(message()))));
        standby.process(&Event::MessageCreate(Box::new(MessageCreate(message()))));

        assert!(stream.next().await.is_some());
        assert!(stream.next().await.is_some());
        drop(stream);
        assert_eq!(1, standby.messages.len());
        standby.process(&Event::MessageCreate(Box::new(MessageCreate(message()))));
        assert!(standby.messages.is_empty());
    }

    /// Test basic functionality of the [`Standby::wait_for_reaction`] method.
    #[tokio::test]
    async fn test_wait_for_reaction() {
        let event = Event::ReactionAdd(Box::new(ReactionAdd(reaction())));

        let standby = Standby::new();
        let wait = standby.wait_for_reaction(Id::new(4), |reaction: &ReactionAdd| {
            reaction.user_id.get() == 3
        });

        standby.process(&event);

        assert_eq!(
            Id::new(3),
            wait.await.map(|reaction| reaction.user_id).unwrap()
        );
        assert!(standby.reactions.is_empty());
    }

    /// Test basic functionality of the [`Standby::wait_for_reaction_stream`]
    /// method.
    #[tokio::test]
    async fn test_wait_for_reaction_stream() {
        let standby = Standby::new();
        let mut stream = standby.wait_for_reaction_stream(Id::new(4), |_: &ReactionAdd| true);
        standby.process(&Event::ReactionAdd(Box::new(ReactionAdd(reaction()))));
        standby.process(&Event::ReactionAdd(Box::new(ReactionAdd(reaction()))));

        assert!(stream.next().await.is_some());
        assert!(stream.next().await.is_some());
        drop(stream);
        assert_eq!(1, standby.reactions.len());
        standby.process(&Event::ReactionAdd(Box::new(ReactionAdd(reaction()))));
        assert!(standby.reactions.is_empty());
    }

    /// Assert that Standby processing some non-matching events will not affect
    /// the matching of a later event.
    #[tokio::test]
    async fn test_wait_for_component() {
        let event = Event::InteractionCreate(Box::new(InteractionCreate(button())));

        let standby = Standby::new();
        let wait = standby.wait_for_component(Id::new(3), |button: &Interaction| {
            button.author_id() == Some(Id::new(2))
        });

        standby.process(&event);

        assert_eq!(
            Some(Id::new(2)),
            wait.await.map(|button| button.author_id()).unwrap()
        );
        assert!(standby.components.is_empty());
    }

    #[tokio::test]
    async fn test_wait_for_component_stream() {
        let standby = Standby::new();
        let mut stream = standby.wait_for_component_stream(Id::new(3), |_: &Interaction| true);
        standby.process(&Event::InteractionCreate(Box::new(InteractionCreate(
            button(),
        ))));
        standby.process(&Event::InteractionCreate(Box::new(InteractionCreate(
            button(),
        ))));

        assert!(stream.next().await.is_some());
        assert!(stream.next().await.is_some());
        drop(stream);
        assert_eq!(1, standby.components.len());
        standby.process(&Event::InteractionCreate(Box::new(InteractionCreate(
            button(),
        ))));
        assert!(standby.components.is_empty());
    }

    #[tokio::test]
    async fn test_handles_wrong_events() {
        let standby = Standby::new();
        let wait = standby.wait_for_event(|event: &Event| event.kind() == EventType::Resumed);

        standby.process(&Event::GatewayHeartbeatAck);
        standby.process(&Event::GatewayHeartbeatAck);
        standby.process(&Event::Resumed);

        assert_eq!(Event::Resumed, wait.await.unwrap());
    }

    /// Test that generic event handlers will be given the opportunity to
    /// process events with specific handlers (message creates, reaction adds)
    /// and guild events. Similarly, guild handlers should be able to process
    /// specific handler events as well.
    #[tokio::test]
    async fn test_process_nonspecific_handling() {
        let standby = Standby::new();

        // generic event handler gets message creates
        let wait = standby.wait_for_event(|event: &Event| event.kind() == EventType::MessageCreate);
        standby.process(&Event::MessageCreate(Box::new(MessageCreate(message()))));
        assert!(matches!(wait.await, Ok(Event::MessageCreate(_))));

        // generic event handler gets reaction adds
        let wait = standby.wait_for_event(|event: &Event| event.kind() == EventType::ReactionAdd);
        standby.process(&Event::ReactionAdd(Box::new(ReactionAdd(reaction()))));
        assert!(matches!(wait.await, Ok(Event::ReactionAdd(_))));

        // generic event handler gets other guild events
        let wait = standby.wait_for_event(|event: &Event| event.kind() == EventType::RoleDelete);
        standby.process(&Event::RoleDelete(RoleDelete {
            guild_id: Id::new(1),
            role_id: Id::new(2),
        }));
        assert!(matches!(wait.await, Ok(Event::RoleDelete(_))));

        // guild event handler gets message creates or reaction events
        let wait = standby.wait_for(Id::new(1), |event: &Event| {
            event.kind() == EventType::ReactionAdd
        });
        standby.process(&Event::ReactionAdd(Box::new(ReactionAdd(reaction()))));
        assert!(matches!(wait.await, Ok(Event::ReactionAdd(_))));
    }
}
