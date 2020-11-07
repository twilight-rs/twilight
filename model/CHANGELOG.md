# Changelog

Changelog for `twilight-model`.

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
[@coadler]: https://github.com/coadler
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@vivian]: https://github.com/vivian

[#582]: https://github.com/twilight-rs/twilight/pull/582
[#556]: https://github.com/twilight-rs/twilight/pull/556
[#526]: https://github.com/twilight-rs/twilight/pull/526
[#511]: https://github.com/twilight-rs/twilight/pull/511
[#509]: https://github.com/twilight-rs/twilight/pull/509
[#499]: https://github.com/twilight-rs/twilight/pull/499

[0.1.5]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.5
[0.1.4]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.4
[0.1.3]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.3
[0.1.2]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.2
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
