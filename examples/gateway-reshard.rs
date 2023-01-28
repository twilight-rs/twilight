use futures_util::StreamExt;
use std::{env, sync::Arc, time::Duration};
use tokio::time;
use twilight_gateway::{
    stream::{ShardEventStream, ShardMessageStream},
    Config, Event, Intents, Shard, ShardId,
};
use twilight_http::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;

    let client = Arc::new(Client::new(token.clone()));

    let config = Config::new(token, Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT);
    let recommended_shards = client.gateway().authed().await?.model().await?.shards;
    let mut shards = (0..recommended_shards)
        .map(|id| Shard::new(ShardId::new(id, recommended_shards), config.clone()))
        .collect();

    loop {
        // Run `gateway_runner` and `reshard` concurrently until the first one
        // finishes.
        tokio::select! {
            // Gateway_runner only finises on errors, so break the loop and exit
            // the program.
            _ = gateway_runner(Arc::clone(&client), shards) => break,
            // Resharding complete! Time to run `gateway_runner` with the new
            // list of shards.
            Ok(Some(new_shards)) = reshard(&client, config.clone()) => {
                    // Assign the new list of shards to `shards`, dropping the
                    // old list.
                    shards = new_shards;
            },
        }
    }

    Ok(())
}

// Instrument to differentiate between the logs produced here and in `reshard`.
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
                .await?;
        }
        _ => {}
    }

    Ok(())
}

// Instrument to differentiate between the logs produced here and
// in `gateway_runner`.
#[tracing::instrument(skip_all)]
async fn reshard(client: &Client, config: Config) -> anyhow::Result<Option<Vec<Shard>>> {
    const RESHARD_DURATION: Duration = Duration::from_secs(60 * 60 * 8);

    // Reshard every eight hours.
    time::sleep(RESHARD_DURATION).await;

    let recommended_shards = client.gateway().authed().await?.model().await?.shards;
    let mut shards = (0..recommended_shards)
        .map(|id| Shard::new(ShardId::new(id, recommended_shards), config.clone()))
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
                    // When returning `None` `reshard` will be called again,
                    // retrying after `RESHARD_DURATION`.
                    // A fatal error will however most likely also be
                    // encountered for the currently running list of shards at
                    // the same time, exciting the application.
                    return Ok(None);
                }

                continue;
            }
            Some((shard, _)) => {
                identified[shard.id().number() as usize] = shard.status().is_identified();
            }
            None => return Ok(None),
        }
    }

    drop(stream);
    Ok(Some(shards))
}
