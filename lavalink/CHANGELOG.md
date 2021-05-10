# Changelog

Changelog for `twilight-lavalink`.

## [0.4.0] - 2021-05-??

### Upgrade Path

The MSRV is now Rust 1.49.

`Lavalink::player`, `PlayerManager::get`, and `PlayerManager::get_or_insert` now
return `Player`s instead of `DashMap` references.

`Player` has had some methods renamed:

- `position_mut` has been renamed to `set_position` and accepts an `i64`
- `time_ref` has been renamed to `time`
- `time_mut` has been renamed to `set_time` and accepts an `i64`
- `volume_ref` has been renamed to `volume`

`Node::players` is no longer an async method.

Errors are no longer enums and don't expose their concrete underlying error
source. You can access the underlying error via the implemented
`std::error::Error::source` method or the `into_parts` or `into_source` methods
on each error struct, which will return a boxed `std::error::Error`. To access
the reason for the error use the `kind` or `into_parts` method on error structs;
the returned error type is an enum with variants for each potential reason the
error occurred.

### Additions

Add `WebsocketClosed` incoming event ([#734] - [@james7132]).

Add `Lavalink::disconnect` to disconnect from a given node, `Node::close` to
close the connection to a node ([#742] - [@james7132]).

### Enhancements

The `futures-channel` dependency has been removed while the `async-tungstenite`
dependency has been switched out for `tokio-tungstenite` to decrease the
dependency tree ([#785] - [@Gelbpunkt]).

### Changes

Simplify player API by not exposing DashMap references and returning Players
instead ([#693] - [@vivian]).

Remove unnecessary `async` qualifier from `Node::players`
([#731] - [@james7132]).

`Node::drop` now removes the node from the Lavalink client
([#742] - [@james7132]).

[#785]: https://github.com/twilight-rs/twilight/pull/785
[#742]: https://github.com/twilight-rs/twilight/pull/742
[#734]: https://github.com/twilight-rs/twilight/pull/734
[#731]: https://github.com/twilight-rs/twilight/pull/731
[#693]: https://github.com/twilight-rs/twilight/pull/693

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

[@Gelbpunkt]: https://github.com/Gelbpunkt
[@MOZGIII]: https://github.com/MOZGIII
[@james7132]: https://github.com/james7132
[@nickelc]: https://github.com/nickelc
[@vivian]: https://github.com/vivian

[#588]: https://github.com/twilight-rs/twilight/pull/588
[#560]: https://github.com/twilight-rs/twilight/pull/560
[#548]: https://github.com/twilight-rs/twilight/pull/548
[#518]: https://github.com/twilight-rs/twilight/pull/518

[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.4.0
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
