use std::{
    env,
    time::{Duration, Instant},
};
use twilight_gateway::{config::ShardId, Event, Intents, Shard};
use twilight_model::gateway::{
    payload::outgoing::UpdatePresence,
    presence::{Activity, ActivityType, Status},
};

async fn shard() -> Shard {
    let token = env::var("DISCORD_TOKEN").unwrap();

    Shard::new(ShardId::ONE, token, Intents::empty())
        .await
        .unwrap()
}

#[ignore]
#[tokio::test]
async fn shard_command_ratelimit() {
    let mut shard = shard().await;

    assert!(matches!(
        shard.next_event().await.unwrap(),
        Event::GatewayHello(_)
    ));
    assert!(matches!(shard.next_event().await.unwrap(), Event::Ready(_)));

    // now that we're connected we can test sending
    let payload = UpdatePresence::new(
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
    )
    .unwrap();
    let now = Instant::now();
    shard.command(&payload).await.unwrap();
    assert!(now.elapsed() < Duration::from_millis(500));
    // check that the ~500ms ratelimit has passed
    shard.command(&payload).await.unwrap();
    assert!(now.elapsed() > Duration::from_millis(500));
    shard.close(None).await.unwrap();
}
