use futures::sink::Sink;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::mpsc::{error::SendError, UnboundedSender};
use tokio_tungstenite::tungstenite::Message;

/// A sink which tungstenite messages can be sunk into. ⚓
///
/// You get this via [`Shard::sink`].
///
/// [`Shard::sink`]: struct.Shard.html#method.sink
#[derive(Clone, Debug)]
pub struct ShardSink(pub(super) UnboundedSender<Message>);

impl Sink<Message> for ShardSink {
    type Error = SendError<Message>;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, msg: Message) -> Result<(), Self::Error> {
        self.0.send(msg)
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl Sink<Message> for &ShardSink {
    type Error = SendError<Message>;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, msg: Message) -> Result<(), Self::Error> {
        self.0.send(msg)
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}
