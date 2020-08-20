use super::super::ShardStream;
use async_tungstenite::tungstenite::Message;
use futures_channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures_util::{
    future::{self, Either},
    sink::SinkExt,
    stream::StreamExt,
};
use std::time::Duration;
use tokio::time::timeout;

pub struct SocketForwarder {
    rx: UnboundedReceiver<Message>,
    pub stream: ShardStream,
    tx: UnboundedSender<Message>,
}

impl SocketForwarder {
    const TIMEOUT: Duration = Duration::from_secs(90);

    pub fn new(
        stream: ShardStream,
    ) -> (Self, UnboundedReceiver<Message>, UnboundedSender<Message>) {
        let (to_user, from_forwarder) = mpsc::unbounded();
        let (to_forwarder, from_user) = mpsc::unbounded();

        (
            Self {
                rx: from_user,
                stream,
                tx: to_user,
            },
            from_forwarder,
            to_forwarder,
        )
    }

    pub async fn run(mut self) {
        tracing::debug!("starting driving loop");

        loop {
            match future::select(self.rx.next(), timeout(Self::TIMEOUT, self.stream.next())).await {
                Either::Left((Some(msg), _)) => {
                    tracing::trace!("sending message: {}", msg);
                    if let Err(err) = self.stream.send(msg).await {
                        tracing::warn!("sending failed: {}", err);
                        break;
                    }
                }
                Either::Left((None, _)) => {
                    tracing::debug!("rx stream ended, closing socket");
                    let _ = self.stream.close(None).await;

                    break;
                }
                Either::Right((Ok(Some(Ok(msg))), _)) => {
                    if self.tx.unbounded_send(msg).is_err() {
                        break;
                    }
                }
                Either::Right((Ok(Some(Err(err))), _)) => {
                    tracing::warn!("socket errored, closing tx: {}", err);
                    //self.tx.close_channel();
                    break;
                }
                Either::Right((Ok(None), _)) => {
                    tracing::debug!("socket ended, closing tx");
                    //self.tx.close_channel();
                    break;
                }
                Either::Right((Err(why), _)) => {
                    tracing::warn!("socket errored, closing tx: {}", why);
                    //self.tx.close_channel();
                    break;
                }
            }
        }
        tracing::debug!("Leaving loop");
    }
}
