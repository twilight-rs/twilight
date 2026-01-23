mod context {
    use http_body_util::Full;
    use hyper::body::Bytes;
    use hyper_util::client::legacy::{Client as HyperClient, connect::HttpConnector};
    use std::{ops::Deref, sync::OnceLock};
    use twilight_http::Client as HttpClient;
    use twilight_lavalink::Lavalink;
    use twilight_standby::Standby;

    pub static CONTEXT: Handle = Handle(OnceLock::new());

    #[derive(Debug)]
    pub struct Context {
        pub http: HttpClient,
        pub hyper: HyperClient<HttpConnector, Full<Bytes>>,
        pub lavalink: Lavalink,
        pub standby: Standby,
    }

    pub fn initialize(
        http: HttpClient,
        hyper: HyperClient<HttpConnector, Full<Bytes>>,
        lavalink: Lavalink,
        standby: Standby,
    ) {
        let context = Context {
            http,
            hyper,
            lavalink,
            standby,
        };
        assert!(CONTEXT.0.set(context).is_ok());
    }

    pub struct Handle(OnceLock<Context>);
    impl Deref for Handle {
        type Target = Context;

        fn deref(&self) -> &Self::Target {
            self.0.get().unwrap()
        }
    }
}

use context::CONTEXT;
use http_body_util::{BodyExt, Full};
use hyper::Request;
use hyper_util::{client::legacy::Client as HyperClient, rt::TokioExecutor};
use std::{borrow::Cow, env};
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
    gateway::payload::{incoming::MessageCreate, outgoing::UpdateVoiceState},
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
};
use twilight_standby::Standby;

const EVENT_TYPES: EventTypeFlags = EventTypeFlags::all();

const INTENTS: Intents = Intents::GUILD_MESSAGES
    .union(Intents::GUILD_VOICE_STATES)
    .union(Intents::MESSAGE_CONTENT);

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    // Select rustls backend
    rustls::crypto::ring::default_provider()
        .install_default()
        .unwrap();

    let token = env::var("DISCORD_TOKEN")?;
    let lavalink_auth = env::var("LAVALINK_AUTHORIZATION")?;
    let lavalink_host = env::var("LAVALINK_HOST")?.parse()?;

    let http = HttpClient::new(token.clone());
    let user = http.current_user().await?.model().await?;
    let hyper = HyperClient::builder(TokioExecutor::new()).build_http();
    let lavalink = Lavalink::new(user.id, 1);
    lavalink.add(lavalink_host, lavalink_auth).await?;
    let standby = Standby::new();
    context::initialize(http, hyper, lavalink, standby);

    let shard = Shard::new(ShardId::ONE, token, INTENTS);
    dispatcher(shard).await;

    Ok(())
}

#[tracing::instrument(fields(shard = %shard.id()), skip_all)]
async fn dispatcher(mut shard: Shard) {
    loop {
        let event = match shard.next_event(EVENT_TYPES).await {
            Some(Ok(event)) => event,
            Some(Err(source)) => {
                tracing::warn!(?source, "error receiving event");
                continue;
            }
            None => break,
        };

        CONTEXT.lavalink.process(&event).await.unwrap();
        CONTEXT.standby.process(&event);

        let handler = match event {
            Event::MessageCreate(event) => message(event, shard.sender()),
            _ => continue,
        };

        tokio::spawn(async move {
            if let Err(source) = handler.await {
                tracing::warn!(?source, "error handling event");
            }
        });
    }
}

#[tracing::instrument(fields(id = %event.id), skip_all)]
async fn message(event: Box<MessageCreate>, sender: MessageSender) -> anyhow::Result<()> {
    match &*event.content {
        "!equalize" if event.guild_id.is_some() => {
            equalize(event.channel_id, event.guild_id.unwrap(), event.author.id).await?;
        }
        "!join" if event.guild_id.is_some() => {
            join(
                event.channel_id,
                event.guild_id.unwrap(),
                sender,
                event.author.id,
            )
            .await?;
        }
        "!leave" if event.guild_id.is_some() => {
            leave(
                event.channel_id,
                event.guild_id.unwrap(),
                sender,
                event.author.id,
            )
            .await?;
        }
        "!pause" if event.guild_id.is_some() => {
            pause(event.channel_id, event.guild_id.unwrap(), event.author.id).await?;
        }
        "!play" if event.guild_id.is_some() => {
            play(event.channel_id, event.guild_id.unwrap(), event.author.id).await?;
        }
        "!seek" if event.guild_id.is_some() => {
            seek(event.channel_id, event.guild_id.unwrap(), event.author.id).await?;
        }
        "!stop" if event.guild_id.is_some() => {
            stop(event.channel_id, event.guild_id.unwrap(), event.author.id).await?;
        }
        "!volume" if event.guild_id.is_some() => {
            volume(event.channel_id, event.guild_id.unwrap(), event.author.id).await?;
        }
        _ => {}
    }

    Ok(())
}

async fn equalize(
    channel: Id<ChannelMarker>,
    guild: Id<GuildMarker>,
    user: Id<UserMarker>,
) -> anyhow::Result<()> {
    CONTEXT
        .http
        .create_message(channel)
        .content("What band should I equalize? (0–14)")
        .await?;

    let band_message = CONTEXT
        .standby
        .wait_for_message(channel, move |message| message.author.id == user)
        .await?;
    let band = band_message.content.parse::<i64>()?;

    CONTEXT
        .http
        .create_message(channel)
        .content("What gain should I set? (-0.25–1.0)")
        .await?;

    let gain_message = CONTEXT
        .standby
        .wait_for_message(channel, move |message| message.author.id == user)
        .await?;
    let gain = gain_message.content.parse::<f64>()?;

    let player = CONTEXT.lavalink.player(guild).await.unwrap();
    player.send(Equalizer::from((
        guild,
        vec![EqualizerBand::new(band, gain)],
    )))?;

    CONTEXT
        .http
        .create_message(channel)
        .content(&format!("Setting the gain to {gain} on band {band}"))
        .await?;

    Ok(())
}

async fn join(
    channel: Id<ChannelMarker>,
    guild: Id<GuildMarker>,
    sender: MessageSender,
    user: Id<UserMarker>,
) -> anyhow::Result<()> {
    CONTEXT
        .http
        .create_message(channel)
        .content("What channel ID should I join?")
        .await?;

    let message = CONTEXT
        .standby
        .wait_for_message(channel, move |message| message.author.id == user)
        .await?;
    let join_channel = message.content.parse()?;

    tracing::debug!(channel = %join_channel, %guild, %user, "joining");
    sender.command(&UpdateVoiceState::new(
        guild,
        Some(join_channel),
        false,
        false,
    ))?;

    CONTEXT
        .http
        .create_message(channel)
        .content(&format!("Joining <#{join_channel}>"))
        .await?;

    Ok(())
}

async fn leave(
    channel: Id<ChannelMarker>,
    guild: Id<GuildMarker>,
    sender: MessageSender,
    user: Id<UserMarker>,
) -> anyhow::Result<()> {
    tracing::debug!(%guild, %user, "leaving");
    let player = CONTEXT.lavalink.player(guild).await.unwrap();
    player.send(Destroy::from(guild))?;
    sender.command(&UpdateVoiceState::new(guild, None, false, false))?;

    CONTEXT
        .http
        .create_message(channel)
        .content("Leaving")
        .await?;

    Ok(())
}

async fn pause(
    channel: Id<ChannelMarker>,
    guild: Id<GuildMarker>,
    user: Id<UserMarker>,
) -> anyhow::Result<()> {
    tracing::debug!(%guild, %user, "pausing");
    let player = CONTEXT.lavalink.player(guild).await.unwrap();
    let paused = player.paused();
    player.send(Pause::from((guild, !paused)))?;

    let content = if paused { "Unpaused" } else { "Paused" };
    CONTEXT
        .http
        .create_message(channel)
        .content(content)
        .await?;

    Ok(())
}

async fn play(
    channel: Id<ChannelMarker>,
    guild: Id<GuildMarker>,
    user: Id<UserMarker>,
) -> anyhow::Result<()> {
    CONTEXT
        .http
        .create_message(channel)
        .content("What URL should I play?")
        .await?;

    let message = CONTEXT
        .standby
        .wait_for_message(channel, move |message| message.author.id == user)
        .await?;

    tracing::debug!(%guild, url = message.content, %user, "playing");
    let player = CONTEXT.lavalink.player(guild).await.unwrap();
    let (parts, body) = twilight_lavalink::http::load_track(
        player.node().config().address,
        &message.content,
        &player.node().config().authorization,
    )?
    .into_parts();
    let req = Request::from_parts(parts, Full::from(body));
    let res = CONTEXT.hyper.request(req).await?;
    let response_bytes = res.collect().await?.to_bytes();
    let loaded = serde_json::from_slice::<LoadedTracks>(&response_bytes)?;

    let track = match loaded.data {
        Track(track) => Some(track),
        Playlist(top_track) => top_track.tracks.first().cloned(),
        Search(result) => result.first().cloned(),
        _ => None,
    };
    let content = match track {
        Some(track) => {
            player.send(Play::from((guild, &track.encoded)))?;
            Cow::Owned(format!(
                "Playing **{}** by **{}**",
                track.info.title, track.info.title
            ))
        }
        None => Cow::Borrowed("Found no results"),
    };
    CONTEXT
        .http
        .create_message(channel)
        .content(&content)
        .await?;

    Ok(())
}

async fn seek(
    channel: Id<ChannelMarker>,
    guild: Id<GuildMarker>,
    user: Id<UserMarker>,
) -> anyhow::Result<()> {
    CONTEXT
        .http
        .create_message(channel)
        .content("Where should I seek to? (s)")
        .await?;

    let message = CONTEXT
        .standby
        .wait_for_message(channel, move |message| message.author.id == user)
        .await?;
    let position = message.content.parse::<i64>()?;

    tracing::debug!(%guild, %position, %user, "seeking");
    let player = CONTEXT.lavalink.player(guild).await.unwrap();
    player.send(Seek::from((guild, position * 1000)))?;

    CONTEXT
        .http
        .create_message(channel)
        .content(&format!("Seeking to {position} s"))
        .await?;

    Ok(())
}

async fn stop(
    channel: Id<ChannelMarker>,
    guild: Id<GuildMarker>,
    user: Id<UserMarker>,
) -> anyhow::Result<()> {
    tracing::debug!(%guild, %user, "stopping");

    let player = CONTEXT.lavalink.player(guild).await.unwrap();
    player.send(Stop::from(guild))?;

    CONTEXT
        .http
        .create_message(channel)
        .content("Stopping")
        .await?;

    Ok(())
}

async fn volume(
    channel: Id<ChannelMarker>,
    guild: Id<GuildMarker>,
    user: Id<UserMarker>,
) -> anyhow::Result<()> {
    CONTEXT
        .http
        .create_message(channel)
        .content("What volume should I set? (0–1000) [default: 100]")
        .await?;

    let message = CONTEXT
        .standby
        .wait_for_message(channel, move |message| message.author.id == user)
        .await?;
    let volume = message.content.parse::<i64>()?;

    let content = match volume {
        0..=1000 => {
            tracing::debug!(%guild, %user, volume, "modifying volume");
            let player = CONTEXT.lavalink.player(guild).await.unwrap();
            player.send(Volume::from((guild, volume)))?;

            Cow::Owned(format!("Setting the volume to {volume}"))
        }
        _ => Cow::Borrowed("Invalid volume"),
    };

    CONTEXT
        .http
        .create_message(channel)
        .content(&content)
        .await?;

    Ok(())
}
