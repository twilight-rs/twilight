//! # twilight-standby
//!
//! Standby is a utility to wait for an event to happen based on a predicate
//! check. For example, you may have a command that has a reaction menu of ✅ and
//! ❌. If you want to handle a reaction to these, using something like an
//! application-level state or event stream may not suit your use case. It may
//! be cleaner to wait for a reaction inline to your function. This is where
//! Twilight Standby comes in.
//!
//! Standby allows you to wait for things like an event in a certain guild
//! ([`Standby::wait_for`]), a new message in a channel
//! ([`Standby::wait_for_message`]), a new reaction on a message
//! ([`Standby::wait_for_reaction`]), and any event that might not take place in
//! a guild, such as a new `Ready` event ([`Standby::wait_for_event`]). Each
//! method also has a stream variant.
//!
//! To use Standby, you must process events with it in your main event loop.
//! Check out the [`Standby::process`] method.
//!
//! ## When to use futures and streams
//!
//! `Standby` has two variants of each method: a future variant and a stream
//! variant. An example is [`Standby::wait_for_message`], which also has a
//! [`Standby::wait_for_message_stream`] variant. The future variant is useful
//! when you want to oneshot an event that you need to wait for. This means that
//! if you only need to wait for one message in a channel to come in, you'd use
//! the future variant. If you need to wait for multiple messages, such as maybe
//! all of the messages within a minute's timespan, you'd use the
//! [`Standby::wait_for_message_stream`] method.
//!
//! The difference is that if you use the futures variant in a loop then you may
//! miss some events while processing a received event. By using a stream, you
//! won't miss any events.
//!
//! ## Examples
//!
//! ### At a glance
//!
//! Wait for a message in channel 123 by user 456 with the content "test":
//!
//! ```rust,no_run
//! # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use twilight_model::{gateway::payload::MessageCreate, id::{ChannelId, UserId}};
//! use twilight_standby::Standby;
//!
//! let standby = Standby::new();
//!
//! let message = standby.wait_for_message(ChannelId(123), |event: &MessageCreate| {
//!     event.author.id == UserId(456) && event.content == "test"
//! }).await?;
//! # Ok(()) }
//! ```
//!
//! ### A full example
//!
//! A full sample bot connecting to the gateway, processing events, and
//! including a handler to wait for reactions:
//!
//! ```rust,no_run
//! use futures_util::StreamExt;
//! use std::{env, error::Error};
//! use twilight_gateway::{Event, Shard};
//! use twilight_model::{
//!     channel::Message,
//!     gateway::payload::ReactionAdd,
//!     id::{ChannelId, UserId},
//! };
//! use twilight_standby::Standby;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     // Start a shard connected to the gateway to receive events.
//!     let mut shard = Shard::new(env::var("DISCORD_TOKEN")?);
//!     let mut events = shard.events();
//!     shard.start().await?;
//!
//!     let standby = Standby::new();
//!
//!     while let Some(event) = events.next().await {
//!         // Have standby process the event, which will fulfill any futures that
//!         // are waiting for an event.
//!         standby.process(&event);
//!
//!         match event {
//!             Event::MessageCreate(msg) if msg.content == "!react" => {
//!                 tokio::spawn(react(msg.0, standby.clone()));
//!             },
//!             _ => {},
//!         }
//!     }
//!
//!     Ok(())
//! }
//!
//! // Wait for a reaction from the user who sent the message, and then print it
//! // once they react.
//! async fn react(msg: Message, standby: Standby) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//!     let author_id = msg.author.id;
//!
//!     let reaction = standby.wait_for_reaction(msg.id, move |event: &ReactionAdd| {
//!         event.user_id == author_id
//!     }).await?;
//!
//!     println!("user reacted with {:?}", reaction.emoji);
//!
//!     Ok(())
//! }
//! ```
//!
//! For more examples, check out each of the methods on [`Standby`].
//!
//! [`Standby`]: struct.Standby.html
//! [`Standby::process`]: struct.Standby.html#method.process
//! [`Standby::wait_for`]: struct.Standby.html#method.wait_for
//! [`Standby::wait_for_event`]: struct.Standby.html#method.wait_for_event
//! [`Standby::wait_for_message`]: struct.Standby.html#method.wait_for_message
//! [`Standby::wait_for_message_stream`]: struct.Standby.html#method.wait_for_message_stream
//! [`Standby::wait_for_reaction`]: struct.Standby.html#method.wait_for_reaction

mod futures;

pub use futures::{
    WaitForEventFuture, WaitForEventStream, WaitForGuildEventFuture, WaitForGuildEventStream,
    WaitForMessageFuture, WaitForMessageStream, WaitForReactionFuture, WaitForReactionStream,
};

use dashmap::DashMap;
use futures_channel::{
    mpsc::{self, UnboundedSender as MpscSender},
    oneshot::{self, Sender as OneshotSender},
};
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    sync::Arc,
};
use twilight_model::{
    channel::Channel,
    gateway::{
        event::{Event, EventType},
        payload::{MessageCreate, ReactionAdd},
    },
    id::{ChannelId, GuildId, MessageId},
};

enum Sender<E> {
    Mpsc(MpscSender<E>),
    Oneshot(OneshotSender<E>),
}

impl<E> Sender<E> {
    fn is_closed(&self) -> bool {
        match self {
            Self::Mpsc(sender) => sender.is_closed(),
            Self::Oneshot(sender) => sender.is_canceled(),
        }
    }
}

struct Bystander<E> {
    func: Box<dyn Fn(&E) -> bool + Send + Sync>,
    sender: Option<Sender<E>>,
}

impl<E> Debug for Bystander<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Bystander")
            .field("check", &"check func")
            .field("sender", &"mpsc sender")
            .finish()
    }
}

#[derive(Debug, Default)]
struct StandbyRef {
    events: DashMap<EventType, Vec<Bystander<Event>>>,
    guilds: DashMap<GuildId, Vec<Bystander<Event>>>,
    messages: DashMap<ChannelId, Vec<Bystander<MessageCreate>>>,
    reactions: DashMap<MessageId, Vec<Bystander<ReactionAdd>>>,
}

/// The `Standby` struct, used by the main event loop to process events and by
/// tasks to wait for an event.
///
/// Refer to the crate-level documentation for more information.
#[derive(Clone, Debug, Default)]
pub struct Standby(Arc<StandbyRef>);

impl Standby {
    /// Create a new instance of `Standby`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Process an event, calling any bystanders that might be waiting on it.
    ///
    /// When a bystander checks to see if an event is what it's waiting for, it
    /// will receive the event by cloning it.
    ///
    /// This function must be called when events are received in order for
    /// futures returned by methods to fulfill.
    pub fn process(&self, event: &Event) {
        tracing::trace!("processing event: {:?}", event);

        match event {
            Event::MessageCreate(e) => return self.process_message(e.0.channel_id, &e),
            Event::ReactionAdd(e) => return self.process_reaction(e.0.message_id, &e),
            _ => {}
        }

        match event_guild_id(event) {
            Some(guild_id) => self.process_guild(guild_id, event),
            None => self.process_event(event),
        }
    }

    /// Wait for an event in a certain guild.
    ///
    /// Returns a Canceled error if the Standby struct was dropped.
    ///
    /// If you need to wait for multiple guild events matching the given
    /// predicate, use [`wait_for_stream`].
    ///
    /// # Examples
    ///
    /// Wait for a `BanAdd` event in guild 123:
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use futures_util::future;
    /// use twilight_model::{
    ///     gateway::event::{EventType, Event},
    ///     id::GuildId,
    /// };
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let reaction = standby.wait_for(GuildId(123), |event: &Event| {
    ///     event.kind() == EventType::BanAdd
    /// }).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`Standby`]: struct.Standby.html
    /// [`wait_for_stream`]: #method.wait_for_stream
    pub fn wait_for<F: Fn(&Event) -> bool + Send + Sync + 'static>(
        &self,
        guild_id: GuildId,
        check: impl Into<Box<F>>,
    ) -> WaitForGuildEventFuture {
        tracing::trace!("waiting for event in guild {}", guild_id);
        let (tx, rx) = oneshot::channel();

        {
            let mut guild = self.0.guilds.entry(guild_id).or_default();
            guild.push(Bystander {
                func: check.into(),
                sender: Some(Sender::Oneshot(tx)),
            });
        }

        WaitForGuildEventFuture { rx }
    }

    /// Wait for a stream of events in a certain guild.
    ///
    /// Returns a Canceled error if the Standby struct was dropped.
    ///
    /// If you need to wait for only one guild event matching the given
    /// predicate, use [`wait_for`].
    ///
    /// # Examples
    ///
    /// Wait for multiple `BanAdd` events in guild 123:
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use futures_util::stream::StreamExt;
    /// use twilight_model::{
    ///     gateway::event::{EventType, Event},
    ///     id::GuildId,
    /// };
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let mut stream = standby.wait_for_stream(GuildId(123), |event: &Event| {
    ///     event.kind() == EventType::BanAdd
    /// });
    ///
    /// while let Some(event) = stream.next().await {
    ///     if let Event::BanAdd(ban) = event {
    ///         println!("user {} was banned in guild {}", ban.user.id, ban.guild_id);
    ///     }
    ///  }
    /// # Ok(()) }
    /// ```
    ///
    /// [`Standby`]: struct.Standby.html
    /// [`wait_for`]: #method.wait_for
    pub fn wait_for_stream<F: Fn(&Event) -> bool + Send + Sync + 'static>(
        &self,
        guild_id: GuildId,
        check: impl Into<Box<F>>,
    ) -> WaitForGuildEventStream {
        tracing::trace!("waiting for event in guild {}", guild_id);
        let (tx, rx) = mpsc::unbounded();

        {
            let mut guild = self.0.guilds.entry(guild_id).or_default();
            guild.push(Bystander {
                func: check.into(),
                sender: Some(Sender::Mpsc(tx)),
            });
        }

        WaitForGuildEventStream { rx }
    }

    /// Wait for an event not in a certain guild. This must be filtered by an
    /// event type.
    ///
    /// Returns a `Canceled` error if the `Standby` struct was dropped.
    ///
    /// If you need to wait for multiple events matching the given predicate,
    /// use [`wait_for_event_stream`].
    ///
    /// # Examples
    ///
    /// Wait for a `Ready` event for shard 5:
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use futures_util::future;
    /// use twilight_model::gateway::event::{EventType, Event};
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let ready = standby.wait_for_event(EventType::Ready, |event: &Event| {
    ///     if let Event::Ready(ready) = event {
    ///         ready.shard.map(|[id, _]| id == 5).unwrap_or(false)
    ///     } else {
    ///         false
    ///     }
    /// }).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`Standby`]: struct.Standby.html
    /// [`wait_for_event_stream`]: #method.wait_for_event_stream
    pub fn wait_for_event<F: Fn(&Event) -> bool + Send + Sync + 'static>(
        &self,
        event_type: EventType,
        check: impl Into<Box<F>>,
    ) -> WaitForEventFuture {
        tracing::trace!("waiting for event {:?}", event_type);
        let (tx, rx) = oneshot::channel();

        {
            let mut guild = self.0.events.entry(event_type).or_default();
            guild.push(Bystander {
                func: check.into(),
                sender: Some(Sender::Oneshot(tx)),
            });
        }

        WaitForEventFuture { rx }
    }

    /// Wait for a stream of events not in a certain guild. This must be
    /// filtered by an event type.
    ///
    /// Returns a `Canceled` error if the `Standby` struct was dropped.
    ///
    /// If you need to wait for only one event matching the given predicate, use
    /// [`wait_for_event`].
    ///
    /// # Examples
    ///
    /// Wait for multiple `Ready` events on shard 5:
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use futures_util::stream::StreamExt;
    /// use twilight_model::gateway::event::{EventType, Event};
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let mut events = standby.wait_for_event_stream(EventType::Ready, |event: &Event| {
    ///     if let Event::Ready(ready) = event {
    ///         ready.shard.map(|[id, _]| id == 5).unwrap_or(false)
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
    /// [`Standby`]: struct.Standby.html
    /// [`wait_for_event`]: #method.wait_for_event
    pub fn wait_for_event_stream<F: Fn(&Event) -> bool + Send + Sync + 'static>(
        &self,
        event_type: EventType,
        check: impl Into<Box<F>>,
    ) -> WaitForEventStream {
        tracing::trace!("waiting for event {:?}", event_type);
        let (tx, rx) = mpsc::unbounded();

        {
            let mut guild = self.0.events.entry(event_type).or_default();
            guild.push(Bystander {
                func: check.into(),
                sender: Some(Sender::Mpsc(tx)),
            });
        }

        WaitForEventStream { rx }
    }

    /// Wait for a message in a certain channel.
    ///
    /// Returns a `Canceled` error if the `Standby` struct was dropped.
    ///
    /// If you need to wait for multiple messages matching the given predicate,
    /// use [`wait_for_message_stream`].
    ///
    /// # Examples
    ///
    /// Wait for a message in channel 123 by user 456 with the content "test":
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use futures_util::future;
    /// use twilight_model::{gateway::payload::MessageCreate, id::{ChannelId, UserId}};
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let message = standby.wait_for_message(ChannelId(123), |event: &MessageCreate| {
    ///     event.author.id == UserId(456) && event.content == "test"
    /// }).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`Standby`]: struct.Standby.html
    /// [`wait_for_message_stream`]: #method.wait_for_message_stream
    pub fn wait_for_message<F: Fn(&MessageCreate) -> bool + Send + Sync + 'static>(
        &self,
        channel_id: ChannelId,
        check: impl Into<Box<F>>,
    ) -> WaitForMessageFuture {
        tracing::trace!("waiting for message in channel {}", channel_id);
        let (tx, rx) = oneshot::channel();

        {
            let mut guild = self.0.messages.entry(channel_id).or_default();
            guild.push(Bystander {
                func: check.into(),
                sender: Some(Sender::Oneshot(tx)),
            });
        }

        WaitForMessageFuture { rx }
    }

    /// Wait for a stream of message in a certain channel.
    ///
    /// Returns a `Canceled` error if the `Standby` struct was dropped.
    ///
    /// If you need to wait for only one message matching the given predicate,
    /// use [`wait_for_message`].
    ///
    /// # Examples
    ///
    /// Wait for multiple messages in channel 123 by user 456 with the content
    /// "test":
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use futures_util::stream::StreamExt;
    /// use twilight_model::{gateway::payload::MessageCreate, id::{ChannelId, UserId}};
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let mut messages = standby.wait_for_message_stream(ChannelId(123), |event: &MessageCreate| {
    ///     event.author.id == UserId(456) && event.content == "test"
    /// });
    ///
    /// while let Some(message) = messages.next().await {
    ///     println!("got message by {}", message.author.id);
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// [`Standby`]: struct.Standby.html
    /// [`wait_for_message`]: #method.wait_for_message
    pub fn wait_for_message_stream<F: Fn(&MessageCreate) -> bool + Send + Sync + 'static>(
        &self,
        channel_id: ChannelId,
        check: impl Into<Box<F>>,
    ) -> WaitForMessageStream {
        tracing::trace!("waiting for message in channel {}", channel_id);
        let (tx, rx) = mpsc::unbounded();

        {
            let mut guild = self.0.messages.entry(channel_id).or_default();
            guild.push(Bystander {
                func: check.into(),
                sender: Some(Sender::Mpsc(tx)),
            });
        }

        WaitForMessageStream { rx }
    }

    /// Wait for a reaction on a certain message.
    ///
    /// Returns a `Canceled` error if the `Standby` struct was dropped.
    ///
    /// If you need to wait for multiple reactions matching the given predicate,
    /// use [`wait_for_reaction_stream`].
    ///
    /// # Examples
    ///
    /// Wait for a reaction on message 123 by user 456:
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use futures_util::future;
    /// use twilight_model::{gateway::payload::ReactionAdd, id::{MessageId, UserId}};
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let reaction = standby.wait_for_reaction(MessageId(123), |event: &ReactionAdd| {
    ///     event.user_id == UserId(456)
    /// }).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`Standby`]: struct.Standby.html
    /// [`wait_for_reaction_stream`]: #method.wait_for_reaction_stream
    pub fn wait_for_reaction<F: Fn(&ReactionAdd) -> bool + Send + Sync + 'static>(
        &self,
        message_id: MessageId,
        check: impl Into<Box<F>>,
    ) -> WaitForReactionFuture {
        tracing::trace!("waiting for reaction on message {}", message_id);
        let (tx, rx) = oneshot::channel();

        {
            let mut guild = self.0.reactions.entry(message_id).or_default();
            guild.push(Bystander {
                func: check.into(),
                sender: Some(Sender::Oneshot(tx)),
            });
        }

        WaitForReactionFuture { rx }
    }

    /// Wait for a stream of reactions on a certain message.
    ///
    /// Returns a `Canceled` error if the `Standby` struct was dropped.
    ///
    /// If you need to wait for only one reaction matching the given predicate,
    /// use [`wait_for_reaction`].
    ///
    /// # Examples
    ///
    /// Wait for multiple reactions on message 123 with unicode reaction "🤠":
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use futures_util::stream::StreamExt;
    /// use twilight_model::{
    ///     channel::ReactionType,
    ///     gateway::payload::ReactionAdd,
    ///     id::{MessageId, UserId},
    /// };
    /// use twilight_standby::Standby;
    ///
    /// let standby = Standby::new();
    ///
    /// let mut reactions = standby.wait_for_reaction_stream(MessageId(123), |event: &ReactionAdd| {
    ///     matches!(&event.emoji, ReactionType::Unicode { name } if name == "🤠")
    /// });
    ///
    /// while let Some(reaction) = reactions.next().await {
    ///     println!("got a reaction by {}", reaction.user_id);
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// [`Standby`]: struct.Standby.html
    /// [`wait_for_reaction`]: #method.wait_for_reaction
    pub fn wait_for_reaction_stream<F: Fn(&ReactionAdd) -> bool + Send + Sync + 'static>(
        &self,
        message_id: MessageId,
        check: impl Into<Box<F>>,
    ) -> WaitForReactionStream {
        tracing::trace!("waiting for reaction on message {}", message_id);
        let (tx, rx) = mpsc::unbounded();

        {
            let mut guild = self.0.reactions.entry(message_id).or_default();
            guild.push(Bystander {
                func: check.into(),
                sender: Some(Sender::Mpsc(tx)),
            });
        }

        WaitForReactionStream { rx }
    }

    fn process_event(&self, event: &Event) {
        tracing::trace!("processing event type {:?}", event);
        let kind = event.kind();

        let remove = match self.0.events.get_mut(&kind) {
            Some(mut bystanders) => {
                self.iter_bystanders(&mut bystanders, event);

                bystanders.is_empty()
            }
            None => {
                tracing::trace!("event type {:?} has no bystanders", kind);

                return;
            }
        };

        if remove {
            tracing::trace!("removing event type {:?}", kind);

            self.0.events.remove(&kind);
        }
    }

    fn process_guild(&self, guild_id: GuildId, event: &Event) {
        let remove = match self.0.guilds.get_mut(&guild_id) {
            Some(mut bystanders) => {
                self.iter_bystanders(&mut bystanders, event);

                bystanders.is_empty()
            }
            None => {
                tracing::trace!("guild {} has no event bystanders", guild_id);

                return;
            }
        };

        if remove {
            tracing::trace!("removing guild {}", guild_id);

            self.0.guilds.remove(&guild_id);
        }
    }

    fn process_message(&self, channel_id: ChannelId, event: &MessageCreate) {
        let remove = match self.0.messages.get_mut(&channel_id) {
            Some(mut bystanders) => {
                self.iter_bystanders(&mut bystanders, event);

                bystanders.is_empty()
            }
            None => {
                tracing::trace!("channel {} has no message bystanders", channel_id);

                return;
            }
        };

        if remove {
            tracing::trace!("removing channel {}", channel_id);

            self.0.messages.remove(&channel_id);
        }
    }

    fn process_reaction(&self, message_id: MessageId, event: &ReactionAdd) {
        let remove = match self.0.reactions.get_mut(&message_id) {
            Some(mut bystanders) => {
                self.iter_bystanders(&mut bystanders, event);

                bystanders.is_empty()
            }
            None => {
                tracing::trace!("message {} has no reaction bystanders", message_id);

                return;
            }
        };

        if remove {
            tracing::trace!("removing message {}", message_id);
            self.0.reactions.remove(&message_id);
        }
    }

    /// Iterate over bystanders and remove the ones that match the predicate.
    fn iter_bystanders<E: Clone>(&self, bystanders: &mut Vec<Bystander<E>>, event: &E) {
        tracing::trace!("iterating over bystanders: {:?}", bystanders);

        let mut idx = 0;

        while idx < bystanders.len() {
            tracing::trace!("checking bystander");
            let bystander = &mut bystanders[idx];

            let sender = match bystander.sender.take() {
                Some(sender) => sender,
                None => {
                    tracing::trace!("bystander has no sender, removing");
                    bystanders.remove(idx);
                    idx += 1;

                    continue;
                }
            };

            if sender.is_closed() {
                tracing::trace!("bystander's rx dropped, removing");
                bystanders.remove(idx);

                continue;
            }

            if !(bystander.func)(event) {
                tracing::trace!("bystander check doesn't match, continuing");
                bystander.sender.replace(sender);
                idx += 1;

                continue;
            }

            match sender {
                Sender::Oneshot(tx) => {
                    let _ = tx.send(event.clone());
                    tracing::trace!("bystander matched event, removing");
                    bystanders.remove(idx);
                }
                Sender::Mpsc(tx) => {
                    if tx.unbounded_send(event.clone()).is_ok() {
                        bystander.sender.replace(Sender::Mpsc(tx));
                        idx += 1;
                    } else {
                        bystanders.remove(idx);
                    }
                }
            }
        }
    }
}

fn event_guild_id(event: &Event) -> Option<GuildId> {
    match event {
        Event::BanAdd(e) => Some(e.guild_id),
        Event::BanRemove(e) => Some(e.guild_id),
        Event::ChannelCreate(e) => channel_guild_id(e),
        Event::ChannelDelete(e) => channel_guild_id(e),
        Event::ChannelPinsUpdate(_) => None,
        Event::ChannelUpdate(e) => channel_guild_id(e),
        Event::GatewayHeartbeatAck => None,
        Event::GatewayHeartbeat(_) => None,
        Event::GatewayHello(_) => None,
        Event::GatewayInvalidateSession(_) => None,
        Event::GatewayReconnect => None,
        Event::GiftCodeUpdate => None,
        Event::GuildCreate(e) => Some(e.id),
        Event::GuildDelete(e) => Some(e.id),
        Event::GuildEmojisUpdate(e) => Some(e.guild_id),
        Event::GuildIntegrationsUpdate(e) => Some(e.guild_id),
        Event::GuildUpdate(e) => Some(e.id),
        Event::InviteCreate(e) => Some(e.guild_id),
        Event::InviteDelete(e) => Some(e.guild_id),
        Event::MemberAdd(e) => Some(e.guild_id),
        Event::MemberChunk(e) => Some(e.guild_id),
        Event::MemberRemove(e) => Some(e.guild_id),
        Event::MemberUpdate(e) => Some(e.guild_id),
        Event::MessageCreate(e) => e.guild_id,
        Event::MessageDelete(_) => None,
        Event::MessageDeleteBulk(_) => None,
        Event::MessageUpdate(_) => None,
        Event::PresenceUpdate(e) => Some(e.guild_id),
        Event::PresencesReplace => None,
        Event::ReactionAdd(e) => e.guild_id,
        Event::ReactionRemove(e) => e.guild_id,
        Event::ReactionRemoveAll(e) => e.guild_id,
        Event::ReactionRemoveEmoji(e) => Some(e.guild_id),
        Event::Ready(_) => None,
        Event::Resumed => None,
        Event::RoleCreate(e) => Some(e.guild_id),
        Event::RoleDelete(e) => Some(e.guild_id),
        Event::RoleUpdate(e) => Some(e.guild_id),
        Event::ShardConnected(_) => None,
        Event::ShardConnecting(_) => None,
        Event::ShardDisconnected(_) => None,
        Event::ShardIdentifying(_) => None,
        Event::ShardPayload(_) => None,
        Event::ShardReconnecting(_) => None,
        Event::ShardResuming(_) => None,
        Event::TypingStart(e) => e.guild_id,
        Event::UnavailableGuild(e) => Some(e.id),
        Event::UserUpdate(_) => None,
        Event::VoiceServerUpdate(e) => e.guild_id,
        Event::VoiceStateUpdate(e) => e.0.guild_id,
        Event::WebhooksUpdate(e) => Some(e.guild_id),
    }
}

fn channel_guild_id(channel: &Channel) -> Option<GuildId> {
    match channel {
        Channel::Guild(c) => c.guild_id(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::Standby;
    use futures_util::StreamExt;
    use std::collections::HashMap;
    use twilight_model::{
        channel::{
            message::{Message, MessageType},
            Reaction, ReactionType,
        },
        gateway::{
            event::{Event, EventType},
            payload::{MessageCreate, ReactionAdd, Ready, RoleDelete},
        },
        id::{ChannelId, GuildId, MessageId, RoleId, UserId},
        user::{CurrentUser, User},
    };

    fn message() -> Message {
        Message {
            id: MessageId(3),
            activity: None,
            application: None,
            attachments: Vec::new(),
            author: User {
                avatar: None,
                bot: false,
                discriminator: "0001".to_owned(),
                email: None,
                flags: None,
                id: UserId(2),
                locale: None,
                mfa_enabled: None,
                name: "twilight".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
            channel_id: ChannelId(1),
            content: "test".to_owned(),
            edited_timestamp: None,
            embeds: Vec::new(),
            flags: None,
            guild_id: Some(GuildId(4)),
            kind: MessageType::Regular,
            member: None,
            mention_channels: Vec::new(),
            mention_everyone: false,
            mention_roles: Vec::new(),
            mentions: HashMap::new(),
            pinned: false,
            reactions: Vec::new(),
            reference: None,
            timestamp: String::new(),
            tts: false,
            webhook_id: None,
        }
    }

    fn reaction() -> Reaction {
        Reaction {
            channel_id: ChannelId(2),
            emoji: ReactionType::Unicode {
                name: "🍎".to_owned(),
            },
            guild_id: Some(GuildId(1)),
            member: None,
            message_id: MessageId(4),
            user_id: UserId(3),
        }
    }

    #[tokio::test]
    async fn test_wait_for() {
        let standby = Standby::new();
        let wait = standby.wait_for(GuildId(1), |event: &Event| match event {
            Event::RoleDelete(e) => e.guild_id == GuildId(1),
            _ => false,
        });
        standby.process(&Event::RoleDelete(RoleDelete {
            guild_id: GuildId(1),
            role_id: RoleId(2),
        }));

        assert!(matches!(
            wait.await,
            Ok(Event::RoleDelete(RoleDelete {
                guild_id: GuildId(1),
                role_id: RoleId(2),
            }))
        ));
        assert!(standby.0.guilds.is_empty());
    }

    #[tokio::test]
    async fn test_wait_for_stream() {
        let standby = Standby::new();
        let mut stream = standby.wait_for_stream(
            GuildId(1),
            |event: &Event| matches!(event, Event::RoleDelete(e) if e.guild_id.0 == 1),
        );
        standby.process(&Event::RoleDelete(RoleDelete {
            guild_id: GuildId(1),
            role_id: RoleId(2),
        }));
        standby.process(&Event::RoleDelete(RoleDelete {
            guild_id: GuildId(1),
            role_id: RoleId(3),
        }));

        assert!(matches!(
            stream.next().await,
            Some(Event::RoleDelete(RoleDelete {
                guild_id: GuildId(1),
                role_id: RoleId(2),
            }))
        ));
        assert!(matches!(
            stream.next().await,
            Some(Event::RoleDelete(RoleDelete {
                guild_id: GuildId(1),
                role_id: RoleId(3),
            }))
        ));
        assert!(!standby.0.guilds.is_empty());
        drop(stream);
        standby.process(&Event::RoleDelete(RoleDelete {
            guild_id: GuildId(1),
            role_id: RoleId(4),
        }));
        assert!(standby.0.guilds.is_empty());
    }

    #[tokio::test]
    async fn test_wait_for_event() {
        let ready = Ready {
            guilds: HashMap::new(),
            session_id: String::new(),
            shard: Some([5, 7]),
            user: CurrentUser {
                avatar: None,
                bot: false,
                discriminator: "0001".to_owned(),
                email: None,
                id: UserId(1),
                mfa_enabled: true,
                name: "twilight".to_owned(),
                verified: false,
            },
            version: 6,
        };
        let event = Event::Ready(Box::new(ready));

        let standby = Standby::new();
        let wait = standby.wait_for_event(EventType::Ready, |event: &Event| match event {
            Event::Ready(ready) => ready.shard.map(|[id, _]| id == 5).unwrap_or(false),
            _ => false,
        });
        assert!(!standby.0.events.is_empty());
        standby.process(&event);

        assert_eq!(Ok(event), wait.await);
        assert!(standby.0.events.is_empty());
    }

    #[tokio::test]
    async fn test_wait_for_event_stream() {
        let standby = Standby::new();
        let mut stream = standby.wait_for_event_stream(EventType::Resumed, |_: &Event| true);
        standby.process(&Event::Resumed);
        assert_eq!(stream.next().await, Some(Event::Resumed));
        assert!(!standby.0.events.is_empty());
        drop(stream);
        standby.process(&Event::Resumed);
        assert!(standby.0.events.is_empty());
    }

    #[tokio::test]
    async fn test_wait_for_message() {
        let message = message();
        let event = Event::MessageCreate(Box::new(MessageCreate(message)));

        let standby = Standby::new();
        let wait = standby.wait_for_message(ChannelId(1), |message: &MessageCreate| {
            message.author.id == UserId(2)
        });
        standby.process(&event);

        assert_eq!(Ok(MessageId(3)), wait.await.map(|msg| msg.id));
        assert!(standby.0.messages.is_empty());
    }

    #[tokio::test]
    async fn test_wait_for_message_stream() {
        let standby = Standby::new();
        let mut stream = standby.wait_for_message_stream(ChannelId(1), |_: &MessageCreate| true);
        standby.process(&Event::MessageCreate(Box::new(MessageCreate(message()))));
        standby.process(&Event::MessageCreate(Box::new(MessageCreate(message()))));

        assert!(stream.next().await.is_some());
        assert!(stream.next().await.is_some());
        drop(stream);
        assert_eq!(1, standby.0.messages.len());
        standby.process(&Event::MessageCreate(Box::new(MessageCreate(message()))));
        assert!(standby.0.messages.is_empty());
    }

    #[tokio::test]
    async fn test_wait_for_reaction() {
        let event = Event::ReactionAdd(Box::new(ReactionAdd(reaction())));

        let standby = Standby::new();
        let wait = standby.wait_for_reaction(MessageId(4), |reaction: &ReactionAdd| {
            reaction.user_id == UserId(3)
        });

        standby.process(&event);

        assert_eq!(Ok(UserId(3)), wait.await.map(|reaction| reaction.user_id));
        assert!(standby.0.reactions.is_empty());
    }

    #[tokio::test]
    async fn test_wait_for_reaction_stream() {
        let standby = Standby::new();
        let mut stream = standby.wait_for_reaction_stream(MessageId(4), |_: &ReactionAdd| true);
        standby.process(&Event::ReactionAdd(Box::new(ReactionAdd(reaction()))));
        standby.process(&Event::ReactionAdd(Box::new(ReactionAdd(reaction()))));

        assert!(stream.next().await.is_some());
        assert!(stream.next().await.is_some());
        drop(stream);
        assert_eq!(1, standby.0.reactions.len());
        standby.process(&Event::ReactionAdd(Box::new(ReactionAdd(reaction()))));
        assert!(standby.0.reactions.is_empty());
    }

    #[tokio::test]
    async fn test_handles_wrong_events() {
        let standby = Standby::new();
        let wait = standby.wait_for_event(EventType::Resumed, |event: &Event| {
            matches!(event, Event::Resumed)
        });

        standby.process(&Event::PresencesReplace);
        standby.process(&Event::PresencesReplace);
        standby.process(&Event::Resumed);

        assert_eq!(Ok(Event::Resumed), wait.await);
    }
}
