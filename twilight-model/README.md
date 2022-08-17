# twilight-model

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

A collection of Discord API types.

Refer to the official [Discord Docs] as the ultimate source of truth!

Some types have associated builders, which can be found in the [`twilight-util`]
crate.

## Resources and their data

Resources, such as channels, guilds and roles, are identified by stable and
globally unique IDs. Resource's data may, however, be updated at any time, so
do not assume that resource ID equality implies resource data equality.

### Example

```rust,no_run
use twilight_model::channel::Message;

/// Fetch a message from the HTTP API.
fn fetch_message() -> Message {
    unimplemented!()
}

let message_a = fetch_message();
let message_b = fetch_message();

if message_a.id == message_b.id {
    println!("received the same message");
    if message_a != message_b {
        println!("message was updated between the calls to `fetch_message`");
    }
}
```

## License

[ISC][license link]

[`twilight-util`]: https://docs.rs/twilight-util
[codecov badge]: https://img.shields.io/codecov/c/gh/twilight-rs/twilight?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
[codecov link]: https://app.codecov.io/gh/twilight-rs/twilight/
[discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
[Discord Docs]: https://discord.com/developers/docs
[discord link]: https://discord.gg/7jj8n7D
[github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
[github link]: https://github.com/twilight-rs/twilight
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
[license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[rust badge]: https://img.shields.io/badge/rust-1.60+-93450a.svg?style=for-the-badge&logo=rust
