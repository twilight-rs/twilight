# Changelog

Changelog for `twilight-model`.

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

[@chamburr]: https://github.com/chamburr
[@coadler]: https://github.com/coadler
[@DusterTheFirst]: https://github.com/DusterTheFirst
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@sam-kirby]: https://github.com/sam-kirby
[@vivian]: https://github.com/vivian

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
[#511]: https://github.com/twilight-rs/twilight/pull/511
[#509]: https://github.com/twilight-rs/twilight/pull/509
[#499]: https://github.com/twilight-rs/twilight/pull/499

[0.2.0-beta.1:app integrations]: https://github.com/discord/discord-api-docs/commit/a926694e2f8605848bda6b57d21c8817559e5cec

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
