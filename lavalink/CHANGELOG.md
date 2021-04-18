# Changelog

Changelog for `twilight-lavalink`.

## [0.3.2] - 2021-04-12

### Fixes

Fix a deadlock, correctly disconnect from channels ([#733] - [@james7132]).

[#733]: https://github.com/twilight-rs/twilight/pull/733

## [0.3.1] - 2021-03-14

### Additions

`PlayerManager::destroy` destroys a player on the remote node ([#728] - [@james7132]).

### Fixes

Correctly update a player's channel ID on `VoiceStateUpdate` ([#728] - [@james7132]).

Store volume in `Player` ([#728] - [@james7132]).

[#728]: https://github.com/twilight-rs/twilight/pull/728

## [0.3.0] - 2021-01-08

Version 0.3 has been released with the primary intent to upgrade to Tokio 1.0.

### Changes

Upgrade `tokio` from v0.2 to v1 ([#664] - [@vivian]).

[#664]: https://github.com/twilight-rs/twilight/pull/664

## [0.2.2] - 2021-01-05

### Enhancements

Upgrade `dashmap` from version 3 to 4.0 ([#666] - [@vivian]).

[#666]: https://github.com/twilight-rs/twilight/pull/666

## [0.2.1] - 2020-11-02

Update the installation instructions to note version 0.2 instead of
version 0.1 ([#588] - [@vivian]).

## [0.2.0] - 2020-10-30

This major version bump of the Lavalink client is primarily done to match all of
the other crates in the ecosystem receiving a major version bump. There are no
significant API changes.

### Enhancements

Update `async-tungstenite` from ^0.8 to ^0.9.3, switching the RusTLS feature
selection from `async-tungstenite/async-tls` to `async-tungstenite/tokio-rustls`
to reduce dependency count ([#548], [#560] - [@nickelc]).

## [0.2.0-beta.1] - 2020-10-23

### Enhancements

Update `async-tungstenite` from ^0.8 to ^0.9.3, switching the RusTLS feature
selection from `async-tungstenite/async-tls` to `async-tungstenite/tokio-rustls`
to reduce dependency count ([#548] - [@nickelc]).

## [0.2.0-beta.0] - 2020-10-10

This major version bump of the Lavalink client is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.1.1] - 2020-09-20

### Fixes

- Correct type of `http::PlaylistInfo::selected_track` ([#518] - [@MOZGIII])

## [0.1.0] - 2020-09-13

Initial release.

[@MOZGIII]: https://github.com/MOZGIII
[@james7132]: https://github.com/james7132
[@nickelc]: https://github.com/nickelc
[@vivian]: https://github.com/vivian

[#588]: https://github.com/twilight-rs/twilight/pull/588
[#560]: https://github.com/twilight-rs/twilight/pull/560
[#548]: https://github.com/twilight-rs/twilight/pull/548
[#518]: https://github.com/twilight-rs/twilight/pull/518

[0.3.2]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-v0.3.2
[0.3.1]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-v0.3.1
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-v0.3.0
[0.2.2]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-v0.2.2
[0.2.1]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-v0.2.1
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-v0.2.0
[0.2.0-beta.1]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-v0.2.0-beta.1
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-v0.2.0-beta.0
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
