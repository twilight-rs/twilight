# Changelog

## [0.16.0] - 2025-01-12

### Bug Fixes

- Address new rust CI errors ([#2285](https://github.com/twilight-rs/twilight/issues/2285))

### Features

- Add mention for commands ([#2290](https://github.com/twilight-rs/twilight/issues/2290))

### Refactor

- Remove redundant imports ([#2316](https://github.com/twilight-rs/twilight/issues/2316))

### Chore

- resolve rust 1.83 issues ([#2391](https://github.com/twilight-rs/twilight/issues/2391))
- Clarify that MSRV may change in semver-compatible releases ([#2408](https://github.com/twilight-rs/twilight/issues/2408))
- Fix clippy 1.84 lints ([#2409](https://github.com/twilight-rs/twilight/issues/2409))

## [0.15.2] - 2023-09-10

### Build

- bump MSRV to 1.67 ([#2208](https://github.com/twilight-rs/twilight/issues/2208))

### Refactor

- reformat code with rustfmt 1.6.0 ([#2233](https://github.com/twilight-rs/twilight/issues/2233))

## [0.15.1] - 2023-02-26

### Refactor

- change deny lints to warn ([#2144](https://github.com/twilight-rs/twilight/issues/2144))

## [0.15.0] - 2023-02-05

This major version bump of the Gateway Queue is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.14.0] - 2022-11-14

MSRV has been bumped to 1.64 ([#1897] - [@vilgotf]).

Otherwise, there are no breaking changes.

[#1897]: https://github.com/twilight-rs/twilight/pull/1897

## [0.12.1] - 2022-07-26

### Documentation

- format doc examples ([#1847](https://github.com/twilight-rs/twilight/issues/1847))

## [0.11.1] - 2022-07-07

### Documentation

- use anyhow on public errorable examples ([#1738](https://github.com/twilight-rs/twilight/issues/1738))

### Refactor

- remove `test_` prexif from tests ([#1767](https://github.com/twilight-rs/twilight/issues/1767))
- standardize clippy lints ([#1784](https://github.com/twilight-rs/twilight/issues/1784))

Changelog for `twilight-mention`.

## [0.11.0] - 2022-05-15

MSRV has been bumped to 1.60.

### Changes

Remove member nickname support as it was removed by Discord
([#1650] - [@zeylahellyer]).

[#1650]: https://github.com/twilight-rs/twilight/pull/1650

## [0.10.0] - 2022-03-10

### Changes

Implementations of `Mention` for `CategoryChannel`, `Group`, `GuildChannel`,
`PrivateChannel`, `TextChannel`, and `VoiceChannel` have been removed now that
`Channel` is a unified type ([#1449] - [@zeylahellyer]).

[#1449]: https://github.com/twilight-rs/twilight/pull/1449

## [0.9.0] - 2022-01-22

### Changes

All types and method signatures have been updated to use the new `Id<T>` syntax
([#1260] - [@zeylahellyer]).

The MSRV has been updated to 1.57 ([#1402] - [@zeylahellyer]).

The Rust edition has been updated to 2021 ([#1412] - [@vilgotf]).

[#1260]: https://github.com/twilight-rs/twilight/pull/1260
[#1402]: https://github.com/twilight-rs/twilight/pull/1402
[#1412]: https://github.com/twilight-rs/twilight/pull/1412

## [0.8.0] - 2021-12-03

This major version bump of the Mention crate is done to match all of the
other crates in the ecosystem receiving a major version bump. There are
no changes.

## [0.7.0] - 2021-10-21

### Changes

The MSRV has been updated to 1.53 ([#1161] - [@7596ff]).

[#1161]: https://github.com/twilight-rs/twilight/pull/1161

## [0.6.0] - 2021-07-31

This major version bump of the Mention crate is done to match all of the
other crates in the ecosystem receiving a major version bump.  There are
no changes.

## [0.5.1] - 2021-07-02

### Additions

Support the new timestamp (`<t:unixtimestamp:?style>`) format via the new
`timestamp` module by adding a formatter and parser implementation
([#945] - [@zeylahellyer]).

### Enhancements

Improve the `Display` implementation performance of `ParseMentionError`'s
`Display` implementation by calling `Formatter` methods directly instead of
calling the `format_args!` and `write!` macros ([#944] - [@zeylahellyer]).

Improve the performance of the `fmt` module's Display implementations by
~10% by directly writing to Formatters instead of calling the `format_args!`
macro ([#942] - [@zeylahellyer]).

[#945]: https://github.com/twilight-rs/twilight/pull/945
[#944]: https://github.com/twilight-rs/twilight/pull/944
[#942]: https://github.com/twilight-rs/twilight/pull/942

## [0.5.0] - 2021-06-13

This major version bump of the Mention crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.4.2] - 2021-06-12

### Enhancements

The mention implementation has been made more generic. There are no breaking
changes ([#883] - [@PyroTechniac]).

[#883]: https://github.com/twilight-rs/twilight/pull/883

## [0.4.1] - 2021-05-30

### Enhancements

The following functions are now `const`:

- `parse::MentionIter::as_str`
- `parse::ParseMentionError::kind`

([#824] - [@vivian]).

[#824]: https://github.com/twilight-rs/twilight/pull/824

## [0.4.0] - 2020-05-12

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

[@7596ff]: https://github.com/7596ff
[@BlackHoleFox]: https://github.com/BlackHoleFox
[@PyroTechniac]: https://github.com/PyroTechniac
[@vivian]: https://github.com/vivian
[@zeylahellyer]: https://github.com/zeylahellyer

[#513]: https://github.com/twilight-rs/twilight/pull/513

[0.11.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-0.11.0
[0.10.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-0.10.0
[0.9.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-0.9.0
[0.8.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-0.8.0
[0.7.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-0.7.0
[0.6.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-0.6.0
[0.5.1]: https://github.com/twilight-rs/twilight/releases/tag/mention-0.5.1
[0.5.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-0.5.0
[0.4.2]: https://github.com/twilight-rs/twilight/releases/tag/mention-0.4.2
[0.4.1]: https://github.com/twilight-rs/twilight/releases/tag/mention-0.4.1
[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-0.4.0
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-v0.3.0
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-v0.2.0
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/mention-v0.2.0-beta.0
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/mention-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
