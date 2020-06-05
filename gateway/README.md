# twilight-gateway

`twilight-gateway` is an implementation of Discord's sharding gateway sessions.
This is responsible for receiving stateful events in real-time from Discord
and sending *some* stateful information.

It includes two primary types: the Shard and Cluster.

The Shard handles a single WebSocket connection and can manage up to 2500
guilds. If you manage a small bot in under about 2000 guilds, then this is
what you use. See the [Discord docs][docs:discord:sharding] for more
information on sharding.

The Cluster is an interface which manages the health of the shards it
manages and proxies all of their events under one unified stream. This is
useful to use if you have a large bot in over 1000 or 2000 guilds.

## Features

`twilight-gateway` includes only a feature: `simd-json`.

`simd` feature enables [simd-json] support to use simd features of the modern cpus
to deserialize json data faster. It is not enabled by default since not every cpu has those features.
To use this feature you need to also add these lines to a file in `<project root>/.cargo/config`
```toml
[build]
rustflags = ["-C", "target-cpu=native"]
```
you can also use this environment variable `RUSTFLAGS="-C target-cpu=native"`. If you enable both 
`serde_json` and `simd-json` at the same time; this crate uses `simd-json`. But it is recommended to
disable `serde_json` if you are going to use `simd-json`. It is easy to switch to out:

```toml
[dependencies]
twilight-gateway = { default-features = false, features = ["simd-json"], git = "https://github.com/twilight-rs/twilight" }
```

[simd-json]: https://github.com/simd-lite/simd-json