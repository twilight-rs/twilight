# Changelog

Changelog for `twilight-util`.

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

## [0.3.0] - 2021-01-08

This major version bump of the Util crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no
breaking changes.

### Upgrade Path

There is no upgrade path.

### Additions

Add `link::webhook::parse` for parsing webhook IDs and tokens out of webhook
URLs ([#658] - [@vivian]).

[#658]: https://github.com/twilight-rs/twilight/pull/658

## [0.2.0] - 2020-10-30

This major version bump of the Util crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.2.0-beta.0] - 2020-10-10

This major version bump of the Util crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.1.0] - 2020-10-07

Initial release.

[@vivian]: https://github.com/vivian

[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/util-0.4.0
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/util-v0.3.0
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/util-v0.2.0
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/util-v0.2.0-beta.0
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/util-v0.1.0
