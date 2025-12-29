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
mod context {
    use std::{ops::Deref, sync::OnceLock};
    use twilight_http::Client;

    pub static CONTEXT: Handle = Handle(OnceLock::new());

    #[derive(Debug)]
    pub struct Context {
        pub http: Client,
    }

    pub fn initialize(http: Client) {
        let context = Context { http };
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
use std::{env, pin::pin};
use tokio::{signal, task::JoinHandle};
use twilight_gateway::{
    CloseFrame, Config, Event, EventTypeFlags, Intents, MessageSender, Shard, StreamExt as _,
};
use twilight_http::Client;
use twilight_model::gateway::payload::{incoming::MessageCreate, outgoing::UpdateVoiceState};

const EVENT_TYPES: EventTypeFlags = EventTypeFlags::MESSAGE_CREATE;
const INTENTS: Intents = Intents::GUILD_MESSAGES.union(Intents::MESSAGE_CONTENT);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    // Select rustls backend
    rustls::crypto::ring::default_provider().install_default().unwrap();

    let token = env::var("DISCORD_TOKEN")?;

    let config = Config::new(token.clone(), INTENTS);
    let http = Client::new(token);
    let shards =
        twilight_gateway::create_recommended(&http, config, |_, builder| builder.build()).await?;
    context::initialize(http);

    let tasks = shards
        .map(|shard| tokio::spawn(dispatcher(shard)))
        .collect::<Vec<_>>();

    // Await shutdown.
    for task in tasks {
        _ = task.await;
    }

    Ok(())
}

#[tracing::instrument(fields(shard = %shard.id()), skip_all)]
async fn dispatcher(mut shard: Shard) {
    let mut ctrl_c = pin!(signal::ctrl_c());
    let mut shutdown = false;
    let mut tasks = Vec::<JoinHandle<()>>::new();
    loop {
        tokio::select! {
            // Do not poll ctrl_c after it's completed.
            _ = &mut ctrl_c, if !shutdown => {
                // Cleanly shut down once we receive the echo close frame.
                shard.close(CloseFrame::NORMAL);
                shutdown = true;
            },
            Some(item) = shard.next_event(EVENT_TYPES) => {
                let event = match item {
                    Ok(event) => event,
                    Err(source) => {
                        tracing::warn!(?source, "error receiving event");
                        continue;
                    }
                };

                let handler = match event {
                    // Clean shutdown exit condition.
                    Event::GatewayClose(_) if shutdown => break,
                    Event::MessageCreate(e) => message_handler(e, shard.sender()),
                    _ => continue,
                };

                // Do not grow the list infinitely.
                tasks.retain(|task| !task.is_finished());
                tasks.push(tokio::spawn(async move {
                    if let Err(source) = handler.await {
                        tracing::warn!(?source, "error handling event");
                    }
                }));
            }
        }
    }

    // Await shutdown.
    for task in tasks {
        _ = task.await;
    }
}

#[tracing::instrument(fields(id = %event.id), skip_all)]
async fn message_handler(event: Box<MessageCreate>, sender: MessageSender) -> anyhow::Result<()> {
    match event.content.as_ref() {
        "!join" if event.guild_id.is_some() => {
            sender.command(&UpdateVoiceState::new(
                event.guild_id.unwrap(),
                Some(event.channel_id),
                false,
                false,
            ))?;
        }
        "!ping" => {
            CONTEXT
                .http
                .create_message(event.channel_id)
                .content("Pong!")
                .await?;
        }
        _ => {}
    }

    Ok(())
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
