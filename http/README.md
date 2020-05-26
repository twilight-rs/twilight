# twilight-http

HTTP support for the twilight ecosystem.

## Features

`twilight-http` includes two features: `native` and `rustls`. `native` is
enabled by default. `native` will enable `reqwest`'s `native-tls` feature,
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

[rustls]: https://github.com/ctz/rustls
