# Changelog

## [unreleased]

### Features

- add `Member::flags` ([#2086](https://github.com/twilight-rs/twilight/issues/2086))

## [0.14.3] - 2023-01-20

### Bug Fixes

- include missing `RoleTags` struct member in test
- signed channel member counts ([#2079](https://github.com/twilight-rs/twilight/issues/2079))
- zeroable forum tag emoji ids ([#2080](https://github.com/twilight-rs/twilight/issues/2080))

### Documentation

- activity timestamp integer values ([#2075](https://github.com/twilight-rs/twilight/issues/2075))

### Features

- add role subscriptions ([#2034](https://github.com/twilight-rs/twilight/issues/2034))
- add `guild_connections` attribute ([#2063](https://github.com/twilight-rs/twilight/issues/2063))
- get current authorization route ([#2049](https://github.com/twilight-rs/twilight/issues/2049))
- gif sticker format type ([#2064](https://github.com/twilight-rs/twilight/issues/2064))
- guild public updates channel id ([#2065](https://github.com/twilight-rs/twilight/issues/2065))
- [**breaking**] support the `GUILD_AUDIT_LOG_ENTRY_CREATE` gateway event ([#2067](https://github.com/twilight-rs/twilight/issues/2067))

### Refactor

- remove prelude imports

## [0.14.2] - 2023-01-08

### Features

- constants for scope values ([#2018](https://github.com/twilight-rs/twilight/issues/2018))
- add new message types for premium ([#2040](https://github.com/twilight-rs/twilight/issues/2040))

## [0.14.1] - 2023-01-07

### Documentation

- update automod keyword limit ([#2020](https://github.com/twilight-rs/twilight/issues/2020))
- update voice state suppress field ([#2037](https://github.com/twilight-rs/twilight/issues/2037))

### Features

- add `Interaction::author` ([#2001](https://github.com/twilight-rs/twilight/issues/2001))
- make `Id<T>` invariant for T ([#1861](https://github.com/twilight-rs/twilight/issues/1861))
- add active developer user flag ([#2014](https://github.com/twilight-rs/twilight/issues/2014))
- rename certified moderator to alumni ([#2015](https://github.com/twilight-rs/twilight/issues/2015))
- support nsfw commands ([#2019](https://github.com/twilight-rs/twilight/issues/2019))
- forum channel layouts ([#2016](https://github.com/twilight-rs/twilight/issues/2016))
- add nitro basic user premium type ([#2035](https://github.com/twilight-rs/twilight/issues/2035))
- add dev support server guild feature ([#2036](https://github.com/twilight-rs/twilight/issues/2036))
- default forum sort orders ([#2038](https://github.com/twilight-rs/twilight/issues/2038))
- message type deletable check methods ([#2028](https://github.com/twilight-rs/twilight/issues/2028))

### Refactor

- abstract null boolean visitor ([#2032](https://github.com/twilight-rs/twilight/issues/2032))

### Testing

- add real-world role tag tests ([#2033](https://github.com/twilight-rs/twilight/issues/2033))

## [0.14.0] - 2022-11-14

MSRV has been bumped to 1.64 ([#1897] - [@vilgotf]).

### Bug Fixes

- [**breaking**] add a unknown variant to `PremiumType` ([#1996](https://github.com/twilight-rs/twilight/issues/1996))

### Documentation

[**breaking**] Document `application::command::permissions` ([#1816] -
[@vilgotf]). Also renames `CommandPermissions` to `CommandPermission`, and
`CommandPermissionsType` to `CommandPermissionType`.

### Performance

[**breaking**] unbox `GatewayEvent::Dispatch` ([#1859]) - [@vilgotf]). Most
gateway events are dispatch events, so this saves one allocation and pointer
redirection.

### Changes

[**breaking**] flatten `CommandOption` ([#1819] - [@vilgotf]).
- Simplify and fix errors in `CommandOption` {de,}serialization by delegating
  to the derive implementation.
- Remove duplicated shared fields for `CommandOption` and `CommandOptionChoice`
  variants.
- Document all public fields.
- Add a note in the `command` module recommending users to use the
  `CommandBuilder`.
- `CommandBuilder` retains its API, but is internally more complicated.
 
Through the following series of PRs, modules and types have been moved around
and renamed in order to be more concise. `MessageReaction` has been renamed to
`Reaction`, and thus the previous `Reaction` has been renamed to
`GatewayReaction`. Besides that, the changes are too extensive to list here;
proper tooling is recommended in order to sort out the new imports. 
- [**breaking**] move related modules under `guild` ([#1814](https://github.com/twilight-rs/twilight/issues/1814))
- [**breaking**] move related modules under `message` ([#1831](https://github.com/twilight-rs/twilight/issues/1831))

Remove deprecated `Id` aliases ([#1976] - [@AEnterprise]). These have been
deprecated since 0.9.

Other changes:

- [**breaking**] cleanup and document `voice` ([#1820](https://github.com/twilight-rs/twilight/issues/1820))
- [**breaking**] update `ChannelType` names ([#1909](https://github.com/twilight-rs/twilight/issues/1909))

### Internal Refactor

- clippy 1.65 lints ([#1985](https://github.com/twilight-rs/twilight/issues/1985))

[#1816]: https://github.com/twilight-rs/twilight/pull/1816
[#1819]: https://github.com/twilight-rs/twilight/pull/1819
[#1859]: https://github.com/twilight-rs/twilight/pull/1859
[#1897]: https://github.com/twilight-rs/twilight/pull/1897
[#1976]: https://github.com/twilight-rs/twilight/pull/1976

## [0.13.7] - 2022-11-01

### Bug Fixes

- [**breaking**] flatten `DefaultReaction` and `ForumTag` ([#1978](https://github.com/twilight-rs/twilight/issues/1978))

## [0.13.6] - 2022-10-28

### Documentation

- update `Id::new_unchecked` and `Id::new_checked` ([#1959](https://github.com/twilight-rs/twilight/issues/1959))

### Features

- [**breaking**] auto moderation http methods and mention spam ([#1846](https://github.com/twilight-rs/twilight/issues/1846))
- forum channels ([#1864](https://github.com/twilight-rs/twilight/issues/1864))

## [0.13.5] - 2022-09-29

### Bug Fixes

- [**breaking**] use `ImageHash` for `InteractionMember::avatar` ([#1924](https://github.com/twilight-rs/twilight/issues/1924))

### Build

- fix or ignore clippy for 1.64

### Features

- add `GuildFeature::InvitesDisabled` ([#1910](https://github.com/twilight-rs/twilight/issues/1910))
- [**breaking**] bring audit log up to date ([#1921](https://github.com/twilight-rs/twilight/issues/1921))
- method to get the `guild_id` from an `Event` ([#1899](https://github.com/twilight-rs/twilight/issues/1899))
- add `scopes` to `GuildIntegration` ([#1915](https://github.com/twilight-rs/twilight/issues/1915))

## [0.13.4] - 2022-09-16

### Bug Fixes

- don't serialize `Mention.member` if None ([#1896](https://github.com/twilight-rs/twilight/issues/1896))

### Features

- add `two_way_link` to `Connection` ([#1918](https://github.com/twilight-rs/twilight/issues/1918))

## [0.13.3] - 2022-09-08

### Features

- add application command badge flag ([#1888](https://github.com/twilight-rs/twilight/issues/1888))
- add handling for gateway resume url ([#1894](https://github.com/twilight-rs/twilight/issues/1894))

## [0.13.2] - 2022-09-01

### Bug Fixes

- autocomplete focused options are always strings ([#1873](https://github.com/twilight-rs/twilight/issues/1873))

### Refactor

- mark c-style enums `#[non_exhaustive]` ([#1862](https://github.com/twilight-rs/twilight/issues/1862))

## [0.13.1] - 2022-08-29

### Bug Fixes

- serialize options localizations as well ([#1871](https://github.com/twilight-rs/twilight/issues/1871))
- deserialize `Event::GuildStickersUpdate` ([#1858](https://github.com/twilight-rs/twilight/issues/1858))
- don't serialize `PartialMember.permissions` if `None` ([#1878](https://github.com/twilight-rs/twilight/issues/1878))

## [0.13.0] - 2022-08-14

### Bug Fixes

- rust 1.63 lints
- [**breaking**] make `Channel::position` `i32` ([#1865](https://github.com/twilight-rs/twilight/issues/1865))

### Documentation

- fully document `channel::message` ([#1792](https://github.com/twilight-rs/twilight/issues/1792))

### Refactor

- [**breaking**] remove `Number` ([#1817](https://github.com/twilight-rs/twilight/issues/1817))
- [**breaking**] update `VoiceServerUpdate` ([#1837](https://github.com/twilight-rs/twilight/issues/1837))

## [0.12.3] - 2022-08-11

### Bug Fixes

- [**breaking**] make `Channel::position` `i32` ([#1865](https://github.com/twilight-rs/twilight/issues/1865))

## [0.12.2] - 2022-07-26

### Documentation

- format doc examples ([#1847](https://github.com/twilight-rs/twilight/issues/1847))

### Refactor

- command field changes ([#1852](https://github.com/twilight-rs/twilight/issues/1852))

## [0.12.1] - 2022-07-18

### Bug Fixes

- bump user_limit from u8 to u32 ([#1840](https://github.com/twilight-rs/twilight/issues/1840))

## [0.12.0] - 2022-07-17

### Bug Fixes

- [**breaking**] multiple options and autocomplete ([#1614](https://github.com/twilight-rs/twilight/issues/1614))

### Features

- [**breaking**] add `GuildFeature` ([#1803](https://github.com/twilight-rs/twilight/issues/1803))
- initial pass at dealing with unknown enum variants ([#1550](https://github.com/twilight-rs/twilight/issues/1550))
- `CommandOption` max and min length ([#1826](https://github.com/twilight-rs/twilight/issues/1826))
- auto moderation models ([#1796](https://github.com/twilight-rs/twilight/issues/1796))
- [**breaking**] builders use `Into<String>` and `IntoIterator` everywhere ([#1774](https://github.com/twilight-rs/twilight/issues/1774))

### Refactor

- [**breaking**] reduce channel integer sizes ([#1775](https://github.com/twilight-rs/twilight/issues/1775))
- [**breaking**] make interaction a struct ([#1813](https://github.com/twilight-rs/twilight/issues/1813))

## [0.11.3] - 2022-07-16

### Bug Fixes

- signed thread member update count ([#1838](https://github.com/twilight-rs/twilight/issues/1838))

### Features

- add `Deref` for `VoiceStateUpdate` ([#1835](https://github.com/twilight-rs/twilight/issues/1835))

## [0.11.2] - 2022-07-09

### Bug Fixes

- fix thread max message count ([#1822](https://github.com/twilight-rs/twilight/issues/1822))

## [0.11.1] - 2022-07-07

### Bug Fixes

- button component serialization length ([#1799](https://github.com/twilight-rs/twilight/issues/1799))
- add missing received events per intent ([#1818](https://github.com/twilight-rs/twilight/issues/1818))

### Documentation

- auto archives not boost locked ([#1747](https://github.com/twilight-rs/twilight/issues/1747))
- add attachment documentation ([#1739](https://github.com/twilight-rs/twilight/issues/1739))
- fix command type typo ([#1800](https://github.com/twilight-rs/twilight/issues/1800))

### Features

- add command data guild_id field ([#1755](https://github.com/twilight-rs/twilight/issues/1755))
- add `From` implementation for `Component` variants ([#1753](https://github.com/twilight-rs/twilight/issues/1753))
- add `MessageType::AutoModerationAction`  ([#1710](https://github.com/twilight-rs/twilight/issues/1710))
- remove renames in identify properties ([#1797](https://github.com/twilight-rs/twilight/issues/1797))
- set track_caller on panicable functions ([#1802](https://github.com/twilight-rs/twilight/issues/1802))
- add `app_permissions` field on interactions ([#1805](https://github.com/twilight-rs/twilight/issues/1805))

### Refactor

- `#[must_use]` on builders, not methods ([#1761](https://github.com/twilight-rs/twilight/issues/1761))
- remove `test_` prexif from tests ([#1767](https://github.com/twilight-rs/twilight/issues/1767))
- rework optional member deserialization ([#1743](https://github.com/twilight-rs/twilight/issues/1743))
- clippy 1.62 ([#1806](https://github.com/twilight-rs/twilight/issues/1806))
- add `#[non_exhaustive]` to c-style enums ([#1795](https://github.com/twilight-rs/twilight/issues/1795))

Changelog for `twilight-model`.

## [0.11.0] - 2022-05-15

MSRV has been bumped to 1.60.

### Additions

Support Guild Scheduled Events gateway events ([#1574] - [@itohatweb]).

Support command localization via `description_localizations` and
`name_localizations` fields on commands ([#1701] - [@baptiste0928]).

### Changes

Simplify the `PresenceUpdate` event by making it a newtype of `Presence`
([#1582] - [@zeylahellyer]).

Move the `datetime` module into the `util` module ([#1597] - [@vilgotf]).

Make user `accent_color` fields a u32 instead of u64 ([#1695] - [@tomocrafter]).

Remove application summary field names ([#1659] - [@zeylahellyer]).

Remove store channel type as it was removed by Discord ([#1595] - [@itohatweb]).

Use `ImageHash` for image related audit log changes ([#1632] - [@itohatweb]).

Drop Clone, Copy implementations on ID markers since they have no effect
([#1617] - [@vilgotf]).

Remove identify referrer properties ([#1609] - [@vilgotf]).

`tracing` is no longer an optional dependency and is always enabled
([#1684], [#1730] - [@vilgotf], [@zeylahellyer]).

[#1730]: https://github.com/twilight-rs/twilight/pull/1730
[#1695]: https://github.com/twilight-rs/twilight/pull/1695
[#1684]: https://github.com/twilight-rs/twilight/pull/1684
[#1659]: https://github.com/twilight-rs/twilight/pull/1659
[#1632]: https://github.com/twilight-rs/twilight/pull/1632
[#1617]: https://github.com/twilight-rs/twilight/pull/1617
[#1609]: https://github.com/twilight-rs/twilight/pull/1609
[#1597]: https://github.com/twilight-rs/twilight/pull/1597
[#1582]: https://github.com/twilight-rs/twilight/pull/1582
[#1574]: https://github.com/twilight-rs/twilight/pull/1574

## [0.10.3] - 2022-05-15

### Additions

Add getter for retrieving interaction tokens ([#1698] - [@laralove143]).

Document interaction types valid for modal submitting
([#1697] - [@laralove143]).

Add `USE_EXTERNAL_STICKERS` permission ([#1694] - [@itohatweb]).

Add `MANAGE_EVENTS` permission ([#1693] - [@itohatweb]).

Document expected types of interactions ([#1687] - [@vilgotf]).

Document message content intent caveats on fields
([#1677] - [@itohatweb], [@zeylahellyer]).

### Fixes

Skip serializing `ModalSubmitInteraction::message` if none
([#1705] - [@itohatweb]).

[#1705]: https://github.com/twilight-rs/twilight/pull/1705
[#1698]: https://github.com/twilight-rs/twilight/pull/1698
[#1697]: https://github.com/twilight-rs/twilight/pull/1697
[#1694]: https://github.com/twilight-rs/twilight/pull/1694
[#1693]: https://github.com/twilight-rs/twilight/pull/1693
[#1687]: https://github.com/twilight-rs/twilight/pull/1687
[#1677]: https://github.com/twilight-rs/twilight/pull/1677

## [0.10.2] - 2022-04-15

### Additions

Add `#[repr(transparent)]` to `Id<T>` ([#1619] - [@PyroTechniac]).

Add quality-of-life methods on some `Interaction` types ([#1620] - [@vilgotf]):
- `Interaction`
  - `fn application_id(&self) -> Id<ApplicationMarker>`
  - `fn kind(&self) -> InteractionType`
- `ApplicationCommand`, `ApplicationCommandAutocomplete`
  - `fn author_id(&self) -> Option<Id<UserMarker>>`
- `ApplicationCommand`, `ApplicationCommandAutocomplete`,
`MessageComponentInteraction`, `ModalSubmitInteraction`
  - `fn is_dm(&self) -> bool`
  - `fn is_guild(&self) -> bool`

Add `AuditLogChange::ImageHash` and `AuditLogChangeKey::ImageHash` ([#1631] -
[@itohatweb]]).

Add `ChannelType::GuildDirectory` ([#1655] - [@zeylahellyer]).

Add `InviteGuild::premium_subscription_count` ([#1661] - [@zeylahellyer]).

Add `Application::{custom_install_url, install_params, tags}` ([#1670] -
[@zeylahellyer]).

Add `ChannelType::GuildForum` ([#1682] - [@7596ff]).

### Changes

Standardize documentation on `Interaction` types ([#1620] - [@vilgotf]).

Rename `CurrentApplicationInfo` to `Application` ([#1648] - [@zeylahellyer]).
Additionally, restructure the `oauth` module and deprecate old exports of its
types.

Update `UserFlags::HYPESQUAD` docs ([#1658] - [@zeylahellyer]).

Make `Application::owner` an `Option` ([#1671] - [@zeylahellyer]).

### Fixes

In order to properly send attachments when using `CreateResponse`, rework
`Attachment` ([#1624] - [@7596ff]). This is a breaking change; the user is now
required to supply a custom unique ID. Changes:
- Add `id: u64` field, add `id` parameter to `from_bytes`
- Skip serializing on `description` if empty
- Skip serializing `file` entirely

[#1619]: https://github.com/twilight-rs/twilight/pull/1619
[#1620]: https://github.com/twilight-rs/twilight/pull/1620
[#1624]: https://github.com/twilight-rs/twilight/pull/1624
[#1631]: https://github.com/twilight-rs/twilight/pull/1631
[#1648]: https://github.com/twilight-rs/twilight/pull/1648
[#1655]: https://github.com/twilight-rs/twilight/pull/1655
[#1661]: https://github.com/twilight-rs/twilight/pull/1661
[#1670]: https://github.com/twilight-rs/twilight/pull/1670
[#1671]: https://github.com/twilight-rs/twilight/pull/1671
[#1682]: https://github.com/twilight-rs/twilight/pull/1682

## [0.10.1] - 2022-03-20

### Additions

Add `StageInstance::guild_scheduled_event_id` ([#1567] - [@itohatweb]).

Document `ApplicationFlags` ([#1578] - [@7596ff]).

Add `Channel::newly_created` ([#1588] - [@7596ff]).

Add `guild_locale`, `locale`, and `message` to `ModalSubmitInteraction` ([#1613]
- [@itohatweb]).

### Changes

Rename `Permissions::START_EMBEDDED_ACTIVITIES` to `USE_EMBEDDED_ACTIVITIES`
([#1568] - [@itohatweb]).

Remove `Invite::stage_instance` ([#1569] - [@itohatweb]).

Remove `Ord` and `PartialOrd` implementations on the following ([#1572] -
[@vilgotf]):
- `ActivityType`
- `ApplicationCommandAutocompleteDataOptionType`
- `AuditLogEventType`
- `ChannelType`
- `CommandOptionType`
- `CommandType`
- `ComponentType`
- `InteractionResponseType`
- `InteractionType`
- `MessageActivityType`
- `MessageType`
- `PremiumType`
- `Status`
- `StickerFormatType`
- `StickerType`
- `TargetType`
- `WebhookType`

### Fixes

Correct time unit on `AutoArchiveDuration::number` ([#1571] - [@vilgotf]).

[#1567]: https://github.com/twilight-rs/twilight/pull/1567
[#1568]: https://github.com/twilight-rs/twilight/pull/1568
[#1569]: https://github.com/twilight-rs/twilight/pull/1569
[#1571]: https://github.com/twilight-rs/twilight/pull/1571
[#1572]: https://github.com/twilight-rs/twilight/pull/1572
[#1578]: https://github.com/twilight-rs/twilight/pull/1578
[#1588]: https://github.com/twilight-rs/twilight/pull/1588
[#1613]: https://github.com/twilight-rs/twilight/pull/1613

## [0.10.0] - 2022-03-10

### Channels

The `Channel` type has been unified into a struct ([#1449] - [@zeylahellyer],
[@itohatweb]). All possible fields of every channel variant and thread variant
are now present on this type. This change was prompted by Discord's own storage
of channels, and that variants do not necessarily have guaranteed fields. See
the PR description for more details.

### New `http` module

Add a new module, `http` ([#1508], [#1521] - [@7596ff]). This module contains
types that are only sent to Discord.

`AttachmentFile` has been moved from `twilight-http` and renamed to
`model::http::attachment::Attachment`.

`InteractionResponse` has been moved to
`model::http::interaction::InteractionResponse`. `CallbackData` has been
renamed to `InteractionResponseData`.

`PermissionOverwrite` now has a separate type in `model::http`; it differs from
a received `PermissionOverwrite` in that its `allow` and `deny` fields are
optional.

### Additions

Add support for modals ([#1300] - [@itohatweb], [@7596ff]):
- Sending
  - add a new component, `TextInput`
  - move `Component` de/serialization to the enum itself, and remove all
    de/serialization from its variants
- Receiving
  - add `Interaction::Modal`, `InteractionType::ModalSubmit`
  - add `ModalSubmitInteraction`, `ModalInteractionData`,
    `ModalInteractionDataActionRow`, `ModalInteractionDataComponent`

Add `GuildStickersUpdate` to the `Event` enum ([#1520] - [@HTG-YT]).

### Changes

`Event` variants have been boxed or unboxed based on a new threshold, making the
size of the enum more consistent ([#1436] - [@vilgotf]).

Rename `Intents::GUILD_EMOJIS` to `GUILD_EMOJIS_AND_STICKERS` ([#1520] -
[@HTG-YT]).

`PermissionOverwrite` has been refactored to more closely represent Discord's
model ([#1521] - [@7596ff]). Its ID is stored with a generic marker, and can be
casted to a member or role ID as needed.

Update to Discord API version 10 ([#1540] - [@zeylahellyer]). This involves two
changes:
- remove `CurrentApplicationInfo`'s `summary` field
- add `Intents::MESSAGE_CONTENT`

[#1300]: https://github.com/twilight-rs/twilight/pull/1300
[#1436]: https://github.com/twilight-rs/twilight/pull/1436
[#1449]: https://github.com/twilight-rs/twilight/pull/1449
[#1508]: https://github.com/twilight-rs/twilight/pull/1508
[#1520]: https://github.com/twilight-rs/twilight/pull/1520
[#1521]: https://github.com/twilight-rs/twilight/pull/1521
[#1540]: https://github.com/twilight-rs/twilight/pull/1540

## [0.9.2] - 2022-02-12

### Additions

Support the `Attachment` command option type ([#1537] - [@Erk-]). This includes
a new variant of `CommandOption` and a new field in
`CommandInteractionDataResolved`.

### Fixes

Autocomplete values no longer use the `ApplicationCommand` structure, they
rather use a separate struct that parses all `options` as `String`s ([#1542] -
[@7596ff]).

Add missing variants of `AuditLogEventType` pertaining to Stage Instances
([#1547] - [@7596ff]).

[#1537]: https://github.com/twilight-rs/twilight/pull/1537
[#1542]: https://github.com/twilight-rs/twilight/pull/1542
[#1547]: https://github.com/twilight-rs/twilight/pull/1547

## [0.9.1] - 2022-02-12

### Additions

Add new `Id<T>` implementations ([#1493] - [@dnaka91]):
- `From<Id<T>> for NonZeroU64`
- `From<Id<T>> for u64`
- `Id::into_nonzero`

Add `CommandData::{kind, target_id}` ([#1522] - [@Liamolucko]).

Add `ScheduledEvent::image` ([#1525] - [@7596ff]).

Add `MessageFlags::{LOADING, FAILED_TO_MENTION_SOME_ROLES_IN_THREAD}` ([#1526] -
[@7596ff]).

Add `MessageInteraction::member` ([#1532] - [@7596ff]).

### Changes

Update many links to Discord documentation with consistent capitalization and
page titles ([#1429] - [@itohatweb], [@7596ff]).

Implement `Display` directly on `Id` ([#1494] - [@vilgotf]).

### Fixes

Update links to builders in `twilight-util` ([#1516] - [@laralove143]).

[#1429]: https://github.com/twilight-rs/twilight/pull/1429
[#1493]: https://github.com/twilight-rs/twilight/pull/1493
[#1494]: https://github.com/twilight-rs/twilight/pull/1494
[#1516]: https://github.com/twilight-rs/twilight/pull/1516
[#1522]: https://github.com/twilight-rs/twilight/pull/1522
[#1525]: https://github.com/twilight-rs/twilight/pull/1525
[#1526]: https://github.com/twilight-rs/twilight/pull/1526
[#1532]: https://github.com/twilight-rs/twilight/pull/1532

## [0.9.0] - 2022-01-22

### `Id<T>`

IDs are now a unified type (`Id`) with marker generics (`ApplicationMarker`,
...) ([#1260] - [@zeylahellyer]). The new type implements all of what each type
used to implement, as well as `FromStr`, `TryFrom<u64>`, and `TryFrom<i64>`, and
others. `Id::cast` aids in converting between IDs without copying. See the PR
and the documentation for more details.

### Additions

Support scheduled events ([#1347] - [@7596ff]). Adds the following types:
`EntityMetadata`, `EntityType`, `GuildScheduledEventUser`,
`GuildScheduledEvent`, `PrivacyLevel`, and `Status`.

### Changes

All types and method signatures have been updated to use the new `Id<T>` syntax
([#1260] - [@zeylahellyer]).

The MSRV has been updated to 1.57 ([#1402] - [@zeylahellyer]).

Image hashes are now parsed and stored in a more efficient struct, rather than
deserializing as a `String` ([#1405] - [@zeylahellyer]).

The Rust edition has been updated to 2021 ([#1412] - [@vilgotf]).

`StageInstance::discoverable_disabled` and `PrivacyLevel::Public` have been
removed, as public stage instances are no longer supported ([#1479] -
[@itohatweb]).

`GuildWidget::channel_id` is now optional ([#1480] - [@itohatweb]).

[#1260]: https://github.com/twilight-rs/twilight/pull/1260
[#1402]: https://github.com/twilight-rs/twilight/pull/1402
[#1405]: https://github.com/twilight-rs/twilight/pull/1405
[#1412]: https://github.com/twilight-rs/twilight/pull/1412
[#1479]: https://github.com/twilight-rs/twilight/pull/1479
[#1480]: https://github.com/twilight-rs/twilight/pull/1480

## [0.8.5] - 2022-01-21

### Additions

In interactions, support `guild_locale` and `locale` ([#1437] - [@itohatweb]).

Support `Invitable` in the audit log ([#1442] - [@itohatweb]).

Add `GuildPreview::stickers` ([#1467] - [@itohatweb]).

Add `ThreadMetadata::created_timestamp` ([#1478] - [@itohatweb]).

### Changes

Depend on at least version `0.1.16` of `tracing` ([#1425] - [@vilgotf]).

[#1425]: https://github.com/twilight-rs/twilight/pull/1425
[#1437]: https://github.com/twilight-rs/twilight/pull/1437
[#1442]: https://github.com/twilight-rs/twilight/pull/1442
[#1467]: https://github.com/twilight-rs/twilight/pull/1467
[#1478]: https://github.com/twilight-rs/twilight/pull/1478

## [0.8.4] - 2022-01-08

### Additions

Add `{Guild, PartialGuild}::premium_progress_bar_enabled` ([#1399] - [@Erk-]).

Support guild timeouts in `MemberUpdate` ([#1414] - [@AEnterprise]).

Add `VoiceState::self_video` ([#1422] - [@AEnterprise]).

Support guild timeouts in the audit log ([#1423] - [@itohatweb]).

### Changes

`CallbackData::embeds` is now an `Option<Vec>` instead of a `Vec` ([#1401] -
[@itohatweb]).

### Fixes

Properly deserialize `AuditLogChange::{RoleAdd, RoleRemove}` as `$add` and
`$remove` ([#1419] - [@itohatweb]).

Change `ThreadListSync::channel_ids` to a list of channel IDs instead of guild
IDs ([#1420] - [@AEnterprise]).

Properly deserialize `ThreadDelete` ([#1426] - [@AEnterprise]).

[#1399]: https://github.com/twilight-rs/twilight/pull/1399
[#1401]: https://github.com/twilight-rs/twilight/pull/1401
[#1414]: https://github.com/twilight-rs/twilight/pull/1414
[#1419]: https://github.com/twilight-rs/twilight/pull/1419
[#1420]: https://github.com/twilight-rs/twilight/pull/1420
[#1422]: https://github.com/twilight-rs/twilight/pull/1422
[#1423]: https://github.com/twilight-rs/twilight/pull/1423
[#1426]: https://github.com/twilight-rs/twilight/pull/1426

## [0.8.3] - 2021-12-27

### Additions

Support guild member timeouts via
`Member::communication_disabled_until` and `Permissions::MODERATE_MEMBERS`
([#1342] - [@HTG-YT]).

[#1342]: https://github.com/twilight-rs/twilight/pull/1342

## [0.8.2] - 2021-12-24

### Changes

All `AuditLogChange::{new, old}` variants are now `Option`s ([#1324] -
[@7596ff]). This is to prevent small changes made by Discord from causing
deserialization errors. Additionally, adds some missing variants from the latest
docs.

`StickerPack::banner_asset_id` is now an `Option` ([#1337] - [@vilgotf]).

Add `avatar`, `permissions`, and `pending` fields to `InteractionMember`
([#1339] - [@itohatweb]).

[#1324]: https://github.com/twilight-rs/twilight/pull/1324
[#1337]: https://github.com/twilight-rs/twilight/pull/1337
[#1339]: https://github.com/twilight-rs/twilight/pull/1339

## [0.8.1] - 2021-12-15

### Changes

`Invite::channel` is now optional ([#1325] - [@zeylahellyer]).

[#1325]: https://github.com/twilight-rs/twilight/pull/1325

## [0.8.0] - 2021-12-03

### Changes

`tracing` is now an optional feature, and enabled by default ([#1203] -
[@Gelbpunkt]).

`MessageType::ApplicationCommand` has been renamed to `ChatInputCommand`
([#1211] - [@7596ff]).

`CommandInteractionDataResolved` now has `HashMap` fields instead of
`Vec`s ([#1225] - [@vilgotf]).

Deprecated `UserFlags` have been removed ([#1274] - [@7596ff]).

`EmbedAuthor::name`, `EmbedImage::url`, and `EmbedThumbnail::url` are
now required fields ([#1290] - [@itohatweb]).

[#1203]: https://github.com/twilight-rs/twilight/pull/1203
[#1211]: https://github.com/twilight-rs/twilight/pull/1211
[#1225]: https://github.com/twilight-rs/twilight/pull/1225
[#1274]: https://github.com/twilight-rs/twilight/pull/1274
[#1290]: https://github.com/twilight-rs/twilight/pull/1290

## [0.7.3] - 2021-12-03

### Fixes

Mark some `old` fields as `Option`s to fix a case where they weren't present
([#1284] - [@7596ff]).

[#1284]: https://github.com/twilight-rs/twilight/pull/1284

## [0.7.2] - 2021-11-20

### Additions

Add the `UserOrId::id` function ([#1219] - [@vilgotf]).

Support `ApplicationCommand` autocomplete ([#1228] - [@vilgotf]).

New struct(s):
- `application::callback::Autocomplete`

New method(s):
- `application::command::CommandOption::is_autocomplete`

New fields/variants:
- `application::callback::ResponseType::ApplicationCommandAutocompleteResult`
- `application::interaction::application_command::CommandDataOption::focused`
- `application::interaction::InteractionType::ApplicationCommandAutocomplete`
- `application::interaction::Interaction::ApplicationCommandAutocomplete`

Add `Permissions::START_EMBEDDED_ACTIVITIES` ([#1229] - [@vilgotf]).

Add `Command::version` field, which is a `CommandVersionId` ([#1230] -
[@vilgotf]).

Add a new `CommandOptionData` variant with `min_value` and `max_value` fields
([#1235] - [@baptiste0928]).

Inject the guild ID while deserializing `ThreadMembersUpdate` ([#1264] -
[@Erk-]).

Add `ActivityFlags::{PARTY_PRIVACY_FRIENDS, PARTY_PRIVACY_VOICE_CHANNEL,
EMBEDDED}` ([#1266] - [@7596ff]).

Add `ApplicationFlags::{GATEWAY_MESSAGE_CONTENT,
GATEWAY_MESSAGE_CONTENT_LIMITED}` ([#1267] - [@7596ff]).

Bring the `UserFlags` struct up to date ([#1268] - [@7596ff]).

Add `SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATION_REPLIES` ([#1269] -
[@7596ff]).

### Changes

`Member::joined_at`, `PartialMember::joined_at`, and
`InteractionMember::joined_at` required ([#1220] - [@vilgotf]).

Depend on at least `serde 1.0.103` and `serde_repr 0.1.5` ([#1277] -
[@vilgotf]).

### Fixes

Properly serialize the `CommandData` struct ([#1234] - [@vilgotf]).

Fix deserialization of the `CommandDataOption` struct ([#1240] - [@Erk-]). This
properly accounts for the `type` field that Discord provides.

Fix the `Invite` struct by moving some fields from the erroneous
`InviteMetadata` struct ([#1247] - [@7596ff]).

Fix an issue in `CommandOption` deserialization where `channel_types` was not
properly deserialized ([#1272] - [@vilgotf]).

[#1217]: https://github.com/twilight-rs/twilight/pull/1217
[#1219]: https://github.com/twilight-rs/twilight/pull/1219
[#1220]: https://github.com/twilight-rs/twilight/pull/1220
[#1228]: https://github.com/twilight-rs/twilight/pull/1228
[#1229]: https://github.com/twilight-rs/twilight/pull/1229
[#1230]: https://github.com/twilight-rs/twilight/pull/1230
[#1235]: https://github.com/twilight-rs/twilight/pull/1235
[#1240]: https://github.com/twilight-rs/twilight/pull/1240
[#1247]: https://github.com/twilight-rs/twilight/pull/1247
[#1264]: https://github.com/twilight-rs/twilight/pull/1264
[#1266]: https://github.com/twilight-rs/twilight/pull/1266
[#1267]: https://github.com/twilight-rs/twilight/pull/1267
[#1268]: https://github.com/twilight-rs/twilight/pull/1268
[#1269]: https://github.com/twilight-rs/twilight/pull/1269
[#1272]: https://github.com/twilight-rs/twilight/pull/1272
[#1277]: https://github.com/twilight-rs/twilight/pull/1277

## [0.7.1] - 2021-10-29

### Additions

Add `AuditLogChangeKey::UnicodeEmoji`, `Role::icon`, and
`Role::unicode_emoji` ([#1212] - [@7596ff]).

Add `Attachment::ephemeral` ([#1213] - [@7596ff]).

### Changes

Remove `Member::hoisted_role` ([#1221] - [@vilgotf]).

Fixes some spelling errors in documentation ([#1223] - [@7596ff]).

### Fixes

Fix subcommands without options throwing an error on deserialization
([#1216] - [@vilgotf]).

[#1212]: https://github.com/twilight-rs/twilight/pull/1212
[#1213]: https://github.com/twilight-rs/twilight/pull/1213
[#1216]: https://github.com/twilight-rs/twilight/pull/1216
[#1221]: https://github.com/twilight-rs/twilight/pull/1221
[#1223]: https://github.com/twilight-rs/twilight/pull/1223

## [0.7.0] - 2021-10-21

### Enhancements

All `Id` models are now based on `NonZeroU64` instead of `u64` ([#1039]
- [@vilgotf]). This type takes up less space in memory than a regular
`u64` when used in an `Option`. Instead of simple initialization of an
`Id`, the models now have methods that map to their `NonZeroU64`
equivalents: `get`, `new_unchecked`, and `new`. Additionally, the types
no longer implement `Default` since there is no default value for a
`NonZeroU64`.

`User::discriminator` is now stored as a `u16` instead of a `String`
([#1068] - [@zeylahellyer]). The display implementation pads the value
to four digits.

`CommandDataOption` is no longer an enum ([#1077] - [@LeSeulArtichaut]).
It is a struct with a `name` field of type `String`, and a `value` field
of type `CommandOptionValue`. `CommandOptionValue` is an enum of each
type of option the user might receive from an interaction. Instead of
coalescing all types as strings, numbers, booleans or subcommands, the
new type explicitly tells the user what type of option they have
received.

Timestamps are now parsed and formatted with a custom implementation
([#1164] - [@zeylahellyer]). `Timestamp` is used in place of `String`s
on fields such as `joined_at`, `premium_since`, `timestamp`, and so on.
Included are `serde::Deserialize` and `serde::Serialize` implementations
on `Timestamp` itself and a `serde::Serialize` implementation on the
Display formatter. `Timestamp`s contain a `NonZeroU64` for greater
efficiency when wrapped in an `Option`. See the PR and the `Timestamp`
documentation for more information.

### Changes

The `gateway::payload` module has been reorganized into two sections:
`incoming` and `outgoing` events ([#1135] - [@zeylahellyer]).

`OptionsCommandOptionData::required` has been removed, since it is
always false ([#1108] - [@vilgotf]).

The MSRV has been updated to 1.53 ([#1161] - [@7596ff]).

`VoiceRegion::vip` has been removed ([#1190] - [@HTG-YT]).

`Permissions::MANAGE_EMOJIS` has been renamed to
`MANAGE_EMOJIS_AND_STICKERS`, with no change in value ([#1197] -
[@7596ff]).

[#1039]: https://github.com/twilight-rs/twilight/pull/1039
[#1068]: https://github.com/twilight-rs/twilight/pull/1068
[#1077]: https://github.com/twilight-rs/twilight/pull/1077
[#1108]: https://github.com/twilight-rs/twilight/pull/1108
[#1135]: https://github.com/twilight-rs/twilight/pull/1135
[#1161]: https://github.com/twilight-rs/twilight/pull/1161
[#1164]: https://github.com/twilight-rs/twilight/pull/1164
[#1190]: https://github.com/twilight-rs/twilight/pull/1190
[#1197]: https://github.com/twilight-rs/twilight/pull/1197

## [0.6.5] - 2021-10-07

### Additions

Adds the `CommandOption::Number` variant, which is a single element
tuple containing an inner `ChoiceCommandOptionData`. Adds the variant
`CommandOptionChoice::Number`. Adds the type `Number`, which is a `f64`
that implements `Eq`, `Hash`, and `PartialEq` comparing their
`from_bits` values ([#1053] - [@tbnritzdoge]).

Adds the following models: `GuildStickersUpdate`,
`StickerBannerAssetId`, `StickerPackSkuId`, `StickerPack`, and
`StickerType` ([#1157] - [@7596ff]).

Adds `AuditLogChange::{Asset, Available, FormatType, GuildId, Tags}`
([#1157] - [@7596ff]).

Adds `Guild::stickers` ([#1157] - [@7596ff]).

Adds `Interaction::id`, which retrieves the inner ID of an interaction
([#1188] - [@PyroTechniac]).

### Changes

Deprecates `Permissions::MANAGE_EMOJIS`, as in the next breaking
release, it will be renamed to `MANAGE_EMOJIS_AND_STICKERS` ([#1157] -
[@7596ff]).

Upgrades `CommandOption::Channel` from a `BaseCommandOptionData` to a
`ChannelCommandOptionData`. This includes the field `channel_types`.
which is a list of `ChannelType`s ([#1180] - [@baptiste0928])

Deprecates `VoiceRegion::vip`, as it is no longer sent by Discord
([#1182] - (@HTG-YT)).

[#1053]: https://github.com/twilight-rs/twilight/pull/1053
[#1157]: https://github.com/twilight-rs/twilight/pull/1157
[#1180]: https://github.com/twilight-rs/twilight/pull/1180
[#1182]: https://github.com/twilight-rs/twilight/pull/1182
[#1188]: https://github.com/twilight-rs/twilight/pull/1188

## [0.6.4] - 2021-09-17

### Thread Support

3 new channel types have been added: `GuildNewsThread`,
`GuildPublicThread`, and `GuildPrivateThread`.

`InteractionChannel` now contains the `parent_id` and `thread_metadata`
fields.

Adds `MessageFlags::HAS_THREAD`, `MessageType::ThreadCreated`, and
`MessageType::ThreadStarterMessage`.

Adds thread-related models: `AutoArchiveDuration`, `ThreadsListing`,
`ThreadMember`, `ThreadMetadata`, `NewsThread`, `PrivateThread`, and
`PublicThread`.

Adds thread-related gateway payloads: `ThreadCreate`, `ThreadDelete`,
`ThreadListSync`, `ThreadMemberUpdate`, `ThreadMembersUpdate`, and
`ThreadUpdate`.

Adds thread-related audit log items:
 - change keys: `Archived`, `AutoArchiveDuration`,
   `DefaultAutoArchiveDuration`, and `Locked`.
 - event types: `ThreadCreate`, `ThreadDelete`, and `ThreadUpdate`.
 - root: `AuditLog::threads`.

Adds thread-related permissions: `MANAGE_THREADS`,
`CREATE_PUBLIC_THREADS`, `CREATE_PRIVATE_THREADS`, and
`SEND_MESSAGES_IN_THREADS`.

## [0.6.3] - 2021-09-17

### Additions

Add `Command::kind`, which is a `CommandType`, in order to support
Message and User application commands ([#1107] - [@7596ff]).

Add `CommandInteractionDataResolved::messages`, which is a list of
messages relevant to the interaction ([#1107] - [@7596ff]).

Add `MessageType::ContextMenuCommand`, which is relevant to Message and
User commands ([#1107] - [@7596ff]).

Add the `{Current, User}::{accent_color, banner}` fields ([#1127] -
[@zeylahellyer]).

### Fixes

`Command` deserialization no longer errors if the `options` field was
missing ([#1112] - [@Erk-]).

[#1107]: https://github.com/twilight-rs/twilight/pull/1107

## [0.6.2] - 2021-08-30

### Additions

Support message components, including action rows, buttons, and select menus
([#1020], [#1043], [#1044], [#1090], aggregate [#1121] - [@AEnterprise],
[@AsianIntel], [@zeylahellyer], [@7596ff]).

### Enhancements

Fix a remaining intradoc link ([#1128] - [@zeylahellyer]).

[#1128]: https://github.com/twilight-rs/twilight/pull/1128
[#1121]: https://github.com/twilight-rs/twilight/pull/1121
[#1090]: https://github.com/twilight-rs/twilight/pull/1090
[#1044]: https://github.com/twilight-rs/twilight/pull/1044
[#1043]: https://github.com/twilight-rs/twilight/pull/1043
[#1020]: https://github.com/twilight-rs/twilight/pull/1020

## [0.6.1] - 2021-08-18

### Fixes

Properly handle `ChoiceCommandOptionData` with a missing choices field
([#1087] - [@MaxOhn]).

[#1087]: https://github.com/twilight-rs/twilight/pull/1087

## [0.6.0] - 2021-07-31

### Enhancements

Fully support audit logs by creating types for Audit Log Changes and
updating Audit Log Change Keys with new variants. Changes were generic
`serde_value::Value`s, which provide no typed information. There are now
variants with typed `new` and `old` values - checked against the API -
which may or may not both always be present, or never.

The `twilight_model::guild::audit_log` module is now entirely documented
and tested as well. This has been manually tested by creating as many
different audit log changes as could be created in the API and then
deserializing the past 100 entries.

([#1022] - [@zeylahellyer]).

### Changes

Rename the `GuildCreate` audit log event to `GuildUpdate` ([#966] -
[@zeylahellyer]).

A few spelling errors have been fixed by adding the `codespell` Action
([#1041] - [@Gelbpunkt].

### Fixes

`PartialApplication` now uses `ApplicationFlags` instead of `UserFlags` ([#1072] - [@A5rocks]).

[#966]: https://github.com/twilight-rs/twilight/pull/966
[#1022]: https://github.com/twilight-rs/twilight/pull/1022
[#1041]: https://github.com/twilight-rs/twilight/pull/1041
[#1072]: https://github.com/twilight-rs/twilight/pull/1072

## [0.5.4] - 2021-07-23

### Changes

`#![deny(unsafe_code)]` has been added, ensuring no unsafe code exists in the
crate ([#1042] - [@zeylahellyer]).

[#1042]: https://github.com/twilight-rs/twilight/pull/1042

## [0.5.3] - 2021-07-14

### Additions

Add `available`, `guild_id`, `sort_value`, and `user` fields to `Sticker`. Also
add the `MessageSticker` struct ([#1029] - [@7596ff]).

### Changes

Replace the `stickers` fields with `sticker_items`, which is a list of
`MessageSticker`s ([#1029] - [@7596ff]).

### Fixes

Properly deserialize the `rtc_region` field in voice and stage channels ([#1030]
- [@7596ff]).

[#1029]: https://github.com/twilight-rs/twilight/pull/1029
[#1030]: https://github.com/twilight-rs/twilight/pull/1030

## [0.5.2] - 2021-07-03

### Additions

Support invite stage instances ([#993] - [@7596ff]).

[#993]: https://github.com/twilight-rs/twilight/pull/993

## [0.5.1] - 2021-07-02

### Upgrade Path

`gateway::payload::reaction_remove_emoji::PartialEmoji` has been
removed.

`gateway::payload::reaction_remove_emoji::ReactionRemoveEmoji::emoji` is no
longer the aforementioned `PartialEmoji` and is now a `channel::ReactionType`.

### Additions

Support the new `channel::webhook::Webhook` fields `source_channel` and
`source_guild` ([#961] - [@7596ff]).

Add the new `channel::webhook::Webhook::url` field ([#957] - [@7596ff]).

### Fixes

`gateway::payload::reaction_remove_emoji::ReactionRemoveEmoji::emoji` previously
did not account for custom reactions without names, but due to now being of the
`channel::ReactionType` type it does ([#958] - [@7596ff]).

### Enhancements

Improve the `Display` implementation performance of various `Display`
implementations by calling `Formatter` methods directly instead of calling the
`format_args!` and `write!` macros ([#944] - [@zeylahellyer]).

### Changes

`channel::Webhook` and `channel::WebhookType` have been moved to a new
`channel::webhook` module, but re-exports have been left in their place
([#961] - [@7596ff]).

`gateway::payload::reaction_remove_emoji::ReactionRemoveEmoji::emoji` is now a
`channel::ReactionType`, `gateway::payload::reaction_remove_emoji::PartialEmoji`
has been removed ([#958] - [@7596ff]).

[#961]: https://github.com/twilight-rs/twilight/pull/961
[#958]: https://github.com/twilight-rs/twilight/pull/958
[#957]: https://github.com/twilight-rs/twilight/pull/957
[#944]: https://github.com/twilight-rs/twilight/pull/944

## [0.5.0] - 2021-06-13

### Upgrade Path

Remove references to `Guild::nsfw`, `Guild::region`, `PartialGuild::nsfw`,
`PartialGuild::region`, and `TemplateGuild::region`.

Replace the following usages:
```diff
-twilight_model::channel::invite::TargetUserType
+twilight_model::channel::invite::TargetType

-twilight_model::gateway::payload::update_status::UpdateStatus
+twilight_model::gateway::payload::update_presence::UpdatePresence

-twilight_model::gateway::payload::update_status::UpdateStatusInfo
+twilight_model::gateway::payload::update_presence::UpdatePresencePayload
```

### Additions

Support for Slash Commands has been added. New models are present in the
`application` module ([#932]).

### Enhancements

The following models have been updated:

- `twilight_model::channel::message::MessageFlags`: added `EPHEMERAL`
- `twilight_model::channel::Message`: added `application_id`, `interaction`
- `twilight_model::channel::WebhookType`: added `Application`
- `twilight_model::gateway::Event`: added `InteractionCreate`
- `twilight_model::gateway::payload::Ready`: added `application`
- `twilight_model::guild::PartialMember`: added `permissions`

### Changes

The `TargetType` re-export (`TargetUserType`) has been removed ([#847] -
[@7596ff]).

`Guild::nsfw` and `PartialGuild::nsfw` have been removed ([#890] - [@7596ff]).

`UpdateStatus` and `UpdateStatusInfo` have been renamed to `UpdatePresence` and
`UpdatePresencePayload` respectively ([#902] - [@7596ff]).

At least one `Activity` is required when building an `UpdatePresence` payload.
`UpdatePresenceError` and `UpdatePresenceErrorType` have been created to
validate this ([#891] - [@7596ff]).

References to `Guild::region` have been removed. This includes
`PartialGuild::region` and `TemplateGuild::region`.

[#847]: https://github.com/twilight-rs/twilight/pull/847
[#890]: https://github.com/twilight-rs/twilight/pull/890
[#891]: https://github.com/twilight-rs/twilight/pull/891
[#902]: https://github.com/twilight-rs/twilight/pull/902
[#932]: https://github.com/twilight-rs/twilight/pull/932

## [0.4.3] - 2021-06-12

### Additions

Support `StageInstanceCreate`, `StageInstanceDelete`, `StageInstanceUpdate`
events ([#845] - [@7596ff]).

Add the `NsfwLevel` change key to audit log ([#848] - [@tbnritzdoge]).

Support `channel::stage_instance::StageInstance::{discoverable_disabled,
privacy_level}` and `channel::stage_instance::PrivacyLevel` ([#867] -
[@7596ff]).

Support `Team::name` ([#881] - [@7596ff]).

Support `CurrentApplicationInfo::{flags, privacy_policy_url,
terms_of_service_url}` ([#882], [#904]
- [@7596ff], [@Gelbpunkt])

Support `StageInstance*` events and `PrivacyLevel` change key in the audit log
([#907] - [@7596ff]).

Support `Webhook::application_id` ([#908] - [@7596ff]).

Support `IntegrationCreate`, `IntegrationDelete`, and `IntegrationUpdate`
events, and add `GuildIntegration::guild_id` as an `Option` ([#914] -
[@7596ff]).

### Changes

Support `Guild::stage_instances`, which are present in the `GuildCreate` event
([#845] - [@7596ff]).

Deprecate `{Guild, PartialGuild}::nsfw` in favor of `nsfw_level`, ([#848] -
[@tbnritzdoge]).

Deprecate `{Guild, PartialGuild, TemplateGuild}::region`, as this field is no
longer provided by Discord. There is no direct alternative ([#887] -
[@BlackHoleFox]).

[#845]: https://github.com/twilight-rs/twilight/pull/845
[#848]: https://github.com/twilight-rs/twilight/pull/848
[#867]: https://github.com/twilight-rs/twilight/pull/867
[#881]: https://github.com/twilight-rs/twilight/pull/881
[#882]: https://github.com/twilight-rs/twilight/pull/882
[#887]: https://github.com/twilight-rs/twilight/pull/887
[#904]: https://github.com/twilight-rs/twilight/pull/904
[#907]: https://github.com/twilight-rs/twilight/pull/907
[#908]: https://github.com/twilight-rs/twilight/pull/908
[#914]: https://github.com/twilight-rs/twilight/pull/914

## [0.4.2] - 2021-05-30

### Upgrade Path

`invite::TargetUserType` is now deprecated; `invite::TargetType` should be used
instead. `invite::Invite::target_user_type` has been renamed to `target_type`.

### Additions

Add the `DISCORD_CERTIFIED_MODERATOR` user flag ([#820] - [@7596ff]).

Add `channel::StageInstance` and `id::StageId` in relation to stage channel
support ([#812] - [@7596ff]).

Add the `invite::Invite::expires_at` field and
`invite::TargetType::EmbeddedApplication` variant ([#809] - [@7596ff]).

Add `gateway::presence::MinimalActivity`, intended for bots to use when setting
presences ([#851] - [@7596ff]).

### Enhancements

The following functions are now `const`:

- `channel::ChannelType::name`;
- `channel::message::allowed_mentions::AllowedMentionsBuilder::new`;
- `channel::message::allowed_mentions::AllowedMentionsBuilder::replied_user`;
- `channel::message::allowed_mentions::AllowedMentions::new`
- `channel::message::sticker::StickerType::value`
- `channel::Channel::id`
- `channel::GuildChannel::guild_id`
- `channel::GuildChannel::id`
- `channel::VideoQualityMode::name`
- `gateway::event::gateway::payload::Heartbeat::new`
- `gateway::event::gateway::payload::identify::Identify::new`
- `gateway::event::gateway::payload::request_guild_members::RequestGuildMembersBuilder::new`
- `gateway::event::gateway::payload::request_guild_members::RequestGuildMembers::builder`
- `gateway::event::gateway::payload::request_guild_members::UserIdsError::kind`
- `gateway::event::gateway::payload::update_status::UpdateStatusInfo::new`
- `gateway::event::gateway::payload::UpdateVoiceState::new`
- `gateway::event::gateway::presence::PresenceDeserializer::new`
- `gateway::event::gateway::presence::PresenceListDeserializer::new`
- `gateway::event::gateway::EventType::name`
- `gateway::event::gateway::Event::kind`
- `gateway::event::gateway::GatewayEventDeserializerOwned::op`
- `gateway::event::gateway::GatewayEventDeserializerOwned::sequence`
- `gateway::event::gateway::GatewayEventDeserializer::new`
- `gateway::event::gateway::GatewayEventDeserializer::event_type_ref`
- `gateway::event::gateway::GatewayEventDeserializer::op`
- `gateway::event::gateway::GatewayEventDeserializer::sequence`
- `gateway::event::gateway::GatewayEventDeserializer::into_parts`
- `gateway::event::DispatchEvent::kind`
- `gateway::event::DispatchEventWithTypeDeserializer::new`
- `guild::member::MemberDeserializer::new`
- `guild::member::MemberListDeserializer::new`
- `guild::member::OptionalMemberDeserializer::new`
- `voice::CloseCodeConversionError::code`

([#824] - [@vivian]).

### Changes

`invite::Invite::target_user_type` has been renamed to `target_type`
([#809] - [@7596ff]).

[#851]: https://github.com/twilight-rs/twilight/pull/851
[#824]: https://github.com/twilight-rs/twilight/pull/824
[#820]: https://github.com/twilight-rs/twilight/pull/820
[#812]: https://github.com/twilight-rs/twilight/pull/812
[#809]: https://github.com/twilight-rs/twilight/pull/809

## [0.4.1] - 2021-05-20

### Additions

Support activity buttons ([#772] - [@vivian]).

[#772]: https://github.com/twilight-rs/twilight/pull/772

## [0.4.0] - 2021-05-12

### Upgrade Path

The MSRV is now Rust 1.49.

Don't reference `GuildStatus`. `Ready::guilds` is now a `Vec<UnavailableGuild>`.

Don't reference `Guild::lazy`.

Errors are no longer enums and don't expose their concrete underlying error
source. You can access the underlying error via the implemented
`std::error::Error::source` method or the `into_parts` or `into_source` methods
on each error struct, which will return a boxed `std::error::Error`. To access
the reason for the error use the `kind` or `into_parts` method on error structs;
the returned error type is an enum with variants for each potential reason the
error occurred.

### Additions

Support `MessageReference::fail_if_not_exists`, which will fail sending a
message if the referenced message does not exist ([#708] - [@7596ff]).

Implement `Ord` for roles based on position and ID ([#762] - [@james7132]).

Add a reworked Allowed Mentions model and builder, moved from the
`twilight-http` crate ([#760] - [@7596ff]).

### Changes

Remove the `guild::GuildStatus` enum because guilds are never online in `Ready`
payloads ([#688] - [@vivian]).

`MessageUpdate::mentions` is now a `Vec<Mention>` instead of a `Vec<User>`
([#699] - [@chamburr]).

Remove the `Guild::lazy` field ([#724] - [@7596ff]).

[#762]: https://github.com/twilight-rs/twilight/pull/762
[#724]: https://github.com/twilight-rs/twilight/pull/724
[#708]: https://github.com/twilight-rs/twilight/pull/708
[#699]: https://github.com/twilight-rs/twilight/pull/699
[#688]: https://github.com/twilight-rs/twilight/pull/688

## [0.3.7] - 2021-04-27

### Additions

Support the `MemberUpdate::{deaf, mute}` fields ([#774] - [@7596ff]).

Support guild stage channels, add `REQUEST_TO_SPEAK` permission
([#793] - [@james7132]).

Support the `USE_SLASH_COMMANDS` permission ([#794] - [@james7132]).

[#794]: https://github.com/twilight-rs/twilight/pull/794
[#793]: https://github.com/twilight-rs/twilight/pull/793
[#774]: https://github.com/twilight-rs/twilight/pull/774

## [0.3.6] - 2021-04-22

### Upgrade Path

Handle the newly optional `AuditLogEntry::user_id` and
`Attachment::{height, width}` fields.

Don't use the `SYSTEM` user flag variant or the `Sticker::preview_asset` field.

### Additions

Support `VoiceChannel::video_quality_mode` to denote the streamed quality mode
([#778] - [@vivian]).

Support `VoiceChannel::rtc_region` ([#779] - [@vivian]).

Support `Guild::nsfw` ([#775] - [@7596ff]).

Support `Attachment::content_type` ([#773] - [@7596ff]).

Support guild templates via the `template` module ([#736] - [@7596ff]).

### Fixes

`AuditLogEntry::user_id` is now wrapped in an `Option` due to a Discord API
change ([#769] - [@vivian]).

`Attachment::height` and `Attachment::width` are now wrapped in an `Option` due
to a Discord API change ([#776] - [@7596ff]).

Remove the `UserFlag::SYSTEM` variant due to a Discord API change
([#777] - [@7596ff]).

Remove the `Sticker::preview_asset` field due to a Discord API change
([#781] - [@7596ff]).

[#781]: https://github.com/twilight-rs/twilight/pull/781
[#779]: https://github.com/twilight-rs/twilight/pull/779
[#778]: https://github.com/twilight-rs/twilight/pull/778
[#777]: https://github.com/twilight-rs/twilight/pull/777
[#776]: https://github.com/twilight-rs/twilight/pull/776
[#775]: https://github.com/twilight-rs/twilight/pull/775
[#773]: https://github.com/twilight-rs/twilight/pull/773
[#769]: https://github.com/twilight-rs/twilight/pull/769
[#736]: https://github.com/twilight-rs/twilight/pull/736

## [0.3.5] - 2021-04-12

### Additions

Support guild discovery grace period message types ([#750] - [@7596ff]).

Support guild invite reminder message type ([#753] - [@tbnritzdoge]).

[#753]: https://github.com/twilight-rs/twilight/pull/753
[#750]: https://github.com/twilight-rs/twilight/pull/750

## [0.3.4] - 2021-04-04

### Additions

Support stage voice channel types ([#748] - [@vivian]).

[#748]: https://github.com/twilight-rs/twilight/pull/748

## [0.3.3] - 2021-03-14

### Additions

Add more audit log keys ([#709] - [@7596ff]).

### Fixes

Add a `#[serde(default)]` on `MemberUpdate::pending` ([#713] - [@AsianIntel] & [@Gelbpunkt]).

Don't omit sending activities field if `None` ([#725] - [@kotx]).

[#709]: https://github.com/twilight-rs/twilight/pull/709
[#713]: https://github.com/twilight-rs/twilight/pull/713
[#725]: https://github.com/twilight-rs/twilight/pull/725

## [0.3.2] - 2021-01-19

### Additions

Support invite welcome screens ([#677] - [@7596ff]).

Support the member pending feature ([#654] - [@AsianIntel]).

[#677]: https://github.com/twilight-rs/twilight/pull/677
[#676]: https://github.com/twilight-rs/twilight/pull/676
[#654]: https://github.com/twilight-rs/twilight/pull/654

## [0.3.1] - 2021-01-11

### Additions

Add the `proxy_url` field to `EmbedVideo` ([#767] - [@7596ff]).

[#676]: https://github.com/twilight-rs/twilight/pull/676

## [0.3.0] - 2021-01-08

### Upgrade Path

`channel::Message::mentions` now contains `channel::message::Mention`s instead
of `User`s, which is like a `User` but with an additional partial `member` field
([#609] - [@vivian]).

The following fields are now Vecs instead of HashMaps:

- `channel::Message::mentions`
- `gateway::payload::GuildEmojisUpdate::emojis`
- `gateway::payload::MemberChunk::members`
- `gateway::payload::MemberChunk::presences`
- `gateway::payload::Ready::guilds`
- `guild::Guild::channels`
- `guild::Guild::emojis`
- `guild::Guild::members`
- `guild::Guild::presences`
- `guild::Guild::roles`
- `guild::Guild::voice_states`
- `guild::PartialGuild::emojis`
- `guild::PartialGuild::roles`
- `user::Connection::integrations`

([#659] - [@vivian]).

### Additions

`guild::PartialMember` now contains an optional `premium_since` field
([#609] - [@vivian]).

`guild::audit_log::AuditLogChangeKey` contains new variants:

- `EnableEmoticons`
- `ExpireBehavior`
- `ExpireGracePeriod`
- `RateLimitPerUser`
- `SystemChannelId`

([#663] - [@jazevedo620]).

### Changes

`channel::Message`'s `mentions` now contains a sequence of `Mention`s, which are
users with partial member information in them when available
([#609] - [@vivian]).

`guild::audit_log::AuditLogChangeKey` is now non-exhaustive
([#663] - [@jazevedo620]).

[#663]: https://github.com/twilight-rs/twilight/pull/663
[#659]: https://github.com/twilight-rs/twilight/pull/659
[#609]: https://github.com/twilight-rs/twilight/pull/609

## [0.2.8] - 2021-01-05

### Fixes

Skip serializing fields when the source field is undefined when None
([#641] - [@chamburr]).

Make the `AuditLog` fields public ([#662] - [@jazevedo620]).

[#662]: https://github.com/twilight-rs/twilight/pull/662
[#641]: https://github.com/twilight-rs/twilight/pull/641

## [0.2.7] - 2020-12-30

### Fixes

Add a `serde` `Visitor::visit_unit` implementation for `RoleTags` to fix
deserialization with `simd-json` ([#648] - [@vivian]).

[#648]: https://github.com/twilight-rs/twilight/pull/648

## [0.2.6] - 2020-12-19

### Additions

Support [Role Tags] ([#638] - [@vivian]).

[#638]: https://github.com/twilight-rs/twilight/pull/638
[Role Tags]: https://github.com/discord/discord-api-docs/commit/7113ceebd549cdf62f286ee57d4ea69af21031e5

## [0.2.5] - 2020-12-18

The MSRV is now set to Rust 1.48.

### Enhancements

The `request::user::get_current_user_guilds::CurrentUserGuild` type has been
moved to `twilight_model::user::CurrentUserGuild`. A re-export has been left
in its place ([#625] - [@AsianIntel]).

### Misc.

Replace documentation links with intra-doc links ([#524] - [@nickelc]).

## [0.2.4] - 2020-11-29

### Additions

Expose `channel::permission_overwrite::PermissionOverwriteTargetType`, which is
a repr enum mapping the "role" and "member" variants to their integer values
([#614] - [@vivian]).

Add message type variant `Reply` mapping to a value of 19. Additionally, add
`Message::referenced_message`, containing the message replied to, if any
([#604] - [@Erk-]).

### Fixes

Correct the name of `IdentityInfo::compression` to `compress`
([#624] - [@chamburr]).

### Changes

`MessageReference::channel_id` is now optional ([#604] - [@Erk-]).

## [0.2.3] - 2020-11-25

### Additions

Support the Message Stickers feature ([#608], [#622] - [@chamburr], [@vivian]).

## [0.2.2] - 2020-11-20

### Fixes

Create a new trimmed down channel type for embedded use in invites
([#601] - [@sam-kirby]).

## [0.2.1] - 2020-11-11

### Additions

Add gateway and voice close codes and voice opcodes ([#586] - [@chamburr]).

### Enhancements

Document gateway opcode variants ([#586] - [@chamburr]).

## [0.2.0] - 2020-10-30

This version of the crate includes changes needed to support version 8 of the
Discord Gateway and HTTP APIs.

### Additions

Implement [Application Integrations][0.2.0-beta.1:app integrations]
([#549], [#579] - [@Erk-]). This adds the
`guild::GuildIntegration::{application, revoked, subscriber_count}`
fields and `guild::IntegrationApplication` type.

Add the `channel::FollowedChannel` struct to include support for the Followed
Channels API feature ([#556] - [@Gelbpunkt]).

Add the fields `flags`, `locale`, `premium_type`, and `public_flags` to
`user::CurrentUser` ([#565] - [@DusterTheFirst]).

### Changes

The following fields have been removed:
- `gateway::presence::Presence::{game, nick}`
- `gateway::payload::PresenceUpdate::{nick, premium_since, roles}`
- `guild::Guild::{embed_channel_id, embed_enabled}`
- `guild::PartialGuild::{embed_channel_id, embed_enabled}`

To match the removal of the `gateway::presence::Presence::game` field, the
`gateway::payload::update_status_info::UpdateStatus::new` method now takes a
list of activities instead of a single activity.

To match the gateway's required intent changes, the following intent-related
fields are now non-optional:
- `gateway::payload::identify::IdentifyInfo::intents`

The following fields no longer (de)serialize with the `_new` suffix:
- `channel::permission_overwrite::PermissionOverwrite::{allow, deny}`

This means that `PermissionOverwrite::allow` (de)serializes to/from "allow" and
and `PermissionOverwrite::deny` (de)serializes to/from "deny".

Similarly, the following permissions fields now (de)serialize to/from
"permissions" instead of "permissions_new":
- `guild::Guild::permissions`
- `guild::PartialGuild::permissions`
- `guild::Role::permissions`

([#532] - [@vivian])

Make `user::CurrentUser::verified` optional to support OAuth 2.0 Bearer requests
without the `email` scope ([#564] - [@DusterTheFirst]).

Correct `oauth::CurrentApplicationInfo::id`'s type from a `UserId` to an
`ApplicationId` ([#569] - [@DusterTheFirst]).

## [0.2.0-beta.2] - 2020-10-22

### Additions

Add the `channel::FollowedChannel` struct to include support for the Followed
Channels API feature ([#556] - [@Gelbpunkt]).

## [0.2.0-beta.1] - 2020-10-17

### Additions

Implement [Application Integrations][0.2.0-beta.1:app integrations]
([#549] - [@Erk-]). This adds the
`guild::GuildIntegration::{application, revoked, subscriber_count}`
fields and `guild::IntegrationApplication` type.

## [0.2.0-beta.0] - 2020-10-10

This beta version of major version 0.2 of the crate includes changes needed to
support version 8 of the Discord Gateway and HTTP APIs.

### Changes

All changes in this version are from PR [#532].

The following fields have been removed:
- `gateway::presence::Presence::{game, nick}`
- `gateway::payload::PresenceUpdate::{nick, premium_since, roles}`
- `guild::Guild::{embed_channel_id, embed_enabled}`
- `guild::PartialGuild::{embed_channel_id, embed_enabled}`

To match the removal of the `gateway::presence::Presence::game` field, the
`gateway::payload::update_status_info::UpdateStatus::new` method now takes a
list of activities instead of a single activity.

To match the gateway's required intent changes, the following intent-related
fields are now non-optional:
- `gateway::payload::identify::IdentifyInfo::intents`

The following fields no longer (de)serialize with the `_new` suffix:
- `channel::permission_overwrite::PermissionOverwrite::{allow, deny}`

This means that `PermissionOverwrite::allow` (de)serializes to/from "allow" and
and `PermissionOverwrite::deny` (de)serializes to/from "deny".

Similarly, the following permissions fields now (de)serialize to/from
"permissions" instead of "permissions_new":
- `guild::Guild::permissions`
- `guild::PartialGuild::permissions`
- `guild::Role::permissions`

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
[@A5rocks]: https://github.com/A5rocks
[@AEnterprise]: https://github.com/AEnterprise
[@AsianIntel]: https://github.com/AsianIntel
[@baptiste0928]: https://github.com/baptiste0928
[@BlackHoleFox]: https://github.com/BlackHoleFox
[@chamburr]: https://github.com/chamburr
[@coadler]: https://github.com/coadler
[@dnaka91]: https://github.com/dnaka91
[@DusterTheFirst]: https://github.com/DusterTheFirst
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@HTG-YT]: https://github.com/HTG-YT
[@itohatweb]: https://github.com/itohatweb
[@james7132]: https://github.com/james7132
[@jazevedo620]: https://github.com/jazevedo620
[@kotx]: https://github.com/kotx
[@laralove143]: https://github.com/laralove143
[@LeSeulArtichaut]: https://github.com/LeSeulArtichaut
[@Liamolucko]: https://github.com/Liamolucko
[@MaxOhn]: https://github.com/MaxOhn
[@nickelc]: https://github.com/nickelc
[@PyroTechniac]: https://github.com/PyroTechniac
[@sam-kirby]: https://github.com/sam-kirby
[@tbnritzdoge]: https://github.com/tbnritzdoge
[@tomocrafter]: https://github.com/tomocrafter
[@vilgotf]: https://github.com/vilgotf
[@vivian]: https://github.com/vivian
[@zeylahellyer]: https://github.com/zeylahellyer

[#625]: https://github.com/twilight-rs/twilight/pull/625
[#624]: https://github.com/twilight-rs/twilight/pull/624
[#622]: https://github.com/twilight-rs/twilight/pull/622
[#614]: https://github.com/twilight-rs/twilight/pull/614
[#608]: https://github.com/twilight-rs/twilight/pull/608
[#604]: https://github.com/twilight-rs/twilight/pull/604
[#601]: https://github.com/twilight-rs/twilight/pull/601
[#586]: https://github.com/twilight-rs/twilight/pull/586
[#579]: https://github.com/twilight-rs/twilight/pull/579
[#569]: https://github.com/twilight-rs/twilight/pull/569
[#565]: https://github.com/twilight-rs/twilight/pull/565
[#564]: https://github.com/twilight-rs/twilight/pull/564
[#556]: https://github.com/twilight-rs/twilight/pull/556
[#549]: https://github.com/twilight-rs/twilight/pull/549
[#532]: https://github.com/twilight-rs/twilight/pull/532
[#526]: https://github.com/twilight-rs/twilight/pull/526
[#524]: https://github.com/twilight-rs/twilight/pull/524
[#511]: https://github.com/twilight-rs/twilight/pull/511
[#509]: https://github.com/twilight-rs/twilight/pull/509
[#499]: https://github.com/twilight-rs/twilight/pull/499

[0.2.0-beta.1:app integrations]: https://github.com/discord/discord-api-docs/commit/a926694e2f8605848bda6b57d21c8817559e5cec

[0.11.0]: https://github.com/twilight-rs/twilight/releases/tag/model-0.11.0
[0.10.3]: https://github.com/twilight-rs/twilight/releases/tag/model-0.10.3
[0.10.2]: https://github.com/twilight-rs/twilight/releases/tag/model-0.10.2
[0.10.1]: https://github.com/twilight-rs/twilight/releases/tag/model-0.10.1
[0.10.0]: https://github.com/twilight-rs/twilight/releases/tag/model-0.10.0
[0.9.2]: https://github.com/twilight-rs/twilight/releases/tag/model-0.9.2
[0.9.1]: https://github.com/twilight-rs/twilight/releases/tag/model-0.9.1
[0.9.0]: https://github.com/twilight-rs/twilight/releases/tag/model-0.9.0
[0.8.5]: https://github.com/twilight-rs/twilight/releases/tag/model-0.8.5
[0.8.4]: https://github.com/twilight-rs/twilight/releases/tag/model-0.8.4
[0.8.3]: https://github.com/twilight-rs/twilight/releases/tag/model-0.8.3
[0.8.2]: https://github.com/twilight-rs/twilight/releases/tag/model-0.8.2
[0.8.1]: https://github.com/twilight-rs/twilight/releases/tag/model-0.8.1
[0.8.0]: https://github.com/twilight-rs/twilight/releases/tag/model-0.8.0
[0.7.3]: https://github.com/twilight-rs/twilight/releases/tag/model-0.7.3
[0.7.2]: https://github.com/twilight-rs/twilight/releases/tag/model-0.7.2
[0.7.1]: https://github.com/twilight-rs/twilight/releases/tag/model-0.7.1
[0.7.0]: https://github.com/twilight-rs/twilight/releases/tag/model-0.7.0
[0.6.5]: https://github.com/twilight-rs/twilight/releases/tag/model-0.6.5
[0.6.4]: https://github.com/twilight-rs/twilight/releases/tag/model-0.6.4
[0.6.3]: https://github.com/twilight-rs/twilight/releases/tag/model-0.6.3
[0.6.2]: https://github.com/twilight-rs/twilight/releases/tag/model-0.6.2
[0.6.1]: https://github.com/twilight-rs/twilight/releases/tag/model-0.6.1
[0.5.4]: https://github.com/twilight-rs/twilight/releases/tag/model-0.5.4
[0.5.3]: https://github.com/twilight-rs/twilight/releases/tag/model-0.5.3
[0.5.2]: https://github.com/twilight-rs/twilight/releases/tag/model-0.5.2
[0.5.1]: https://github.com/twilight-rs/twilight/releases/tag/model-0.5.1
[0.5.0]: https://github.com/twilight-rs/twilight/releases/tag/model-0.5.0
[0.4.3]: https://github.com/twilight-rs/twilight/releases/tag/model-0.4.3
[0.4.2]: https://github.com/twilight-rs/twilight/releases/tag/model-0.4.2
[0.4.1]: https://github.com/twilight-rs/twilight/releases/tag/model-0.4.1
[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/model-0.4.0
[0.3.7]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.3.7
[0.3.5]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.3.5
[0.3.4]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.3.4
[0.3.3]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.3.3
[0.3.2]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.3.2
[0.3.1]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.3.1
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.3.0
[0.2.8]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.2.8
[0.2.7]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.2.7
[0.2.6]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.2.6
[0.2.5]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.2.5
[0.2.4]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.2.4
[0.2.3]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.2.3
[0.2.2]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.2.2
[0.2.1]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.2.1
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.2.0
[0.2.0-beta.2]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.2.0-beta.2
[0.2.0-beta.1]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.2.0-beta.1
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.2.0-beta.0
[0.1.3]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.3
[0.1.2]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.2
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
