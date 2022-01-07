# Changelog

Changelog for `twilight-http`.

## [0.8.3] - 2021-12-27

### Additions

Support guild member timeouts via
`UpdateGuildMember::communication_disabled_until` ([#1342] - [@HTG-YT]).

Support pre-flight cancellation to cancel requests after passing ratelimit
queues but before sending ([#1353] - [@zeylahellyer]).

### Fixes

Fix display implementation for Get Active Threads route
([#1386] - [@zeylahellyer]).

Fix display implementation for Get Current User Application Info route
([#1389] - [@Erk-]).

[#1389]: https://github.com/twilight-rs/twilight/pull/1389
[#1386]: https://github.com/twilight-rs/twilight/pull/1386
[#1353]: https://github.com/twilight-rs/twilight/pull/1353
[#1342]: https://github.com/twilight-rs/twilight/pull/1342

## [0.8.2] - 2021-12-24

### Additions

Add the API error code `50109 RequestInvalidJson` ([#1338] - [@vilgotf]).

[#1326]: https://github.com/twilight-rs/twilight/pull/1326

## [0.8.1] - 2021-12-15

### Additions

Add the API error codes `20029 WriteActionsReached` and `50055 InvalidGuild`
([#1326] - [@zeylahellyer]).

### Changes

Only send the interaction authorization token when responding to interactions
([#1317] - [@zeylahellyer]).

### Fixes

Fix an issue where interaction and webhook tokens were invalidating the entire
client ([#1318] - [@zeylahellyer]). The client now only invalidates a token if
the request that failed was actually using it.

`CommandBorrowed::kind` is now properly serialized as `type` ([#1323] -
[@7596ff]).

Fix a variety of routes in `RouteDisplay` after adding tests ([#1327] -
[@zeylahellyer]). The fixed routes are `CreateGuildPrune`,
`SyncGuildIntegration`, and `UpdateUserVoiceState`.

[#1317]: https://github.com/twilight-rs/twilight/pull/1317
[#1318]: https://github.com/twilight-rs/twilight/pull/1318
[#1323]: https://github.com/twilight-rs/twilight/pull/1323
[#1326]: https://github.com/twilight-rs/twilight/pull/1326
[#1327]: https://github.com/twilight-rs/twilight/pull/1327

## [0.8.0] - 2021-12-03

### Additions

Add a feature, `trust-dns`, which uses `hyper-trust-dns` to use a fully
async and featured DNS resolver ([#1310] - [@Gelbpunkt]).

Add the `ErrorType::RatelimiterTicket` variant ([#1191] - [@Gelbpunkt]).

### Changes

`ClientBuilder::ratelimiter` now accepts an `Option<Box<dyn
Ratelimiter>>`, which is a trait provided by the `http-ratelimiting`
crate ([#1191] - [@Gelbpunkt]). Additionally, `routing::{Path,
PathParseError, PathParseErrorType}` and `request::Method` have been
moved to `http-ratelimiting`, with a re-export that will remain for one
major version.

`tracing` is now enabled by default ([#1203] - [@Gelbpunkt]).

`CreateThread` and `CreateThreadFromMessage` no longer accept an
`AutoArchiveDuration` as a default parameter ([#1256] - [@7596ff]). This
can still be set with a method on the request builder.

`http::prelude` has been removed ([#1273] - [@7596ff]).

`CreateFollowupMessage::{avatar_url, username}` have been removed
([#1287] - [@itohatweb]).

### Dependency Updates

`hyper-rustls` has been updated to `0.23` ([#1276] - [@Gelbpunkt]).

[#1191]: https://github.com/twilight-rs/twilight/pull/1191
[#1203]: https://github.com/twilight-rs/twilight/pull/1203
[#1256]: https://github.com/twilight-rs/twilight/pull/1256
[#1273]: https://github.com/twilight-rs/twilight/pull/1273
[#1276]: https://github.com/twilight-rs/twilight/pull/1276
[#1287]: https://github.com/twilight-rs/twilight/pull/1287
[#1310]: https://github.com/twilight-rs/twilight/pull/1310

## [0.7.3] - 2021-12-03

### Additions

`DeleteWebhookMessage`, `UpdateWebhookMessage`, and `GetWebhookMessage` now
support targeting a `thread_id` instead of the channel itself. `ExecuteWebhook`
already had a method to do this, but it was not functioning until now ([#1286],
[#1311] - [@7596ff]]).

Added some missing error codes ([#1291] - [@itohatweb]).

[#1286]: https://github.com/twilight-rs/twilight/pull/1286
[#1291]: https://github.com/twilight-rs/twilight/pull/1291
[#1311]: https://github.com/twilight-rs/twilight/pull/1311

## [0.7.2] - 2021-11-20

### Additions

Add `Client::update_current_member`, and deprecate
`Client::update_current_user_nick` ([#1253] - [@7596ff]).

Add `Client::thread_member`, which gets a member of a thread ([#1258] -
[@7596ff]).

### Changes

Add the `attach` function to many create-message-like methods, which accepts a
list of `AttachmentFile`s ([#1206] - [@Erk-]). Additionally, deprecate the
`files` method, since it could not support adding descriptions to attachments.

Deprecate `http::prelude` ([#1257], [#1265] - [@7595ff]).

Deprecate `CreateFollowupMessage::{avatar_url, username}`, as they did not have
an effect ([#1289] - [@itohatweb]).

[#1206]: https://github.com/twilight-rs/twilight/pull/1206
[#1253]: https://github.com/twilight-rs/twilight/pull/1253
[#1257]: https://github.com/twilight-rs/twilight/pull/1257
[#1258]: https://github.com/twilight-rs/twilight/pull/1258
[#1265]: https://github.com/twilight-rs/twilight/pull/1265

## [0.7.1] - 2021-10-29

### Additions

`CreateRole` and `UpdateRole` now support setting either the `icon`
(image data) or the `unicode_emoji` fields ([#1212] - [@7596ff]).

Add new API error codes: `MaximumGuildWidgets`, `ServerNeedsBoosts`, and
`CannotReplyWithoutMessageHistory` ([#1215] - [@7596ff]).

### Changes

Fixes some spelling errors in documentation ([#1223] - [@7596ff]).

### Fixes

Add a missing `?` to `GetPublicArchivedThreads` ([#1214] -
[@cherryblossom000]).

Actually serialize the JSON body on `UpdateThread` ([#1218] -
[@7596ff]).

[#1212]: https://github.com/twilight-rs/twilight/pull/1212
[#1214]: https://github.com/twilight-rs/twilight/pull/1214
[#1215]: https://github.com/twilight-rs/twilight/pull/1215
[#1218]: https://github.com/twilight-rs/twilight/pull/1218
[#1223]: https://github.com/twilight-rs/twilight/pull/1223

## [0.7.0] - 2021-10-21

### Changes

The MSRV has been updated to 1.53 ([#1161] - [@7596ff]).

Rework the header API ([#1066] - [@zeylahellyer]) by taking an iterator
of key-value header pairs instead of an `http` crate `HeaderMap`,
removing the ratelimit error types in favor of header parsing error
types, and providing additional information about how parsing failed.
See the PR for more details.

`Client` no longer implements `Clone`, because it is no longer
internally wrapped in an `Arc` ([#1067] - [@zeylahellyer]). To retain
this functionality, you can wrap them it an `Arc` or a `Rc` manually.

`Client::{new_create_global_command, new_create_guild_command}` have
been renamed to `create_global_command` and `create_guild_command`
respectively, and their previous implementations have been removed.

Deprecated re-exports of application request builders have been removed
([#1193] - [@7596ff]).

`Ratelimiter::get` has been removed in favor of `ticket` ([#1195] -
[@7596ff]).

[#1066]: https://github.com/twilight-rs/twilight/pull/1066
[#1067]: https://github.com/twilight-rs/twilight/pull/1067
[#1161]: https://github.com/twilight-rs/twilight/pull/1161
[#1193]: https://github.com/twilight-rs/twilight/pull/1193
[#1195]: https://github.com/twilight-rs/twilight/pull/1195

## [0.6.6] - 2021-10-07

### Additions

Adds decompression support. This is enabled by default. The feature is
named `decompression`. This pulls in the [`brotli`] dependency ([#1104]
- [@vilgotf]).

When this feature is enabled, the variant
`DeserializeBodyErrorKind::Decompressing` is added.

 Adds the following `Client` methods: `create_guild_sticker`,
 `delete_guild_sticker`, `guild_sticker`, `guild_stickers`,
 `nitro_sticker_packs`, `sticker`, and `update_guild_sticker` ([#1157] -
 [@7596ff]).

### Changes

`UpdateCurrentUser` now implements `AuditLogReason` ([#1184] - [@Erk-]).

[`brotli`]: https://crates.io/crates/brotli

[#1104]: https://github.com/twilight-rs/twilight/pull/1104
[#1157]: https://github.com/twilight-rs/twilight/pull/1157
[#1184]: https://github.com/twilight-rs/twilight/pull/1184

## [0.6.5] - 2021-09-17

### Thread Support

The http API version has been updated to `v9`.

9 new error codes have been added: `GuildPremiumTooLow`,
`MaxActiveAnnouncementThreads`, `MaxActiveThreads`,
`ThreadAlreadyCreated`, `ThreadArchived`, `ThreadInvalidBeforeValue`,
`ThreadInvalidNotificationSettings`, `ThreadLocked`, and
`ThreadMaxParticipants`.

12 new HTTP requests have been added: `AddThreadMember`, `CreateThread`,
`CreateThreadFromMessage`, `GetJoinedPrivateArchivedThreads`,
`GetPrivateArchivedThreads`, `GetPublicArchivedThreads`,
`GetThreadMembers`, `JoinThread`, `LeaveThread`, `RemoveThreadMember`,
`ThreadValidationError`, and `UpdateThread`.

`ExecuteWebhook` allows setting a `thread_id` parameter, which sends the
payload to the thread instead.

## [0.6.4] - 2021-09-17

### Additions

Support creating of Message and User application commands, via the
methods `new_create_global_command` and `new_create_guild_command`.
These will replace the methods `create_global_command` and
`create_guild_command` in the next major version ([#1107] - [@7596ff]).

Support the "Get Global Command" and "Get Guild Command" requests, which
can be used to get a full command object ([#1107] - [@7596ff]).

Support the "Get Followup Message" request, which can be used to
retrieve one of the followup messages created for an application
interaction ([#1133] - [@zeylahellyer]).

### Fixes

Fix links to Discord documentation that pointed to the old `slash`
section ([#1107] - [@7596ff]).

### Changes

Application-related HTTP request builders have been separated into
`command` and `interaction` modules, for ease of understanding ([#1107]
- [@7596ff]).

[#1131]: https://github.com/twilight-rs/twilight/pull/1131
[#1133]: https://github.com/twilight-rs/twilight/pull/1133

## [0.6.3] - 2021-08-30

### Additions

Support message components, including action rows, buttons, and select menus
([#1020], [#1043], [#1044], [#1090], aggregate [#1121] - [@AEnterprise],
[@AsianIntel], [@zeylahellyer], [@7596ff]).

Add comparing `StatusCode` with `u16` ([#1131] - [@zeylahellyer]).

Add API error code 30040, described as "Maximum number of prune requests has
been reached. Try again later" ([#1125] - [@zeylahellyer]).

### Enhancements

Document that `tracing` is now disabled by default ([#1129] - [@zeylahellyer]).

Add `Response<ListBody<T>>::model` and `Response<MemberListBody>::model` aliases
corresponding to their `models` equivalents ([#1123] - [@zeylahellyer]).

Display body parsing errors as a legible string if they're UTF-8 valid
([#1118] - [@AEnterprise]).

[#1131]: https://github.com/twilight-rs/twilight/pull/1131
[#1129]: https://github.com/twilight-rs/twilight/pull/1129
[#1125]: https://github.com/twilight-rs/twilight/pull/1125
[#1123]: https://github.com/twilight-rs/twilight/pull/1123
[#1121]: https://github.com/twilight-rs/twilight/pull/1121
[#1120]: https://github.com/twilight-rs/twilight/pull/1120
[#1118]: https://github.com/twilight-rs/twilight/pull/1118
[#1090]: https://github.com/twilight-rs/twilight/pull/1090
[#1044]: https://github.com/twilight-rs/twilight/pull/1044
[#1043]: https://github.com/twilight-rs/twilight/pull/1043
[#1020]: https://github.com/twilight-rs/twilight/pull/1020

## [0.6.2] - 2021-08-18

### Additions

Add 6 new HTTP API error codes:

- 10049: Unknown stream
- 10050: Unknown premium server subscribe cooldown
- 10070: Unknown Guild Scheduled Event
- 10071: Unknown Guild Scheduled Event User
- 50095: This server is not available in your location
- 50097: This server needs monetization enabled in order to perform this action

([#1094] - [@Erk-]).

### Fixes

Fix `ResponseFuture` returning mismatched `RequestTimedOut` and `RequestError`
error type variants ([#1100] - [@vilgotf]).

### Enhancements

Improve the performance of `Response::bytes` ([#1103] - [@vilgotf]).

Add `[#must_use]` to typed HTTP request builders ([#1099] - [@zeylahellyer]).

[#1103]: https://github.com/twilight-rs/twilight/pull/1103
[#1100]: https://github.com/twilight-rs/twilight/pull/1100
[#1099]: https://github.com/twilight-rs/twilight/pull/1099
[#1094]: https://github.com/twilight-rs/twilight/pull/1094

## [0.6.1] - 2021-08-01

### Additions

Adds `RequestBuilder::raw` back in, hiding the existing fields behind
methods due to the type of one of the public fields being inaccurate in
the first place. This is considered a hotfix ([#1084] -
[@zeylahellyer]).

BREAKING CHANGES: `request::Request` fields are now private and behind
getters, `request::RequestBuilder` and `request::Request` initialization
methods now take route references.

Adds `ClientBuilder::remember_invalid_token`. By default we remember
when the HTTP client encounters an invalid token (via the 401 response
status). When one is encountered, all future requests are
short-circuited and return a 401 ([#1085] - [@zeylahellyer]).

This option allows a user to disable this functionality and to always
continue with a request, even if past ones have encountered a 401
response.

[#1084]: https://github.com/twilight-rs/twilight/pull/1084
[#1085]: https://github.com/twilight-rs/twilight/pull/1085

## [0.6.0] - 2021-07-31

### Enhancements

Many functions have been made constant ([#1010] - [@zeylahellyer]).

### Changes

There are significant changes to how users make HTTP requests. When
users make a request, they must pass borrowed types instead of owned
types. To execute the request, users must call `exec` on the request
builder. Once the request has completed execution, users may use the
`ResponseFuture` struct methods to access the status code of the
request. To access a returned model, if there is one, users must call
`model` on the response.

A call to `Client::create_message` like this:

```rust
client.create_message(ChannelId(1))
    .content("some content")?
    .embed(Embed {})?
    .await?;
```

is now written like this:

```rust
client.create_message(ChannelId(1))
    .content(&"some content")?
    .embeds(&[&Embed {}])?
    .exec()
    .await?
    .model()
    .await?;
```

For more information on the motivation behind these changes, see the PR
descriptions of [#923], [#1008], and [#1009]. These changes were
authored by [@zeylahellyer].

Rename `ErrorCode::UnallowedWordsForPublicStage` variant to
`UnallowedWords` ([#956] - [@7596ff])

`CreateGlobalCommand`, `CreateGuildCommand`, `SetGlobalCommands`, and
`SetGuildCommands` now return command(s) ([#1037] - [@vilgotf]).

A few spelling errors have been fixed by adding the `codespell` Action
([#1041] - [@Gelbpunkt].

[#923]: https://github.com/twilight-rs/twilight/pull/923
[#956]: https://github.com/twilight-rs/twilight/pull/956
[#1008]: https://github.com/twilight-rs/twilight/pull/1008
[#1009]: https://github.com/twilight-rs/twilight/pull/1009
[#1010]: https://github.com/twilight-rs/twilight/pull/1010
[#1037]: https://github.com/twilight-rs/twilight/pull/1037
[#1041]: https://github.com/twilight-rs/twilight/pull/1041

## [0.5.7] - 2021-07-23

### Changes

`#![deny(unsafe_code)]` has been added, ensuring no unsafe code exists in the
crate ([#1042] - [@zeylahellyer]).

### Fixes

The `JSON` body is now actually serialized on the `update_channel` route
([#1051] - [@Learath2]).

[#1042]: https://github.com/twilight-rs/twilight/pull/1042
[#1051]: https://github.com/twilight-rs/twilight/pull/1051

## [0.5.6] - 2021-07-20

### Fixes

Fix the display implementation for `Route::GetGuildMembers` ([#1050] - [@Erk-]).

[#1050]: https://github.com/twilight-rs/twilight/pull/1050

## [0.5.5] - 2021-07-19

### Upgrade Path

Instead of using the `attachment`, `embed`, or `file` methods on requests, use
the `attachments`, `embeds`, or `files` methods.

### Changes

Deprecate requests' `attachment`, `embed`, and `file` methods in favor of their
plural forms ([#1034] - [@zeylahellyer]).

[#1034]: https://github.com/twilight-rs/twilight/pull/1034

## [0.5.4] - 2021-07-14

### Additions

Add `Client::get_interaction_original` ([#1013] - [@SuperiorJT]).

### Changes

Channel name validation has been updated to support a length of 1 character
([#1012] - [@zeylahellyer]).

Embed description validation has been updated to support a length of 4096
characters ([#1024] - [@zeylahellyer]).

The route `Client::update_original_response` now returns a `Message` ([#1023] -
[@AsianIntel]).

[#1012]: https://github.com/twilight-rs/twilight/pull/1012
[#1013]: https://github.com/twilight-rs/twilight/pull/1013
[#1023]: https://github.com/twilight-rs/twilight/pull/1023
[#1024]: https://github.com/twilight-rs/twilight/pull/1024

## [0.5.3] - 2021-07-03

### Additions

Add support for the following API error codes:

- 10059: Unknown discoverable server category
- 10060: Unknown sticker
- 10068: Unknown Guild Member Verification Form
- 10069: Unknown Guild Welcome Screen
- 30030: Maximum number of server categories has been reached
- 30039: Maximum number of stickers reached

([#1006] - [@zeylahellyer]).

[#1006]: https://github.com/twilight-rs/twilight/pull/1006

## [0.5.2] - 2021-07-02

### Fixes

The display formatters for `routing::Route`'s `SetGuildCommands` and
`UpdateGlobalCommand` variants have been fixed, which were resulting in 404 Not
Found errors ([#1005] - [@zeylahellyer]).

[#1005]: https://github.com/twilight-rs/twilight/pull/1005

## [0.5.1] - 2021-07-02

### Additions

Support setting multiple embeds in the `CreateMessage` and `UpdateMessage`
requests via their new `embeds` methods ([#987] - [@7596ff]).

Add a URL display formatter for
`request::channel::reaction::RequestReactionType` returned via the new
`RequestReactionType::display` method ([#967] - [@zeylahellyer]).

Add the ability to create raw requests without a `routing::Route` via the
`request::RequestBuilder::raw` method ([#963] - [@zeylahellyer]).

3 new methods have been added which return the individual components of the
now-deprecated `routing::Route::into_parts` method: `Route::display` which
returns a type implementing `Display` to format the path of a route,
`Route::method` to determine the HTTP method of a route, and `Route::path` to
get the ratelimiting path of a route. This also improves the performance of
the display implementation of `Route::display` over `Route::into_parts`
([#962] - [@zeylahellyer]).

Add the new stage API error codes "Unknown Stage Instance" (10067) and
"Stage Already Open" (150006) ([#955] - [@7596ff]).

### Fixes

Correctly remove the nickname when a `None` nickname is provided in the
`UpdateGuildMember` request ([#949] - [@zeylahellyer]).

Fix the request sending of the `CreateStageInstance` request
([#936] - [@7596ff]).

### Enhancements

Reduce allocations when parsing responses without using `serde_json`
([#994] - [@MaxOhn]).

Improve the `Display` implementation performance of various `Display`
implementations by calling `Formatter` methods directly instead of calling the
`format_args!` and `write!` macros ([#944] - [@zeylahellyer]).

### Changes

`routing::Route::into_parts` has been deprecated in favor of `Route::display`,
`Route::method`, and `Route::path` ([#962] - [@zeylahellyer]).

[#994]: https://github.com/twilight-rs/twilight/pull/994
[#987]: https://github.com/twilight-rs/twilight/pull/987
[#967]: https://github.com/twilight-rs/twilight/pull/967
[#963]: https://github.com/twilight-rs/twilight/pull/963
[#962]: https://github.com/twilight-rs/twilight/pull/962
[#955]: https://github.com/twilight-rs/twilight/pull/955
[#949]: https://github.com/twilight-rs/twilight/pull/949
[#944]: https://github.com/twilight-rs/twilight/pull/944
[#936]: https://github.com/twilight-rs/twilight/pull/936

## [0.5.0] - 2021-06-13

### Upgrade Path

Remove usage of `GetReactions::before`, `ErrorType::Formatting`,
`ErrorType::Ratelimiting`, `CreateMessage::attachment`,
`CreateMessage::attachments`, `CreateGuild::region`, `UpdateGuild::region`, and
`Result`.

Update usage of `CreateGuildFromTemplateError`, `CreateTemplateError`,
`SearchGuildMembersError`, `UpdateTemplateError`.

Replace the following usages:
```diff
-twilight_http::routing::Route::DeleteMessageSpecficReaction
+twilight_http::routing::Route::DeleteMessageSpecificReaction

-twilight_http::request::channel::invite::CreateInvite::target_user
+twilight_http::request::channel::invite::CreateInvite::target_user_id

-twilight_http::request::channel::invite::CreateInvite::target_user_type
+twilight_http::request::channel::invite::CreateInvite::target_type
```

`UpdateStageInstance` requests now look like this:

```diff
-client.update_stage_instance(channel_id, topic)?.await?;
+client.update_stage_instance(channel_id)
+    .topic(topic)?
+    .await?;
```

### Additions

Support for Slash Commands has been added.

The following HTTP requests have been added:

- `create_followup_message`
- `create_global_command`
- `create_guild_command`
- `delete_followup_message`
- `delete_global_command`
- `delete_guild_command`
- `delete_interaction_original`
- `get_command_permissions`
- `get_global_commands`
- `get_guild_command_permissions`
- `get_guild_commands`
- `interaction_callback`
- `set_command_permissions`
- `set_global_commands`
- `set_guild_commands`
- `update_command_permissions`
- `update_followup_message`
- `update_global_command`
- `update_guild_command`
- `update_interaction_original`

### Enhancements

The following HTTP errors have been added:

- `10063` `UnknownApplicationCommand`
- `10066` `UnknownApplicationCommandPermissions`
- `40041` `CommandNameAlreadyExists`

The following HTTP ratelimiting paths have been added:

- `ApplicationCommand(u64)`
- `ApplicationCommandId(u64)`
- `ApplicationGuildCommand(u64)`
- `ApplicationGuildCommandId(u64)`
- `InteractionCallback(u64)`

The following HTTP routes have been added:

`CreateGlobalCommand { application_id }`
`CreateGuildCommand { application_id, guild_id }`
`DeleteGlobalCommand { application_id, command_id }`
`DeleteGuildCommand { application_id, command_id, guild_id }`
`DeleteInteractionOriginal { application_id, interaction_token }`
`GetCommandPermissions { application_id, command_id, guild_id }`
`GetGlobalCommands { application_id }`
`GetGuildCommandPermissions { application_id, guild_id }`
`GetGuildCommands { application_id, guild_id }`
`InteractionCallback { interaction_id, interaction_token }`
`SetCommandPermissions { application_id, guild_id }`
`SetGlobalCommands { application_id }`
`SetGuildCommands { application_id, guild_id }`
`UpdateCommandPermissions { application_id, command_id, guild_id }`
`UpdateGlobalCommand { application_id, command_id }`
`UpdateGuildCommand { application_id, command_id, guild_id }`
`UpdateInteractionOriginal { application_id, interaction_token }`

### Changes

`GetReactions::before`, and its corresponding `Route::GetReactionUsers::before`
has been removed ([#810] - [@7596ff]).

`CreateInvite::{target_user, target_user_type}` have been removed [#847] -
[@7596ff]).

The `Formatting` and `Ratelimiting` HTTP error variants have been removed, as
they are unused ([#854] - [@vivian]).

In `UpdateStageInstance`, the `topic` field is no longer required ([#895] -
[@7596ff]).

The `tracing` dependency is now optional, and enabled by default ([#910] -
[@vivian]).

The following errors have been properly converted to the new `Error` standard
([#898] - [@7596ff]):
- `CreateGuildFromTemplateError`
- `CreateTemplateError`
- `SearchGuildMembersError`
- `UpdateTemplateError`

A typo in the name of `DeleteMessageSpecificReaction` has been fixed ([#927] -
[@vivian]).

`CreateMessage::{attachment, attachments}` have been removed ([#929] -
[@7596ff]).

References to `Guild::region` have been removed. This includes the `region`
method on `CreateGuild` and `UpdateGuild` ([#930] - [@7596ff]).

`Result` has been removed ([#931] - [@7596ff]).

[#810]: https://github.com/twilight-rs/twilight/pull/810
[#847]: https://github.com/twilight-rs/twilight/pull/847
[#854]: https://github.com/twilight-rs/twilight/pull/854
[#895]: https://github.com/twilight-rs/twilight/pull/895
[#898]: https://github.com/twilight-rs/twilight/pull/898
[#910]: https://github.com/twilight-rs/twilight/pull/910
[#927]: https://github.com/twilight-rs/twilight/pull/927
[#929]: https://github.com/twilight-rs/twilight/pull/929
[#930]: https://github.com/twilight-rs/twilight/pull/930
[#931]: https://github.com/twilight-rs/twilight/pull/931

## [0.4.3] - 2021-06-12

### Additions

Added new `ErrorCode`s:

- `20031 UnallowedWordsForPublicStage` ([#886] - [@BlackHoleFox])
- `30018 MaximumAnimatedEmojisReached` ([#918] - [@7596ff])
- `30019 MaximumGuildMembersReached` ([#918] - [@7596ff])

Added new request methods ([#867] - [@7596ff]):

- `CreateStageInstance::privacy_level`
- `UpdateStageInstance::privacy_level`
- `UpdateStageInstance::topic`

Note: it is currently impossible to change only the `privacy_level` of an
existing stage instance.  The change required to fix this is breaking, and will
be fixed in a later version.

### Changes

The following dependencies have been removed:

- `bytes` ([#909] - [@vivian])
- `futures-util` ([#912] - [@vivian])
- `native-tls` ([#911] - [@vivian])
- `serde_repr` ([#920] - [@vivian])

`Client::update_guild_channel_positions` was changed ([#880] - [@7596ff]):
- It now accepts a more generic parameter.
- `twilight_http::request::guild::update_guild_channel_positions::Position` is
  now `pub`, and has new fields `lock_permissions` and `parent_id`.

`error::Result` has been deprecated, as importing `error::Error` is similar
([#821] - [@vivian]).

The minimum channel name length is now 1 ([#885] - [@BlackHoleFox]).

[#821]: https://github.com/twilight-rs/twilight/pull/821
[#867]: https://github.com/twilight-rs/twilight/pull/867
[#880]: https://github.com/twilight-rs/twilight/pull/880
[#885]: https://github.com/twilight-rs/twilight/pull/885
[#886]: https://github.com/twilight-rs/twilight/pull/886
[#909]: https://github.com/twilight-rs/twilight/pull/909
[#911]: https://github.com/twilight-rs/twilight/pull/911
[#912]: https://github.com/twilight-rs/twilight/pull/912
[#918]: https://github.com/twilight-rs/twilight/pull/918
[#920]: https://github.com/twilight-rs/twilight/pull/920

## [0.4.2] - 2021-05-30

### Upgrade Path

`Request::new` and the `From` implementations on `Request` have been
deprecated; use the new `RequestBuilder` instead.

`CreateInvite::target_user_type` has been deprecated; use
`CreateInvite::target_type` instead.

### Additions

Add `request::RequestBuilder` for constructing manual `Request`s
([#814] - [@vivian], [#831] - [@vivian]).

Add support for 18 new HTTP API error codes ([#818] - [@7596ff]).

Add the following new `CreateGuild` and `UpdateGuild` methods:

- `CreateGuild::afk_channel_id`
- `CreateGuild::afk_timeout`
- `CreateGuild::system_channel_id`
- `CreateGuild::system_channel_flags`
- `UpdateGuild::discovery_splash`
- `UpdateGuild::features`
- `UpdateGuild::system_channel_flags`

([#819] - [@7596ff]).

Support retrieving a webhook's message, exposed via `Client::webhook_message`
([#817] - [@7596ff]).

Support Stage Instances by adding the following request methods:

- `Client::create_stage_instance`
- `Client::delete_stage_instance`
- `Client::stage_instance`
- `Client::update_stage_instance`

([#812], [#830] - [@7596ff]).

Bring invites up to date by adding `CreateInvite::target_application_id`, adding
`GetInvite::with_expiration`, and adding `CreateInvite::target_type` while
deprecating `CreateInvite::target_user_type` ([#809] - [@7596ff]).

### Fixes

Fix request sending logic in `UpdateWebhookMessage` ([#844] - [@7596ff]).

### Enhancements

Don't send client authentication in webhook requests using a webhook token
([#828] - [@vivian]).

The following functions are now `const`:

- `api_error::ErrorCode::num`
- `client::ClientBuilder::timeout`
- `client::Client::delete_channel_permission`
- `client::Client::update_channel_permission`
- `ratelimiting::RatelimitError::error::kind`
- `ratelimiting::RatelimitHeaders::global`
- `request::channel::invite::GetInvite::with_counts`
- `request::guild::create_guild::GuildChannelFieldsBuilder::new`
- `request::guild::GetGuild::with_counts`
- `request::AuditLogReasonError::kind`
- `Error::kind`

([#824] - [@vivian]).

### Changes

`Request::new` and `Request`'s `From` implementations have been deprecated
([#814] - [@vivian]).

[#844]: https://github.com/twilight-rs/twilight/pull/844
[#831]: https://github.com/twilight-rs/twilight/pull/831
[#830]: https://github.com/twilight-rs/twilight/pull/830
[#828]: https://github.com/twilight-rs/twilight/pull/828
[#824]: https://github.com/twilight-rs/twilight/pull/824
[#818]: https://github.com/twilight-rs/twilight/pull/818
[#817]: https://github.com/twilight-rs/twilight/pull/817
[#814]: https://github.com/twilight-rs/twilight/pull/814
[#812]: https://github.com/twilight-rs/twilight/pull/812
[#809]: https://github.com/twilight-rs/twilight/pull/809

## [0.4.1] - 2021-05-20

### Upgrade Path

`CreateMessage::attachments` and `CreateMessage::attachment` have been
deprecated. Use the `files` and `file` methods instead.

### Additions

Add support for sending non-attachment files for reference in embeds in
`ExecuteWebhook`, `UpdateMessage`, and `UpdateWebhookMessage`, and sending
attachments in `UpdateMessage` and `UpdateWebhookMessage`
([#797] - [@7596ff], [@AsianIntel]).

### Fixes

Fix file uploading functionality in `CreateMessage::payload_json` and
`ExecuteWebhook::payload_json` ([#797] - [@7596ff], [@AsianIntel]).

[#797]: https://github.com/twilight-rs/twilight/pull/797

## [0.4.0] - 2021-05-12

### Upgrade Path

The MSRV is now Rust 1.49.

Remove calls to:

- `Client::create_guild_integration`
- `Client::current_user_private_channels`
- `Client::delete_guild_integration`
- `Client::sync_guild_integration`

Remove references to:

- `request::guild::integration`
- `request::user::GetCurrentUserPrivateChannels`

Replace references to `Path::WebhooksIdTokenMessageId` with
`Path::WebhooksIdTokenMessagesId`.

`CreateInvite::{max_age, max_uses}` now return validation errors, so the results
returned from them need to be handled.

Don't re-use `hyper` clients via the builder. If you need to configure the
underlying `hyper` client please create an issue with the reason why.

Errors are no longer enums and don't expose their concrete underlying error
source. You can access the underlying error via the implemented
`std::error::Error::source` method or the `into_parts` or `into_source` methods
on each error struct, which will return a boxed `std::error::Error`. To access
the reason for the error use the `kind` or `into_parts` method on error structs;
the returned error type is an enum with variants for each potential reason the
error occurred.

When specifying a method in a custom request use `request::Method` instead of
`hyper`'s.

The Allowed Mentions API has been reworked and moved to `twilight-model`. Use it
like so:

```rust
use twilight_model::channel::message::AllowedMentions;

let allowed_mentions = AllowedMentions::builder()
    .replied_user()
    .user_ids(user_ids)
    .build();
```

### Additions

Support `CreateMessage::fail_if_not_exists` which will fail creating the message
if the referenced message does not exist ([#708] - [@7596ff]).

### Enhancements

Update `simd-json` to 0.4 ([#786] - [@Gelbpunkt]).

The `futures-channel` dependency has been removed ([#785] - [@Gelbpunkt]).

### Changes

Remove support for guild integration calls, which are blocked for bots due to a
Discord API change ([#751] - [@vivian]).

Rename `Path::WebhooksIdTokenMessageId` to `Path::WebhooksIdTokenMessagesId`
([#755] - [@vivian]).

Return validation errors for `CreateInvite::max_age` and
`CreateInvite::max_uses` ([#757] - [@vivian]).

Remove ability to get current user's DM channels ([#782] - [@vivian]).

Remove `ClientBuilder::hyper_client` and `From<HyperClient> for Client` which
were available to re-use `hyper` clients ([#768] - [@vivian]).

Return updated copy of member when updating a member ([#758] - [@vivian]).

The `request::channel::message::allowed_mentions` API has been reworked and
moved to the `twilight-model` crate ([#760] - [@7596ff]).

Add custom method enum type for use rather than `hyper`'s ([#767] - [@vivian]).

[#786]: https://github.com/twilight-rs/twilight/pull/786
[#785]: https://github.com/twilight-rs/twilight/pull/785
[#782]: https://github.com/twilight-rs/twilight/pull/782
[#768]: https://github.com/twilight-rs/twilight/pull/768
[#767]: https://github.com/twilight-rs/twilight/pull/767
[#760]: https://github.com/twilight-rs/twilight/pull/760
[#758]: https://github.com/twilight-rs/twilight/pull/758
[#757]: https://github.com/twilight-rs/twilight/pull/757
[#755]: https://github.com/twilight-rs/twilight/pull/755
[#751]: https://github.com/twilight-rs/twilight/pull/751
[#708]: https://github.com/twilight-rs/twilight/pull/708

## [0.3.9] - 2021-05-09

### Additions

Support the Update Current User Voice State and Update User Voice State
stage channel routes ([#795] - [@7596ff]).

Support error codes 10057, 20028, 30031, 30035, and 50074 ([#800] - [@Erk-]).

### Fixes

Percent-encode reasons when creating a member ban ([#803] - [@Erk-]).

[#803]: https://github.com/twilight-rs/twilight/pull/803
[#800]: https://github.com/twilight-rs/twilight/pull/800
[#795]: https://github.com/twilight-rs/twilight/pull/795

## [0.3.8] - 2021-04-27

### Additions

Support the "Get Guild Welcome Screen" and "Update Guild Welcome Screen"
endpoints ([#792] - [@7596ff]).

[#792]: https://github.com/twilight-rs/twilight/pull/792

## [0.3.7] - 2021-04-22

### Upgrade Path

`CreateInvite::target_user` is deprecated, use `CreateInvite::target_user_id`.

### Additions

Add HTTP error code 60003, "Two factor is required for this operation"
([#770] - [@vivian]).

Add HTTP error code 10062, "Unknown interaction." ([#780] - [@vivian]).

Support searching the members of a guild via HTTP ([#587] - [@Gelbpunkt]).

Support guild templates via the following request handlers:

- `CreateGuildFromTemplate`
- `CreateTemplate`
- `DeleteTemplate`
- `GetTemplates`
- `GetTemplate`
- `SyncTemplate`
- `UpdateTemplate`

([#736] - [@7596ff]).

### Fixes

Add missing route mappings for Path parsing ([#743] - [@AEnterprise]).

Deprecate `CreateInvite::target_user`, add `CreateInvite::target_user_id` to
match the corrected field name when creating an invite ([#771] - [@vivian]).

Correctly parse global ratelimit headers ([#766] - [@vivian]).

Fix how long to sleep when encountering a global ratelimit ([#787] - [@vivian]).

[#787]: https://github.com/twilight-rs/twilight/pull/787
[#780]: https://github.com/twilight-rs/twilight/pull/780
[#771]: https://github.com/twilight-rs/twilight/pull/771
[#770]: https://github.com/twilight-rs/twilight/pull/770
[#766]: https://github.com/twilight-rs/twilight/pull/766
[#743]: https://github.com/twilight-rs/twilight/pull/743
[#736]: https://github.com/twilight-rs/twilight/pull/736
[#587]: https://github.com/twilight-rs/twilight/pull/587

## [0.3.6] - 2021-04-04

### Fixes

Set request & default headers after content headers ([#737] - [@AsianIntel]).

[#737]: https://github.com/twilight-rs/twilight/pull/737

## [0.3.5] - 2021-03-14

### Additions

Add an allowed mentions builder to `ExecuteWebhook` ([#719] - [@7596ff]).

[#719]: https://github.com/twilight-rs/twilight/pull/719

## [0.3.4] - 2021-01-25

### Fixes

Fix the check in the `request::guild::GetGuildPruneCount::days` method
([#696] - [@Silvea12]).

### Documentation

Add a note about using bearer tokens with the client ([#697] - [@vivian]).

[#697]: https://github.com/twilight-rs/twilight/pull/697
[#696]: https://github.com/twilight-rs/twilight/pull/696

## [0.3.3] - 2021-01-19

### Additions

Support setting default headers to be used for every request
([#654] - [@tbnritzdoge]).

[#654]: https://github.com/twilight-rs/twilight/pull/654

## [0.3.2] - 2021-01-18

### Fixes

Set the correct HTTPS connector in `Client::new` ([#689] - [@Gelbpunkt]).

Fix setting duplicate headers ([#686] - [@Gelbpunkt]).

Percent-encode emojis in URIs, fixing routes like `CreateEmoji`
([#685] - [@sam-kirby]).

[#689]: https://github.com/twilight-rs/twilight/pull/689
[#686]: https://github.com/twilight-rs/twilight/pull/686
[#685]: https://github.com/twilight-rs/twilight/pull/685

## [0.3.1] - 2021-01-11

This release fixes the `native` feature and adds support for the
"Add Guild Member" endpoint.

### Additions

Implement the "Add Guild Member" endpoint ([#653] - [@sam-kirby]).

### Fixes

Manually send ALPN headers to fix the `native` feature ([#683] - [@Gelbpunkt]).

[#683]: https://github.com/twilight-rs/twilight/pull/683
[#653]: https://github.com/twilight-rs/twilight/pull/653

## [0.3.0] - 2021-01-08

Version 0.3 has been released with the primary intent to switch from `reqwest`
to the lighter `hyper` as the HTTP client and upgrade to Tokio 1.0.

### Upgrade Path

`client::Client`'s `delete_webhook_from_url`, `execute_webhook_from_url`,
`update_webhook_from_url`, and `update_webhook_with_token_from_url` methods have
been removed. Instead, use the `twilight-util` crate's new
`link::webhook::parse` functionality to parse webhook IDs and tokens out of
URLs. Then, pass the webhook ID and token to the method variants without
`_from_url`; for example, `client::Client::execute_webhook`.

`client::Client::add_role` has been removed because it was a duplicate method.
Instead use `client::Client::add_guild_member_role`.

When attaching files to a message, pass in the bytes of the attachment instead
of `reqwest`'s `Body` type.

If supplying an HTTP client to `twilight-http`, pass a `hyper` client instead of
a `reqwest` client and use `client::ClientBuilder::hyper_client` instead of
`reqwest_client`.

Instead of using `client::ClientBuilder::proxy` to pass a `reqwest` proxy, use
it to pass a string of the URL of the proxy. This is now specifically noted to
be used with applications like Twilight's `http-proxy` instead of connecting to
a proxy, which can be configured with a manual `hyper` client.

`error::UrlError`'s `UrlParsing` variant has been removed as it can no longer
occur. `error::Error`'s `BuildingClient` has been removed as building clients
can no longer fail. All Reqwest errors are now `hyper` errors.

A couple of re-exports have been removed. Use
`twilight_model::user::CurrentUserGuild` instead of
`request::user::get_current_user_guilds::CurrentUserGuild`. Additionally, use
`request::channel::allowed_mentions` instead of
`request::channel::message::allowed_mentions`.

### Changes

Remove old re-exports that were deprecated in v0.2 ([#673] - [@vivian]).

Upgrade from `tokio` v0.2 to v1 ([#664] - [@vivian]).

`reqwest` has been switched out for `hyper` 0.14. With this comes some API
breakage:

- `client::ClientBuilder::build` no longer returns a `Result`
- `client::ClientBuilder::reqwest_client` has been renamed to `hyper_client`
- `client::ClientBuilder::{proxy_http, proxy}` have been combined into `proxy`
- `client::Client::raw` now returns a `hyper` response instead of a `reqwest`
response
- `error::Error::BuildingClient` has been removed
- `error::Error::{ChunkingResponse, RequestError}` now include `hyper` source
errors instead of `reqwest` ones
- `request::channel::message::CreateMessage::attachment{,s}` now takes
`impl Into<Vec<u8>>` instead of `impl Into<reqwest::Body>`

([#657] - [@vivian], [#670] - [@Gelbpunkt]).

Remove `client::Client::add_role` ([#669] - [@vivian]).

Remove webhook URL variant client methods and move webhook URL parsing to
`twilight-util` ([#658] - [@vivian]).

[#673]: https://github.com/twilight-rs/twilight/pull/673
[#670]: https://github.com/twilight-rs/twilight/pull/670
[#669]: https://github.com/twilight-rs/twilight/pull/669
[#664]: https://github.com/twilight-rs/twilight/pull/664
[#658]: https://github.com/twilight-rs/twilight/pull/658
[#657]: https://github.com/twilight-rs/twilight/pull/657

## [0.2.8] - 2021-01-05

### Additions

Support deleting and updating the messages created by webhooks
([#643] - [@vivian]).

### Fixes

Properly construct `Route::GetAuditLogs` path string ([#662] - [@jazevedo620]).

[#662]: https://github.com/twilight-rs/twilight/pull/662
[#643]: https://github.com/twilight-rs/twilight/pull/643

## [0.2.7] - 2020-12-29

### Fixes

Specify a minimum `twilight-model` dependency version of `^0.2.5` instead of
`^0.2`.

Make `api_error::RatelimitedApiError::retry_after` an `f64` instead of a `u64`.
This allows the value to be correctly parsed ([#644] - [@vivian]).

### Enhancements

Use `Box<str>` instead of `String` internally in order to reduce struct size
([#647] - [@vivian]).

## [0.2.6] - 2020-12-18

The MSRV is now set to Rust 1.48.

### Enhancements

The `request::user::get_current_user_guilds::CurrentUserGuild` type has been
moved to `twilight_model::user::CurrentUserGuild`. A re-export has been left
in its place ([#625] - [@AsianIntel]).

### Misc.

Replace documentation links with intra-doc links ([#524] - [@nickelc]).

## [0.2.5] - 2020-11-29

### Additions

Support inline replies by adding `AllowedMentionsBuilder::replied_user` to
determine whether to mention the user being replied to and
`CreateMessage::reply` to specify the message to reply to ([#604] - [@Erk-]).

### Fixes

Use integers instead of strings for target types in the
`UpdateChannelPermission` request ([#614] - [@vivian]).

## [0.2.4] - 2020-11-25

### Additions

Support Message Stickers by adding HTTP error code variant 50'081 "Invalid
Sticker Sent" ([#608] - [@vivian]).

### Fixes

Use Reqwest's header name constants, which fixes the name of a hardcoded header
in an error ([#620] - [@vivian]).

### Enhancements

Clarify the cloning behavior of the `Client` ([#607] - [@vivian]).

## [0.2.3] - 2020-11-20

### Additions

Add an `API_VERSION` constant to the root of the library, which is the version
of the Discord HTTP API in use ([#598] - [@AEnterprise]).

### Fixes

Properly handle optional messages in the Execute Webhook request when `wait`
is `false` ([#599] - [@Erk-]).

Serialize guild creation role permissions to the correct field name
"permissions" instead of "permissions_new" ([#602] - [@sam-kirby]).

## [0.2.2] - 2020-11-11

### Additions

Handle service unavailability (503) errors, returning a new error variant when
encountered ([#592] - [@vivian]).

Support 3 new HTTP error codes ([#594] - [@vivian]):

- 20022: This message cannot be edited due to announcement rate limits
- 50024: Cannot execute action on this channel type
- 50033: Invalid Recipient(s)

Take note of invalid tokens, short-circuiting attempts to execute in the
future. This will cause the client to return an `Unauthorized` error variant
when an Unauthorized (401) status code is encountered in order to prevent API
bans ([#597] - [@vivian]).

## [0.2.1] - 2020-11-02

Update the installation instructions to note version 0.2 instead of
version 0.1 ([#588] - [@vivian]).

## [0.2.0] - 2020-10-30

This major version of the crate primarily includes changes needed to support
version 8 of the Discord HTTP API.

### Additions

Implement [Application Integrations][0.2.0-beta.1:app integrations]
([#549], [#579] - [@Erk-])

Add support for the Followed Channels API feature ([#556] - [@Gelbpunkt]):

- `api_error::ErrorCode::MessageAlreadyCrossposted` error type (40033)
- `Client::follow_news_channel` method to follow a news channel accompanied by
  its `request::channel::follow_news_channel` request
- `Client::crosspost_message` method to crosspost a message accompanied by its
  `request::channel::message::crosspost_message` request
- `routing::Path::ChannelsIdMessagesIdCrosspost` API path variant to crosspost a
  channel message
- `routing::Path::ChannelsIdFollowers` API path variant to operate on a news
  channel's followers
- `routing::Route::CrosspostMessage` to retrieve the route to crosspost a
  channel message
- `routing::Route::FollowNewsChannel` to retrieve the route to follow a channel
  message

Add `Client::ratelimiter` to retrieve the active ratelimiter and add remaining
time estimation for buckets to the
Ratelimiter (`Ratelimiter::time_until_available`) ([#547] - [@Gelbpunkt]).

### Fixes

The Future output type of the
`request::user::get_current_user_guilds::GetCurrentUserGuilds` request has been
changed from `PartialGuild` to a new, slimmed down partial guild struct when
listing the current user's
guilds ([#550] - [@DusterTheFirst], [#567] - [@chamburr]).

Use the configured Reqwest client in the
`ClientBuilder` ([#563] - [@DusterTheFirst]).

### Changes

Remove the deprecated request `reason` methods in favor of the
`request::AuditLogReason` trait ([#581] - [@vivian]).

## [0.2.0-beta.2] - 2020-10-22

### Additions

Add support for the Followed Channels API feature ([#556] - [@Gelbpunkt]):

- `api_error::ErrorCode::MessageAlreadyCrossposted` error type (40033)
- `Client::follow_news_channel` method to follow a news channel accompanied by
  its `request::channel::follow_news_channel` request
- `Client::crosspost_message` method to crosspost a message accompanied by its
  `request::channel::message::crosspost_message` request
- `routing::Path::ChannelsIdMessagesIdCrosspost` API path variant to crosspost a
  channel message
- `routing::Path::ChannelsIdFollowers` API path variant to operate on a news
  channel's followers
- `routing::Route::CrosspostMessage` to retrieve the route to crosspost a
  channel message
- `routing::Route::FollowNewsChannel` to retrieve the route to follow a channel
  message

### Fixes

The Future output type of the
`request::user::get_current_user_guilds::GetCurrentUserGuilds` request has been
changed from `PartialGuild` to a new, slimmed down partial guild struct when
listing the current user's guilds ([#550] - [@DusterTheFirst]).

## [0.2.0-beta.1] - 2020-10-17

### Additions

Implement [Application Integrations][0.2.0-beta.1:app integrations]
([#549] - [@Erk-])

## [0.2.0-beta.0] - 2020-10-10

This beta version of major version 0.2 of the crate includes changes needed to
support version 8 of the Discord HTTP API.

### Changes

All changes in this version are from PR [#532].

- Now depends on `twilight-model` 0.2

## [0.1.6] - 2020-10-05

### Fixes

- Use correct route for `GetUserApplicationInfo` request ([#534] - [@Erk-])

## [0.1.5] - 2020-09-27

### Added

- Add `AuditLogReason` trait, deprecate existing `reason` request methods ([#522] - [@Erk-])

### Fixes

- Handle invalid timestamp embed errors in responses ([#529] - [@coadler])

## [0.1.4] - 2020-09-20

### Fixes

- Take `RequestReactionType` in `Client::reactions` ([#520] - [@7596ff])
- Set `content-length` header for `PATCH`, `POST`, and `PUT` methods ([#519] - [@AEnterprise])
- Fix typos in documentation links ([#515] - [@nickelc])

## [0.1.3] - 2020-09-19

### Fixes

- Only set content headers if there's a body ([#514] - [@AEnterprise])

## [0.1.2] - 2020-09-17

### Added

- Impl Eq, PartialEq, From for RequestReactionType ([#507] - [@7596ff])
- Support `/oauth2/applications/@me` endpoint ([#510] - [@AEnterprise])

## [0.1.1] - 2020-09-15

### Fixes

- Handle webhooks with tokens in path parsing ([#495] - [@AEnterprise])

## [0.1.0] - 2020-09-13

Initial release.

[@7596ff]: https://github.com/7596ff
[@AEnterprise]: https://github.com/AEnterprise
[@AsianIntel]: https://github.com/AsianIntel
[@BlackHoleFox]: https://github.com/BlackHoleFox
[@chamburr]: https://github.com/chamburr
[@cherryblossom000]: https://github.com/cherryblossom000
[@coadler]: https://github.com/coadler
[@DusterTheFirst]: https://github.com/DusterTheFirst
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@HTG-YT]: https://github.com/HTG-YT
[@itohatweb]: https://github.com/itohatweb
[@jazevedo620]: https://github.com/jazevedo620
[@Learath2]: https://github.com/Learath2
[@MaxOhn]: https://github.com/MaxOhn
[@nickelc]: https://github.com/nickelc
[@sam-kirby]: https://github.com/sam-kirby
[@Silvea12]: https://github.com/Silvea12
[@SuperiorJT]: https://github.com/SuperiorJT
[@tbnritzdoge]: https://github.com/tbnritzdoge
[@vilgotf]: https://github.com/vilgotf
[@vivian]: https://github.com/vivian
[@zeylahellyer]: https://github.com/zeylahellyer

[#647]: https://github.com/twilight-rs/twilight/pull/647
[#644]: https://github.com/twilight-rs/twilight/pull/644
[#625]: https://github.com/twilight-rs/twilight/pull/625
[#620]: https://github.com/twilight-rs/twilight/pull/620
[#614]: https://github.com/twilight-rs/twilight/pull/614
[#608]: https://github.com/twilight-rs/twilight/pull/608
[#607]: https://github.com/twilight-rs/twilight/pull/607
[#604]: https://github.com/twilight-rs/twilight/pull/604
[#602]: https://github.com/twilight-rs/twilight/pull/602
[#599]: https://github.com/twilight-rs/twilight/pull/599
[#598]: https://github.com/twilight-rs/twilight/pull/598
[#597]: https://github.com/twilight-rs/twilight/pull/597
[#594]: https://github.com/twilight-rs/twilight/pull/594
[#592]: https://github.com/twilight-rs/twilight/pull/592
[#588]: https://github.com/twilight-rs/twilight/pull/588
[#581]: https://github.com/twilight-rs/twilight/pull/581
[#579]: https://github.com/twilight-rs/twilight/pull/579
[#567]: https://github.com/twilight-rs/twilight/pull/567
[#556]: https://github.com/twilight-rs/twilight/pull/556
[#550]: https://github.com/twilight-rs/twilight/pull/550
[#549]: https://github.com/twilight-rs/twilight/pull/549
[#547]: https://github.com/twilight-rs/twilight/pull/547
[#534]: https://github.com/twilight-rs/twilight/pull/534
[#532]: https://github.com/twilight-rs/twilight/pull/532
[#529]: https://github.com/twilight-rs/twilight/pull/529
[#524]: https://github.com/twilight-rs/twilight/pull/524
[#522]: https://github.com/twilight-rs/twilight/pull/522
[#520]: https://github.com/twilight-rs/twilight/pull/520
[#519]: https://github.com/twilight-rs/twilight/pull/519
[#515]: https://github.com/twilight-rs/twilight/pull/515
[#514]: https://github.com/twilight-rs/twilight/pull/514
[#510]: https://github.com/twilight-rs/twilight/pull/510
[#507]: https://github.com/twilight-rs/twilight/pull/507
[#495]: https://github.com/twilight-rs/twilight/pull/495

[0.2.0-beta.1:app integrations]: https://github.com/discord/discord-api-docs/commit/a926694e2f8605848bda6b57d21c8817559e5cec

[0.8.3]: https://github.com/twilight-rs/twilight/releases/tag/http-0.8.3
[0.8.1]: https://github.com/twilight-rs/twilight/releases/tag/http-0.8.1
[0.8.0]: https://github.com/twilight-rs/twilight/releases/tag/http-0.8.0
[0.7.3]: https://github.com/twilight-rs/twilight/releases/tag/http-0.7.3
[0.7.2]: https://github.com/twilight-rs/twilight/releases/tag/http-0.7.2
[0.7.1]: https://github.com/twilight-rs/twilight/releases/tag/http-0.7.1
[0.7.0]: https://github.com/twilight-rs/twilight/releases/tag/http-0.7.0
[0.6.6]: https://github.com/twilight-rs/twilight/releases/tag/http-0.6.6
[0.6.5]: https://github.com/twilight-rs/twilight/releases/tag/http-0.6.5
[0.6.4]: https://github.com/twilight-rs/twilight/releases/tag/http-0.6.4
[0.6.3]: https://github.com/twilight-rs/twilight/releases/tag/http-0.6.3
[0.6.2]: https://github.com/twilight-rs/twilight/releases/tag/http-0.6.2
[0.5.7]: https://github.com/twilight-rs/twilight/releases/tag/http-0.5.7
[0.5.6]: https://github.com/twilight-rs/twilight/releases/tag/http-0.5.6
[0.5.5]: https://github.com/twilight-rs/twilight/releases/tag/http-0.5.5
[0.5.4]: https://github.com/twilight-rs/twilight/releases/tag/http-0.5.4
[0.5.3]: https://github.com/twilight-rs/twilight/releases/tag/http-0.5.3
[0.5.2]: https://github.com/twilight-rs/twilight/releases/tag/http-0.5.2
[0.5.1]: https://github.com/twilight-rs/twilight/releases/tag/http-0.5.1
[0.5.0]: https://github.com/twilight-rs/twilight/releases/tag/http-0.5.0
[0.4.3]: https://github.com/twilight-rs/twilight/releases/tag/http-0.4.3
[0.4.2]: https://github.com/twilight-rs/twilight/releases/tag/http-0.4.2
[0.4.1]: https://github.com/twilight-rs/twilight/releases/tag/http-0.4.1
[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/http-0.4.0
[0.3.9]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.3.9
[0.3.8]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.3.8
[0.3.6]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.3.6
[0.3.5]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.3.5
[0.3.4]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.3.4
[0.3.3]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.3.3
[0.3.2]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.3.2
[0.3.1]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.3.1
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.3.0
[0.2.8]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.2.8
[0.2.7]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.2.7
[0.2.6]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.2.6
[0.2.5]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.2.5
[0.2.4]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.2.4
[0.2.3]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.2.3
[0.2.2]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.2.2
[0.2.1]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.2.1
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.2.0
[0.2.0-beta.2]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.2.0-beta.2
[0.2.0-beta.1]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.2.0-beta.1
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.2.0-beta.0
[0.1.6]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.6
[0.1.5]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.5
[0.1.4]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.4
[0.1.3]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.3
[0.1.2]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.2
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
