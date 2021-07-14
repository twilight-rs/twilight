# Changelog

Changelog for `twilight-model`.

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
[@AsianIntel]: https://github.com/AsianIntel
[@BlackHoleFox]: https://github.com/BlackHoleFox
[@chamburr]: https://github.com/chamburr
[@coadler]: https://github.com/coadler
[@DusterTheFirst]: https://github.com/DusterTheFirst
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@james7132]: https://github.com/james7132
[@jazevedo620]: https://github.com/jazevedo620
[@kotx]: https://github.com/kotx
[@nickelc]: https://github.com/nickelc
[@sam-kirby]: https://github.com/sam-kirby
[@tbnritzdoge]: https://github.com/tbnritzdoge
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
