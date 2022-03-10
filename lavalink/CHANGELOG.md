# Changelog

Changelog for `twilight-lavalink`.

## [0.10.0] - 2022-03-10

This major version bump of the Lavalink crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.9.1] - 2022-02-12

### Changes

Update `dashmap` to `5.1`, which fixes unsoundness present in `5.0` (which
previously forced a downgrade to `4.0`) ([#1517] - [@Gelbpunkt]).

[#1517]: https://github.com/twilight-rs/twilight/pull/1517

## [0.9.0] - 2022-01-22

### Changes

All types and method signatures have been updated to use the new `Id<T>` syntax
([#1260] - [@zeylahellyer]).

Support lavalink 3.4 ([#1292] - [@james7132]). `PlayerUpdateState` now contains
a `connected` field, and its `position` field is now `Option`al.

The `rustls` feature has been removed ([#1314] - [@Gelbpunkt]). Users must
manually select one of `rustls-native-roots` or `rustls-webpki-roots`.

The MSRV has been updated to 1.57 ([#1402] - [@zeylahellyer]).

The Rust edition has been updated to 2021 ([#1412] - [@vilgotf]).

[#1260]: https://github.com/twilight-rs/twilight/pull/1260
[#1292]: https://github.com/twilight-rs/twilight/pull/1292
[#1314]: https://github.com/twilight-rs/twilight/pull/1314
[#1402]: https://github.com/twilight-rs/twilight/pull/1402
[#1412]: https://github.com/twilight-rs/twilight/pull/1412

## [0.8.3] - 2022-01-11

### Fixes

Downgrade `dashmap` to `4.0`, to prevent an issue with `Ref::value` and `dashmap
5.0` ([#1434] - [@baptiste0928]).

[#1434]: https://github.com/twilight-rs/twilight/pull/1434

## [0.8.2] - 2022-01-08

### Changes

This release contains internal refactors, there are no public facing
changes.

## [0.8.1] - 2021-12-24

### Changes

Upgrade `dashmap` to 5.0 ([#1336] - [@vilgotf]). `dashmap` 4.0 is still allowed.

[#1336]: https://github.com/twilight-rs/twilight/pull/1336

## [0.8.0] - 2021-12-03

### Changes

`tracing` is now an optional feature, and enabled by default ([#1203] -
[@Gelbpunkt]).

The `rustls` feature now defaults to `rustls-native-roots`; users may
optionally select `rustls-webpki-roots` ([#1276] - [@Gelbpunkt]). This
matches the functionality of other crates that include a `rustls`
feature.

### Dependency Updates

`tokio-tungstenite` has been updated to `0.16` ([#1276] - [@Gelbpunkt]).

[#1203]: https://github.com/twilight-rs/twilight/pull/1203
[#1276]: https://github.com/twilight-rs/twilight/pull/1276

## [0.7.3] - 2021-12-03

### Fixes

Support deserializing negative track vales in `PlaylistInfo` as `None` ([#1304]
- [@james7132]).

[#1304]: https://github.com/twilight-rs/twilight/pull/1304

## [0.7.2] - 2021-11-20

### Fixes

Allow the crate to compile with `default-features = false` set ([#1248] -
[@7596ff]).

[#1248]: https://github.com/twilight-rs/twilight/pull/1248

## [0.7.1] - 2021-10-29

### Changes

Fixes some spelling errors in documentation ([#1223] - [@7596ff]).

[#1223]: https://github.com/twilight-rs/twilight/pull/1223

## [0.7.0] - 2021-10-21

### Changes

`Lavalink`, `Node`, and `Player`, no longer implement `Clone`, because
they are no longer internally wrapped in an `Arc` ([#1067] -
[@zeylahellyer]). To retain this functionality, you can wrap them it an
`Arc` or a `Rc` manually.

The MSRV has been updated to 1.53 ([#1161] - [@7596ff]).

[#1067]: https://github.com/twilight-rs/twilight/pull/1067
[#1161]: https://github.com/twilight-rs/twilight/pull/1161

## [0.6.1] - 2021-09-17

### Changes

This release contains internal refactors, there are no public facing
changes.

## [0.6.0] - 2021-07-31

### Changes

A few spelling errors have been fixed by adding the `codespell` Action
([#1041] - [@Gelbpunkt].

[#1041]: https://github.com/twilight-rs/twilight/pull/1041

## [0.5.2] - 2021-07-23

### Changes

`#![deny(unsafe_code)]` has been added, ensuring no unsafe code exists in the
crate ([#1042] - [@zeylahellyer]).

[#1042]: https://github.com/twilight-rs/twilight/pull/1042

## [0.5.1] - 2021-07-02

### Enhancements

Improve the `Display` implementation performance of `NodeError`'s `Display`
implementation by calling `Formatter` methods directly instead of calling the
`format_args!` and `write!` macros ([#944] - [@zeylahellyer]).

[#944]: https://github.com/twilight-rs/twilight/pull/944

## [0.5.0] - 2021-06-13

This major version bump of the Lavalink crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.4.1] - 2021-05-30

### Enhancements

The following functions are now `const`:

- `client::ClientError::kind`
- `model::outgoing::Destroy::new`
- `node::NodeError::kind`
- `node::NodeSenderError::kind`
- `node::Resume::new`

([#824] - [@vivian]).

[#824]: https://github.com/twilight-rs/twilight/pull/824

## [0.4.0] - 2021-05-12

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

[@7596ff]: https://github.com/7596ff
[@baptiste0928]: https://github.com/baptiste0928
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@james7132]: https://github.com/james7132
[@MOZGIII]: https://github.com/MOZGIII
[@nickelc]: https://github.com/nickelc
[@vilgotf]: https://github.com/vilgotf
[@vivian]: https://github.com/vivian
[@zeylahellyer]: https://github.com/zeylahellyer

[#588]: https://github.com/twilight-rs/twilight/pull/588
[#560]: https://github.com/twilight-rs/twilight/pull/560
[#548]: https://github.com/twilight-rs/twilight/pull/548
[#518]: https://github.com/twilight-rs/twilight/pull/518

[0.10.0]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.10.0
[0.9.0]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.9.0
[0.8.3]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.8.3
[0.8.2]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.8.2
[0.8.1]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.8.1
[0.8.0]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.8.0
[0.7.2]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.7.2
[0.7.1]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.7.1
[0.7.0]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.7.0
[0.6.1]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.6.1
[0.6.0]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.6.0
[0.5.2]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.5.2
[0.5.1]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.5.1
[0.5.0]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.5.0
[0.4.1]: https://github.com/twilight-rs/twilight/releases/tag/lavalink-0.4.1
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
