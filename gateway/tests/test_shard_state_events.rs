use futures::stream::StreamExt;
use std::{env, error::Error};
use twilight_gateway::{Event, Shard};

fn shard() -> Result<Shard, Box<dyn Error>> {
    let token = env::var("DISCORD_TOKEN")?;

    Ok(Shard::new(token))
}

#[ignore]
#[tokio::test]
async fn test_shard_event_emits() -> Result<(), Box<dyn Error>> {
    let mut shard = shard()?;
    let mut events = shard.events();
    shard.start().await?;

    assert!(matches!(events.next().await.unwrap(), Event::ShardConnecting(c) if c.shard_id == 0));
    assert!(matches!(events.next().await.unwrap(), Event::ShardIdentifying(c) if c.shard_id == 0));
    assert!(matches!(events.next().await.unwrap(), Event::GatewayHello(x) if x > 0));
    assert!(matches!(events.next().await.unwrap(), Event::ShardConnected(c) if c.shard_id == 0));
    assert!(matches!(events.next().await.unwrap(), Event::Ready(_)));
    assert!(matches!(
        events.next().await.unwrap(),
        Event::GuildCreate(_)
    ));
    shard.command(&"bad command").await?;
    // Might have more guilds or something.
    while let Some(event) = events.next().await {
        if matches!(event, Event::ShardDisconnected(_)) {
            break;
        }
    }

    assert!(matches!(
        events.next().await.unwrap(),
        Event::ShardResuming(_)
    ));
    assert!(matches!(events.next().await.unwrap(), Event::ShardConnecting(c) if c.shard_id == 0));
    shard.shutdown();

    Ok(())
}
