use super::super::ShardStream;
use futures_util::{future::FutureExt, sink::SinkExt, stream::StreamExt};
use std::time::Duration;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    time::sleep,
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
        tracing::debug!("starting driving loop");

        // This seems to come from the `if let` in the macro and may be a false
        // positive.
        #[allow(clippy::mut_mut)]
        loop {
            let timeout = sleep(Self::TIMEOUT).fuse();
            tokio::pin!(timeout);

            tokio::select! {
                maybe_msg = self.rx.recv().fuse() => {
                    if let Some(msg) = maybe_msg {
                        tracing::trace!("sending message: {}", msg);

                        if let Err(err) = self.stream.send(msg).await {
                            tracing::warn!("sending failed: {}", err);
                            break;
                        }
                    } else {
                        tracing::debug!("rx stream ended, closing socket");
                        let _res = self.stream.close(None).await;

                        break;
                    }
                },
                try_msg = self.stream.next().fuse() => {
                    match try_msg {
                        Some(Ok(msg)) => {
                            if self.tx.send(msg).is_err() {
                                break;
                            }
                        },
                        Some(Err(err)) => {
                            tracing::warn!("socket errored: {}", err);
                            break;
                        },
                        None => {
                            tracing::debug!("socket ended");
                            break;
                        }
                    }
                },
                _ = timeout => {
                    tracing::warn!("socket timed out");
                    break;
                }
            };
        }

        tracing::debug!("Leaving loop");
    }
}
