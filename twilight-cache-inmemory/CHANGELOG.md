# Changelog

## [unreleased]

MSRV has been bumped to 1.64 ([#1897] - [@vilgotf]).

`cache-inmemory` is affected by the following `model` changes:

- [**breaking**] cleanup and document `voice` ([#1820](https://github.com/twilight-rs/twilight/issues/1820))
- [**breaking**] move related modules under `message` ([#1831](https://github.com/twilight-rs/twilight/issues/1831))
- [**breaking**] update `ChannelType` names ([#1909](https://github.com/twilight-rs/twilight/issues/1909))

### Internal Refactors

- clippy 1.65 lints round 2 ([#1991](https://github.com/twilight-rs/twilight/issues/1991))

[#1897]: https://github.com/twilight-rs/twilight/issues/1897

## [0.13.1] - 2022-10-28

### Features

- forum channels ([#1864](https://github.com/twilight-rs/twilight/issues/1864))

## [0.13.0] - 2022-08-14

### Bug Fixes

- ResourceType::GUILD required for other resources ([#1616](https://github.com/twilight-rs/twilight/issues/1616))

## [0.12.1] - 2022-07-26

### Documentation

- format doc examples ([#1847](https://github.com/twilight-rs/twilight/issues/1847))

## [0.12.0] - 2022-07-17

### Features

- [**breaking**] expose channel_messages as vecdeque ([#1770](https://github.com/twilight-rs/twilight/issues/1770))
- [**breaking**] add `GuildFeature` ([#1803](https://github.com/twilight-rs/twilight/issues/1803))
- auto moderation models ([#1796](https://github.com/twilight-rs/twilight/issues/1796))

### Refactor

- [**breaking**] make interaction a struct ([#1813](https://github.com/twilight-rs/twilight/issues/1813))

## [0.11.1] - 2022-07-07

### Bug Fixes

- compare reaction emojis by resource instead of object ([#1812](https://github.com/twilight-rs/twilight/issues/1812))

### Features

- account for `communication_disabled_until` field ([#1669](https://github.com/twilight-rs/twilight/issues/1669))
- add command data guild_id field ([#1755](https://github.com/twilight-rs/twilight/issues/1755))
- add `app_permissions` field on interactions ([#1805](https://github.com/twilight-rs/twilight/issues/1805))

### Refactor

- `Reference::new` ignore `clippy::missing_const_for_fn` ([#1759](https://github.com/twilight-rs/twilight/issues/1759))
- remove `test_` prexif from tests ([#1767](https://github.com/twilight-rs/twilight/issues/1767))
- standardize clippy lints ([#1777](https://github.com/twilight-rs/twilight/issues/1777))
- cleanup test imports ([#1778](https://github.com/twilight-rs/twilight/issues/1778))

Changelog for `twilight-cache-inmemory`.

## [0.11.0] - 2022-05-15

MSRV has been bumped to 1.60.

This major version bump of the Cache crate is done to match all
of the other crates in the ecosystem receiving a major version bump.
There are no changes.

## [0.10.4] - 2022-05-15

Pin `dashmap` to 5.3 to receive library security fixes. This bumps the crate's
MSRV to 1.59.

## [0.10.3] - 2022-05-15

### Additions

Fix some typos in the stats interface documentation ([#1680] - [@zeylahellyer]).

Document the stats interface in the crate documentation
([#1679] - [@zeylahellyer]).

Document message content intent caveats on fields
([#1677] - [@itohatweb], [@zeylahellyer]).

### Fixes

Fix off-by-one on configuration for message cache size ([#1703] - [@vilgotf]).

[#1703]: https://github.com/twilight-rs/twilight/pull/1703
[#1680]: https://github.com/twilight-rs/twilight/pull/1680
[#1679]: https://github.com/twilight-rs/twilight/pull/1679
[#1677]: https://github.com/twilight-rs/twilight/pull/1677

## [0.10.2] - 2022-04-15

### Changes

Add a note that enabling `ResourceType::GUILD` is required for certain
permission operations ([#1607] - [@laralove143]).

### Fixes

Fix a permission calculator issue by properly retrieving the parent channel of a
thread ([#1645] - [@7596ff]).

[#1607]: https://github.com/twilight-rs/twilight/pull/1607
[#1645]: https://github.com/twilight-rs/twilight/pull/1645

## [0.10.1] - 2022-03-20

### Additions

Cached field initializers are now abstracted into a function, which exhaustively
destructures the given model, ensuring it remains up to date ([#1583] -
[@zeylahellyer]]). As a result, the following missing cached fields have been
added:
- `CachedGuild::max_video_channel_users`
- `CachedMessage::application_id`
- `CachedMessage::components`
- `CachedMessage::interaction`
- `CachedMessage::thread_id`

### Changes

Improve performance when caching stickers ([#1580] - [@zeylahellyer]).

[#1580]: https://github.com/twilight-rs/twilight/pull/1580
[#1583]: https://github.com/twilight-rs/twilight/pull/1583

## [0.10.0] - 2022-03-10

### Changes

Update cache accessor methods and internal logic to support the new `Channel`
type ([#1449] - [@zeylahellyer]). `InMemoryCache::{group, guild_channel,
private_channel}` have been replaced with `channel`,
`InMemoryCacheIter::{groups, guild_channels, private_channels}` have been
replaced with `channels`, and `InMemoryCacheStats::{groups, private_channels}`
have been replaced with `channels`.

Use a previously unused `CachedVoiceState` type ([#1491] - [@zeylahellyer]), add
the `self_video` and `request_to_speak_timestamp` to it ([#1492] -
[@zeylahellyer]), and make `channel_id` and `guild_id` non-optional ([#1503] -
[@vilgotf]).

[#1449]: https://github.com/twilight-rs/twilight/pull/1449
[#1491]: https://github.com/twilight-rs/twilight/pull/1491
[#1492]: https://github.com/twilight-rs/twilight/pull/1492
[#1503]: https://github.com/twilight-rs/twilight/pull/1503

## [0.9.1] - 2022-02-12

### Changes

Update `dashmap` to `5.1`, which fixes unsoundness present in `5.0` (which
previously forced a downgrade to `4.0`) ([#1517] - [@Gelbpunkt]).

### Fixes

Update `member_count` on `MEMBER_ADD`/`MEMBER_REMOVE` events ([#1461] -
[@Gelbpunkt]).

Properly remove unavailable guilds from the cache ([#1506] - [@Gelbpunkt]).

[#1461]: https://github.com/twilight-rs/twilight/pull/1461
[#1506]: https://github.com/twilight-rs/twilight/pull/1506
[#1517]: https://github.com/twilight-rs/twilight/pull/1517

## [0.9.0] - 2022-01-22

### Changes

All types and method signatures have been updated to use the new `Id<T>` syntax
([#1260] - [@zeylahellyer]).

The MSRV has been updated to 1.57 ([#1402] - [@zeylahellyer]).

Methods that return an image hash, such as `CachedGuild::banner`, now return an
`ImageHash` instead of a string ([#1405] - [@zeylahellyer]).

The Rust edition has been updated to 2021 ([#1412] - [@vilgotf]).

`UpdateCache` trait is now sealed ([#1431] - [@vilgotf]).

[#1260]: https://github.com/twilight-rs/twilight/pull/1260
[#1402]: https://github.com/twilight-rs/twilight/pull/1402
[#1405]: https://github.com/twilight-rs/twilight/pull/1405
[#1412]: https://github.com/twilight-rs/twilight/pull/1412
[#1431]: https://github.com/twilight-rs/twilight/pull/1431

## [0.8.6] - 2022-01-21

### Changes

Support `Interaction::{guild_locale, locale}` ([#1437] - [@itohatweb]).

Support `ThreadMetadata::create_timestamp` ([#1478] - [@itohatweb]).

### Fixes

Presences are now properly stored in `guild_presences` ([#1459] - [@itohatweb]).

[#1459]: https://github.com/twilight-rs/twilight/pull/1459
[#1478]: https://github.com/twilight-rs/twilight/pull/1478

## [0.8.5] - 2022-01-11

### Fixes

Downgrade `dashmap` to `4.0`, to prevent an issue with `Ref::value` and `dashmap
5.0` ([#1434] - [@baptiste0928]).

[#1434]: https://github.com/twilight-rs/twilight/pull/1434

## [0.8.4] - 2022-01-08

### Additions

Support `Guild::premium_progress_bar_enabled` ([#1399] - [@Erk-]).

Add an iterator over the guild's voice states ([#1410] - [@Gelbpunkt]).

Support `Member::communication_disabled_until` ([#1414] - [@AEnterprise]).

[#1399]: https://github.com/twilight-rs/twilight/pull/1399
[#1410]: https://github.com/twilight-rs/twilight/pull/1410
[#1414]: https://github.com/twilight-rs/twilight/pull/1414

## [0.8.3] - 2021-12-27

### Additions

Support guild member timeouts ([#1342] - [@HTG-YT]).

Support iterating over a channel's list of cached messages via
`InMemoryCache::channel_messages` ([#1362] - [@zeylahellyer]).

[#1362]: https://github.com/twilight-rs/twilight/pull/1362
[#1342]: https://github.com/twilight-rs/twilight/pull/1342

## [0.8.2] - 2021-12-24

### Changes

Upgrade `dashmap` to 5.0 ([#1336] - [@vilgotf]). `dashmap` 4.0 is still allowed.

### Fixes

Correctly return a `CachedMember`'s avatar instead of nickname ([#1341] -
[@Ratismal]).

Fix documentation on `InMemoryCache` that implied it could be `clone`d ([#1349]
- [@zeylahellyer]).

[#1336]: https://github.com/twilight-rs/twilight/pull/1336
[#1341]: https://github.com/twilight-rs/twilight/pull/1341
[#1349]: https://github.com/twilight-rs/twilight/pull/1336

## [0.8.1] - 2021-12-15

### Fixes

The documentation for `CachedEmoji::available` and `animated` have been
corrected. ([#1329] - [@Purpzie]).

[#1329]: https://github.com/twilight-rs/twilight/pull/1329

## [0.8.0] - 2021-12-03

### Changes

`CachedMember::joined_at` no longer returns an `Option` ([#1278] -
[@vilgotf]).

The `PartialEq` implementation for CachedMember has been normalized to
use owned variants ([#1279] - [@vilgotf]).

[#1278]: https://github.com/twilight-rs/twilight/pull/1278
[#1279]: https://github.com/twilight-rs/twilight/pull/1279

## [0.7.2] - 2021-11-20

### Additions

Add `CachedMember::avatar` ([#1252] - [@7596ff]).

[#1252]: https://github.com/twilight-rs/twilight/pull/1252

## [0.7.1] - 2021-10-29

### Additions

Now supports role icons ([#1212] - [@7596ff]).

### Changes

Since sticker descriptions can be null, and to prevent a breaking change
from that, this state is represented as an empty `String` ([#1200] -
[@7596ff]). The `PartialEq` implementation has also been updated to
equate `CachedSticker`s with `description: ""` to `Stickers` with
`description: None`.

[#1200]: https://github.com/twilight-rs/twilight/pull/1200
[#1212]: https://github.com/twilight-rs/twilight/pull/1212

## [0.7.0] - 2021-10-21

### Additions

Add an interface to create iterators over various resource types stored
by the cache ([#1154] - [@zeylahellyer]). To access these iterators,
call `InMemoryCache::iter`. Emojis, groups, guilds, guild channels,
integrations, members, messages, presences, private channels, roles,
stage instances, stickers, users, and voice states may be iterated over.

### Changes

`Cached` variants of models now have accessor methods for held data
([#1064] - [@zeylahellyer]). The fields themselves have been made
private. This will make it easier to add fields to cached models without
causing a breaking change.

`InMemoryCache` no longer implements `Clone`, because it is no longer
internally wrapped in an `Arc` ([#1067] - [@zeylahellyer]). To retain
this functionality, you can wrap it in an `Arc` or a `Rc` manually.

`InMemoryCacheBuilder` no longer implements `Clone`, `Eq`, or
`PartialEq` ([#1147] - [@vilgotf]).

Accessor methods no longer return owned copies of cached data ([#1153] -
[@zeylahellyer]). They now return a `Reference<'_, K, V>`, which is a
wrapping type over a DashMap `Ref`. `Reference` has the methods `key`
and `value`. Cached items that are associated with a guild, such as an
emoji, are returned with a type such as `Reference<'_, EmojiId,
GuildResource<CachedEmoji>>`. `GuildResource<T>` has the methods
`guild_id` and `resource`. In the case of guild members, however, the
return type is `Reference<'_, (GuildId, UserId), CachedMember>`. All
accessor methods still return an `Option`.

Care must be taken to ensure the cache does not become blocked. Cache
references should be held for as short as possible. If the cache needs
to mutate the underlying item, it may block until it can lock the item.
See the PR and the documentation on `InMemoryCache` for more details.

The MSRV has been updated to 1.53 ([#1161] - [@7596ff]).

[#1067]: https://github.com/twilight-rs/twilight/pull/1067
[#1147]: https://github.com/twilight-rs/twilight/pull/1147
[#1153]: https://github.com/twilight-rs/twilight/pull/1153
[#1154]: https://github.com/twilight-rs/twilight/pull/1154
[#1161]: https://github.com/twilight-rs/twilight/pull/1161

## [0.6.4] - 2021-10-07

### Additions

Adds `ResourceType::STICKER` ([#1157] - [@7596ff]).

Adds caching for stickers in the `GuildCreate` and `GuildStickersUpdate`
events ([#1157] - [@7596ff]).

Adds `InMemoryCache::guild_stickers` and `InMemoryCache::sticker`
([#1157] - [@7596ff]).

Adds `CachedSticker` that replaces the inner `User` with a `UserId`
([#1157] - [@7596ff]).

[#1157]: https://github.com/twilight-rs/twilight/pull/1157

## [0.6.3] - 2021-09-17

### Thread Support

The cache now processes the `ThreadCreate`, `ThreadDelete`,
`ThreadListSync`, and `ThreadUpdate` events. Thread channels are stored
in the same place as guild channels, and can be accessed through the
`guild_channel` and `guild_channels` methods.

## [0.6.2] - 2021-09-17

### Changes

This release contains internal refactors, there are no public facing
changes.

## [0.6.1] - 2021-08-30

### Additions

Add `InMemoryCache::guild_integrations` to retrieve a guild's list of
integration IDs and `InMemoryCache::integration` to retrieve an integration by
guild and integration ID ([#1134] - [@zeylahellyer]).

[#1134]: https://github.com/twilight-rs/twilight/pull/1134

## [0.6.0] - 2021-07-31

### Changes

Remove the deprecated method `InMemoryCache::user_ref`, as it was
mistakenly left in during testing ([@7596ff]).

Remove the deprecated field `CachedMember::stickers` as it is no longer
in use by Discord ([@7596ff]).

A few spelling errors have been fixed by adding the `codespell` Action
([#1041] - [@Gelbpunkt].

[#1041]: https://github.com/twilight-rs/twilight/pull/1041

## [0.5.3] - 2021-07-23

### Changes

`#![deny(unsafe_code)]` has been added, ensuring no unsafe code exists in the
crate ([#1042] - [@zeylahellyer]).

[#1042]: https://github.com/twilight-rs/twilight/pull/1042

## [0.5.2] - 2021-07-14

### Additions

Add an integration for the in-memory cache that uses the `util` crate's
permission calculator. This feature is gated behind the optional, non-default
`permission-calculator` feature. See the PR and the docs for more details
([#874] - [@zeylahellyer]).

### Changes

The `stickers` field on `Message`s has been deprecated in favor of
`sticker_items` ([#1029] - [@7596ff]).

[#874]: https://github.com/twilight-rs/twilight/pull/874
[#1029]: https://github.com/twilight-rs/twilight/pull/1029

## [0.5.1] - 2021-07-02

### Fixes

Properly add new roles to guilds' roles relation; roles would be added to the
cache and could be retrieved but would not be in a guild's list of roles
([#952] - [@7596ff]).

### Enhancements

Document all of the remaining undocumented API ([#989] - [@7596ff]).

Remove the dependency on `tracing` and remove the `rc` feature on `serde`
([#960] - [@zeylahellyer]).

Refactor internals to be less unruly, which will improve maintainability
([#938], [#953] - [@7596ff]).

### Changes

`InMemoryCache::user_ref` has been deprecated and hidden; this should never have
been released with 0.5.0 ([#953] - [@7596ff]).

[#960]: https://github.com/twilight-rs/twilight/pull/960
[#953]: https://github.com/twilight-rs/twilight/pull/953
[#952]: https://github.com/twilight-rs/twilight/pull/952
[#938]: https://github.com/twilight-rs/twilight/pull/938

## [0.5.0] - 2021-06-13

### Upgrade Path

`CachedEmoji`, `CachedMember`, and `CachedPresence` have had their `user` fields
replaced with `user_id`. In order to access `user` data, make a separate call to
`InMemoryCache::user`.

Remove references to `CachedGuild::{nsfw, region}`.

Update usage of `CachedMember::{deaf, mute}`.

Cache methods now return clones of the cached data. When accessing data, update
any logic that would require the return type to be an `Arc<T>`.

### Changes

`CachedEmoji::user` has been renamed to `user_id`, and its type has changed from
`Option<Arc<User>>` to `Option<UserId>` ([#871] - [@vivian]).

`CachedMember::user` has been renamed to `user_id`, and its type has changed
from `Option<Arc<User>>` to `Option<UserId>` ([#871] - [@vivian]).

`Presence` has been replaced with `CachedPresence`. The new model contains a
`user_id` field in lieu of `Presence::user` ([#872] - [@vivian]).

`CachedGuild::nsfw` has been removed ([#890] - [@7596ff]).

When caching `MessageCreate` events, each `ResourceType` is checked and cached
individually ([#921] - [@vilgotf]).

The following methods no longer return an `Arc` of cached data, and instead
return a clone ([#900] - [@vivian]):
- `current_user`
- `emoji`
- `group`
- `guild_channel`
- `guild`
- `member`
- `message`
- `presence`
- `private_channel`
- `role`
- `stage_instance`
- `user`
- `voice_channel_states`
- `voice_state`

References to `Guild::region` have been removed. This includes
`CachedGuild::region` ([#930] - [@7596ff]).

`CachedMember::{deaf, mute}` have been made `Option`s ([#932]).

[#871]: https://github.com/twilight-rs/twilight/pull/871
[#872]: https://github.com/twilight-rs/twilight/pull/872
[#890]: https://github.com/twilight-rs/twilight/pull/890
[#900]: https://github.com/twilight-rs/twilight/pull/900
[#921]: https://github.com/twilight-rs/twilight/pull/921
[#930]: https://github.com/twilight-rs/twilight/pull/930
[#932]: https://github.com/twilight-rs/twilight/pull/932

## [0.4.3] - 2021-06-12

### Additions

Support stage instances ([#845] - [@7596ff]).

Support `Guild::nsfw_level`.

Support integration events ([#914] - [@7596ff]).

### Changes

The `CachedGuild::nsfw` field has been deprecated, as Discord no longer supports
it. Read `CachedGuild::nsfw_level` instead ([#848] - [@tbnritzdoge]).

The `CachedGuild::region` field has been deprecated, as Discord no longer
supports it. There is no direct alternative ([#887] - [@BlackHoleFox]).

[#845]: https://github.com/twilight-rs/twilight/pull/845
[#848]: https://github.com/twilight-rs/twilight/pull/848
[#887]: https://github.com/twilight-rs/twilight/pull/887
[#914]: https://github.com/twilight-rs/twilight/pull/914

## [0.4.2] - 2021-05-30

### Enhancements

The following functions are now `const`:

- `Config::new`
- `Config::message_cache_size`
- `Config::resource_types`
- `InMemoryCacheBuilder::new`
- `InMemoryCacheBuilder::message_cache_size`
- `InMemoryCacheBuilder::resource_types`
- `InMemoryCache::builder`

([#824] - [@vivian]).

[#824]: https://github.com/twilight-rs/twilight/pull/824

## [0.4.1] - 2021-05-20

### Additions

Add API for accessing statistics about the cache, accessible through
`InMemoryCache::stats` ([#806] - [@vivian]).

[#806]: https://github.com/twilight-rs/twilight/pull/806

## [0.4.0] - 2021-05-12

### Upgrade Path

The MSRV is now Rust 1.49.

## [0.3.6] - 2021-04-27

### Additions

Support the `MemberUpdate::{deaf, mute}` fields ([#774] - [@7596ff]).

Support guild stage channels ([#793] - [@james7132]).

[#793]: https://github.com/twilight-rs/twilight/pull/793
[#774]: https://github.com/twilight-rs/twilight/pull/774

## [0.3.5] - 2021-04-12

### Enhancements

Use a `VecDeque` instead of a `BTreeMap` to store messages, resulting in faster
performance ([#749] - [@MaxOhn]).

[#749]: https://github.com/twilight-rs/twilight/pull/749

## [0.3.4] - 2021-03-14

This release fixes an accidental double bump of the previous number, and aligns everything.

## [0.3.2] - 2021-03-14

### Fixes

Removed emojis are now deleted ([#702] - [@BlackHoleFox]).

[#702]: https://github.com/twilight-rs/twilight/pull/702

## [0.3.1] - 2021-01-19

### Additions

Support the member pending feature ([#654] - [@AsianIntel]).

[#654]: https://github.com/twilight-rs/twilight/pull/654

## [0.3.0] - 2021-01-08

### Upgrade Path

Instead of specifying individual events to process via `config::EventType`,
specify individual resources to process. For example, previously enabling the
`EventType::MESSAGE_CREATE` and `EventType::MESSAGE_DELETE` event types were
intended to cache the messages, members, and users within these message events.
Now `ResourceType::MESSAGE` can be specified to cache the messages from all
message events, but not the users and members. This avoids an inconsistent cache
and not enabling all of a grouping of an event type was typically an error.

### Changes

Replace `config::EventType` with a simpler and less error prone
`config::ResourceType` ([#660] - [@vivian]).

[#660]: https://github.com/twilight-rs/twilight/pull/660

## [0.2.6] - 2021-01-05

### Fixes

Update cached message's reactions when a Reaction Remove Emoji event is
processed ([#652] - [@sam-kirby]).

### Enhancements

Upgrade `dashmap` from version 3 to 4.0 ([#666] - [@vivian]).

[#666]: https://github.com/twilight-rs/twilight/pull/666
[#652]: https://github.com/twilight-rs/twilight/pull/652

## [0.2.5] - 2020-12-30

### Additions

Cache members from voice state updates ([#651] - [@sam-kirby]).

[#651]: https://github.com/twilight-rs/twilight/pull/651

## [0.2.4] - 2020-12-18

The MSRV is now set to Rust 1.48.

### Fixes

Update `InMemoryCache::clear` to actually clear all fields, as it was only
clearing some ([#639] - [@vivian]).

### Misc.

Replace documentation links with intra-doc links ([#524] - [@nickelc]).

## [0.2.3] - 2020-11-29

### Misc.

Use the renamed
`twilight_model::gateway::payload::identify::IdentityInfo::compress` model
field ([#624] - [@chamburr]).

## [0.2.2] - 2020-11-20

### Enhancements

Don't create useless HashSets in internal functions ([#591] - [@MaxOhn]).

## [0.2.1] - 2020-11-11

## Additions

Cache members and users received from new messages ([#590] - [@MaxOhn]).

## [0.2.0] - 2020-10-30

### Fixes

Correctly maintain relation sets for guilds. For example, the set of
the IDs of roles within a guild was not being inserted into with new
role IDs ([#540] - [@DusterTheFirst]).

Fix voice state map retaining user states, causing false duplicate voice states
for a single user ([#555] - [@DusterTheFirst]).

Properly track guilds' lists of their emojis ([#557] - [@DusterTheFirst]).

### Changes

The following model fields have been removed ([#532] - [@vivian]):
- `model::CachedGuild::{embed_channel_id, embed_enabled}`
- `model::CachedPresence::nick`

### Enhancements

Remove old and removed method of cache building in
documentation ([#576] - [@nickelc]).

## [0.2.0-beta.2] - 2020-10-22

### Fixes

Fix voice state map retaining user states, causing false duplicate voice states
for a single user ([#555] - [@DusterTheFirst]).

## [0.2.0-beta.1] - 2020-10-17

### Fixes

Correctly maintain relation sets for guilds. For example, the set of
the IDs of roles within a guild was not being inserted into with new
role IDs ([#540] - [@DusterTheFirst])

Correctly update guilds in `GuildUpdate` events
([#553] - [@DusterTheFirst])

## [0.2.0-beta.0] - 2020-10-10

This beta version of major version 0.2 of the crate includes changes needed to
support `twilight-model` 0.2.

### Changes

All changes in this version are from PR [#532].

The following model fields have been removed:
- `model::CachedGuild::{embed_channel_id, embed_enabled}`
- `model::CachedPresence::nick`

The crate now depends on version 0.2 of `twilight-model`.

## [0.1.1] - 2020-10-05

### Added

- Add methods to fetch guild relation ID sets ([#528] - [@Erk-])

## [0.1.0] - 2020-09-13

Initial release.

[@7596ff]: https://github.com/7596ff
[@AEnterprise]: https://github.com/AEnterprise
[@AsianIntel]: https://github.com/AsianIntel
[@baptiste0928]: https://github.com/baptiste0928
[@BlackHoleFox]: https://github.com/BlackHoleFox
[@chamburr]: https://github.com/chamburr
[@DusterTheFirst]: https://github.com/DusterTheFirst
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@HTG-YT]: https://github.com/HTG-YT
[@itohatweb]: https://github.com/itohatweb
[@james7132]: https://github.com/james7132
[@laralove143]: https://github.com/laralove143
[@MaxOhn]: https://github.com/MaxOhn
[@nickelc]: https://github.com/nickelc
[@Purpzie]: https://github.com/Purpzie
[@Ratismal]: https://github.com/Ratismal
[@sam-kirby]: https://github.com/sam-kirby
[@tbnritzdoge]: https://github.com/tbnritzdoge
[@vilgotf]: https://github.com/vilgotf
[@vivian]: https://github.com/vivian
[@zeylahellyer]: https://github.com/zeylahellyer

[#639]: https://github.com/twilight-rs/twilight/pull/639
[#624]: https://github.com/twilight-rs/twilight/pull/624
[#591]: https://github.com/twilight-rs/twilight/pull/591
[#590]: https://github.com/twilight-rs/twilight/pull/590
[#576]: https://github.com/twilight-rs/twilight/pull/576
[#557]: https://github.com/twilight-rs/twilight/pull/557
[#555]: https://github.com/twilight-rs/twilight/pull/555
[#553]: https://github.com/twilight-rs/twilight/pull/553
[#540]: https://github.com/twilight-rs/twilight/pull/540
[#532]: https://github.com/twilight-rs/twilight/pull/532
[#528]: https://github.com/twilight-rs/twilight/pull/528
[#524]: https://github.com/twilight-rs/twilight/pull/524

[0.11.0]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.11.0
[0.10.4]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.10.4
[0.10.3]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.10.3
[0.10.2]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.10.2
[0.10.1]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.10.1
[0.10.0]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.10.0
[0.9.1]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.9.1
[0.9.0]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.9.0
[0.8.6]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.8.6
[0.8.5]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.8.5
[0.8.4]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.8.4
[0.8.3]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.8.3
[0.8.2]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.8.2
[0.8.1]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.8.1
[0.8.0]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.8.0
[0.7.2]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.7.2
[0.7.1]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.7.1
[0.7.0]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.7.0
[0.6.4]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.6.4
[0.6.3]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.6.3
[0.6.2]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.6.2
[0.6.1]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.6.1
[0.5.3]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.5.3
[0.5.2]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.5.2
[0.5.1]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.5.1
[0.5.0]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.5.0
[0.4.3]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.4.3
[0.4.2]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.4.2
[0.4.1]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.4.1
[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-0.4.0
[0.3.6]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.3.6
[0.3.5]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.3.5
[0.3.4]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.3.4
[0.3.2]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.3.2
[0.3.1]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.3.1
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.3.0
[0.2.6]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.2.6
[0.2.5]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.2.5
[0.2.4]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.2.4
[0.2.3]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.2.3
[0.2.2]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.2.2
[0.2.1]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.2.1
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.2.0
[0.2.0-beta.2]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.2.0-beta.2
[0.2.0-beta.1]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.2.0-beta.1
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.2.0-beta.0
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/cache-in-memory-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
