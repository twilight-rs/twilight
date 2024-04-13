# twilight-gateway-queue

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

Rate limiting functionality for gateway `IDENTIFY` commands.

Discord allows bot's shards to send a limited amount of `IDENTIFY` commands
every 5 seconds, with a daily limit from 1000 to 2000 commands, and invalidates
*all* shard sessions upon exceeding it. Each identify interval may be filled by
shards' IDs modulo `max_concurrency` and such a set of shards is called a
bucket. See [Discord Docs/Sharding].

To coordinate this, a [`Queue`] should process each identify request and shards
should wait for its signal to proceed before continuing and otherwise retry. The
provided [`InMemoryQueue`] never fails or cancels requests and is therefore a
good starting point for custom implementations. It can also be composed to
support multiple processes; see [`gateway-queue-http`] and [`gateway-queue`] for
a HTTP client and server implementation, respectively.

[codecov badge]: https://img.shields.io/codecov/c/gh/twilight-rs/twilight?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
[codecov link]: https://app.codecov.io/gh/twilight-rs/twilight/
[discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
[Discord Docs/Sharding]: https://discord.com/developers/docs/topics/gateway#sharding
[discord link]: https://discord.gg/twilight-rs
[`gateway-queue`]: https://github.com/twilight-rs/gateway-queue
[`gateway-queue-http`]: https://github.com/twilight-rs/twilight/blob/main/examples/gateway-queue-http.rs
[github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
[github link]: https://github.com/twilight-rs/twilight
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
[license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[rust badge]: https://img.shields.io/badge/rust-1.67+-93450a.svg?style=for-the-badge&logo=rust
