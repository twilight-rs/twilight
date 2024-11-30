//! Convenient `Stream` extension trait for message deserialization.

use crate::{error::ReceiveMessageError, EventTypeFlags, Message};
use futures_core::Stream;

/// An extension trait for the [`Stream`] trait.
///
/// If you need utilities from multiple `StreamExt` traits, [underscore import]
/// this one.
///
/// [underscore import]: https://doc.rust-lang.org/reference/items/use-declarations.html#underscore-imports
pub trait StreamExt: Stream {
    /// Consumes and returns the next wanted [`Event`] in the stream or `None`
    /// if the stream is finished.
    ///
    /// `next_event()` takes a `EventTypeFlags` which is then passed along to
    /// [`parse`]. Unwanted event types are skipped.
    ///
    /// Close messages are always considered wanted and map onto
    /// [`Event::GatewayClose`].
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// async fn next_event(&mut self, wanted_event_types: EventTypeFlags) -> Option<Result<Event, ReceiveMessageError>>
    /// ```
    ///
    /// Note that because `next_event` doesn’t take ownership over the stream,
    /// the [`Stream`] type must be [`Unpin`]. If you want to use `next` with a
    /// [`!Unpin`](Unpin) stream, you’ll first have to pin the stream. This
    /// can be done by boxing the stream using [`Box::pin`] or pinning it to
    /// the stack using [`pin!`].
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. The returned future only holds onto a
    /// reference to the underlying stream, so dropping it will never lose a
    /// value.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use twilight_gateway::{Intents, Shard, ShardId};
    /// # #[tokio::main] async fn main() {
    /// # let mut shard = Shard::new(ShardId::ONE, String::new(), Intents::empty());
    /// use twilight_gateway::{Event, EventTypeFlags, StreamExt as _};
    ///
    /// while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
    ///     let Ok(event) = item else {
    ///         tracing::warn!(source = ?item.unwrap_err(), "error receiving event");
    ///
    ///         continue;
    ///     };
    ///
    ///     match event {
    ///         Event::Ready(_) => tracing::info!("ready!"),
    ///         _ => {}
    ///     }
    /// }
    /// # }
    /// ```
    ///
    /// [`Event`]: crate::Event
    /// [`Event::GatewayClose`]: crate::Event::GatewayClose
    /// [`parse`]: crate::parse
    /// [`pin!`]: std::pin::pin
    fn next_event(&mut self, wanted_event_types: EventTypeFlags) -> private::NextEvent<Self>
    where
        Self: Unpin,
    {
        private::NextEvent::new(self, wanted_event_types)
    }
}

impl<St: ?Sized> StreamExt for St where St: Stream<Item = Result<Message, ReceiveMessageError>> {}

mod private {
    //! Private module to hide the returned type from the [`next_event`](super::StreamExt::next_event)
    //! method.
    //!
    //! Effectively disallows consumers from implementing the trait.

    use crate::{error::ReceiveMessageError, json::parse, EventTypeFlags, Message};
    use futures_core::Stream;
    use std::{
        future::Future,
        pin::Pin,
        task::{ready, Context, Poll},
    };
    use twilight_model::gateway::event::Event;

    /// Future for the [`next_event`](super::StreamExt::next_event) method.
    pub struct NextEvent<'a, St: ?Sized> {
        /// Gateway event types to deserialize.
        events: EventTypeFlags,
        /// Inner wrapped stream.
        stream: &'a mut St,
    }

    impl<'a, St: ?Sized> NextEvent<'a, St> {
        /// Create a new future.
        pub fn new(stream: &'a mut St, events: EventTypeFlags) -> Self {
            Self { events, stream }
        }
    }

    impl<St: ?Sized + Stream<Item = Result<Message, ReceiveMessageError>> + Unpin> Future
        for NextEvent<'_, St>
    {
        type Output = Option<Result<Event, ReceiveMessageError>>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let events = self.events;
            let try_from_message = |message| match message {
                Message::Text(json) => parse(json, events).map(|opt| opt.map(Into::into)),
                Message::Close(frame) => Ok(Some(Event::GatewayClose(frame))),
            };

            loop {
                match ready!(Pin::new(&mut self.stream).poll_next(cx)) {
                    Some(item) => {
                        if let Some(event) = item.and_then(try_from_message).transpose() {
                            return Poll::Ready(Some(event));
                        }
                    }
                    None => return Poll::Ready(None),
                }
            }
        }
    }
}
