# Changelog

## [0.15.1] - 2023-02-26

### Refactor

- change deny lints to warn ([#2144](https://github.com/twilight-rs/twilight/issues/2144))

## [0.15.0] - 2023-02-05

This major version bump of the Gateway Queue is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.14.1] - 2023-02-05

### Bug Fixes

- skip sleep on failed messages ([#2113](https://github.com/twilight-rs/twilight/issues/2113))

## [0.14.0] - 2022-11-14

MSRV has been bumped to 1.64 ([#1897] - [@vilgotf]).

### Refactor

[**breaking**] drop TLS features ([#1842] - [@vilgotf]). `LargeBotQueue`
requires a `Client` from twilight-http, meaning that users must also depend on
twilight-http to use it. TLS requirements should therefore only be specified in
twilight-http, with twilight-gateway-queue inheriting said requirements.

[#1842]: https://github.com/twilight-rs/twilight/issues/1842
[#1897]: https://github.com/twilight-rs/twilight/issues/1897

## [0.13.1] - 2022-09-11

### Documentation

- clarify `buckets` for large bot queue ([#1895](https://github.com/twilight-rs/twilight/issues/1895))

## [0.11.1] - 2022-07-07

### Documentation

- fix sharding hyperlink ([#1741](https://github.com/twilight-rs/twilight/issues/1741))

### Refactor

- standardize clippy lints ([#1779](https://github.com/twilight-rs/twilight/issues/1779))

Changelog for `twilight-gateway-queue`.

## [0.11.0] - 2022-05-15

MSRV has been bumped to 1.60.

### Changes

`tracing` is no longer an optional dependency and is always enabled
([#1684], [#1730] - [@vilgotf], [@zeylahellyer]).

[#1730]: https://github.com/twilight-rs/twilight/pull/1730
[#1684]: https://github.com/twilight-rs/twilight/pull/1684

## [0.10.1] - 2022-04-15

### Changes

Link to `tracing` in the README ([#1652] - [@zeylahellyer]).

[#1652]: https://github.com/twilight-rs/twilight/pull/1652

## [0.10.0] - 2022-03-10

### Changes

`twilight-http` is now an optional feature, and enabled by default ([#1489] -
[@Gelbpunkt]).

[#1489]: https://github.com/twilight-rs/twilight/pull/1489

## [0.9.1] - 2022-02-12

### Additions

Add `NoOpQueue`, which does not ratelimit any requests ([#1490] - [@Gelbpunkt]).

[#1490]: https://github.com/twilight-rs/twilight/pull/1490

## [0.9.0] - 2022-01-22

### Changes

The `rustls` feature has been removed ([#1314] - [@Gelbpunkt]). Users must
manually select one of `rustls-native-roots` or `rustls-webpki-roots`.

The MSRV has been updated to 1.57 ([#1402] - [@zeylahellyer]).

The Rust edition has been updated to 2021 ([#1412] - [@vilgotf]).

[#1314]: https://github.com/twilight-rs/twilight/pull/1314
[#1402]: https://github.com/twilight-rs/twilight/pull/1402
[#1412]: https://github.com/twilight-rs/twilight/pull/1412

## [0.8.1] - 2022-01-21

### Changes

Remove two unneeded calls to `clone` ([#1440] - [@vilgotf]).

[#1440]: https://github.com/twilight-rs/twilight/pull/1440

## [0.8.0] - 2021-12-03

### Changes

`tracing` is now an optional feature, and enabled by default ([#1203] -
[@Gelbpunkt]).

[#1203]: https://github.com/twilight-rs/twilight/pull/1203

## [0.7.0] - 2021-10-21

### Changes

The MSRV has been updated to 1.53 ([#1161] - [@7596ff]).

[#1161]: https://github.com/twilight-rs/twilight/pull/1147

## [0.6.0] - 2021-07-31

This major version bump of the Gateway Queue is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.5.1] - 2021-07-23

### Changes

`#![deny(unsafe_code)]` has been added, ensuring no unsafe code exists in the
crate ([#1042] - [@zeylahellyer]).

[#1042]: https://github.com/twilight-rs/twilight/pull/1042

## [0.5.0] - 2021-06-13

This major version bump of the Gateway Queue is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.4.0] - 2021-05-12

### Upgrade Path

The MSRV is now Rust 1.49.

Errors are no longer enums and don't expose their concrete underlying error
source. You can access the underlying error via the implemented
`std::error::Error::source` method or the `into_parts` or `into_source` methods
on each error struct, which will return a boxed `std::error::Error`. To access
the reason for the error use the `kind` or `into_parts` method on error structs;
the returned error type is an enum with variants for each potential reason the
error occurred.

### Enhancements

The `futures-channel` and `futures-util` dependencies have been removed
([#785] - [@Gelbpunkt]).

[#785]: https://github.com/twilight-rs/twilight/pull/785

## [0.3.0] - 2021-01-08

This major version bump of the Gateway Queue is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.2.1] - 2020-11-11

### Enhancements

Remove broken cross-crate links ([#595] - [@vivian]).

## [0.2.0] - 2020-10-30

This major version bump of the Gateway Queue crate is done to match all of the
other crates in the ecosystem receiving a major version bump. There are no
changes.

## [0.2.0-beta.0] - 2020-10-10

This major version bump of the Gateway Queue crate is done to match all of the
other crates in the ecosystem receiving a major version bump. There are no
changes.

## [0.1.0] - 2020-10-07

Initial release.

[@7596ff]: https://github.com/7596ff
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@vilgotf]: https://github.com/vilgotf
[@vivian]: https://github.com/vivian
[@zeylahellyer]: https://github.com/zeylahellyer

[#595]: https://github.com/twilight-rs/twilight/pull/595

[0.11.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-0.11.0
[0.10.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-0.10.1
[0.10.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-0.10.0
[0.9.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-0.9.1
[0.9.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-0.9.0
[0.8.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-0.8.1
[0.8.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-0.8.0
[0.7.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-0.7.0
[0.6.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-0.6.0
[0.5.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-0.5.1
[0.5.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-0.5.0
[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-0.4.0
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-v0.3.0
[0.2.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-v0.2.1
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-v0.2.0
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-v0.2.0-beta.0
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-v0.1.0
