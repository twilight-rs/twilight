# Changelog

Changelog for `twilight-gateway-queue`.

## [0.4.0] - 2021-05-??

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

[@Gelbpunkt]: https://github.com/Gelbpunkt
[@vivian]: https://github.com/vivian

[#595]: https://github.com/twilight-rs/twilight/pull/595

[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-0.4.0
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-v0.3.0
[0.2.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-v0.2.1
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-v0.2.0
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-v0.2.0-beta.0
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-queue-v0.1.0
