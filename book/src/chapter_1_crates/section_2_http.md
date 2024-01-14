# HTTP

`twilight-http` is an HTTP client wrapping all of the documented Discord HTTP API.
It is built on top of [hyper], and allows you to pick your own TLS backend.
By default, it uses [RusTLS] a Rust TLS implementation, but it can be changed to
use NativeTLS, which uses the TLS native to the platform, and on Unix uses OpenSSL.

Ratelimiting is included out-of-the-box, along with support for proxies.

## Features

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

`twilight-http` has features to enable certain HTTPS TLS connectors.

These features are mutually exclusive. `rustls` is enabled by default.

#### Native-TLS

The `native-tls` feature causes the client to use [`hyper-tls`]. This will use the
native TLS backend, such as OpenSSL on Linux.

#### RusTLS

The `rustls` feature causes the client to use [`hyper-rustls`]. This enables
usage of the [RusTLS] crate as the TLS backend.

This is enabled by default.

## Example

A quick example showing how to get the current user's name:

```rust,no_run
use std::{env, error::Error};
use twilight_http::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let client = Client::new(env::var("DISCORD_TOKEN")?);

    let me = client.current_user().await?.model().await?;
    println!("Current user: {}#{}", me.name, me.discriminator);

    Ok(())
}
```

## Links

*source*: <https://github.com/twilight-rs/twilight/tree/main/twilight-http>

*docs*: <https://docs.rs/twilight-http>

*crates.io*: <https://crates.io/crates/twilight-http>

[hyper]: https://github.com/hyperium/hyper
[RusTLS]: https://github.com/ctz/rustls
[`hyper-rustls`]: https://crates.io/crates/hyper-rustls
[`hyper-tls`]: https://crates.io/crates/hyper-tls
[`serde_json`]: https://crates.io/crates/serde_json
[`simd-json`]: https://crates.io/crates/simd-json
