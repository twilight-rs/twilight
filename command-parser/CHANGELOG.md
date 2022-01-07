# Changelog

Changelog for `twilight-command-parser`.

## [0.8.1] - 2021-12-27

The Command Parser crate has been deprecated in favor of Gateway or HTTP
interactions and will be in critical bugfixing mode ([#1352] - [@zeylahellyer]).

[#1352]: https://github.com/twilight-rs/twilight/pull/1352

## [0.8.0] - 2021-12-03

This major version bump of the Command Parser crate is done to match all
of the other crates in the ecosystem receiving a major version bump.
There are no changes.

## [0.7.0] - 2021-10-21

### Changes

The MSRV has been updated to 1.53 ([#1161] - [@7596ff]).

[#1161]: https://github.com/twilight-rs/twilight/pull/1161

## [0.6.0] - 2021-07-31

This major version bump of the Command Parser crate is done to match all of the
other crates in the ecosystem receiving a major version bump. There are no
changes.

## [0.5.1] - 2021-07-23

### Changes

`#![deny(unsafe_code)]` has been added, ensuring no unsafe code exists in the
crate ([#1042] - [@zeylahellyer]).

[#1042]: https://github.com/twilight-rs/twilight/pull/1042

## [0.5.0] - 2021-06-13

This major version bump of the Command Parser crate is done to match all of the
other crates in the ecosystem receiving a major version bump. There are no
changes.

## [0.4.2] - 2021-06-12

### Enhancements

`Arguments::as_str` now uses the borrowed string's lifetime ([#852] -
[@vilgotf]).

[#852]: https://github.com/twilight-rs/twilight/pull/852

## [0.4.1] - 2021-05-30

### Enhancements

The following functions are now `const`:

- `Arguments::as_str`
- `CaseSensitivity::is_sensitive`
- `CommandParserConfig::new`
- `Parser::config`

([#824] - [@vivian]).

[#824]: https://github.com/twilight-rs/twilight/pull/824

## [0.4.0] - 2021-05-12

### Upgrade Path

The MSRV is now Rust 1.49.

Create an `Arguments` instance via `Arguments::new` instead of using the `From`
implementation.

When checking if a command is case-sensitive use
`CaseSensitivity::is_sensitive`.

### Changes

Remove `From<&str>` implementation for `Arguments` ([#763] - [@vivian]).

Hide the `unicase` dependency by offering alternatives in the API:

- Add `CaseSensitivity::is_sensitive` to check if the command is case-sensitive
- Implement `AsMut<str>` for `CaseSensitivity`
- `Commands` now iterates over `(&str, bool)` instead of `&CaseSensitivity`
- `CommandsMut` now iterates over `(&mut str, bool)` instead of
`&mut CaseSensitivity`

([#692] - [@vivian]).

[#692]: https://github.com/twilight-rs/twilight/pull/692
[#763]: https://github.com/twilight-rs/twilight/pull/763

## [0.3.0] - 2021-01-08

This major version bump of the Command Parser crate is done to match all of the
other crates in the ecosystem receiving a major version bump. There are no
changes.

## [0.2.3] - 2020-12-30

### Enhancements

Export the `config` module's iterators to allow them to be documented by rustdoc
([#646] - [@vivian]).

[#646]: https://github.com/twilight-rs/twilight/pull/646

## [0.2.2] - 2020-11-02

Remove the `unicode-segmentation` dependency due to the functionality used from
it also being in the stdlib ([#585] - [@vivian]).

## [0.2.1] - 2020-10-31

### Fixes

Take whitespace after prefixes into account when creating the initial index for
argument iterating, for example when the prefix is a user mention. This fixes
the first argument being the last letter of the command
name ([#584] - [@vivian]).

## [0.2.0] - 2020-10-30

This major version bump of the Command Parser is primarily done to match all of
the other crates in the ecosystem receiving a major version bump. There are no
significant API changes.

### Fixes

Add unicode support to the `Arguments` iterator ([#575] - [@AsianIntel]).

## [0.2.0-beta.0] - 2020-10-10

This major version bump of the Command Parser is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.1.2] - 2020-10-07

### Fixes

- Fix typos in links ([#515] - [@nickelc])

## [0.1.1] - 2020-09-17

### Fixes

- Fix benchmark compilation ([#511] - [@Erk-])

## [0.1.0] - 2020-09-13

Initial release.

[@7596ff]: https://github.com/7596ff
[@AsianIntel]: https://github.com/AsianIntel
[@Erk-]: https://github.com/Erk-
[@nickelc]: https://github.com/nickelc
[@vilgotf]: https://github.com/vilgotf
[@vivian]: https://github.com/vivian
[@zeylahellyer]: https://github.com/zeylahellyer

[#585]: https://github.com/twilight-rs/twilight/pull/585
[#584]: https://github.com/twilight-rs/twilight/pull/584
[#575]: https://github.com/twilight-rs/twilight/pull/575
[#515]: https://github.com/twilight-rs/twilight/pull/515
[#511]: https://github.com/twilight-rs/twilight/pull/511

[0.8.1]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-0.8.1
[0.8.0]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-0.8.0
[0.7.0]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-0.7.0
[0.6.0]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-0.6.0
[0.5.1]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-0.5.1
[0.5.0]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-0.5.0
[0.4.2]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-0.4.2
[0.4.1]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-0.4.1
[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-0.4.0
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-v0.3.0
[0.2.2]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-v0.2.2
[0.2.1]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-v0.2.1
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-v0.2.0
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-v0.2.0-beta.0
[0.1.2]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-v0.1.2
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
