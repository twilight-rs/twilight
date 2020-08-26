use futures_util::stream::StreamExt;
use std::{
    env,
    time::{Duration, Instant},
};
use twilight_gateway::{Event, Shard};
use twilight_model::{gateway::payload::RequestGuildMembers, id::GuildId};

fn shard() -> Shard {
    let token = env::var("DISCORD_TOKEN").unwrap();

    Shard::new(token)
}

#[ignore]
#[tokio::test]
async fn test_shard_command_ratelimit() {
    let mut shard = shard();
    let mut events = shard.events();
    shard.start().await.unwrap();

    assert!(matches!(
        events.next().await.unwrap(),
        Event::ShardConnecting(_)
    ));
    assert!(matches!(
        events.next().await.unwrap(),
        Event::ShardIdentifying(_)
    ));
    assert!(matches!(
        events.next().await.unwrap(),
        Event::GatewayHello(_)
    ));
    assert!(matches!(
        events.next().await.unwrap(),
        Event::ShardConnected(_)
    ));
    assert!(matches!(events.next().await.unwrap(), Event::Ready(_)));

    // now that we're connected we can test sending
    let payload = RequestGuildMembers::builder(GuildId(1)).query("", None);
    let now = Instant::now();
    shard.command(&payload).await.unwrap();
    assert!(now.elapsed() < Duration::from_millis(500));
    // check that the ~500ms ratelimit has passed
    shard.command(&payload).await.unwrap();
    assert!(now.elapsed() > Duration::from_millis(500));
    shard.shutdown();
}
