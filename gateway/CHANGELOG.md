# Changelog

Changelog for `twilight-gateway`.

## [0.2.5] - 2020-11-29

### Misc.

Use the renamed
`twilight_model::gateway::payload::identify::IdentityInfo::compress` model
field ([#624] - [@chamburr]).

## [0.2.4] - 2020-11-28

### Additions

Add serde `Deserialize` and `Serialize` derives to `shard::ResumeSession`
([#623] - [@tbnritzdoge]).

## [0.2.3] - 2020-11-25

### Additions

Add `Deserialize, Serialize` to the shard information, shard latency, and
connection stage types ([#621] - [@tbnritzdoge]).

### Fixes

Properly use the configured gateway URL in the cluster builder
([#618] - [@chamburr]).

## [0.2.2] - 2020-11-24

### Additions

Add the shard's session ID to the information provided about shards
(`Shard::info`) ([#612] - [@chamburr]).

### Enhancements

Clarify the cloning behavior of the `Cluster` and `Shard` ([#607] - [@vivian]).

## [0.2.1] - 2020-11-02

Update the installation instructions to note version 0.2 instead of version
0.1 ([#588] - [@vivian]).

## [0.2.0] - 2020-10-30

This major version of the crate primarily includes changes needed to support
version 8 of the Discord Gateway API.

### Additions

Add `ShardBuilder::gateway_url` and `ClusterBuilder::gateway_url` to customize
the URL of the gateway to connect to. This may be useful for proxies or
custom gateway implementations ([#568] - [@Erk-]).

### Changes

`twilight_model::gateway::Intents` is now re-exported as
`twilight_gateway::Intents`.

The following methods now take a second "intents" parameter, as this is now
required to be specified by the API:
- `cluster::ClusterBuilder::new`
- `cluster::Cluster::builder`
- `cluster::Cluster::new`
- `shard::ShardBuilder::new`
- `shard::Shard::builder`
- `shard::Shard::new`

The `shard::Config::intents` method no longer returns an option and now returns
a copy of the
intents (returning `twilight_gateway::Intents`) ([#532] - [@vivian]).

### Enhancements

Update `async-tungstenite` from `^0.8` to `^0.9.3`, switching the RusTLS feature
selection from `async-tungstenite/async-tls` to `async-tungstenite/tokio-rustls`
to reduce dependency count ([#548], [#560] - [@nickelc]).

## [0.2.0-beta.1] - 2020-10-23

### Enhancements

Update `async-tungstenite` from ^0.8 to ^0.9.3, switching the RusTLS feature
selection from `async-tungstenite/async-tls` to `async-tungstenite/tokio-rustls`
to reduce dependency count ([#548] - [@nickelc]).

## [0.2.0-beta.0] - 2020-10-10

This beta version of major version 0.2 of the crate includes changes needed to
support version 8 of the Discord Gateway API.

### Changes

`twilight-gateway` now depends on `twilight-http` 0.2 and `twilight-model` 0.2.

`twilight_model::gateway::Intents` is now re-exported as
`twilight_gateway::Intents`.

The following methods now take a second "intents" parameter, as this is now
required to be specified by the API:
- `cluster::ClusterBuilder::new`
- `cluster::Cluster::builder`
- `cluster::Cluster::new`
- `shard::ShardBuilder::new`
- `shard::Shard::builder`
- `shard::Shard::new`

The `shard::Config::intents` method no longer returns an option and now returns
a copy of the intents (returning `twilight_gateway::Intents`).

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
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@nickelc]: https://github.com/nickelc
[@tbnritzdoge]: https://github.com/tbnritzdoge
[@vivian]: https://github.com/vivian

[#624]: https://github.com/twilight-rs/twilight/pull/624
[#623]: https://github.com/twilight-rs/twilight/pull/623
[#621]: https://github.com/twilight-rs/twilight/pull/621
[#618]: https://github.com/twilight-rs/twilight/pull/618
[#612]: https://github.com/twilight-rs/twilight/pull/612
[#607]: https://github.com/twilight-rs/twilight/pull/607
[#588]: https://github.com/twilight-rs/twilight/pull/588
[#568]: https://github.com/twilight-rs/twilight/pull/568
[#560]: https://github.com/twilight-rs/twilight/pull/560
[#548]: https://github.com/twilight-rs/twilight/pull/548
[#537]: https://github.com/twilight-rs/twilight/pull/537
[#532]: https://github.com/twilight-rs/twilight/pull/532
[#521]: https://github.com/twilight-rs/twilight/pull/521
[#515]: https://github.com/twilight-rs/twilight/pull/515
[#512]: https://github.com/twilight-rs/twilight/pull/512

[0.2.5]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.2.5
[0.2.4]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.2.4
[0.2.3]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.2.3
[0.2.2]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.2.2
[0.2.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.2.1
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.2.0
[0.2.0-beta.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.2.0-beta.1
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.2.0-beta.0
[0.1.3]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.1.3
[0.1.2]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.1.2
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
