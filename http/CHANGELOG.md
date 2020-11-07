# Changelog

Changelog for `twilight-http`.

## [0.1.8] - 2020-11-07

This release includes a few bugfixes. While v0.1 will be maintained until the
deprecation of version 6 of the Discord API, we recommend upgrading to v0.2.

### Additions

Add `Client::ratelimiter` to retrieve the active ratelimiter and add remaining
time estimation for buckets to the
Ratelimiter (`Ratelimiter::time_until_available`) ([#547] - [@Gelbpunkt]).

### Fixes

Use the configured Reqwest client in the
`ClientBuilder` ([#563] - [@DusterTheFirst]).

## [0.1.7] - 2020-10-22

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
listing the current user's guilds. This is a breaking change, but due to being
a bugfix is included in a patch version ([#550] - [@DusterTheFirst]).

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
[@coadler]: https://github.com/coadler
[@DusterTheFirst]: https://github.com/DusterTheFirst
[@Erk-]: https://github.com/Erk-
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@nickelc]: https://github.com/nickelc

[#563]: https://github.com/twilight-rs/twilight/pull/563
[#556]: https://github.com/twilight-rs/twilight/pull/556
[#550]: https://github.com/twilight-rs/twilight/pull/550
[#547]: https://github.com/twilight-rs/twilight/pull/547
[#534]: https://github.com/twilight-rs/twilight/pull/534
[#529]: https://github.com/twilight-rs/twilight/pull/529
[#522]: https://github.com/twilight-rs/twilight/pull/522
[#520]: https://github.com/twilight-rs/twilight/pull/520
[#519]: https://github.com/twilight-rs/twilight/pull/519
[#515]: https://github.com/twilight-rs/twilight/pull/515
[#514]: https://github.com/twilight-rs/twilight/pull/514
[#510]: https://github.com/twilight-rs/twilight/pull/510
[#507]: https://github.com/twilight-rs/twilight/pull/507
[#495]: https://github.com/twilight-rs/twilight/pull/495

[0.1.8]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.8
[0.1.7]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.7
[0.1.6]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.6
[0.1.5]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.5
[0.1.4]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.4
[0.1.3]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.3
[0.1.2]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.2
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/http-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
