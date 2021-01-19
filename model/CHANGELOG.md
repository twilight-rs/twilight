# Changelog

Changelog for `twilight-model`.

## [0.1.7] - 2021-01-19

Version 0.1 will only receive bugfixes until the deprecation of Discord gateway
version 6. We recommend upgrading.

### Fixes

Make the `AuditLog` fields public ([#662] - [@jazevedo620]).

[#662]: https://github.com/twilight-rs/twilight/pull/662

## [0.1.6] - 2020-11-28

While v0.1 will be maintained until the deprecation of version 6 of the Discord
API, we recommend upgrading to v0.2.

### Additions

Support the Message Stickers feature ([#608], [#622] - [@chamburr], [@vivian]).

Add gateway and voice close codes and voice opcodes ([#586] - [@chamburr]).

### Enhancements

Document gateway opcode variants ([#586] - [@chamburr]).

### Fixes

Create a new trimmed down channel type for embedded use in invites
([#601] - [@sam-kirby]).

## [0.1.5] - 2020-11-07

This release includes a few bugfixes. While v0.1 will be maintained until the
deprecation of version 6 of the Discord API, we recommend upgrading to v0.2.

### Enhancements

Document gateway intents ([#582] - [@7596ff]).

## [0.1.4] - 2020-10-22

### Additions

Add the `channel::FollowedChannel` struct to include support for the Followed
Channels API feature ([#556] - [@Gelbpunkt]).

## [0.1.3] - 2020-09-25

### Added

- Support deserializing user discriminators from integers ([#526] - [@vivian])

## [0.1.2] - 2020-09-17

### Added

- Implement `serde_mappable_seq::Key` for UserOrId ([#509] - [@coadler])

### Fixes

- Fix compilation of benchmarks ([#511] - [@Erk-])

## [0.1.1] - 2020-09-14

### Fixes

- support deserializing IDs from integers ([#499] - [@vivian])

## [0.1.0] - 2020-09-13

Initial release.

[@7596ff]: https://github.com/7596ff
[@chamburr]: https://github.com/chamburr
[@coadler]: https://github.com/coadler
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@jazevedo620]: https://github.com/jazevedo620
[@sam-kirby]: https://github.com/sam-kirby
[@vivian]: https://github.com/vivian

[#622]: https://github.com/twilight-rs/twilight/pull/622
[#608]: https://github.com/twilight-rs/twilight/pull/608
[#601]: https://github.com/twilight-rs/twilight/pull/601
[#586]: https://github.com/twilight-rs/twilight/pull/586
[#582]: https://github.com/twilight-rs/twilight/pull/582
[#556]: https://github.com/twilight-rs/twilight/pull/556
[#526]: https://github.com/twilight-rs/twilight/pull/526
[#511]: https://github.com/twilight-rs/twilight/pull/511
[#509]: https://github.com/twilight-rs/twilight/pull/509
[#499]: https://github.com/twilight-rs/twilight/pull/499

[0.1.7]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.7
[0.1.6]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.6
[0.1.5]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.5
[0.1.4]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.4
[0.1.3]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.3
[0.1.2]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.2
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
