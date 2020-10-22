# Changelog

Changelog for `twilight-cache-inmemory`.

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

[@DusterTheFirst]: https://github.com/DusterTheFirst
[@Erk-]: https://github.com/Erk-

[#555]: https://github.com/twilight-rs/twilight/pull/555
[#553]: https://github.com/twilight-rs/twilight/pull/553
[#540]: https://github.com/twilight-rs/twilight/pull/540
[#528]: https://github.com/twilight-rs/twilight/pull/528

[0.1.3]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.1.3
[0.1.2]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.1.2
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
