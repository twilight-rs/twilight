# twilight-http

HTTP support for the twilight ecosystem.

## Features

`twilight-http` includes three features: `native`, `simd` and `rustls`. `native` is
enabled by default. `native` will enable `reqwest`'s `default-tls` feature,
which will use the TLS library native to your OS (for example, OpenSSL on
Linux). `rustls` will enable `reqwest`'s `rustls-tls` feature, which will use
[rustls]. 

If you want to use Rustls instead of your native library, it's easy to switch it
out:

```toml
[dependencies]
twilight-http = { default-features = false, features = ["rustls"], git = "https://github.com/twilight-rs/twilight" }
```

You can also choose to use neither feature. This is only useful if you provide
your own configured Reqwest client to the HTTP client, otherwise you will
encounter TLS errors.

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

[rustls]: https://github.com/ctz/rustls
[simd-json]: https://github.com/simd-lite/simd-json
