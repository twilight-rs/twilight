use futures::StreamExt;
use reqwest::Client as ReqwestClient;
use std::{convert::TryInto, env, error::Error, future::Future, net::SocketAddr, str::FromStr};
use twilight_gateway::{Event, Shard};
use twilight_http::Client as HttpClient;
use twilight_lavalink::{
    http::LoadedTracks,
    model::{Destroy, Pause, Play, Seek, Stop, Volume},
    Lavalink,
};
use twilight_model::{channel::Message, gateway::payload::MessageCreate, id::ChannelId};
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
            tracing::debug!("Got an error from a handler: {:?}", why);
        }
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let state = {
        let token = env::var("DISCORD_TOKEN")?;
        let lavalink_host = SocketAddr::from_str(&env::var("LAVALINK_HOST")?)?;
        let lavalink_auth = env::var("LAVALINK_AUTHORIZATION")?;
        let shard_count = 1u64;

        let http = HttpClient::new(&token);
        let user_id = http.current_user().await?.id;

        let lavalink = Lavalink::new(user_id, shard_count);
        lavalink.add(lavalink_host, lavalink_auth).await?;

        let mut shard = Shard::new(token);
        shard.start().await?;

        State {
            http,
            lavalink,
            reqwest: ReqwestClient::new(),
            shard,
            standby: Standby::new(),
        }
    };

    let mut events = state.shard.events().await;

    while let Some(event) = events.next().await {
        state.standby.process(&event);
        state.lavalink.process(&event).await?;

        if let Event::MessageCreate(msg) = event {
            if msg.guild_id.is_none() || !msg.content.starts_with('!') {
                continue;
            }

            match msg.content.splitn(2, ' ').next() {
                Some("!join") => spawn(join(msg.0, state.clone())),
                Some("!leave") => spawn(leave(msg.0, state.clone())),
                Some("!pause") => spawn(pause(msg.0, state.clone())),
                Some("!play") => spawn(play(msg.0, state.clone())),
                Some("!seek") => spawn(seek(msg.0, state.clone())),
                Some("!stop") => spawn(stop(msg.0, state.clone())),
                Some("!volume") => spawn(volume(msg.0, state.clone())),
                _ => continue,
            }
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

async fn leave(msg: Message, state: State) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing::debug!(
        "Got a leave command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );

    let guild_id = msg.guild_id.unwrap();
    let player = state.lavalink.player(guild_id).await.unwrap();
    player.send(Destroy::from(guild_id))?;
    state
        .shard
        .command(&serde_json::json!({
            "op": 4,
            "d": {
                "channel_id": None::<ChannelId>,
                "guild_id": msg.guild_id,
                "self_mute": false,
                "self_deaf": false,
            }
        }))
        .await?;

    state
        .http
        .create_message(msg.channel_id)
        .content("Left the channel")?
        .await?;

    Ok(())
}

async fn play(msg: Message, state: State) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing::debug!(
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
        player.send(Play::from((guild_id, &track.track)))?;

        let content = format!(
            "Playing **{:?}** by **{:?}**",
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

async fn pause(msg: Message, state: State) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing::debug!(
        "Got a pause command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );

    let guild_id = msg.guild_id.unwrap();
    let player = state.lavalink.player(guild_id).await.unwrap();
    let paused = player.paused();
    player.send(Pause::from((guild_id, !paused)))?;

    let action = if paused { "Unpaused " } else { "Paused" };

    state
        .http
        .create_message(msg.channel_id)
        .content(format!("{} the track", action))?
        .await?;

    Ok(())
}

async fn seek(msg: Message, state: State) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing::debug!(
        "Got a seek command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );
    state
        .http
        .create_message(msg.channel_id)
        .content("Where in the track do you want to seek to (in seconds)?")?
        .await?;

    let author_id = msg.author.id;
    let msg = state
        .standby
        .wait_for_message(msg.channel_id, move |new_msg: &MessageCreate| {
            new_msg.author.id == author_id
        })
        .await?;
    let guild_id = msg.guild_id.unwrap();
    let position = msg.content.parse::<i64>()?;

    let player = state.lavalink.player(guild_id).await.unwrap();
    player.send(Seek::from((guild_id, position * 1000)))?;

    state
        .http
        .create_message(msg.channel_id)
        .content(format!("Seeked to {}s", position))?
        .await?;

    Ok(())
}

async fn stop(msg: Message, state: State) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing::debug!(
        "Got a stop command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );

    let guild_id = msg.guild_id.unwrap();
    let player = state.lavalink.player(guild_id).await.unwrap();
    player.send(Stop::from(guild_id))?;

    state
        .http
        .create_message(msg.channel_id)
        .content("Stopped the track")?
        .await?;

    Ok(())
}

async fn volume(msg: Message, state: State) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing::debug!(
        "Got a volume command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );
    state
        .http
        .create_message(msg.channel_id)
        .content("What's the volume you want to set (0-1000, 100 being the default)?")?
        .await?;

    let author_id = msg.author.id;
    let msg = state
        .standby
        .wait_for_message(msg.channel_id, move |new_msg: &MessageCreate| {
            new_msg.author.id == author_id
        })
        .await?;
    let guild_id = msg.guild_id.unwrap();
    let volume = msg.content.parse::<i64>()?;

    if volume > 1000 || volume < 0 {
        state
            .http
            .create_message(msg.channel_id)
            .content("That's more than 1000")?
            .await?;

        return Ok(());
    }

    let player = state.lavalink.player(guild_id).await.unwrap();
    player.send(Volume::from((guild_id, volume)))?;

    state
        .http
        .create_message(msg.channel_id)
        .content(format!("Set the volume to {}", volume))?
        .await?;

    Ok(())
}
