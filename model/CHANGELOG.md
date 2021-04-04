# Changelog

Changelog for `twilight-model`.

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
[@AsianIntel]: https://github.com/AsianIntel
[@DusterTheFirst]: https://github.com/DusterTheFirst
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@chamburr]: https://github.com/chamburr
[@coadler]: https://github.com/coadler
[@jazevedo620]: https://github.com/jazevedo620
[@kotx]: https://github.com/kotx
[@nickelc]: https://github.com/nickelc
[@sam-kirby]: https://github.com/sam-kirby
[@vivian]: https://github.com/vivian

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
