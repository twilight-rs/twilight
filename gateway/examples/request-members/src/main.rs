use futures::StreamExt;
use std::{env, error::Error};
use twilight_gateway::{Event, Intents, Shard};
use twilight_model::{
    gateway::payload::RequestGuildMembers,
    id::{GuildId, UserId},
};

/// simple example of how to request one or more members from the gateway
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    // to interact with the gateway we first need to connect to it (with a shard or cluster)
    let (shard, mut events) = Shard::new(
        env::var("DISCORD_TOKEN")?,
        Intents::GUILD_MEMBERS | Intents::GUILDS,
    );
    shard.start().await?;
    println!("Created shard");

    while let Some(event) = events.next().await {
        match event {
            Event::GuildCreate(guild) => {
                // Let's request all of the guild's members for caching.
                shard
                    .command(&RequestGuildMembers::builder(guild.id).query("", None))
                    .await?;
            }
            Event::Ready(_) => {
                // You can also specify an individual member within a guild.
                //
                // Additionally, you can pass in a "nonce" and get it back in
                // the received member chunk. This can be used to help identify
                // which request the member is from.
                let request = RequestGuildMembers::builder(GuildId::new(1).expect("non zero"))
                    .nonce("requesting a single member")
                    .user_id(UserId::new(2).expect("non zero"));

                shard.command(&request).await?;

                // Similarly, you can also request multiple members. Only 100
                // members by ID can be requested at a time, so the builder will
                // check to make sure you're requesting at most that many:
                let request = RequestGuildMembers::builder(GuildId::new(1).expect("non zero"))
                    .nonce("requesting two member")
                    .user_ids(vec![
                        UserId::new(2).expect("non zero"),
                        UserId::new(3).expect("non zero"),
                    ])
                    .unwrap();

                shard.command(&request).await?;

                // Instead of specifying user IDs, you can also search for
                // members that you don't know the IDs of through their names.
                // A name query can be specified, and an optional limit to the
                // number of members to retrieve can be specified. Here we'll
                // request a list of up to 50 members and their current presence
                // details whose names start with the letters "tw":
                let request = RequestGuildMembers::builder(GuildId::new(1).expect("non zero"))
                    .nonce("querying for members")
                    .presences(true)
                    .query("tw", Some(50));

                shard.command(&request).await?;
            }
            Event::MemberChunk(chunk) => {
                // Member chunks are received in response to requests for guild
                // members. They may each contain only a portion of the
                // requested members within an individual guild.
                match chunk.nonce.as_deref() {
                    Some("requesting a single member") => {
                        println!(
                            "received the single member; found: {:?}; missing: {:?}",
                            chunk.members, chunk.not_found,
                        );
                    }
                    Some("requesting two users") => {
                        println!(
                            "received response for requesting two members; found: {:?}; missing: {:?}",
                            chunk.members,
                            chunk.not_found,
                        );
                    }
                    Some("querying for members") => {
                        println!(
                            "found members starting with 'tw'; found: {:?}; missing: {:?}",
                            chunk.members, chunk.not_found,
                        );
                    }
                    _ => println!(
                        "Received chunk {:?}/{:?} for guild {:?}",
                        chunk.chunk_index + 1,
                        chunk.chunk_count,
                        chunk.guild_id
                    ),
                }
            }

            _ => {}
        }
    }

    Ok(())
}
