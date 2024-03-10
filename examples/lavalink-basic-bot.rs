use http_body_util::{BodyExt, Full};
use hyper::{Request, body::Bytes};
use hyper_util::{
    client::legacy::{Client as HyperClient, connect::HttpConnector},
    rt::TokioExecutor,
};
use std::{env, future::Future, net::SocketAddr, str::FromStr, sync::Arc};
use twilight_gateway::{
    Event, EventTypeFlags, Intents, MessageSender, Shard, ShardId, StreamExt as _,
};
use twilight_http::Client as HttpClient;
use twilight_lavalink::{
    Lavalink,
    http::{
        LoadResultData::{Playlist, Search, Track},
        LoadedTracks,
    },
    model::{Destroy, Equalizer, EqualizerBand, Pause, Play, Seek, Stop, Volume},
};
use twilight_model::{
    channel::Message,
    gateway::payload::{incoming::MessageCreate, outgoing::UpdateVoiceState},
};
use twilight_standby::Standby;

type State = Arc<StateRef>;

#[derive(Debug)]
struct StateRef {
    http: HttpClient,
    lavalink: Lavalink,
    hyper: HyperClient<HttpConnector, Full<Bytes>>,
    sender: MessageSender,
    standby: Standby,
}

fn spawn(fut: impl Future<Output = anyhow::Result<()>> + Send + 'static) {
    tokio::spawn(async move {
        if let Err(why) = fut.await {
            tracing::debug!("handler error: {why:?}");
        }
    });
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let (mut shard, state) = {
        let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
        let lavalink_host =
            SocketAddr::from_str(&env::var("LAVALINK_HOST").expect("Missing LAVALINK_HOST"))?;
        let lavalink_auth =
            env::var("LAVALINK_AUTHORIZATION").expect("Missing LAVALINK_AUTHORIZATION");
        let shard_count = 1u32;

        let http = HttpClient::new(token.clone());
        let user_id = http.current_user().await?.model().await?.id;

        let lavalink = Lavalink::new(user_id, shard_count);
        lavalink.add(lavalink_host, lavalink_auth).await?;

        let intents =
            Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES | Intents::MESSAGE_CONTENT;
        let shard = Shard::new(ShardId::ONE, token, intents);
        let sender = shard.sender();

        (
            shard,
            Arc::new(StateRef {
                http,
                lavalink,
                hyper: HyperClient::builder(TokioExecutor::new()).build_http(),
                sender,
                standby: Standby::new(),
            }),
        )
    };

    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        let Ok(event) = item else {
            tracing::warn!(source = ?item.unwrap_err(), "error receiving event");

            continue;
        };

        state.standby.process(&event);

        state.lavalink.process(&event).await?;

        if let Event::MessageCreate(msg) = event {
            if msg.guild_id.is_none() || !msg.content.starts_with('!') {
                continue;
            }

            match msg.content.split_whitespace().next() {
                Some("!join") => spawn(join(msg.0, Arc::clone(&state))),
                Some("!leave") => spawn(leave(msg.0, Arc::clone(&state))),
                Some("!pause") => spawn(pause(msg.0, Arc::clone(&state))),
                Some("!play") => spawn(play(msg.0, Arc::clone(&state))),
                Some("!seek") => spawn(seek(msg.0, Arc::clone(&state))),
                Some("!stop") => spawn(stop(msg.0, Arc::clone(&state))),
                Some("!volume") => spawn(volume(msg.0, Arc::clone(&state))),
                Some("!equalize") => spawn(equalize(msg.0, Arc::clone(&state))),
                _ => continue,
            }
        }
    }

    Ok(())
}

async fn join(msg: Message, state: State) -> anyhow::Result<()> {
    state
        .http
        .create_message(msg.channel_id)
        .content("What's the channel ID you want me to join?")
        .await?;

    let author_id = msg.author.id;
    let msg = state
        .standby
        .wait_for_message(msg.channel_id, move |new_msg: &MessageCreate| {
            new_msg.author.id == author_id
        })
        .await?;
    let channel_id = msg.content.parse()?;
    let guild_id = msg.guild_id.expect("known to be present");

    state.sender.command(&UpdateVoiceState::new(
        guild_id,
        Some(channel_id),
        false,
        false,
    ))?;

    state
        .http
        .create_message(msg.channel_id)
        .content(&format!("Joined <#{channel_id}>!"))
        .await?;

    Ok(())
}

async fn leave(msg: Message, state: State) -> anyhow::Result<()> {
    tracing::debug!(
        "leave command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );

    let guild_id = msg.guild_id.unwrap();
    let player = state.lavalink.player(guild_id).await.unwrap();
    player.send(Destroy::from(guild_id))?;
    state
        .sender
        .command(&UpdateVoiceState::new(guild_id, None, false, false))?;

    state
        .http
        .create_message(msg.channel_id)
        .content("Left the channel")
        .await?;

    Ok(())
}

async fn play(msg: Message, state: State) -> anyhow::Result<()> {
    tracing::debug!(
        "play command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );
    state
        .http
        .create_message(msg.channel_id)
        .content("What's the URL of the audio to play?")
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
    let (parts, body) = twilight_lavalink::http::load_track(
        player.node().config().address,
        &msg.content,
        &player.node().config().authorization,
    )?
    .into_parts();
    let req = Request::from_parts(parts, Full::from(body));
    let res = state.hyper.request(req).await?;
    let response_bytes = res.collect().await?.to_bytes();

    let loaded = serde_json::from_slice::<LoadedTracks>(&response_bytes)?;

    let track = match loaded.data {
        Track(track) => Some(track),
        Playlist(top_track) => top_track.tracks.first().cloned(),
        Search(result) => result.first().cloned(),
        _ => None,
    };

    if let Some(track) = track {
        player.send(Play::from((guild_id, &track.encoded)))?;

        let content = format!(
            "Playing **{:?}** by **{:?}**",
            track.info.title, track.info.author
        );

        state
            .http
            .create_message(msg.channel_id)
            .content(&content)
            .await?;
    } else {
        state
            .http
            .create_message(msg.channel_id)
            .content("Didn't find any results")
            .await?;
    }

    Ok(())
}

async fn pause(msg: Message, state: State) -> anyhow::Result<()> {
    tracing::debug!(
        "pause command in channel {} by {}",
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
        .content(&format!("{action} the track"))
        .await?;

    Ok(())
}

async fn seek(msg: Message, state: State) -> anyhow::Result<()> {
    tracing::debug!(
        "seek command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );
    state
        .http
        .create_message(msg.channel_id)
        .content("Where in the track do you want to seek to (in seconds)?")
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
        .content(&format!("Seeked to {position}s"))
        .await?;

    Ok(())
}

async fn equalize(msg: Message, state: State) -> anyhow::Result<()> {
    tracing::debug!(
        "equalize command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );
    state
        .http
        .create_message(msg.channel_id)
        .content("What band do you want to equalize (0-14)?")
        .await?;

    let author_id = msg.author.id;
    let band_msg = state
        .standby
        .wait_for_message(msg.channel_id, move |new_msg: &MessageCreate| {
            new_msg.author.id == author_id
        })
        .await?;
    let guild_id = msg.guild_id.unwrap();
    let band = band_msg.content.parse::<i64>()?;

    state
        .http
        .create_message(msg.channel_id)
        .content("What gain do you want to equalize (-0.25 to 1.0)?")
        .await?;

    let gain_msg = state
        .standby
        .wait_for_message(msg.channel_id, move |new_msg: &MessageCreate| {
            new_msg.author.id == author_id
        })
        .await?;
    let gain = gain_msg.content.parse::<f64>()?;

    let player = state.lavalink.player(guild_id).await.unwrap();
    player.send(Equalizer::from((
        guild_id,
        vec![EqualizerBand::new(band, gain)],
    )))?;

    state
        .http
        .create_message(msg.channel_id)
        .content(&format!("Changed gain level to {gain} on band {band}."))
        .await?;

    Ok(())
}

async fn stop(msg: Message, state: State) -> anyhow::Result<()> {
    tracing::debug!(
        "stop command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );

    let guild_id = msg.guild_id.unwrap();
    let player = state.lavalink.player(guild_id).await.unwrap();
    player.send(Stop::from(guild_id))?;

    state
        .http
        .create_message(msg.channel_id)
        .content("Stopped the track")
        .await?;

    Ok(())
}

async fn volume(msg: Message, state: State) -> anyhow::Result<()> {
    tracing::debug!(
        "volume command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );
    state
        .http
        .create_message(msg.channel_id)
        .content("What's the volume you want to set (0-1000, 100 being the default)?")
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

    if !(0..=1000).contains(&volume) {
        state
            .http
            .create_message(msg.channel_id)
            .content("That's more than 1000")
            .await?;

        return Ok(());
    }

    let player = state.lavalink.player(guild_id).await.unwrap();
    player.send(Volume::from((guild_id, volume)))?;

    state
        .http
        .create_message(msg.channel_id)
        .content(&format!("Set the volume to {volume}"))
        .await?;

    Ok(())
}
