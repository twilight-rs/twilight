use std::env;
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_model::{gateway::payload::outgoing::RequestGuildMembers, id::Id};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Select rustls backend
    rustls::crypto::ring::default_provider().install_default().unwrap();

    let mut shard = Shard::new(
        ShardId::ONE,
        env::var("DISCORD_TOKEN")?,
        Intents::GUILD_MEMBERS | Intents::GUILDS,
    );

    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        let Ok(event) = item else {
            tracing::warn!(source = ?item.unwrap_err(), "error receiving event");

            continue;
        };

        match event {
            Event::GuildCreate(guild) => {
                // Let's request all of the guild's members for caching.
                shard.command(&RequestGuildMembers::builder(guild.id()).query("", None));
            }
            Event::Ready(_) => {
                // You can also specify an individual member within a guild.
                //
                // Additionally, you can pass in a "nonce" and get it back in
                // the received member chunk. This can be used to help identify
                // which request the member is from.
                let request = RequestGuildMembers::builder(Id::new(1))
                    .nonce("requesting a single member")
                    .user_id(Id::new(2));

                shard.command(&request);

                // Similarly, you can also request multiple members. Only 100
                // members by ID can be requested at a time, so the builder will
                // check to make sure you're requesting at most that many:
                let request = RequestGuildMembers::builder(Id::new(1))
                    .nonce("requesting two member")
                    .user_ids(vec![Id::new(2), Id::new(3)])
                    .unwrap();

                shard.command(&request);

                // Instead of specifying user IDs, you can also search for
                // members that you don't know the IDs of through their names.
                // A name query can be specified, and an optional limit to the
                // number of members to retrieve can be specified. Here we'll
                // request a list of up to 50 members and their current presence
                // details whose names start with the letters "tw":
                let request = RequestGuildMembers::builder(Id::new(1))
                    .nonce("querying for members")
                    .presences(true)
                    .query("tw", Some(50));

                shard.command(&request);
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
                            chunk.members, chunk.not_found,
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
