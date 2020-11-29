# Changelog

Changelog for `twilight-http`.

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
[@chamburr]: https://github.com/chamburr
[@coadler]: https://github.com/coadler
[@DusterTheFirst]: https://github.com/DusterTheFirst
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@nickelc]: https://github.com/nickelc
[@sam-kirby]: https://github.com/sam-kirby
[@vivian]: https://github.com/vivian

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
[#522]: https://github.com/twilight-rs/twilight/pull/522
[#520]: https://github.com/twilight-rs/twilight/pull/520
[#519]: https://github.com/twilight-rs/twilight/pull/519
[#515]: https://github.com/twilight-rs/twilight/pull/515
[#514]: https://github.com/twilight-rs/twilight/pull/514
[#510]: https://github.com/twilight-rs/twilight/pull/510
[#507]: https://github.com/twilight-rs/twilight/pull/507
[#495]: https://github.com/twilight-rs/twilight/pull/495


[0.2.0-beta.1:app integrations]: https://github.com/discord/discord-api-docs/commit/a926694e2f8605848bda6b57d21c8817559e5cec

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
