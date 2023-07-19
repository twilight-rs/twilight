use hyper::client::{Client, HttpConnector};
use std::env;
use tokio::sync::oneshot;
use twilight_gateway::{queue::Queue, ConfigBuilder, Intents, Shard, ShardId};

#[derive(Debug)]
struct HttpQueue(Client<HttpConnector>);

impl Queue for HttpQueue {
    fn enqueue(&self, id: u32) -> oneshot::Receiver<()> {
        let (mut tx, rx) = oneshot::channel();
        let uri = format!("http://127.0.0.1:8000/?shard={id}");
        let req = self.0.get(uri.parse().unwrap());

        tokio::spawn(async move {
            tokio::select! {
                _ = tx.closed() => {}
                res = req => {
                    match res {
                        Ok(_) => _ = tx.send(()),
                        Err(source) => tracing::info!("error sending request: {source}"),
                    }
                }
            }
        });

        rx
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let intents = Intents::GUILDS | Intents::GUILD_VOICE_STATES;

    let config = ConfigBuilder::new(token, intents)
        .queue(HttpQueue(Client::new()))
        .build();

    let mut shard = Shard::with_config(ShardId::ONE, config);

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        tracing::debug!(?event, "received event");
    }

    Ok(())
}
