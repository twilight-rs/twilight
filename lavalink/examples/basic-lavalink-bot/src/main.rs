use futures::StreamExt;
use reqwest::Client as ReqwestClient;
use std::{convert::TryInto, env, error::Error, future::Future, net::SocketAddr, str::FromStr};
use twilight_gateway::{Event, Shard};
use twilight_http::Client as HttpClient;
use twilight_lavalink::{http::LoadedTracks, model::Play, Lavalink};
use twilight_model::{channel::Message, gateway::payload::MessageCreate};
use twilight_standby::Standby;

#[derive(Clone, Debug)]
struct State {
    http: HttpClient,
    lavalink: Lavalink,
    reqwest: ReqwestClient,
    shard: Shard,
    standby: Standby,
}

fn spawn(
    fut: impl Future<Output = Result<(), Box<dyn Error + Send + Sync + 'static>>> + Send + 'static,
) {
    tokio::spawn(async move {
        if let Err(why) = fut.await {
            log::debug!("Got an error from a handler: {:?}", why);
        }
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    pretty_env_logger::init_timed();

<<<<<<< Updated upstream
    let (state, _) = {
=======
    let (state, _rx) = {
>>>>>>> Stashed changes
        let token = env::var("DISCORD_TOKEN")?;
        let lavalink_host = SocketAddr::from_str(&env::var("LAVALINK_HOST")?)?;
        let lavalink_auth = env::var("LAVALINK_AUTHORIZATION")?;
        let shard_count = 1u64;

        let http = HttpClient::new(&token);
        let user_id = http.current_user().await?.id;

        let lavalink = Lavalink::new(user_id, shard_count);
        let rx = lavalink.add(lavalink_host, lavalink_auth).await?;

        let shard = Shard::new(token).await?;

        (
            State {
                http,
                lavalink,
                reqwest: ReqwestClient::new(),
                shard,
                standby: Standby::new(),
            },
            rx,
        )
    };

    let mut events = state.shard.events().await;

    while let Some(event) = events.next().await {
        state.standby.process(&event).await;
        state.lavalink.process(&event).await?;

<<<<<<< Updated upstream
        match event {
            Event::MessageCreate(msg) => {
=======
        log::debug!("got event");

        match event {
            Event::MessageCreate(msg) => {
                log::debug!("got msg create");

>>>>>>> Stashed changes
                if msg.guild_id.is_none() || !msg.content.starts_with("!") {
                    continue;
                }

                match msg.content.splitn(2, ' ').next() {
                    Some("!join") => spawn(join(msg.0, state.clone())),
                    Some("!play") => spawn(play(msg.0, state.clone())),
                    _ => continue,
                };
            }
            _ => {}
        }
    }

    Ok(())
}

async fn join(msg: Message, state: State) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    state
        .http
        .create_message(msg.channel_id)
        .content("What's the channel ID you want me to join?")?
        .await?;

    let author_id = msg.author.id;
    let msg = state
        .standby
        .wait_for_message(msg.channel_id, move |new_msg: &MessageCreate| {
            new_msg.author.id == author_id
        })
        .await?;
    let channel_id = msg.content.parse::<u64>()?;

    state
        .shard
        .command(&serde_json::json!({
            "op": 4,
            "d": {
                "channel_id": channel_id,
                "guild_id": msg.guild_id,
                "self_mute": false,
                "self_deaf": false,
            }
        }))
        .await?;

    state
        .http
        .create_message(msg.channel_id)
        .content(format!("Joined <#{}>!", channel_id))?
        .await?;

    Ok(())
}

async fn play(msg: Message, state: State) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    log::debug!(
        "Got a play command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );
    state
        .http
        .create_message(msg.channel_id)
        .content("What's the URL of the audio to play?")?
        .await?;

    let author_id = msg.author.id;
    let msg = state
        .standby
        .wait_for_message(msg.channel_id, move |new_msg: &MessageCreate| {
            new_msg.author.id == author_id
        })
        .await?;
    let guild_id = msg.guild_id.unwrap();

    let player = state.lavalink.player(guild_id).await.unwrap();
    let req = twilight_lavalink::http::load_track(
        player.node().config().address,
        &msg.content,
        &player.node().config().authorization,
    )?
    .try_into()?;
    let res = state.reqwest.execute(req).await?;
    let loaded = res.json::<LoadedTracks>().await?;

    if let Some(track) = loaded.tracks.first() {
<<<<<<< Updated upstream
        player.send(Play::from((guild_id, &track.track, 0, track.info.length)))?;
=======
        player.send(Play::from((guild_id, &track.track)))?;
>>>>>>> Stashed changes

        let content = format!(
            "Playing **{}** by **{}**",
            track.info.title, track.info.author
        );
        state
            .http
            .create_message(msg.channel_id)
            .content(content)?
            .await?;
    } else {
        state
            .http
            .create_message(msg.channel_id)
            .content("Didn't find any results")?
            .await?;
    }

    Ok(())
}
