# Changelog

## [unreleased]

### Bug Fixes

- Address new rust CI errors ([#2285](https://github.com/twilight-rs/twilight/issues/2285))

### Build

- Update to rustls 0.22, tokio-websockets 0.5 ([#2302](https://github.com/twilight-rs/twilight/issues/2302))
- [**breaking**] Rename native feature to native-tls ([#2308](https://github.com/twilight-rs/twilight/issues/2308))

### Features

- [**breaking**] Reduce size of gateway rest models ([#2168](https://github.com/twilight-rs/twilight/issues/2168))
- [**breaking**] Turn `Queue` into a generic type ([#2249](https://github.com/twilight-rs/twilight/issues/2249))
- [**breaking**] Impl Stream for Shard ([#2309](https://github.com/twilight-rs/twilight/issues/2309))

### Refactor

- [**breaking**] Remove deprecated apis ([#2132](https://github.com/twilight-rs/twilight/issues/2132))
- [**breaking**] Update bitflags crate to v2 ([#2199](https://github.com/twilight-rs/twilight/issues/2199))
- [**breaking**] Remove undocumented gateway events ([#2237](https://github.com/twilight-rs/twilight/issues/2237))
- [**breaking**] Changes for compatibility with twilight-gateway-queue rewrite ([#2228](https://github.com/twilight-rs/twilight/issues/2228))
- Switch to fastrand and tokio-websockets ([#2239](https://github.com/twilight-rs/twilight/issues/2239))

## [0.15.3] - 2023-09-10

### Bug Fixes

- don't swallow events on send error ([#2247](https://github.com/twilight-rs/twilight/issues/2247))
- defer queued commands until sent `Resume` ([#2264](https://github.com/twilight-rs/twilight/issues/2264))
- resume pending identify requests ([#2260](https://github.com/twilight-rs/twilight/issues/2260))

### Build

- bump MSRV to 1.67 ([#2208](https://github.com/twilight-rs/twilight/issues/2208))

### Features

- impl `From<Config>` for ConfigBuilder ([#2248](https://github.com/twilight-rs/twilight/issues/2248))

### Performance

- reuse ratelimiter's cleanup instant ([#2212](https://github.com/twilight-rs/twilight/issues/2212))

### Refactor

- reformat code with rustfmt 1.6.0 ([#2233](https://github.com/twilight-rs/twilight/issues/2233))
- remove ratelimiter sleep from new ([#2232](https://github.com/twilight-rs/twilight/issues/2232))
- inline `future` into `shard` ([#2226](https://github.com/twilight-rs/twilight/issues/2226))
- replace `StreamExt::next` with `Stream::poll_next` ([#2251](https://github.com/twilight-rs/twilight/issues/2251))
- cleanup dispatch sequence handling ([#2225](https://github.com/twilight-rs/twilight/issues/2225))

### Testing

- assert spurious ratelimiter poll doesn't reset deadline ([#2185](https://github.com/twilight-rs/twilight/issues/2185))

## [0.15.2] - 2023-04-27

### Build

- allow simd-json 0.8 and 0.9 ([#2202](https://github.com/twilight-rs/twilight/issues/2202))

### Performance

- ratelimiter algorithm improvements ([#2143](https://github.com/twilight-rs/twilight/issues/2143))
- speed up spurious ratelimiter poll ([#2186](https://github.com/twilight-rs/twilight/issues/2186))

## [0.15.1] - 2023-02-26

### Bug Fixes

- ignore unknown event errors ([#2154](https://github.com/twilight-rs/twilight/issues/2154))

### Documentation

- remove stale resume_gateway_url notice ([#2126](https://github.com/twilight-rs/twilight/issues/2126))
- resume behavior of presences ([#2150](https://github.com/twilight-rs/twilight/issues/2150))
- all event types can be excluded ([#2149](https://github.com/twilight-rs/twilight/issues/2149))
- `next_event` close variant ([#2140](https://github.com/twilight-rs/twilight/issues/2140))

### Refactor

- change deny lints to warn ([#2144](https://github.com/twilight-rs/twilight/issues/2144))

## [0.15.0] - 2023-02-05

Check out [the migration guide] for a summary of Gateway changes.

[the migration guide]: https://twilight.rs/versions/0.15

## [0.14.2] - 2023-02-05

### Bug Fixes

- backport unexpected eof handling ([#2099](https://github.com/twilight-rs/twilight/issues/2099))

## [0.14.1] - 2023-01-20

### Features

- [**breaking**] support the `GUILD_AUDIT_LOG_ENTRY_CREATE` gateway event ([#2067](https://github.com/twilight-rs/twilight/issues/2067))

## [0.14.0] - 2022-11-14

MSRV has been bumped to 1.64 ([#1897] - [@vilgotf]).

### Performance

[**breaking**] unbox `GatewayEvent::Dispatch` ([#1859]) - [@vilgotf]). Most
gateway events are dispatch events, so this saves one allocation and pointer
redirection.

### Internal Refactors

- clippy 1.65 lints ([#1985](https://github.com/twilight-rs/twilight/issues/1985))

[#1859]: https://github.com/twilight-rs/twilight/issues/1859
[#1897]: https://github.com/twilight-rs/twilight/issues/1897

## [0.13.3] - 2022-09-29

### Build

- fix or ignore clippy for 1.64

### Features

- add gateway url to the shard information ([#1926](https://github.com/twilight-rs/twilight/issues/1926))

## [0.13.2] - 2022-09-11

### Bug Fixes

- reconnect handling of gateway query parameters ([#1901](https://github.com/twilight-rs/twilight/issues/1901))

## [0.13.1] - 2022-09-08 (YANKED)

### Features

- add handling for gateway resume url ([#1894](https://github.com/twilight-rs/twilight/issues/1894))

## [0.12.1] - 2022-07-26

### Refactor

- move bitflag constants into macro ([#1828](https://github.com/twilight-rs/twilight/issues/1828))

## [0.12.0] - 2022-07-17

### Features

- auto moderation models ([#1796](https://github.com/twilight-rs/twilight/issues/1796))

### Refactor

- [**breaking**] use hardcoded gateway url ([#1763](https://github.com/twilight-rs/twilight/issues/1763))
- [**breaking**] don't validate tokens in shard builder ([#1776](https://github.com/twilight-rs/twilight/issues/1776))

## [0.11.1] - 2022-07-07

### Bug Fixes

- add missing received events per intent ([#1818](https://github.com/twilight-rs/twilight/issues/1818))

### Documentation

- use anyhow on public errorable examples ([#1738](https://github.com/twilight-rs/twilight/issues/1738))

### Features

- set track_caller on panicable functions ([#1802](https://github.com/twilight-rs/twilight/issues/1802))

### Refactor

- allow `ptr_arg` lint on conditional ([#1745](https://github.com/twilight-rs/twilight/issues/1745))
- `#[must_use]` on builders, not methods ([#1761](https://github.com/twilight-rs/twilight/issues/1761))
- remove `test_` prexif from tests ([#1767](https://github.com/twilight-rs/twilight/issues/1767))

Changelog for `twilight-gateway`.

## [0.11.0] - 2022-05-15

MSRV has been bumped to 1.60.

### Additions

Support Guild Scheduled Events gateway events ([#1574] - [@itohatweb]).

Support Application Command Permissions V2's `COMMAND_PERMISSIONS_UPDATE`
event ([#1708] - [@baptiste0928]).

### Fixes

Deduplicate `EventTypeFlags`' `GATEWAY_INVALIDATE_SESSION` value from
`GATEWAY_HELLO` ([#1577] - [@7596ff]).

### Changes

Panic instead of returning an error on `ClusterBuilder::large_threshold` and
`ShardBuilder::large_threshold` ([#1599] - [@vilgotf]).

Remove `ShardScheme::Auto` and make it the cluster default ([#1600] - [@vilgotf]).

Remove option around parameter in `ClusterBuilder::gateway_url` and
`ShardBuilder::gateway_url` ([#1601] - [@vilgotf]).

`tracing` is no longer an optional dependency and is always enabled
([#1684], [#1730] - [@vilgotf], [@zeylahellyer]).

[#1730]: https://github.com/twilight-rs/twilight/pull/1730
[#1684]: https://github.com/twilight-rs/twilight/pull/1684
[#1601]: https://github.com/twilight-rs/twilight/pull/1601
[#1600]: https://github.com/twilight-rs/twilight/pull/1600
[#1599]: https://github.com/twilight-rs/twilight/pull/1599
[#1574]: https://github.com/twilight-rs/twilight/pull/1574
[#1577]: https://github.com/twilight-rs/twilight/pull/1577
[#1574]: https://github.com/twilight-rs/twilight/pull/1574

## [0.10.2] - 2022-05-15

Support gateway connections without TLS ([#1483], [#1727] - [@vilgotf]).

Prominently document member chunking with an example
([#1678] - [@zeylahellyer]).

[#1727]: https://github.com/twilight-rs/twilight/pull/1727
[#1678]: https://github.com/twilight-rs/twilight/pull/1678
[#1483]: https://github.com/twilight-rs/twilight/pull/1483

## [0.10.1] - 2022-03-20

### Changes

`tokio-tungstenite` has been updated to `0.17` ([#1551] - [@Gelbpunkt]).

### Fixes

`value-trait`, a dependency of `simd-json`, has been limited to at most `0.2.10`
([#1596] - [@7596ff], [@vilgotf]). The crate updated its MSRV in a minor
version, which Twilight is unable to follow.

[#1551]: https://github.com/twilight-rs/twilight/pull/1551
[#1596]: https://github.com/twilight-rs/twilight/pull/1596

## [0.10.0] - 2022-03-10

### Additions

Add `{ClusterBuilder, ShardBuilder}::ratelimit_payloads`, which if set to false,
enables the sending of un-ratelimited payloads ([#1501] - [@Gelbpunkt]). This is
`true` by default. Additionally, `Information::{ratelimit_refill,
ratelimit_requests}` are now optional.

Add `EventTypeFlags::GUILD_STICKERS_UPDATE`, and rename/update
`GUILD_EMOJIS_AND_STICKERS` ([#1520] - [@HTG-YT]).

### Changes

Remove the `compression` feature, which was not used in any code ([#1497] -
[@vilgotf]).

Update to Discord API version 10 ([#1540] - [@zeylahellyer]).

[#1497]: https://github.com/twilight-rs/twilight/pull/1497
[#1501]: https://github.com/twilight-rs/twilight/pull/1501
[#1520]: https://github.com/twilight-rs/twilight/pull/1520
[#1540]: https://github.com/twilight-rs/twilight/pull/1540

## [0.9.1] - 2022-02-12

### Fixes

The `compression` feature was broken, so "fix" it by making it do nothing at all
([#1482] - [@vilgotf]). Feature gates that previously required `compression` now
require one of `zlib-stock` or `zlib-simd`. See the readme for more details.

[#1482]: https://github.com/twilight-rs/twilight/pull/1482

## [0.9.0] - 2022-01-22

### Additions

Add `Information::{ratelimit_refill, ratelimit_requests}` to view how much time
until the shard ratelimiter resets and how many requests until the next refill
([#1368] - [@itohatweb])

Add `ClusterBuilder::shard_presence`, which allows setting a custom presence
based on a shard's ID ([#1474] - [@7596ff]).

### Changes

The `rustls` feature has been removed ([#1314] - [@Gelbpunkt]). Users must
manually select one of `rustls-native-roots` or `rustls-webpki-roots`.

`{CommandErrorType, SendErrorType}::ExecutorShutDown` have been removed, since
they are no longer possible ([#1368] - [@itohatweb]).

No longer derive `Deserialize` and `Serialize` on `Information` ([#1368] -
[@itohatweb]).

The MSRV has been updated to 1.57 ([#1402] - [@zeylahellyer]).

The Rust edition has been updated to 2021 ([#1412] - [@vilgotf]).

Constructors that used to take `impl Into<String>` now simply accept a `String`
([#1481] - [@vilgotf]).

[#1314]: https://github.com/twilight-rs/twilight/pull/1314
[#1368]: https://github.com/twilight-rs/twilight/pull/1368
[#1402]: https://github.com/twilight-rs/twilight/pull/1402
[#1412]: https://github.com/twilight-rs/twilight/pull/1412
[#1474]: https://github.com/twilight-rs/twilight/pull/1474
[#1481]: https://github.com/twilight-rs/twilight/pull/1481

## [0.8.5] - 2022-01-21

This release contains internal refactors ([#1335] - [@vilgotf]). There are no
public facing changes.

[#1335]: https://github.com/twilight-rs/twilight/pull/1335

## [0.8.4] - 2022-01-11

### Changes

Gateway parsing failures now emit at an `error` level ([#1427] -
[@AEnterprise]).

[#1427]: https://github.com/twilight-rs/twilight/pull/1427

## [0.8.3] - 2022-01-08

### Additions

Support the following close codes: `4010 Invalid Shard`, `4011 Sharding
Required`, `4012 Invalid API Version` ([#1400] - [@Erk-]).

### Fixes

Reserve capacity in the ratelimiter for heartbeat responses ([#1421] -
[@AEnterprise]).

[#1400]: https://github.com/twilight-rs/twilight/pull/1400
[#1421]: https://github.com/twilight-rs/twilight/pull/1421

## [0.8.2] - 2021-12-27

### Fixes

Tick shard ratelimiter before sending command to gateway instead of after
([#1360] - [@zeylahellyer]).

[#1360]: https://github.com/twilight-rs/twilight/pull/1360

## [0.8.1] - 2021-12-24

### Additions

Reuse TLS connectors between shards when connecting a cluster ([#1058] -
[@Erk]). This saves a large amount of memory when using `native-tls`, and a
decent amount when using `rustls`.

### Fixes

Fix documentation on `Cluster` that implied it could be `clone`d ([#1349]
- [@zeylahellyer]).

[#1058]: https://github.com/twilight-rs/twilight/pull/1058
[#1349]: https://github.com/twilight-rs/twilight/pull/1349

## [0.8.0] - 2021-12-03

### Changes

The default value for `ShardBuilder::large_threshold` has been corrected
to `50` ([#1255] - [@7596ff]).

### Dependency Updates

`tokio-tungstenite` has been updated to `0.16` ([#1276] - [@Gelbpunkt]).

[#1255]: https://github.com/twilight-rs/twilight/pull/1255
[#1276]: https://github.com/twilight-rs/twilight/pull/1276

## [0.7.1] - 2021-10-29

### Changes

Fixes some spelling errors in documentation ([#1223] - [@7596ff]).

[#1223]: https://github.com/twilight-rs/twilight/pull/1223

## [0.7.0] - 2021-10-21

### Changes

The gateway ratelimiter has been reworked in multiple PRs ([#1061] -
[@Gelbpunkt], [#1101] and [#1102] - [@zeylahellyer]). It now depends on
`leaky-bucket-lite`'s `LeakyBucket` instead of using the now-removed
internal `Throttle` implementation. There are two new `CommandErrorType`
and `ShardErrorType` variants: `ExecutorShutDown` and
`HeartbeaterNotStarted`.

`Cluster` and `Shard` no longer implement `Clone`, because they are no
longer internally wrapped in an `Arc` ([#1067] - [@zeylahellyer]). To
retain this functionality, you can wrap them in an `Arc` or a `Rc`
manually.  Additionally, the `Cluster::shard` method now returns a
reference, and the `Cluster::shards` method now returns a type
implementing `Iterator<Item = Shard>`.

As part of an internal refactor of the `Cluster`, its methods
`event_types`, `http_client`, and `shard_config` have been removed
([#1073] - [@vilgotf]).  These can instead be retrieved through
individual `Shard`s.

A dependency on `once-cell` has been removed, and replaced with `tokio
^1.5`'s implementation ([#1075] - [@Gelbpunkt]).

`Cluster::command` and `Shard::command` now take a `Command` marker
trait instead of anything that implements `serde::Serialize` ([#1132] -
[@zeylahellyer]).

`ShardBuilder` no longer implements `Clone` ([#1147] - [@vilgotf]).

The MSRV has been updated to 1.53 ([#1161] - [@7596ff]).

`Cluster::command_raw` and `Shard::command_raw` have been removed
([#1193] - [@7596ff]).

[#1061]: https://github.com/twilight-rs/twilight/pull/1061
[#1067]: https://github.com/twilight-rs/twilight/pull/1061
[#1073]: https://github.com/twilight-rs/twilight/pull/1073
[#1101]: https://github.com/twilight-rs/twilight/pull/1101
[#1102]: https://github.com/twilight-rs/twilight/pull/1102
[#1132]: https://github.com/twilight-rs/twilight/pull/1132
[#1147]: https://github.com/twilight-rs/twilight/pull/1147
[#1161]: https://github.com/twilight-rs/twilight/pull/1147
[#1193]: https://github.com/twilight-rs/twilight/pull/1193

## [0.6.5] - 2021-10-07

### Changes

This release contains internal refactors, there are no public facing
changes.

## [0.6.4] - 2021-09-17

### Thread Support

The gateway API version has been updated to `v9`. Six new events have
been added: `THREAD_CREATE`, `THREAD_DELETE`, `THREAD_LIST_SYNC`,
`THREAD_MEMBERS_UPDATE`, `THREAD_MEMBER_UPDATE`, and `THREAD_UPDATE`.

## [0.6.3] - 2021-09-17

### Additions

Support users setting the properties to send during identify via the new
`ClusterBuilder::identify_properties` and
`ShardBuilder::identify_properties` methods. When not set a default will
be used ([#1130] - [@zeylahellyer]).

[#1130]: https://github.com/twilight-rs/twilight/pull/1130

## [0.6.2] - 2021-08-30

### Enhancements

Reduce the log level of shard resumes from INFO to DEBUG ([#1137] - [@vilgotf]).

Fix two remaining intradoc links ([#1128] - [@zeylahellyer]).

[#1137]: https://github.com/twilight-rs/twilight/pull/1137
[#1128]: https://github.com/twilight-rs/twilight/pull/1128

## [0.6.1] - 2021-08-18

### Enhancements

Bump the `metrics` crate from version 0.14 to >= 0.14, < 0.18
([#1097] - [@vilgotf]).

Refactor internal shard storage, improving the performance of retrieving
shards from a cluster ([#1076] - [@vilgotf]).

Remove some unnecessary internal Cluster fields, reducing memory usage
([#1079] - [@vilgotf]).

[#1097]: https://github.com/twilight-rs/twilight/pull/1097
[#1079]: https://github.com/twilight-rs/twilight/pull/1079
[#1076]: https://github.com/twilight-rs/twilight/pull/1076

## [0.6.0] - 2021-07-31

### Enhancements

The `tracing` feature is now optional ([#986] - [@zeylahellyer]).

### Changes

A few spelling errors have been fixed by adding the `codespell` Action
([#1041] - [@Gelbpunkt].

### Fixes

When calling `Shard::shutdown`, `Shard::shutdown_resumable`,
`Cluster::down`, or `Cluster::down_resumable`, shards would be stopped
but the events stream returned by shards and clusters wouldn't return
`None`. This is due to the events stream containing a receiver of
events, while shard processors contained a sender. However, shards would
keep a copy of the sender, so while the shard processor would be aborted
and its sender dropped the shard's would not be dropped.

To fix this we can move the sender into shard processors. When the
shard processor is dropped so will the only sender. However, individual
shard instances will now only be able to be started the first time;
`Shard::start` can no longer be called multiple times. If a user shuts
down a shard and wants to start it again they will need to create a new
shard instance.

([#1070] - [@zeylahellyer]).

[#986]: https://github.com/twilight-rs/twilight/pull/986
[#1041]: https://github.com/twilight-rs/twilight/pull/1041
[#1070]: https://github.com/twilight-rs/twilight/pull/1070

## [0.5.5] - 2021-07-25

This is a hotfix to actually include the changes that were supposed to be in
0.5.5; they were erroneously left out during the release.

### Documentation

Fix a typo in the documentation for `Shard::new` ([#1071] - [@kotx]).

[#1071]: https://github.com/twilight-rs/twilight/pull/1071

## [0.5.4] - 2021-07-23

### Additions

Add `EventTypeFlags` constants with categories of flags that are equivalent to
their Intents counterpart. For example, the new `EventTypeFlags::GUILD_BANS`
associated constant includes the `BAN_ADD` and `BAN_REMOVE` event type flags.

The following categories have been added ([#1049] - [@vilgotf]):

- `DIRECT_MESSAGES`
- `DIRECT_MESSAGE_REACTIONS`
- `DIRECT_MESSAGE_TYPING`
- `GUILDS`
- `GUILD_BANS`
- `GUILD_EMOJIS`
- `GUILD_INTEGRATIONS`
- `GUILD_INVITES`
- `GUILD_MEMBERS`
- `GUILD_MESSAGES`
- `GUILD_MESSAGE_REACTIONS`
- `GUILD_MESSAGE_TYPING`
- `GUILD_PRESENCES`
- `GUILD_VOICE_STATES`
- `GUILD_WEBHOOKS`

### Changes

`#![deny(unsafe_code)]` has been added, ensuring no unsafe code exists in the
crate. To comply with this, while using the `simd-json` feature, the mutable
buffer is directly used instead of casting from bytes -> str -> bytes ([#1042] -
[@zeylahellyer]).

[#1042]: https://github.com/twilight-rs/twilight/pull/1042
[#1049]: https://github.com/twilight-rs/twilight/pull/1049

## [0.5.3] - 2021-07-14

### Changes

The event stream returned by `Cluster::new` and `ClusterBuilder::build` is now a
named concrete type. It still a `Stream<Item = (u64, event)>` that implements
`Send` and `Sync` ([#1021] - [@zeylahellyer]).

[#1021]: https://github.com/twilight-rs/twilight/pull/1021

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
[@AEnterprise]: https://github.com/AEnterprise
[@baptiste0928]: https://github.com/baptiste0928
[@chamburr]: https://github.com/chamburr
[@dvtkrlbs]: https://github.com/dvtkrlbs
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@HTG-YT]: https://github.com/HTG-YT
[@itohatweb]: https://github.com/itohatweb
[@james7132]: https://github.com/james7132
[@kotx]: https://github.com/kotx
[@nickelc]: https://github.com/nickelc
[@tbnritzdoge]: https://github.com/tbnritzdoge
[@vilgotf]: https://github.com/vilgotf
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

[0.11.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.11.0
[0.10.2]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.10.2
[0.10.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.10.1
[0.10.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.10.0
[0.9.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.9.1
[0.9.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.9.0
[0.8.5]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.8.5
[0.8.4]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.8.4
[0.8.3]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.8.3
[0.8.2]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.8.2
[0.8.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.8.1
[0.8.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.8.0
[0.7.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.7.1
[0.7.0]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.7.0
[0.6.3]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.6.3
[0.6.2]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.6.2
[0.6.1]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.6.1
[0.5.5]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.5.5
[0.5.4]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.5.4
[0.5.3]: https://github.com/twilight-rs/twilight/releases/tag/gateway-0.5.3
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
