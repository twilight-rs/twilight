use futures_util::stream::StreamExt;
use std::{
    env,
    time::{Duration, Instant},
};
use twilight_gateway::{Event, Intents, Shard};
use twilight_model::gateway::{
    payload::UpdateStatus,
    presence::{Activity, ActivityType, Status},
};

fn shard() -> Shard {
    let token = env::var("DISCORD_TOKEN").unwrap();

    Shard::new(token, Intents::empty())
}

#[ignore]
#[tokio::test]
async fn test_shard_command_ratelimit() {
    let shard = shard();
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
    let payload = UpdateStatus::new(
        vec![Activity {
            application_id: None,
            assets: None,
            buttons: Vec::new(),
            created_at: None,
            details: None,
            emoji: None,
            flags: None,
            id: None,
            instance: None,
            kind: ActivityType::Playing,
            name: "test".to_owned(),
            party: None,
            secrets: None,
            state: None,
            timestamps: None,
            url: None,
        }],
        false,
        Some(1),
        Status::DoNotDisturb,
    );
    let now = Instant::now();
    shard.command(&payload).await.unwrap();
    assert!(now.elapsed() < Duration::from_millis(500));
    // check that the ~500ms ratelimit has passed
    shard.command(&payload).await.unwrap();
    assert!(now.elapsed() > Duration::from_millis(500));
    shard.shutdown();
}
