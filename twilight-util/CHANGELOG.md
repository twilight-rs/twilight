# Changelog

## [unreleased]

### Documentation

- fix mention on how color works ([#1893](https://github.com/twilight-rs/twilight/issues/1893))

## [0.13.1] - 2022-09-01

### Refactor

- mark c-style enums `#[non_exhaustive]` ([#1862](https://github.com/twilight-rs/twilight/issues/1862))

## [0.13.0] - 2022-08-14

### Refactor

- [**breaking**] remove `Number` ([#1817](https://github.com/twilight-rs/twilight/issues/1817))

## [0.12.1] - 2022-07-26

### Bug Fixes

- member overwrites take precedence over roles ([#1824](https://github.com/twilight-rs/twilight/issues/1824))

### Documentation

- format doc examples ([#1847](https://github.com/twilight-rs/twilight/issues/1847))

## [0.12.0] - 2022-07-17

### Features

- initial pass at dealing with unknown enum variants ([#1550](https://github.com/twilight-rs/twilight/issues/1550))
- `CommandOption` max and min length ([#1826](https://github.com/twilight-rs/twilight/issues/1826))
- [**breaking**] builders use `Into<String>` and `IntoIterator` everywhere ([#1774](https://github.com/twilight-rs/twilight/issues/1774))

### Refactor

- [**breaking**] remove `EmbedAuthorBuilder::name` ([#1807](https://github.com/twilight-rs/twilight/issues/1807))

## [0.11.1] - 2022-07-07

### Refactor

- remove `test_` prexif from tests ([#1767](https://github.com/twilight-rs/twilight/issues/1767))
- standardize clippy lints ([#1786](https://github.com/twilight-rs/twilight/issues/1786))
- add `#[non_exhaustive]` to c-style enums ([#1795](https://github.com/twilight-rs/twilight/issues/1795))

Changelog for `twilight-util`.

## [0.11.0] - 2022-05-15

MSRV has been bumped to 1.60.

This major version bump of the Util crate is done to match all of the
other crates in the ecosystem receiving a major version bump.  There are
no changes.

## [0.10.1] - 2022-03-20

### Additions

Add `builder::embed` ([#1539] - [@7596ff], [@vilgotf]).

Add `CommandBuilder::validate` ([#1549] - [@7586ff]).

### Changes

Update documentation for `InteractionResponseDataBuilder::flags` ([#1579] -
[@laralove143]).

[#1539]: https://github.com/twilight-rs/twilight/pull/1539
[#1549]: https://github.com/twilight-rs/twilight/pull/1549
[#1579]: https://github.com/twilight-rs/twilight/pull/1579

## [0.10.0] - 2022-03-10

### Changes

Rename `CallbackDataBuilder` to `InteractionResponseDataBuilder`, and add the
methods `attachments`, `choices`, `custom_id`, and `title` ([#1300], [#1508] -
[@itohatweb], [@7596ff]).

Update the permission calculator to use the new `PermissionOverwrite` type
([#1521] - [@7596ff]).

[#1300]: https://github.com/twilight-rs/twilight/pull/1300
[#1508]: https://github.com/twilight-rs/twilight/pull/1508
[#1521]: https://github.com/twilight-rs/twilight/pull/1521

## [0.9.1] - 2022-02-21

### Additions

Add an `AttachmentBuilder` option to `CommandBuilder` ([#1545] - [@Erk-]).

[#1545]: https://github.com/twilight-rs/twilight/pull/1545

## [0.9.0] - 2022-01-22

### Changes

All types and method signatures have been updated to use the new `Id<T>` syntax
([#1260] - [@zeylahellyer]).

The MSRV has been updated to 1.57 ([#1402] - [@zeylahellyer]).

The Rust edition has been updated to 2021 ([#1412] - [@vilgotf]).

[#1260]: https://github.com/twilight-rs/twilight/pull/1260
[#1402]: https://github.com/twilight-rs/twilight/pull/1402
[#1412]: https://github.com/twilight-rs/twilight/pull/1412

## [0.8.2] - 2022-01-08

### Changes

Support `CallbackData::embeds: Option<Vec<Embed>>` ([#1401] - [@itohatweb]).

[#1401]: https://github.com/twilight-rs/twilight/pull/1401

## [0.8.1] - 2021-12-15

### Fixes

Add a missing implementation of `Snowflake` for `CommandVersionId` ([#1315] -
[@vilgotf]).

[#1315]: https://github.com/twilight-rs/twilight/pull/1315

## [0.8.0] - 2021-12-03

### Changes

`CommandBuilder::{application_id, id}` have been removed since they are
not sent to Discord regardless ([#1233] - [@vilgotf]).

`SubCommandGroupBuilder::options` has been renamed to `subcommands`
([#1233] - [@vilgotf]).

[#1233]: https://github.com/twilight-rs/twilight/pull/1233

## [0.7.1] - 2021-11-20

### Additions

Add autocomplete support to the `CommandBuilder` ([#1228] - [@vilgotf]).

Add `max_value` and `min_value` support to `IntegerBuilder` and `NumberBuilder`
([#1235] - [@baptiste0928]).

[#1228]: https://github.com/twilight-rs/twilight/pull/1228
[#1235]: https://github.com/twilight-rs/twilight/pull/1235

## [0.7.0] - 2021-10-21

### Changes

`PermissionCalculator::owner_id` is now an `Option<UserId>` ([#1039] -
[@vilgotf]).

The MSRV has been updated to 1.53 ([#1161] - [@7596ff]).

[#1039]: https://github.com/twilight-rs/twilight/pull/1039
[#1161]: https://github.com/twilight-rs/twilight/pull/1161

## [0.6.2] - 2021-10-07

Adds the `builder` module, which is currently populated with the
`CommandBuilder` ([#1048] - [@vilgotf]) and the `CallbackDataBuilder`
([#1146] - [@baptiste0928]).

[#1048]: https://github.com/twilight-rs/twilight/pull/1048
[#1146]: https://github.com/twilight-rs/twilight/pull/1146

## [0.6.1] - 2021-09-17

### Changes

The example for `Snowflake::timestamp` has been updated to use version
0.3 of the `time` crate ([#1145] - [@vilgotf]).

[#1145]: https://github.com/twilight-rs/twilight/pull/1145

## [0.6.0] - 2021-07-31

This major version bump of the Util crate is done to match all of the
other crates in the ecosystem receiving a major version bump.  There are
no changes.

## [0.5.2] - 2021-07-14

### Additions

Document the `link` feature. This was already present, just not mentioned in any
documentation ([#1011] - [@zeylahellyer]).

[#1011]: https://github.com/twilight-rs/twilight/pull/1011

## [0.5.1] - 2021-07-02

### Additions

Add a calculator for calculating the permissions of a member on a guild level or
in a specific channel, exposed via the `permission-calculator` feature flag
([#834] - [@zeylahellyer]).

Implement the `snowflake::Snowflake` trait on the new `twilight_model::id` types
`ApplicationId`, `CommandId`, `IntegrationId`, and `InteractionId`
([#950] - [@zeylahellyer]).

[#950]: https://github.com/twilight-rs/twilight/pull/950
[#834]: https://github.com/twilight-rs/twilight/pull/834

## [0.5.0] - 2021-06-13

This major version bump of the Util crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no
breaking changes.

## [0.4.1] - 2021-05-30

### Enhancements

`link::webhook::WebhookParseError::kind` is now `const` ([#824] - [@vivian]).

[#824]: https://github.com/twilight-rs/twilight/pull/824

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

[@7596ff]: https://github.com/7596ff
[@baptiste0928]: https://github.com/baptiste0928
[@itohatweb]: https://github.com/itohatweb
[@laralove143]: https://github.com/laralove143
[@vilgotf]: https://github.com/vilgotf
[@vivian]: https://github.com/vivian
[@zeylahellyer]: https://github.com/zeylahellyer

[0.11.0]: https://github.com/twilight-rs/twilight/releases/tag/util-0.11.0
[0.10.1]: https://github.com/twilight-rs/twilight/releases/tag/util-0.10.1
[0.10.0]: https://github.com/twilight-rs/twilight/releases/tag/util-0.10.0
[0.9.1]: https://github.com/twilight-rs/twilight/releases/tag/util-0.9.1
[0.9.0]: https://github.com/twilight-rs/twilight/releases/tag/util-0.9.0
[0.8.2]: https://github.com/twilight-rs/twilight/releases/tag/util-0.8.2
[0.8.1]: https://github.com/twilight-rs/twilight/releases/tag/util-0.8.1
[0.8.0]: https://github.com/twilight-rs/twilight/releases/tag/util-0.8.0
[0.7.0]: https://github.com/twilight-rs/twilight/releases/tag/util-0.7.0
[0.6.2]: https://github.com/twilight-rs/twilight/releases/tag/util-0.6.2
[0.6.1]: https://github.com/twilight-rs/twilight/releases/tag/util-0.6.1
[0.6.0]: https://github.com/twilight-rs/twilight/releases/tag/util-0.6.0
[0.5.2]: https://github.com/twilight-rs/twilight/releases/tag/util-0.5.2
[0.5.1]: https://github.com/twilight-rs/twilight/releases/tag/util-0.5.1
[0.5.0]: https://github.com/twilight-rs/twilight/releases/tag/util-0.5.0
[0.4.1]: https://github.com/twilight-rs/twilight/releases/tag/util-0.4.1
[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/util-0.4.0
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/util-v0.3.0
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/util-v0.2.0
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/util-v0.2.0-beta.0
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/util-v0.1.0
