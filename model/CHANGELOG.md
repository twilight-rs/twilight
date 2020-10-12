# Changelog

Changelog for `twilight-model`.

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

[@coadler]: https://github.com/coadler
[@Erk-]: https://github.com/Erk-
[@vivian]: https://github.com/vivian

[#532]: https://github.com/twilight-rs/twilight/pull/532
[#526]: https://github.com/twilight-rs/twilight/pull/526
[#511]: https://github.com/twilight-rs/twilight/pull/511
[#509]: https://github.com/twilight-rs/twilight/pull/509
[#499]: https://github.com/twilight-rs/twilight/pull/499

[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.2.0-beta.0
[0.1.3]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.3
[0.1.2]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.2
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/model-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
