use super::super::ShardStream;
use futures_channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures_util::{
    future::{self, Either},
    sink::SinkExt,
    stream::StreamExt,
};
#[allow(unused_imports)]
use log::{debug, info, trace, warn};
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;

pub struct SocketForwarder {
    rx: UnboundedReceiver<Message>,
    pub stream: ShardStream,
    tx: UnboundedSender<Message>,
}

impl SocketForwarder {
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
        const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
        debug!("[SocketForwarder] Starting driving loop");
        loop {
            match future::select(self.rx.next(), timeout(TIMEOUT, self.stream.next())).await {
                Either::Left((Some(msg), _)) => {
                    trace!("[SocketForwarder] Sending msg: {}", msg);
                    if let Err(err) = self.stream.send(msg).await {
                        warn!("[SocketForwarder] Got error when sending: {}", err);
                        break;
                    }
                },
                Either::Left((None, _)) => {
                    warn!("[SocketForwarder] Got None, closing stream");
                    let _ = self.stream.close(None).await;

                    break;
                },
                Either::Right((Ok(Some(Ok(msg))), _)) => {
                    if self.tx.unbounded_send(msg).is_err() {
                        break;
                    }
                },
                Either::Right((Ok(Some(Err(err))), _)) => {
                    warn!("[SocketForwarder] Got error: {}, closing tx", err);
                    self.tx.close_channel();
                    break;
                },
                Either::Right((Ok(None), _)) => {
                    warn!("[SocketForwarder] Got None, closing tx");
                    self.tx.close_channel();
                    break;
                },
                Either::Right((Err(why), _)) => {
                    warn!("[SocketForwarder] Error: {}", why);
                    self.tx.close_channel();
                    break;
                },
            }
        }
        warn!("[SocketForwarder] Leaving looo");
    }
}
