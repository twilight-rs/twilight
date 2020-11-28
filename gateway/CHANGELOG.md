# Changelog

Changelog for `twilight-gateway`.

## [0.1.5] - 2020-11-28

While v0.1 will be maintained until the deprecation of version 6 of the Discord
API, we recommend upgrading to v0.2.

### Additions

Add the shard's session ID to the information provided about shards
(`Shard::info`) ([#612] - [@chamburr]).

Add `Deserialize, Serialize` to the shard information, shard latency, and
connection stage types ([#621] - [@tbnritzdoge]).

Add serde `Deserialize` and `Serialize` derives to `shard::ResumeSession`
([#623] - [@tbnritzdoge]).

### Fixes

Properly use the configured gateway URL in the cluster builder
([#618] - [@chamburr]).

### Enhancements

Clarify the cloning behavior of the `Cluster` and `Shard` ([#607] - [@vivian]).

## [0.1.4] - 2020-11-07

This release includes a few bugfixes. While v0.1 will be maintained until the
deprecation of version 6 of the Discord API, we recommend upgrading to v0.2.

### Additions

Add `ShardBuilder::gateway_url` and `ClusterBuilder::gateway_url` to customize
the URL of the gateway to connect to. This may be useful for proxies or
custom gateway implementations ([#568] - [@Erk-]).

## [0.1.3] - 2020-10-07

### Enhancements

- Split the `queue` module into the `twilight-gateway-queue` crate to avoid
pulling in all of the gateway when creating shard queue brokers ([#537] - [@Gelbpunkt])

## [0.1.2] - 2020-09-27

### Added

- Add `Cluster::shards` method to retrieve all shards of a cluster ([#521] - [@dvtkrlbs])

### Fixes

- Fix typos in links ([#515] - [@nickelc])

## [0.1.1] - 2020-09-19

### Enhancements

- Add doubling delay between reconnect attempts ([#512] - [@vivian])

## [0.1.0] - 2020-09-13

Initial release.

[@chamburr]: https://github.com/chamburr
[@dvtkrlbs]: https://github.com/dvtkrlbs
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@Erk-]: https://github.com/Erk-
[@nickelc]: https://github.com/nickelc
[@tbnritzdoge]: https://github.com/tbnritzdoge
[@vivian]: https://github.com/vivian

[#623]: https://github.com/twilight-rs/twilight/pull/623
[#621]: https://github.com/twilight-rs/twilight/pull/621
[#618]: https://github.com/twilight-rs/twilight/pull/618
[#612]: https://github.com/twilight-rs/twilight/pull/612
[#607]: https://github.com/twilight-rs/twilight/pull/607
[#568]: https://github.com/twilight-rs/twilight/pull/568
[#537]: https://github.com/twilight-rs/twilight/pull/537
[#521]: https://github.com/twilight-rs/twilight/pull/521
[#515]: https://github.com/twilight-rs/twilight/pull/515
[#512]: https://github.com/twilight-rs/twilight/pull/512

[0.1.4]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.1.4
[0.1.3]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.1.3
[0.1.2]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.1.2
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
