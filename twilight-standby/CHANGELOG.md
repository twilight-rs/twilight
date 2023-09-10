# Changelog

## [unreleased]

### Bug Fixes

- clippy 1.72 lints ([#2268](https://github.com/twilight-rs/twilight/issues/2268))

### Build

- bump MSRV to 1.67 ([#2208](https://github.com/twilight-rs/twilight/issues/2208))

### Features

- Support new username system ([#2231](https://github.com/twilight-rs/twilight/issues/2231))
- add `message_author_id` for reaction add events ([#2244](https://github.com/twilight-rs/twilight/issues/2244))

## [0.15.2] - 2023-04-27

### Documentation

- timeout example ([#2174](https://github.com/twilight-rs/twilight/issues/2174))

### Features

- add `channel` field to `Interaction` ([#2191](https://github.com/twilight-rs/twilight/issues/2191))

## [0.15.1] - 2023-02-26

### Refactor

- change deny lints to warn ([#2144](https://github.com/twilight-rs/twilight/issues/2144))

## [0.15.0] - 2023-02-05

### Refactor

- [**breaking**] move ShardId from gateway to model ([#2097](https://github.com/twilight-rs/twilight/issues/2097))

## [0.14.1] - 2023-01-20

### Features

- add role subscriptions ([#2034](https://github.com/twilight-rs/twilight/issues/2034))

## [0.14.0] - 2022-11-14

MSRV has been bumped to 1.64 ([#1897] - [@vilgotf]).

`standby` is affected by the following `model` changes:

- [**breaking**] move related modules under `message` ([#1831](https://github.com/twilight-rs/twilight/issues/1831))

### Internal Refactor

- clippy 1.65 lints ([#1985](https://github.com/twilight-rs/twilight/issues/1985))

[#1897]: https://github.com/twilight-rs/twilight/pull/1897

## [0.13.2] - 2022-09-29

### Features

- method to get the `guild_id` from an `Event` ([#1899](https://github.com/twilight-rs/twilight/issues/1899))

## [0.13.1] - 2022-09-08

### Features

- add handling for gateway resume url ([#1894](https://github.com/twilight-rs/twilight/issues/1894))

## [0.13.0] - 2022-08-14

### Refactor

- [**breaking**] update `VoiceServerUpdate` ([#1837](https://github.com/twilight-rs/twilight/issues/1837))

## [0.12.1] - 2022-07-26

### Documentation

- format doc examples ([#1847](https://github.com/twilight-rs/twilight/issues/1847))

## [0.12.0] - 2022-07-17

### Features

- auto moderation models ([#1796](https://github.com/twilight-rs/twilight/issues/1796))

### Refactor

- [**breaking**] make interaction a struct ([#1813](https://github.com/twilight-rs/twilight/issues/1813))

## [0.11.1] - 2022-07-07

### Features

- add `app_permissions` field on interactions ([#1805](https://github.com/twilight-rs/twilight/issues/1805))

### Refactor

- standardize clippy lints ([#1785](https://github.com/twilight-rs/twilight/issues/1785))

Changelog for `twilight-standby`.

## [0.11.0] - 2022-05-15

MSRV has been bumped to 1.60.

### Changes

Pin `dashmap` to 5.3 ([#1702] - [@baptiste0928]).

`tracing` is no longer an optional dependency and is always enabled
([#1684], [#1730] - [@vilgotf], [@zeylahellyer]).

[#1730]: https://github.com/twilight-rs/twilight/pull/1730
[#1702]: https://github.com/twilight-rs/twilight/pull/1702
[#1684]: https://github.com/twilight-rs/twilight/pull/1684

## [0.10.0] - 2022-03-10

This major version bump of the Standby crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no public
changes.

## [0.9.1] - 2022-02-12

### Changes

Update `dashmap` to `5.1`, which fixes unsoundness present in `5.0` (which
previously forced a downgrade to `4.0`) ([#1517] - [@Gelbpunkt]).

[#1517]: https://github.com/twilight-rs/twilight/pull/1517

## [0.9.0] - 2022-01-22

### Changes

All types and method signatures have been updated to use the new `Id<T>` syntax
([#1260] - [@zeylahellyer]).

The MSRV has been updated to 1.57 ([#1402] - [@zeylahellyer]).

The Rust edition has been updated to 2021 ([#1412] - [@vilgotf]).

### Fixes

Missing ID marker implementations have been added ([#1471] - [@zeylahellyer]).

[#1260]: https://github.com/twilight-rs/twilight/pull/1260
[#1402]: https://github.com/twilight-rs/twilight/pull/1402
[#1412]: https://github.com/twilight-rs/twilight/pull/1412
[#1471]: https://github.com/twilight-rs/twilight/pull/1471

## [0.8.3] - 2022-01-11

### Fixes

Downgrade `dashmap` to `4.0`, to prevent an issue with `Ref::value` and `dashmap
5.0` ([#1434] - [@baptiste0928]).

[#1434]: https://github.com/twilight-rs/twilight/pull/1434

## [0.8.2] - 2022-01-08

### Changes

Support the fixed `ThreadDelete` event ([#1426] - [@AEnterprise]).

[#1426]: https://github.com/twilight-rs/twilight/pull/1426

## [0.8.1] - 2021-12-24

### Changes

Upgrade `dashmap` to 5.0 ([#1336] - [@vilgotf]). `dashmap` 4.0 is still allowed.

[#1336]: https://github.com/twilight-rs/twilight/pull/1336

## [0.8.0] - 2021-12-03

### Additions

Results of event processing are now exposed through a `ProcessResults`
struct ([#1160] - [@zeylahellyer]).

[#1160]: https://github.com/twilight-rs/twilight/pull/1160

## [0.7.1] - 2021-10-29

### Additions

Add `Standby::wait_for_component` and `wait_for_component_stream`, which
filter events based on a matching `MessageComponentInteraction` ([#1189]
- [@PyroTechniac]).

[#1189]: https://github.com/twilight-rs/twilight/pull/1189

## [0.7.0] - 2021-10-21

### Changes

`Standby` no longer implements `Clone`, because it is no longer
internally wrapped in an `Arc` ([#1067] - [@zeylahellyer]). To retain
this functionality, you can wrap them it an `Arc` or a `Rc` manually.

Refactor and fully document internals ([#1159] - [@zeylahellyer]).
Internal logic and functionality is largely untouched.

The MSRV has been updated to 1.53 ([#1161] - [@7596ff]).

[#1067]: https://github.com/twilight-rs/twilight/pull/1067
[#1159]: https://github.com/twilight-rs/twilight/pull/1159
[#1161]: https://github.com/twilight-rs/twilight/pull/1161

## [0.6.2] - 2021-09-17

### Thread Support

Supports thread-related events.

## [0.6.1] - 2021-09-17

### Changes

This release contains internal refactors, there are no public facing
changes.

## [0.6.0] - 2021-07-31

### Enhancements

The `tracing` feature is now optional ([#985] - [@zeylahellyer]).

[#985]: https://github.com/twilight-rs/twilight/pull/985

## [0.5.1] - 2021-07-23

### Changes

`#![deny(unsafe_code)]` has been added, ensuring no unsafe code exists in the
crate ([#1042] - [@zeylahellyer]).

[#1042]: https://github.com/twilight-rs/twilight/pull/1042

## [0.5.0] - 2021-06-13

This major version bump of the Standby crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.4.1] - 2021-06-12

### Changes

Support the new events added in model: `IntegrationCreate, IntegrationDelete,
IntegrationUpdate, StageInstanceCreate`, `StageInstanceDelete`,
`StageInstanceUpdate` ([#845], [#914] - [@7596ff]).

[#845]: https://github.com/twilight-rs/twilight/pull/845
[#914]: https://github.com/twilight-rs/twilight/pull/914

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

`tokio` is now a required runtime dependency.

### Enhancements

The `futures-channel` dependency has been removed in favor of `tokio` due to the
dependency of it on other Twilight crates ([#785] - [@Gelbpunkt]).

[#785]: https://github.com/twilight-rs/twilight/pull/785

## [0.3.0] - 2021-01-08

This major version bump of the Standby crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.2.2] - 2021-01-05

### Enhancements

Upgrade `dashmap` from version 3 to 4.0 ([#666] - [@vivian]).

[#666]: https://github.com/twilight-rs/twilight/pull/666

## [0.2.1] - 2020-11-29

### Misc.

Use the renamed
`twilight_model::gateway::payload::identify::IdentityInfo::compress`
model field ([#624] - [@chamburr]).

## [0.2.0] - 2020-10-30

This major version bump of Standby is done to match all of the other crates in
the ecosystem receiving a major version bump. There are no changes.

## [0.2.0-beta.0] - 2020-10-10

This major version bump of Standby is done to match all of the other crates in
the ecosystem receiving a major version bump. There are no changes.

## [0.1.1] - 2020-09-25

### Fixes

- Fix typo in documentation link ([#523] - [@nickelc])

## [0.1.0] - 2020-09-13

Initial release.

[@7596ff]: https://github.com/7596ff
[@AEnterprise]: https://github.com/AEnterprise
[@baptiste0928]: https://github.com/baptiste0928
[@chamburr]: https://github.com/chamburr
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@nickelc]: https://github.com/nickelc
[@PyroTechniac]: https://github.com/PyroTechniac
[@vilgotf]: https://github.com/vilgotf
[@vivian]: https://github.com/vivian
[@zeylahellyer]: https://github.com/zeylahellyer

[#624]: https://github.com/twilight-rs/twilight/pull/624
[#523]: https://github.com/twilight-rs/twilight/pull/523

[0.11.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.11.0
[0.10.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.10.0
[0.9.1]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.9.1
[0.9.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.9.0
[0.8.3]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.8.3
[0.8.2]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.8.2
[0.8.1]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.8.1
[0.8.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.8.0
[0.7.1]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.7.1
[0.7.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.7.0
[0.6.2]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.6.2
[0.6.1]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.6.1
[0.6.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.6.0
[0.5.1]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.5.1
[0.5.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.5.0
[0.4.1]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.4.1
[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.4.0
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-v0.3.0
[0.2.2]: https://github.com/twilight-rs/twilight/releases/tag/standby-v0.2.2
[0.2.1]: https://github.com/twilight-rs/twilight/releases/tag/standby-v0.2.1
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-v0.2.0
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-v0.2.0-beta.0
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/standby-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
