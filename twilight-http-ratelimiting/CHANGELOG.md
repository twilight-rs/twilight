# Changelog

## [0.17.1] - 2025-12-13

**NOTE**: We recommend everyone to update to this version as soon as
possible as the ratelimiter may panic in some cases on version 0.17.0.

### Bug Fixes

- fix new clippy errors in the tests ([#2486](https://github.com/twilight-rs/twilight/issues/2486))
- filter removed queues in actor gc ([#2490](https://github.com/twilight-rs/twilight/issues/2490))

### Features

- improve actor panic message ([#2487](https://github.com/twilight-rs/twilight/issues/2487))

### Performance

- drop stale requests early ([#2469](https://github.com/twilight-rs/twilight/issues/2469))

## [0.17.0] - 2025-11-08

### Bug Fixes

- resolve clippy 1.86 lints ([#2426](https://github.com/twilight-rs/twilight/issues/2426))

### Features

- [**breaking**] rewrite crate ([#2418](https://github.com/twilight-rs/twilight/issues/2418))
- add /oauth2/token path ([#2452](https://github.com/twilight-rs/twilight/issues/2452))
- [**breaking**] support unknown paths ([#2453](https://github.com/twilight-rs/twilight/issues/2453))

### Refactor

- [**breaking**] remove retry delay logic ([#2455](https://github.com/twilight-rs/twilight/issues/2455))

### Testing

- shared resource emulation ([#2457](https://github.com/twilight-rs/twilight/issues/2457))

### Chore

- Update all dependencies ([#2450](https://github.com/twilight-rs/twilight/issues/2450))

## [0.16.0] - 2025-01-12

### Bug Fixes

- Resolved some warnings when building docs. ([#2331](https://github.com/twilight-rs/twilight/issues/2331))
- clippy 1.79 lints ([#2355](https://github.com/twilight-rs/twilight/issues/2355))
- Add missing automoderation paths. ([#2356](https://github.com/twilight-rs/twilight/issues/2356))
- Add missing @ symbol in REST path ([#2357](https://github.com/twilight-rs/twilight/issues/2357))

### Build

- Remove dependency on futures-util ([#2309](https://github.com/twilight-rs/twilight/issues/2309))

### Features

- [**breaking**] Hide `http` dependency ([#2163](https://github.com/twilight-rs/twilight/issues/2163))
- Add support for application editing and new application fields ([#2284](https://github.com/twilight-rs/twilight/issues/2284))
- Add support for premium apps ([#2282](https://github.com/twilight-rs/twilight/issues/2282))
- add support for polls ([#2341](https://github.com/twilight-rs/twilight/issues/2341))
- Add support for application emojis ([#2364](https://github.com/twilight-rs/twilight/issues/2364))

### Chore

- Clarify that MSRV may change in semver-compatible releases ([#2408](https://github.com/twilight-rs/twilight/issues/2408))

## [0.15.2] - 2023-09-10

### Build

- bump MSRV to 1.67 ([#2208](https://github.com/twilight-rs/twilight/issues/2208))

### Features

- Add support for guild onboarding ([#2130](https://github.com/twilight-rs/twilight/issues/2130))

## [0.15.1] - 2023-02-26

### Refactor

- change deny lints to warn ([#2144](https://github.com/twilight-rs/twilight/issues/2144))

## [0.15.0] - 2023-02-05

### Features

- [**breaking**] bring widgets up to date ([#1848](https://github.com/twilight-rs/twilight/issues/1848))

## [0.14.2] - 2023-01-20

### Features

- get current authorization route ([#2049](https://github.com/twilight-rs/twilight/issues/2049))

### Refactor

- remove prelude imports

## [0.14.1] - 2023-01-07

### Features

- add `Bucket::new` ([#2026](https://github.com/twilight-rs/twilight/issues/2026))

## [0.14.0] - 2022-11-14

MSRV has been bumped to 1.64 ([#1897] - [@vilgotf]).

Otherwise, there are no breaking changes.

[#1897]: https://github.com/twilight-rs/twilight/pull/1897

## [0.13.3] - 2022-10-28

### Features

- auto moderation http methods and mention spam ([#1846](https://github.com/twilight-rs/twilight/issues/1846))

## [0.13.2] - 2022-09-08

### Bug Fixes

- explicitly drop lock ([#1885](https://github.com/twilight-rs/twilight/issues/1885))

## [0.13.1] - 2022-09-01

### Bug Fixes

- `/channels/id/thread-members/id` parsing ([#1880](https://github.com/twilight-rs/twilight/issues/1880))

## [0.13.0] - 2022-08-14

### Documentation

- interaction endpoints not bound to global ratelimit ([#1853](https://github.com/twilight-rs/twilight/issues/1853))

## [0.12.2] - 2022-08-11

### Documentation

- interaction endpoints not bound to global ratelimit ([#1853](https://github.com/twilight-rs/twilight/issues/1853))

## [0.12.1] - 2022-07-26

### Documentation

- format doc examples ([#1847](https://github.com/twilight-rs/twilight/issues/1847))

### Features

- add `UpdateGuildMfa` ([#1850](https://github.com/twilight-rs/twilight/issues/1850))

## [0.12.0] - 2022-07-17

### Refactor

- [**breaking**] rework headers ([#1728](https://github.com/twilight-rs/twilight/issues/1728))

## [0.11.1] - 2022-07-07

### Documentation

- use anyhow on public errorable examples ([#1738](https://github.com/twilight-rs/twilight/issues/1738))

### Refactor

- use `instrument` for spans ([#1736](https://github.com/twilight-rs/twilight/issues/1736))
- remove `test_` prexif from tests ([#1767](https://github.com/twilight-rs/twilight/issues/1767))
- standardize clippy lints ([#1782](https://github.com/twilight-rs/twilight/issues/1782))

Changelog for `twilight-http-ratelimiting`.

## [0.11.0] - 2022-05-15

MSRV has been bumped to 1.60.

### Changes

Rename `Ratelimiter::globally_locked` to `Ratelimiter::is_globally_locked`
([#1729] - [@vilgotf]).

`tracing` is no longer an optional dependency and is always enabled
([#1684], [#1730] - [@vilgotf], [@zeylahellyer]).

[#1730]: https://github.com/twilight-rs/twilight/pull/1730
[#1729]: https://github.com/twilight-rs/twilight/pull/1729
[#1684]: https://github.com/twilight-rs/twilight/pull/1684

## [0.10.2] - 2022-05-15

### Additions

Catch erroneous bool header values ([#1724] - [@vilgotf]).

[#1724]: https://github.com/twilight-rs/twilight/pull/1724

## [0.10.1] - 2022-04-15

### Changes

Link to `tracing` in the README ([#1652] - [@zeylahellyer]).

[#1652]: https://github.com/twilight-rs/twilight/pull/1652

## [0.10.0] - 2022-03-10

This major version bump of the HTTP Ratelimiting crate is done to match all
of the other crates in the ecosystem receiving a major version bump.
There are no changes.

## [0.9.0] - 2022-01-22

### Additions

Add `Path` variants for scheduled events ([#1347] - [@7596ff]).

### Changes

`Method::into_http` has been renamed to `to_http` ([#1398] - [@vilgotf]).

`Path` variants that contained a `Box<str>` now contain a `String` ([#1398] -
[@vilgotf]).

`Present::into_bucket` now returns a `String` ([#1398] - [@vilgotf]).

The MSRV has been updated to 1.57 ([#1402] - [@zeylahellyer]).

The Rust edition has been updated to 2021 ([#1412] - [@vilgotf]).

[#1347]: https://github.com/twilight-rs/twilight/pull/1347
[#1398]: https://github.com/twilight-rs/twilight/pull/1398
[#1402]: https://github.com/twilight-rs/twilight/pull/1402
[#1412]: https://github.com/twilight-rs/twilight/pull/1412

## [0.8.4] - 2022-01-21

### Changes

Ensure the `rt` feature of `tokio` is enabled ([#1425] - [@vilgotf]).

[#1425]: https://github.com/twilight-rs/twilight/pull/1425

## [0.8.3] - 2022-01-08

### Additions

Support the `scope` header ([#1351] - [@zeylahellyer]).

Add `Path::UsersIdGuildsIdMember` ([#1355] - [@HTG-YT]).

[#1351]: https://github.com/twilight-rs/twilight/pull/1351
[#1355]: https://github.com/twilight-rs/twilight/pull/1355

## [0.8.2] - 2021-12-25

### Fixes

Revert [#1348], which was causing overflows ([#1357] - [@zeylahellyer]).

[#1357]: https://github.com/twilight-rs/twilight/pull/1357

## [0.8.1] - 2021-12-24

### Fixes

Rework `started_at` header storage so that requests do not start too early
([#1348] - [@zeylahellyer]).

[#1348]: https://github.com/twilight-rs/twilight/pull/1348

## [0.8.0] - 2021-12-03

Initial release ([#1191] - [@Gelbpunkt]). `http` now internally depends
on this crate to ratelimit requests. This crate exposes a trait and
provides a reference implementation using an in-memory backend.

### Fixes

As this is mostly code taken from `http`, it also contains some
bugfixes.

Fixes the template paths to match the Discord API ([#1205] -
[@Gelbpunkt]).

For webhooks, the token is now a major parameter ([#1263] -
[@Gelbpunkt]).

[#1191]: https://github.com/twilight-rs/twilight/pull/1191
[#1205]: https://github.com/twilight-rs/twilight/pull/1205
[#1263]: https://github.com/twilight-rs/twilight/pull/1263

[@7596ff]: https://github.com/7596ff
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@HTG-YT]: https://github.com/HTG-YT
[@vilgotf]: https://github.com/vilgotf
[@zeylahellyer]: https://github.com/zeylahellyer

[0.11.0]: https://github.com/twilight-rs/twilight/releases/tag/http-ratelimiting-0.11.0
[0.10.2]: https://github.com/twilight-rs/twilight/releases/tag/http-ratelimiting-0.10.2
[0.10.1]: https://github.com/twilight-rs/twilight/releases/tag/http-ratelimiting-0.10.1
[0.10.0]: https://github.com/twilight-rs/twilight/releases/tag/http-ratelimiting-0.10.0
[0.9.0]: https://github.com/twilight-rs/twilight/releases/tag/http-ratelimiting-0.9.0
[0.8.4]: https://github.com/twilight-rs/twilight/releases/tag/http-ratelimiting-0.8.4
[0.8.3]: https://github.com/twilight-rs/twilight/releases/tag/http-ratelimiting-0.8.3
[0.8.2]: https://github.com/twilight-rs/twilight/releases/tag/http-ratelimiting-0.8.2
[0.8.1]: https://github.com/twilight-rs/twilight/releases/tag/http-ratelimiting-0.8.1
[0.8.0]: https://github.com/twilight-rs/twilight/releases/tag/http-ratelimiting-0.8.0
