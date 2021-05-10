# Changelog

Changelog for `twilight-mention`.

## [0.4.0] - 2020-05-??

### Upgrade Path

The MSRV is now Rust 1.49.

Instead of importing `twilight_mention::MentionFormat` import
`twilight_mention::fmt::MentionFormat`.

Errors are no longer enums and don't expose their concrete underlying error
source. You can access the underlying error via the implemented
`std::error::Error::source` method or the `into_parts` or `into_source` methods
on each error struct, which will return a boxed `std::error::Error`. To access
the reason for the error use the `kind` or `into_parts` method on error structs;
the returned error type is an enum with variants for each potential reason the
error occurred.

### Enhancements

Remove the `Debug` bound on the `parse::MentionIter`'s `Iterator` implementation
([#764] - [@vivian]).

### Changes

Remove `fmt::MentionFormat` re-export from crate root
([#735] - [@BlackHoleFox]).

[#764]: https://github.com/twilight-rs/twilight/pull/764
[#735]: https://github.com/twilight-rs/twilight/pull/735

## [0.3.0] - 2021-01-08

This major version bump of the Mention crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.2.0] - 2020-10-30

This major version bump of the Mention crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.2.0-beta.0] - 2020-10-10

This major version bump of the Mention crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.1.1] - 2020-09-20

### Added

- Add mention parsing functionality ([#513] - [@vivian])

## [0.1.0] - 2020-09-13

Initial release.

[@BlackHoleFox]: https://github.com/BlackHoleFox
[@vivian]: https://github.com/vivian

[#513]: https://github.com/twilight-rs/twilight/pull/513

[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-0.4.0
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-v0.3.0
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-v0.2.0
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-v0.2.0-beta.0
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/mention-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
