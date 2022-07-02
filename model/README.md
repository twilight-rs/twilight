# twilight-model

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

This crate models types from the Discord API with a few convenience methods on
top. Types are reproducible: payloads may be serialized and deserialized
without any information loss.

Resources can be compared as objects, but as Discord may update them at any
time, it is recommended to compare them by ID. Resources' IDs are stable and
globally unique.

```rust,no_run
use twilight_model::channel::Message;

fn retrieve_message() -> Message {
    unimplemented!()
}

let message_a = retrieve_message();
let message_b = retrieve_message();

if message_a.id == message_b.id {
    println!("received the same message");
    if message_a != message_b {
        println!("message was updated between the calls to `retrieve_message`")
    }
}
```

Related types are grouped together in modules, with `guild` and `channel` being
the largest ones. Other crates may return, build on top of, or extend these
types.

Refer to the [Discord Docs] as the source of truth.

Some models have associated builders, which can be found in the
[`twilight-util`] crate.

## License

[ISC][LICENSE.md]

[LICENSE.md]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[`twilight-util`]: https://docs.rs/twilight-util
[`twilight`]: https://docs.rs/twilight
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
