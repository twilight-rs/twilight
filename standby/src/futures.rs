use futures_util::{future::FutureExt, stream::Stream};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::{
    mpsc::UnboundedReceiver as MpscReceiver,
    oneshot::{error::RecvError, Receiver},
};
use twilight_model::gateway::{
    event::Event,
    payload::incoming::{MessageCreate, ReactionAdd},
};

/// Future canceled due to Standby being dropped.
#[derive(Debug)]
pub struct Canceled(RecvError);

impl Canceled {
    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::must_use_candidate)]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        Some(Box::new(self.0))
    }
}

impl Display for Canceled {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl Error for Canceled {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

/// The future returned from [`Standby::wait_for_event`].
///
/// [`Standby::wait_for_event`]: crate::Standby::wait_for_event
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForEventFuture {
    pub(crate) rx: Receiver<Event>,
}

impl Future for WaitForEventFuture {
    type Output = Result<Event, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx).map_err(Canceled)
    }
}

/// The stream returned from [`Standby::wait_for_event_stream`].
///
/// [`Standby::wait_for_event_stream`]: crate::Standby::wait_for_event_stream
#[derive(Debug)]
#[must_use = "streams do nothing unless you poll them"]
pub struct WaitForEventStream {
    pub(crate) rx: MpscReceiver<Event>,
}

impl Stream for WaitForEventStream {
    type Item = Event;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

/// The future returned from [`Standby::wait_for`].
///
/// [`Standby::wait_for`]: crate::Standby::wait_for
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForGuildEventFuture {
    pub(crate) rx: Receiver<Event>,
}

impl Future for WaitForGuildEventFuture {
    type Output = Result<Event, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx).map_err(Canceled)
    }
}

/// The stream returned from [`Standby::wait_for_stream`].
///
/// [`Standby::wait_for_stream`]: crate::Standby::wait_for_stream
#[derive(Debug)]
#[must_use = "streams do nothing unless you poll them"]
pub struct WaitForGuildEventStream {
    pub(crate) rx: MpscReceiver<Event>,
}

impl Stream for WaitForGuildEventStream {
    type Item = Event;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

/// The future returned from [`Standby::wait_for_message`].
///
/// [`Standby::wait_for_message`]: crate::Standby::wait_for_message
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForMessageFuture {
    pub(crate) rx: Receiver<MessageCreate>,
}

impl Future for WaitForMessageFuture {
    type Output = Result<MessageCreate, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx).map_err(Canceled)
    }
}

/// The stream returned from [`Standby::wait_for_message_stream`].
///
/// [`Standby::wait_for_message_stream`]: crate::Standby::wait_for_message_stream
#[derive(Debug)]
#[must_use = "streams do nothing unless you poll them"]
pub struct WaitForMessageStream {
    pub(crate) rx: MpscReceiver<MessageCreate>,
}

impl Stream for WaitForMessageStream {
    type Item = MessageCreate;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

/// The future returned from [`Standby::wait_for_reaction`].
///
/// [`Standby::wait_for_reaction`]: crate::Standby::wait_for_reaction
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForReactionFuture {
    pub(crate) rx: Receiver<ReactionAdd>,
}

impl Future for WaitForReactionFuture {
    type Output = Result<ReactionAdd, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx).map_err(Canceled)
    }
}

/// The stream returned from [`Standby::wait_for_reaction_stream`].
///
/// [`Standby::wait_for_reaction_stream`]: crate::Standby::wait_for_reaction_stream
#[derive(Debug)]
#[must_use = "streams do nothing unless you poll them"]
pub struct WaitForReactionStream {
    pub(crate) rx: MpscReceiver<ReactionAdd>,
}

impl Stream for WaitForReactionStream {
    type Item = ReactionAdd;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        WaitForEventFuture, WaitForEventStream, WaitForGuildEventFuture, WaitForGuildEventStream,
        WaitForMessageFuture, WaitForMessageStream, WaitForReactionFuture, WaitForReactionStream,
    };
    use futures_util::stream::Stream;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, future::Future};

    assert_impl_all!(WaitForEventFuture: Debug, Future, Send, Sync);
    assert_impl_all!(WaitForGuildEventFuture: Debug, Future, Send, Sync);
    assert_impl_all!(WaitForMessageFuture: Debug, Future, Send, Sync);
    assert_impl_all!(WaitForReactionFuture: Debug, Future, Send, Sync);
    assert_impl_all!(WaitForEventStream: Debug, Stream, Send, Sync);
    assert_impl_all!(WaitForGuildEventStream: Debug, Stream, Send, Sync);
    assert_impl_all!(WaitForMessageStream: Debug, Stream, Send, Sync);
    assert_impl_all!(WaitForReactionStream: Debug, Stream, Send, Sync);
}
