use futures::{
    channel::mpsc::{SendError, TrySendError, UnboundedSender},
    sink::Sink,
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio_tungstenite::tungstenite::Message;

/// A sink which tungstenite messages can be sunk into. ⚓
///
/// You get this via [`Shard::sink`].
///
/// [`Shard::sink`]: struct.Shard.html#method.sink
#[derive(Clone, Debug)]
pub struct ShardSink(pub(super) UnboundedSender<Message>);

impl Sink<Message> for ShardSink {
    type Error = SendError;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, msg: Message) -> Result<(), Self::Error> {
        self.0.start_send(msg)
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
            .unbounded_send(msg)
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
