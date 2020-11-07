# Changelog

Changelog for `twilight-command-parser`.

## [0.1.3] - 2020-11-07

This release includes a few bugfixes. While v0.1 will be maintained until the
deprecation of version 6 of the Discord API, we recommend upgrading to v0.2.

### Fixes

Add unicode support to the `Arguments`
iterator ([#575], [#585] - [@AsianIntel], [@vivian]).

Take whitespace after prefixes into account when creating the initial index for
argument iterating, for example when the prefix is a user mention. This fixes
the first argument being the last letter of the command
name ([#584] - [@vivian]).

## [0.1.2] - 2020-10-07

### Fixes

- Fix typos in links ([#515] - [@nickelc])

## [0.1.1] - 2020-09-17

### Fixes

- Fix benchmark compilation ([#511] - [@Erk-])

## [0.1.0] - 2020-09-13

Initial release.

[@AsianIntel]: https://github.com/AsianIntel
[@Erk-]: https://github.com/Erk-
[@nickelc]: https://github.com/nickelc
[@vivian]: https://github.com/vivian

[#585]: https://github.com/twilight-rs/twilight/pull/585
[#584]: https://github.com/twilight-rs/twilight/pull/584
[#575]: https://github.com/twilight-rs/twilight/pull/575
[#515]: https://github.com/twilight-rs/twilight/pull/515
[#511]: https://github.com/twilight-rs/twilight/pull/511

[0.1.3]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-v0.1.3
[0.1.2]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-v0.1.2
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/command-parser-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
