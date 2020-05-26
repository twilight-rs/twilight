use futures::StreamExt;
use std::{borrow::Borrow, env, error::Error};
use twilight_gateway::{shard::Event, Shard};
use twilight_model::{
    gateway::payload::RequestGuildMembers,
    id::{GuildId, UserId},
};

/// simple example of how to request one or more members from the gateway
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    // to interact with the gateway we first need to connect to it (with a shard or cluster)
    let shard = Shard::new(env::var("DISCORD_TOKEN")?).await?;
    println!("Created shard");

    let mut events = shard.events().await;
    while let Some(event) = events.next().await {
        match event {
            Event::GuildCreate(guildcreate) => {
                // let's request all members for caching
                // keep in mind this is also fired once for all guilds when we connect
                shard
                    .command(&RequestGuildMembers::new_all(guildcreate.id, Some(false)))
                    .await?;
            },
            Event::Ready(_ready) => {
                //commands can be send with the command function

                //we can also request the info about a single person on a server.
                // if we give it a nonce we will receive it back in the chunk
                shard
                    .command(&RequestGuildMembers::new_single_user_with_nonce(
                        GuildId(365_498_559_174_410_241),
                        UserId(106_354_106_196_570_112),
                        Some(true),
                        Some(String::from("looking_by_id")),
                    ))
                    .await?;

                // multiple is also possible
                shard
                    .command(&RequestGuildMembers::new_multi_user_with_nonce(
                        GuildId(365_498_559_174_410_241),
                        vec![
                            UserId(77_469_400_222_932_992),
                            UserId(77_812_253_511_913_472),
                        ],
                        Some(true),
                        Some(String::from("looking_by_ids")),
                    ))
                    .await?;

                // need a list of hoisters?
                shard
                    .command(&RequestGuildMembers::new_with_nonce(
                        GuildId(365_498_559_174_410_241),
                        0,
                        "!",
                        Some(false),
                        Some(String::from("hoister_list")),
                    ))
                    .await?;
            },

            Event::MemberChunk(chunk) => {
                //this is where the magic happens

                match chunk.nonce {
                    Some(nonce) => {
                        match nonce.borrow() {
                            // make sure to keep in mind chunks are limited to 1000 each
                            // so if request something that might contain more make sure to account for that
                            "looking_by_id" => {
                                println!("Received the info by id lookup {:?}.  missing info for {:?}",
                                         chunk.members,
                                         chunk.not_found)
                            }
                            "looking_by_ids" => {
                                println!("Received the info by multiple id lookup {:?}. missing info for {:?}",
                                         chunk.members,
                                         chunk.not_found)
                            },
                            "hoister_list" => {
                                println!("Received hoister list part {:?}/{:?} containing {:?} hoisters.",
                                         chunk.chunk_index+1,
                                         chunk.chunk_count,
                                         chunk.members.len())
                            }
                            _ => {
                                // just to keep the compiler happy, empty nonces are not a thing
                            }
                        }
                    },
                    None => println!(
                        "Received chunk {:?}/{:?} for guilds {:?}",
                        chunk.chunk_index + 1,
                        chunk.chunk_count,
                        chunk.guild_id
                    ),
                }
            },

            _ => {},
        }
    }

    Ok(())
}
