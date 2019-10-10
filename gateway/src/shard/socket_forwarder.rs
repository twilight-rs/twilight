use super::{Result, ShardStream};
use futures_channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures_util::{
    future::{self, Either},
    stream::StreamExt,
};
use log::debug;
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

    pub async fn run(&mut self) -> Result<()> {
        debug!("[SocketForwarder] Starting driving loop");

        loop {
            match future::select(self.rx.next(), self.stream.next()).await {
                Either::Left((Some(msg), _)) => {
                    self.stream.send(msg).await.unwrap();
                },
                Either::Left((None, _)) => {
                    self.stream.close(None).await.unwrap();

                    return Ok(());
                },
                Either::Right((Some(Ok(msg)), _)) => {
                    self.tx.unbounded_send(msg).unwrap();
                },
                Either::Right((Some(Err(_)), _)) | Either::Right((None, _)) => {
                    self.tx.close_channel();
                },
            }
        }
    }
}
