# twilight-util

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

`twilight-util` is a set of utility types and functions for the [`twilight-rs`] ecosystem to
augment or enhance default functionality.

## Features

### `builder`

Provides builders for large structs.

### `link`

Provides implementations for parsing and formatting entities' URLs, such as
webhook URLs.

### `permission-calculator`

Allows the use of a calculator to determine the permissions of a member in
a guild or channel.

### `snowflake`

Allows the use of the `Snowflake` trait, which provides methods for the extraction of
structured information from [Discord snowflakes].

### `signature-validation`

Provides utilities for doing [HTTP Interaction] [signature validation].

[`twilight-rs`]: https://github.com/twilight-rs/twilight
[codecov badge]: https://img.shields.io/codecov/c/gh/twilight-rs/twilight?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
[codecov link]: https://app.codecov.io/gh/twilight-rs/twilight/
[discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
[discord link]: https://discord.gg/7jj8n7D
[github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
[github link]: https://github.com/twilight-rs/twilight
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
[license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[rust badge]: https://img.shields.io/badge/rust-1.67+-93450a.svg?style=for-the-badge&logo=rust
[Discord snowflakes]: https://discord.com/developers/docs/reference#snowflakes
[HTTP Interaction]: https://discord.com/developers/docs/interactions/receiving-and-responding#receiving-an-interaction
[signature validation]: https://discord.com/developers/docs/interactions/receiving-and-responding#security-and-authorization
