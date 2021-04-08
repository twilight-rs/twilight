use super::raw_message::Message;
use async_tungstenite::tungstenite::Message as TungsteniteMessage;
use futures_channel::mpsc::{SendError, TrySendError, UnboundedSender};
use futures_util::sink::Sink;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

/// A sink which tungstenite messages can be sunk into. ⚓
///
/// You get this via [`Shard::sink`].
///
/// [`Shard::sink`]: super::Shard::sink
#[derive(Clone, Debug)]
pub struct ShardSink(pub(super) UnboundedSender<TungsteniteMessage>);

impl Sink<Message> for ShardSink {
    type Error = SendError;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, msg: Message) -> Result<(), Self::Error> {
        self.0.start_send(msg.into_tungstenite())
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.disconnect();

        Poll::Ready(Ok(()))
    }
}

impl Sink<Message> for &ShardSink {
    type Error = SendError;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, msg: Message) -> Result<(), Self::Error> {
        self.0
            .unbounded_send(msg.into_tungstenite())
            .map_err(TrySendError::into_send_error)
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.close_channel();

        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::{Message, ShardSink};
    use futures_util::sink::Sink;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ShardSink: Clone, Debug, Send, Sink<Message>, Sync);
}
