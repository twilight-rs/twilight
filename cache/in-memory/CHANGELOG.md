# Changelog

Changelog for `twilight-cache-inmemory`.

## [0.1.5] - 2020-11-28

While v0.1 will be maintained until the deprecation of version 6 of the Discord
API, we recommend upgrading to v0.2.

### Additions

Cache members and users received from new messages ([#590] - [@MaxOhn]).

### Enhancements

Don't create useless HashSets in internal functions ([#591] - [@MaxOhn]).

## [0.1.4] - 2020-11-07

This release includes a few bugfixes. While v0.1 will be maintained until the
deprecation of version 6 of the Discord API, we recommend upgrading to v0.2.

### Fixes

Remove old and removed method of cache building in
documentation ([#576] - [@nickelc]).

Properly track guilds' lists of their emojis ([#557] - [@DusterTheFirst]).

### Enhancements

Document which cache access methods require which gateway
intents ([#582] - [@7596ff]).

## [0.1.3] - 2020-10-22

### Fixes

Fix voice state map retaining user states, causing false duplicate voice states
for a single user ([#555] - [@DusterTheFirst]).

## [0.1.2] - 2020-10-17

### Fixes

Correctly maintain relation sets for guilds. For example, the set of the IDs of
roles within a guild was not being inserted into with new role
IDs ([#540] - [@DusterTheFirst])

Correctly update guilds in `GuildUpdate` events ([#553] - [@DusterTheFirst])

## [0.1.1] - 2020-10-05

### Added

- Add methods to fetch guild relation ID sets ([#528] - [@Erk-])

## [0.1.0] - 2020-09-13

Initial release.

[@7596ff]: https://github.com/7596ff
[@DusterTheFirst]: https://github.com/DusterTheFirst
[@Erk-]: https://github.com/Erk-
[@MaxOhn]: https://github.com/MaxOhn
[@nickelc]: https://github.com/nickelc

[#591]: https://github.com/twilight-rs/twilight/pull/591
[#590]: https://github.com/twilight-rs/twilight/pull/590
[#582]: https://github.com/twilight-rs/twilight/pull/582
[#576]: https://github.com/twilight-rs/twilight/pull/576
[#557]: https://github.com/twilight-rs/twilight/pull/557
[#555]: https://github.com/twilight-rs/twilight/pull/555
[#553]: https://github.com/twilight-rs/twilight/pull/553
[#540]: https://github.com/twilight-rs/twilight/pull/540
[#528]: https://github.com/twilight-rs/twilight/pull/528

[0.1.5]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.1.5
[0.1.4]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.1.4
[0.1.3]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.1.3
[0.1.2]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.1.2
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
