//! Convenient `Stream` extension trait for message deserialization.

use crate::{error::ReceiveMessageError, EventTypeFlags, Message};
use futures_core::Stream;

/// An extension trait for the [`Stream`] trait.
///
/// If you need utilities from multiple `StreamExt` traits, [underscore import]
/// this one.
///
/// [underscore import]: https://doc.rust-lang.org/reference/items/use-declarations.html#underscore-imports
pub trait StreamExt: Stream<Item = Result<Message, ReceiveMessageError>> + Unpin + Sized {
    /// Deserialize the messages in a given stream.
    ///
    /// `deserialize()` takes a `EventTypeFlags` to filter deserialization to
    /// only wanted event types.
    ///
    /// Close messages are always considered wanted and map onto
    /// [`Event::GatewayClose`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use twilight_gateway::{Intents, Shard, ShardId};
    /// # #[tokio::main] async fn main() {
    /// # let mut shard = Shard::new(ShardId::ONE, String::new(), Intents::empty());
    /// use tokio_stream::StreamExt;
    /// use twilight_gateway::{Event, EventTypeFlags, StreamExt as _};
    ///
    /// while let Some(item) = shard.deserialize(EventTypeFlags::all()).next().await {
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
    /// [`Event::GatewayClose`]: crate::Event::GatewayClose
    fn deserialize(&mut self, wanted_event_types: EventTypeFlags) -> private::EventStream<Self> {
        private::EventStream::new(self, wanted_event_types)
    }
}

impl<St: Stream<Item = Result<Message, ReceiveMessageError>> + Unpin> StreamExt for St {}

mod private {
    //! Private module to hide the returned type from the [`deserialize`](super::StreamExt::deserialize)
    //! method.
    //!
    //! Effectively disallows consumers from implementing the trait.

    use crate::{error::ReceiveMessageError, json::parse, EventTypeFlags, Message};
    use futures_core::Stream;
    use std::{
        pin::Pin,
        task::{ready, Context, Poll},
    };
    use twilight_model::gateway::event::Event;

    /// Stream for the [`deserialize`](super::StreamExt::deserialize) method.
    pub struct EventStream<'a, St> {
        /// Gateway event types to deserialize.
        events: EventTypeFlags,
        /// Inner wrapped stream.
        stream: &'a mut St,
    }

    impl<'a, St> EventStream<'a, St> {
        /// Create a new event stream.
        pub fn new(stream: &'a mut St, events: EventTypeFlags) -> Self {
            Self { events, stream }
        }
    }

    impl<'a, St: Stream<Item = Result<Message, ReceiveMessageError>> + Unpin> Stream
        for EventStream<'a, St>
    {
        type Item = Result<Event, ReceiveMessageError>;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
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
