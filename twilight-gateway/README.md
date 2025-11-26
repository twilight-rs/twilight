# twilight-gateway

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

`twilight-gateway` is an implementation of Discord's sharding gateway sessions.
This is responsible for receiving stateful events in real-time from Discord and
sending *some* stateful information.

The primary type is the `Shard`, a stateful interface to maintain a Websocket
connection to Discord's gateway. Much of its functionality can be configured,
and it's used to receive gateway events or raw Websocket messages, useful for
load balancing and microservices.

Multiple shards may easily be created at once, with a per shard config created
from a `Fn(ShardId, ConfigBuilder) -> Config` closure, with the help of the
`create_` set of functions. These functions will reuse shards' TLS context and
[session queue][queue], something otherwise achieved by cloning an existing
[`Config`].

## Features

* `simd-json`: use [`simd-json`] instead of [`serde_json`] for deserializing
  events
* TLS (mutually exclusive)
  * `native-tls`: platform's native TLS implementation via [`native-tls`]
  * `rustls-native-roots`: [`rustls`] using native root certificates
  * `rustls-platform-verifier` (*default*): [`rustls`] using operating system's
    certificate facilities via [`rustls-platform-verifier`]
  * `rustls-webpki-roots`: [`rustls`] using [`webpki-roots`] for root
    certificates, useful for `scratch` containers
* `twilight-http` (*default*): enable the `stream::create_recommended` function
* Transport compression (mutually exclusive)
  * `zlib`: Zlib transport compression using [`zlib-rs`][^1]
  * `zstd` (*default*): Zstandard transport compression using [`zstd-sys`]

## Example

Create the recommended number of shards and loop over their guild messages:

```rust,no_run
use std::{env, sync::Arc};
use tokio::{signal, sync::watch};
use twilight_gateway::{
    CloseFrame, Config, Event, EventTypeFlags, Intents, MessageSender, Shard, StreamExt as _,
};
use twilight_http::Client;
use twilight_model::gateway::payload::{incoming::MessageCreate, outgoing::UpdateVoiceState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let client = Arc::new(Client::new(token.clone()));
    let config = Config::new(token, Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT);

    let tasks = twilight_gateway::create_recommended(&client, config, |_, builder| builder.build())
        .await?
        .map(|shard| tokio::spawn(dispatcher(Arc::clone(&client), shard, shutdown_rx.clone())))
        .collect::<Vec<_>>();

    signal::ctrl_c().await?;
    _ = shutdown_tx.send(true);

    for task in tasks {
        _ = task.await;
    }

    Ok(())
}

#[tracing::instrument(fields(shard = %shard.id()), skip_all)]
async fn dispatcher(client: Arc<Client>, mut shard: Shard, mut shutdown: watch::Receiver<bool>) {
    loop {
        tokio::select! {
            _ = shutdown.changed() => shard.close(CloseFrame::NORMAL),
            Some(item) = shard.next_event(EventTypeFlags::all()) => {
                let event = match item {
                    Ok(event) => event,
                    Err(source) => {
                        tracing::warn!(?source, "error receiving event");
                        continue;
                    }
                };

                match event {
                    Event::GatewayClose(_) if *shutdown.borrow() => break,
                    Event::MessageCreate(e) => {
                        tokio::spawn(msg_handler(Arc::clone(&client), e, shard.sender()));
                    }
                    _ => {}
                }
            }
        }
    }
}

#[tracing::instrument(fields(id = %event.id), skip_all)]
async fn msg_handler(client: Arc<Client>, event: Box<MessageCreate>, sender: MessageSender) {
    match event.content.as_ref() {
        "!join" if event.guild_id.is_some() => {
            let _result = sender.command(&UpdateVoiceState::new(
                event.guild_id.unwrap(),
                Some(event.channel_id),
                false,
                false,
            ));
        }
        "!ping" => {
            let _result = client
                .create_message(event.channel_id)
                .content("pong!")
                .await;
        }
        _ => {}
    }
}
```

There are a few additional examples located in the
[repository][github examples link].

[^1]: Except for the s390x arch, where [`zlib-ng-sys`] is used instead.

[`native-tls`]: https://crates.io/crates/native-tls
[`rustls`]: https://crates.io/crates/rustls
[`rustls-platform-verifier`]: https://crates.io/crates/rustls-platform-verifier
[`serde_json`]: https://crates.io/crates/serde_json
[`simd-json`]: https://crates.io/crates/simd-json
[`webpki-roots`]: https://crates.io/crates/webpki-roots
[`zlib-ng-sys`]: https://crates.io/crates/libz-ng-sys
[`zlib-rs`]: https://crates.io/crates/zlib-rs
[`zstd-sys`]: https://crates.io/crates/zstd-sys
[codecov badge]: https://img.shields.io/codecov/c/gh/twilight-rs/twilight?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
[codecov link]: https://app.codecov.io/gh/twilight-rs/twilight/
[discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
[discord link]: https://discord.gg/7jj8n7D
[github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
[github examples link]: https://github.com/twilight-rs/twilight/tree/main/examples
[github link]: https://github.com/twilight-rs/twilight
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
[license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[rust badge]: https://img.shields.io/badge/rust-1.79+-93450a.svg?style=for-the-badge&logo=rust
