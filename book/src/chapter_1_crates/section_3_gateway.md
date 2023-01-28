# Gateway

`twilight-gateway` is an implementation of a client over Discord's websocket
gateway.

The main type is the `Shard`: it connects to the gateway, receives messages,
parses and processes them, and then gives them to you. It will automatically
reconnect, resume, and identify, as well as do some additional connectivity
checks.

Also provided is the `Cluster`, which will automatically manage a collection of
shards and unify their messages into one stream. It doesn't have a large API, you
usually want to spawn a task to bring it up such that you can begin to receive
tasks as soon as they arrive.

```rust,no_run
# use std::sync::Arc;
# use futures::StreamExt;
# use twilight_gateway::{Cluster, Intents};
#
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
#    let token = String::from("dummy");
let intents = Intents::GUILD_MESSAGES | Intents::GUILDS;
let (cluster, mut events) = Cluster::new(token, intents).await?;
let cluster = Arc::new(cluster);

let cluster_spawn = cluster.clone();

tokio::spawn(async move {
    cluster_spawn.up().await;
});
# let _ = events.next().await;
#     Ok(())
# }
```

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

### Metrics

The `metrics` feature provides metrics information via the [`metrics`] crate.
Some of the metrics logged are counters about received event counts and their
types and gauges about the capacity and efficiency of the inflater of each
shard.

This is disabled by default.

### TLS

`twilight-gateway` has features to enable [`async-tungstenite`] and
[`twilight-http`]'s TLS features. These features are mutually exclusive. `rustls`
is enabled by default.

#### Native

The `native` feature enables [`async-tungstenite`]'s `tokio-native-tls` feature
as well as [`twilight-http`]'s `native` feature which uses `hyper-tls`.

#### RusTLS

The `rustls` feature enables [`async-tungstenite`]'s `tokio-rustls` feature and
[`twilight-http`]'s `rustls` feature, which use [RusTLS] as the TLS backend.

This is enabled by default.

### Zlib

#### Stock

The `zlib-stock` feature makes [flate2] use of the stock Zlib which is either
upstream or the one included with the operating system.

#### SIMD

`zlib-simd` enables the use of [zlib-ng] which is a modern fork of zlib that in
most cases will be more effective. However, this will add an external dependency
on [cmake].

If both are enabled or if the `zlib` feature of [flate2] is enabled anywhere in
the dependency tree it will make use of that instead of [zlib-ng].

## Example

Starting a `Shard` and printing the contents of new messages as they come in:

```rust,no_run
use futures::StreamExt;
use std::{env, error::Error};
use twilight_gateway::{Intents, Shard};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let (shard, mut events) = Shard::new(token, Intents::GUILD_MESSAGES);

    shard.start().await?;
    println!("Created shard");

    while let Some(event) = events.next().await {
        println!("Event: {:?}", event);
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
[cmake]: https://cmake.org/
[flate2]: https://github.com/alexcrichton/flate2-rs
[zlib-ng]: https://github.com/zlib-ng/zlib-ng
[`async-tungstenite`]: https://crates.io/crates/async-tungstenite
[`hyper-rustls`]: https://crates.io/crates/hyper-rustls
[`hyper-tls`]: https://crates.io/crates/hyper-tls
[`metrics`]: https://crates.io/crates/metrics
[`serde_json`]: https://crates.io/crates/serde_json
[`simd-json`]: https://crates.io/crates/simd-json
[`twilight-http`]: ./section_2_http.md
