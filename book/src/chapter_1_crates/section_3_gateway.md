# Gateway

`twilight-gateway` is an implementation of a client over Discord's websocket
gateway.

The main type is the `Shard`: it connects to the gateway, receives messages,
parses and processes them, and then gives them to you. It will automatically
reconnect, resume, and identify, as well as do some additional connectivity
checks.

## Features

`twilight-gateway` includes a number of features for things ranging from
payload deserialization to TLS features.

### Deserialization

`twilight-gateway` supports [`serde_json`] and [`simd-json`] for deserializing
and serializing events.

#### SIMD

The `simd-json` feature enables usage of [`simd-json`], which uses modern CPU
features to more efficiently deserialize JSON data. It is not enabled by
default.

In addition to enabling the feature, you will need to add the following to your
`<project_root>/.cargo/config`:

```toml
[build]
rustflags = ["-C", "target-cpu=native"]
```

### TLS

`twilight-gateway` has features to enable [`tokio-websockets`]' TLS features.
These features are mutually exclusive. `rustls-native-roots` is enabled by
default.

#### Native-TLS

The `native-tls` feature enables [`tokio-websockets`]' `native-tls` feature.

#### RusTLS

RusTLS allows specifying from where certificate roots are retrieved from.

##### Native roots

The `rustls-native-roots` feature enables [`tokio-websockets`]'
`rustls-native-roots` feature.

This is enabled by default.

##### Web PKI roots

The `rustls-webpki-roots` feature enables [`tokio-websockets`]'
`rustls-webpki-roots` feature.

### Transport compression

`twilight-gateway` supports both Zlib and Zstandard transport compression.

#### Zlib

The `zlib` feature implementation is `target_arch` dependent:

| s390x           | other       |
| --------------- | ----------- |
| [`zlib-ng-sys`] | [`zlib-rs`] |

#### Zstandard

The `zstd` feature uses Facebook's zstd library to decompresses incoming messages.

This feature takes precedence over the zlib features.

## Example

Starting a `Shard` and printing the contents of new messages as they come in:

```rust,no_run
use std::{env, error::Error};
use twilight_gateway::{EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    // Select rustls backend
    rustls::crypto::ring::default_provider().install_default().unwrap();

    let token = env::var("DISCORD_TOKEN")?;
    let intents = Intents::GUILD_MESSAGES;
    let mut shard = Shard::new(ShardId::ONE, token, intents);
    tracing::info!("created shard");

    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        let Ok(event) = item else {
            tracing::warn!(source = ?item.unwrap_err(), "error receiving event");

            continue;
        };

        tracing::debug!(?event, "event");
    }

    Ok(())
}
```

## Links

*source*: <https://github.com/twilight-rs/twilight/tree/main/twilight-gateway>

*docs*: <https://docs.rs/twilight-gateway>

*crates.io*: <https://crates.io/crates/twilight-gateway>

[img:shard]: ./section_3_shard.png
[RusTLS]: https://crates.io/crates/rustls
[`hyper-rustls`]: https://crates.io/crates/hyper-rustls
[`hyper-tls`]: https://crates.io/crates/hyper-tls
[`serde_json`]: https://crates.io/crates/serde_json
[`simd-json`]: https://crates.io/crates/simd-json
[`tokio-websockets`]: https://crates.io/crates/tokio-websockets
[`zlib-ng-sys`]: https://crates.io/crates/libz-ng-sys
[`zlib-rs`]: https://crates.io/crates/zlib-rs
