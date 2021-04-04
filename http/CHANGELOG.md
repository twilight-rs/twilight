# Changelog

Changelog for `twilight-http`.

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
can no longer fail. All Weqwest errors are now `hyper` errors.

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
[@chamburr]: https://github.com/chamburr
[@coadler]: https://github.com/coadler
[@DusterTheFirst]: https://github.com/DusterTheFirst
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@jazevedo620]: https://github.com/jazevedo620
[@nickelc]: https://github.com/nickelc
[@sam-kirby]: https://github.com/sam-kirby
[@Silvea12]: https://github.com/Silvea12
[@tbnritzdoge]: https://github.com/tbnritzdoge
[@vivian]: https://github.com/vivian

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
