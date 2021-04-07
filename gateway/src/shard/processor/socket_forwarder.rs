use super::super::ShardStream;
use async_tungstenite::tungstenite::Message;
use futures_channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures_timer::Delay;
use futures_util::{future::FutureExt, sink::SinkExt, stream::StreamExt};
use std::time::Duration;

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

        // This seems to come from the `if let` in the macro and may be a false
        // positive.
        #[allow(clippy::mut_mut)]
        loop {
            let mut rx = self.rx.next();
            let mut stream = self.stream.next().fuse();
            let mut timeout = Delay::new(Self::TIMEOUT).fuse();

            futures_util::select! {
                maybe_msg = rx => {
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
                try_msg = stream => {
                    match try_msg {
                        Some(Ok(msg)) => {
                            if self.tx.unbounded_send(msg).is_err() {
                                break;
                            }
                        },
                        Some(Err(err)) => {
                            tracing::warn!("socket errored, closing tx: {}", err);
                            self.tx.close_channel();
                            break;
                        },
                        None => {
                            tracing::debug!("socket ended, closing tx");
                            self.tx.close_channel();
                            break;
                        }
                    }
                },
                _ = timeout => {
                    tracing::warn!("socket timed out, closing tx");
                    self.tx.close_channel();
                    break;
                }
            };
        }

        tracing::debug!("Leaving loop");
    }
}
