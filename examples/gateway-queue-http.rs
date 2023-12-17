use http_body_util::Empty;
use hyper::body::Bytes;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};
use std::env;
use tokio::sync::oneshot;
use tokio_stream::StreamExt;
use twilight_gateway::{queue::Queue, ConfigBuilder, EventTypeFlags, Intents, Shard, ShardId};

#[derive(Debug)]
struct HttpQueue(Client<HttpConnector, Empty<Bytes>>);

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
        .queue(HttpQueue(
            Client::builder(TokioExecutor::new()).build_http(),
        ))
        .build();

    let mut shard = Shard::with_config(ShardId::ONE, config);

    while let Some(item) = shard.next().await {
        let event = match item.and_then(|message| {
            twilight_gateway::deserialize_wanted(message, EventTypeFlags::all())
        }) {
            Ok(Some(event)) => event,
            Ok(None) => continue,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                continue;
            }
        };

        tracing::debug!(?event, "received event");
    }

    Ok(())
}
