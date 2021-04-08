# Changelog

Changelog for `twilight-gateway`.

## [0.3.4] - 2021-04-04

### Additions

Support bucket shard schemes for very large bots ([#698] - [@vivian]).

[#698]: https://github.com/twilight-rs/twilight/pull/698

### Fixes

Remove frame size limits ([#730] - [@Erk-]).

[#730]: https://github.com/twilight-rs/twilight/pull/730

## [0.3.3] - 2021-03-14

### Enhancements

Compression is now optional ([#700] - [@Erk-]).

[#700]: https://github.com/twilight-rs/twilight/pull/700

## [0.3.2] - 2021-01-19

### Fixes

Expose the `ClusterSendError` type so it can be named ([#690] - [@vivian]).

[#690]: https://github.com/twilight-rs/twilight/pull/690

## [0.3.1] - 2021-01-11

### Additions

Support sending raw WebSocket messages via `Cluster::send` and `Shard::send`
([#679] - [@vivian]).

[#679]: https://github.com/twilight-rs/twilight/pull/679

## [0.3.0] - 2021-01-08

Version 0.3 has been released with the primary intent to upgrade to Tokio 1.0.

### Upgrade Path

When using `shard::Sink` pass in the new `shard::raw_message::Message` type
instead of `tungstenite::Message`. This is mostly equivalent to `tungstenite`'s
message but prevents exposing it directly, which avoids API breakage when
upgrading internal websocket dependencies.

### Changes

Hide the `tungstenite` dependency from the public API by creating an equivalent
to a websocket message that can be constructed and passed in
([#667] - [@vivian]).

Upgrade `tokio` from v0.2 to v1 ([#664] - [@vivian]).

[#667]: https://github.com/twilight-rs/twilight/pull/667
[#664]: https://github.com/twilight-rs/twilight/pull/664

## [0.2.7] - 2021-01-05

### Enhancements

Shrink the internal inflater buffer every minute instead of shrinking when the
capacity is 4 times the length on periodic checks ([#661] - [@chamburr]).

Upgrade `dashmap` from version 3 to 4.0 ([#666] - [@vivian]).

[#666]: https://github.com/twilight-rs/twilight/pull/666
[#661]: https://github.com/twilight-rs/twilight/pull/661

## [0.2.6] - 2020-12-29

### Fixes

Specify a minimum `twilight-model` dependency version of `^0.2.4` instead of
`^0.2`.

### Enhancements

Use `Box<str>` instead of `String` internally in order to reduce struct size
([#647] - [@vivian]).

Document the `metrics` feature ([#642] - [@vivian]).

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

[#647]: https://github.com/twilight-rs/twilight/pull/647
[#642]: https://github.com/twilight-rs/twilight/pull/642
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

[0.3.4]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.3.4
[0.3.3]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.3.3
[0.3.2]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.3.2
[0.3.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.3.1
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.3.0
[0.2.7]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.2.7
[0.2.6]: https://github.com/twilight-rs/twilight/releases/tag/gateway-v0.2.6
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
