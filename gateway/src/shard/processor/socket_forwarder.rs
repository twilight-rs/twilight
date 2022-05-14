use super::super::ShardStream;
use futures_util::{
    future::{self, Either},
    sink::SinkExt,
    stream::StreamExt,
};
use std::time::Duration;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    time::timeout,
};
use tokio_tungstenite::tungstenite::Message;

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
        let (to_user, from_forwarder) = mpsc::unbounded_channel();
        let (to_forwarder, from_user) = mpsc::unbounded_channel();

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
        #[cfg(feature = "tracing")]
        tracing::debug!("starting driving loop");

        loop {
            tokio::pin! {
                let rx = self.rx.recv();
                let tx = self.stream.next();
            }

            match timeout(Self::TIMEOUT, future::select(rx, tx)).await {
                // `rx` future finished first.
                Ok(Either::Left((maybe_msg, _))) => {
                    if let Some(msg) = maybe_msg {
                        #[cfg(feature = "tracing")]
                        tracing::trace!("sending message: {}", msg);

                        if let Err(_source) = self.stream.send(msg).await {
                            #[cfg(feature = "tracing")]
                            tracing::warn!("sending failed: {}", _source);

                            break;
                        }
                    } else {
                        #[cfg(feature = "tracing")]
                        tracing::debug!("rx stream ended, closing socket");

                        let _res = self.stream.close(None).await;

                        break;
                    }
                }
                // `tx` future finished first.
                Ok(Either::Right((try_msg, _))) => match try_msg {
                    Some(Ok(msg)) => {
                        if self.tx.send(msg).is_err() {
                            break;
                        }
                    }
                    Some(Err(_source)) => {
                        #[cfg(feature = "tracing")]
                        tracing::warn!("socket errored: {}", _source);

                        break;
                    }
                    None => {
                        #[cfg(feature = "tracing")]
                        tracing::debug!("socket ended");

                        break;
                    }
                },
                // Timeout future finished first.
                Err(_) => {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("socket timed out");

                    break;
                }
            }
        }

        #[cfg(feature = "tracing")]
        tracing::debug!("Leaving loop");
    }
}
