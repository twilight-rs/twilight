use futures_util::StreamExt;
use std::{env, sync::Arc, time::Duration};
use tokio::time;
use twilight_gateway::{
    queue::{LocalQueue, Queue},
    stream::{self, ShardEventStream, ShardMessageStream},
    Config, ConnectionStatus, Event, Intents, Shard, ShardId,
};
use twilight_http::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let client = Arc::new(Client::new(token.clone()));
    let queue: Arc<dyn Queue> = Arc::new(LocalQueue::new());

    let config_callback = |_| {
        // A queue must be specified in the builder for the shards to reuse the
        // same one, which is necessary to not hit any gateway queue ratelimit.
        Config::builder(token.clone(), Intents::GUILDS)
            .queue(Arc::clone(&queue))
            .build()
    };
    let mut shards = stream::start_recommended(&client, &config_callback)
        .await?
        .collect::<Vec<_>>();

    loop {
        // Run `gateway_runner` and `reshard` concurrently until the first one
        // finishes.
        tokio::select! {
            // Gateway_runner only finises on errors, so break the loop and exit
            // the program.
            _ = gateway_runner(Arc::clone(&client), shards) => break,
            // Resharding complete! Time to run `gateway_runner` with the new
            // list of shards.
            Ok(Some(new_shards)) = reshard(&client, config_callback) => {
                    // Assign the new list of shards to `shards`, dropping the
                    // old list.
                    shards = new_shards;
            },
        }
    }

    Ok(())
}

// Instrument to diferentiate between the logs produced here and in `reshard`.
#[tracing::instrument(skip_all)]
async fn gateway_runner(client: Arc<Client>, mut shards: Vec<Shard>) {
    let mut stream = ShardEventStream::new(shards.iter_mut());

    loop {
        let event = match stream.next().await {
            Some((_, Ok(event))) => event,
            Some((_, Err(source))) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
            None => break,
        };

        tokio::spawn(event_handler(Arc::clone(&client), event));
    }
}

async fn event_handler(client: Arc<Client>, event: Event) -> anyhow::Result<()> {
    match event {
        Event::MessageCreate(message) if message.content == "!ping" => {
            client
                .create_message(message.channel_id)
                .content("Pong!")?
                .exec()
                .await?;
        }
        _ => {}
    }

    Ok(())
}

// Instrument to diferentiate between the logs produced here and
// in `gateway_runner`.
#[tracing::instrument(skip_all)]
async fn reshard(
    client: &Client,
    config_callback: impl Fn(ShardId) -> Config,
) -> anyhow::Result<Option<Vec<Shard>>> {
    const RESHARD_DURATION: Duration = Duration::from_secs(60 * 60 * 8);

    // Reshard every eight hours.
    time::sleep(RESHARD_DURATION).await;

    let mut shards = stream::start_recommended(client, config_callback)
        .await?
        .collect::<Vec<_>>();

    let mut identified = vec![false; shards.len()];
    // Don't deserialize any events (with `ShardEventStream`) as the already
    // running shards will handle them (the events are duplicated).
    let mut stream = ShardMessageStream::new(shards.iter_mut());

    // Drive the new list of shards until they are all identified.
    while !identified.iter().all(|&shard| shard) {
        match stream.next().await {
            Some((_, Err(source))) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    // When returing `None` `reshard` will be called again,
                    // retrying after `RESHARD_DURATION`.
                    // A fatal error will however most likely also be
                    // encountered for the currenty running list of shards at
                    // the same time, exciting the application.
                    return Ok(None);
                }

                continue;
            }
            Some((shard, _)) => {
                identified[shard.id().number() as usize] = matches!(
                    shard.status(),
                    ConnectionStatus::Connected | ConnectionStatus::Resuming
                );
            }
            None => return Ok(None),
        }
    }

    drop(stream);
    Ok(Some(shards))
}
