//! This example demonstrates the usage of [`InMemoryCache`] with custom cached
//! types. The actual fields stored here are kept to a minimum for the sake of
//! simplicity, in reality you may want to store a lot more information.

use std::env;
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client;

mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Select rustls backend
    rustls::crypto::ring::default_provider().install_default().unwrap();

    let event_types = EventTypeFlags::MESSAGE_CREATE
        | EventTypeFlags::GUILD_CREATE
        | EventTypeFlags::GUILD_UPDATE
        | EventTypeFlags::GUILD_DELETE
        | EventTypeFlags::MEMBER_ADD
        | EventTypeFlags::MEMBER_REMOVE;

    let mut shard = Shard::new(
        ShardId::ONE,
        env::var("DISCORD_TOKEN")?,
        Intents::GUILDS
            | Intents::GUILD_MEMBERS
            | Intents::GUILD_MESSAGES
            | Intents::MESSAGE_CONTENT,
    );

    let client = Client::new(env::var("DISCORD_TOKEN")?);

    let cache = models::CustomInMemoryCache::new();

    while let Some(item) = shard.next_event(event_types).await {
        let Ok(event) = item else {
            tracing::warn!(source = ?item.unwrap_err(), "error receiving event");

            continue;
        };

        cache.update(&event);

        if let Event::MessageCreate(msg) = event {
            if !msg.content.starts_with("!guild-info") {
                continue;
            }

            let Some(guild_id) = msg.guild_id else {
                continue;
            };

            let Some(guild) = cache.guild(guild_id) else {
                continue;
            };

            let text = format!(
                "The owner of this server is <@{}>. The guild currently has {} members.",
                guild.owner_id,
                guild
                    .member_count
                    .map_or(String::from("N/A"), |c| c.to_string()),
            );

            client.create_message(msg.channel_id).content(&text).await?;
        }
    }

    Ok(())
}
