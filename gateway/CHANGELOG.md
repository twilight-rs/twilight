# Changelog

Changelog for `twilight-gateway`.

## [0.5.2] - 2021-07-02

### Fixes

Tick ratelimiter only on successful message sends via shards and clusters, now
avoiding doing so when there is a failure sending a message
([#965] - [@zeylahellyer]).

### Enhancements

Shards would only emit `ShardDisconnected` events when the remote closed the
connection via a Websocket close code. They are now emitted in additional
circumstances ([#964] - [@zeylahellyer]).

Improve the `Display` implementation performance on the `EmbedError` by calling
`Formatter` methods directly instead of calling the `format_args!` and `write!`
macros ([#944] - [@zeylahellyer]).

[#965]: https://github.com/twilight-rs/twilight/pull/965
[#964]: https://github.com/twilight-rs/twilight/pull/964
[#944]: https://github.com/twilight-rs/twilight/pull/944

## [0.5.1] - 2021-06-24

### Additions

Add the bounds `Send + Sync + Unpin + 'static` to the event stream returned by
`Cluster::new` and `ClusterBuilder::build`
([#939] - [@Erk-], [#959] - [@7596ff]).

[#959]: https://github.com/twilight-rs/twilight/pull/959
[#939]: https://github.com/twilight-rs/twilight/pull/939

## [0.5.0] - 2021-06-13

### Upgrade Path

Replace `zlib` features with their new names.

Create a `Cluster` or `Shard` like this:

```diff
-let cluster = Cluster::new(token, intents).await?;
-let mut events = cluster.events();
+let (cluster, mut events) = Cluster::new(token, intents).await?;
```

Replace references to `UpdateStatus` and `UpdateStatusInfo` with
`UpdatePresence` and `UpdatePresencePayload` respectively.

Ensure at least one `Activity` is present in `UpdatePresence`.

### Changes

The `zlib` feature choices have been renamed from `'stock-zlib` and `simd-zlib`
to `zlib-stock` and `zlib-simd` respectively ([#829] - [@vivian]).

`Cluster::new`, `ClusterBuilder::build`, `Shard::new`, and `ShardBuilder::build`
now return a tuple with two elements: the cluster or shard itself and a stream
of events. See the PR for more details ([#832] - [@vivian]).

`UpdateStatus` and `UpdateStatusInfo` have been renamed to `UpdatePresence` and
`UpdatePresencePayload` respectively ([#902] - [@7596ff]).

At least one `Activity` is required when building an `UpdatePresence` payload.
`UpdatePresenceError` and `UpdatePresenceErrorType` have been created to
validate this ([#891] - [@7596ff]).

[#829]: https://github.com/twilight-rs/twilight/pull/829
[#832]: https://github.com/twilight-rs/twilight/pull/832
[#891]: https://github.com/twilight-rs/twilight/pull/891
[#902]: https://github.com/twilight-rs/twilight/pull/902

## [0.4.2] - 2021-06-12

### Additions

There are 6 new `EventTypeFlags` ([#845], [#914] - [@7596ff]):

- `EventTypeFlags::INTEGRATION_CREATE`
- `EventTypeFlags::INTEGRATION_DELETE`
- `EventTypeFlags::INTEGRATION_UPDATE`
- `EventTypeFlags::STAGE_INSTANCE_CREATE`
- `EventTypeFlags::STAGE_INSTANCE_DELETE`
- `EventTypeFlags::STAGE_INSTANCE_UPDATE`

### Enhancements

The WebSocket connection max message size limit has been removed ([#853] -
[@vivian]).

[#845]: https://github.com/twilight-rs/twilight/pull/845
[#853]: https://github.com/twilight-rs/twilight/pull/853
[#914]: https://github.com/twilight-rs/twilight/pull/914

## [0.4.1] - 2021-05-30

### Enhancements

A peer feature dependency of `tokio`'s `macros` feature has been removed
([#826] - [@vivian]).

`Shard::start` no longer requires mutability ([#827] - [@vivian]).

The following functions are now `const`:

- `cluster::ClusterCommandError::kind`
- `cluster::ClusterSendError::kind`
- `cluster::ClusterStartError::kind`
- `cluster::scheme::ShardSchemeRangeError::kind`
- `cluster::scheme::ShardScheme::from`
- `cluster::scheme::ShardScheme::total`
- `cluster::Config::http_client`
- `cluster::Config::shard_config`
- `cluster::Config::shard_scheme`
- `shard::stage::StageConverserionError::kind`
- `shard::CommandError::kind`
- `shard::Config::http_client`
- `shard::Config::intents`
- `shard::Config::large_threshold`
- `shard::Config::presence`
- `shard::Config::shard`
- `shard::Config::token`
- `shard::Events::event_types`
- `shard::Information::id`
- `shard::Information::latency`
- `shard::Information::seq`
- `shard::Information::stage`
- `shard::LargeThresholdError::kind`
- `shard::Latency::average`
- `shard::Latency::heartbeats`
- `shard::Latency::recent`
- `shard::Latency::received`
- `shard::Latency::sent`
- `shard::SendError::kind`
- `shard::ShardIdError::kind`
- `shard::ShardStartError::kind`

([#824] - [@vivian]).

Add documentation to `shard::ShardBuilder::presence` on using the new
`MinimalActivity` model to easily set a presence ([#851] - [@7596ff]).

[#851]: https://github.com/twilight-rs/twilight/pull/851
[#827]: https://github.com/twilight-rs/twilight/pull/827
[#826]: https://github.com/twilight-rs/twilight/pull/826
[#824]: https://github.com/twilight-rs/twilight/pull/824

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

### Additions

Add `rustls-webpki-roots` feature to use WebPKI roots for `rustls`
([#720] - [@Gelbpunkt]).

### Enhancements

Update `simd-json` to 0.4 ([#786] - [@Gelbpunkt]).

Update `metrics` to v0.14 ([#789] - [@james7132]).

The `futures-channel` and `futures-timer` dependencies have been removed while
the `async-tungstenite` dependency has been switched out for `tokio-tungstenite`
to decrease the dependency tree ([#785] - [@Gelbpunkt]).

[#789]: https://github.com/twilight-rs/twilight/pull/789
[#786]: https://github.com/twilight-rs/twilight/pull/786
[#785]: https://github.com/twilight-rs/twilight/pull/785
[#720]: https://github.com/twilight-rs/twilight/pull/720

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

[@7596ff]: https://github.com/7596ff
[@chamburr]: https://github.com/chamburr
[@dvtkrlbs]: https://github.com/dvtkrlbs
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@james7132]: https://github.com/james7132
[@nickelc]: https://github.com/nickelc
[@tbnritzdoge]: https://github.com/tbnritzdoge
[@vivian]: https://github.com/vivian
[@zeylahellyer]: https://github.com/zeylahellyer

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

[0.5.2]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.5.2
[0.5.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.5.1
[0.5.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.5.0
[0.4.2]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.4.2
[0.4.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.4.1
[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.4.0
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
