# Version 0.16 - 2024-??-??

| ❗  This is a pre-release version  |
|------------------------------------|

Version 0.16 of the Twilight ecosystem brings a lot of internal changes and
refactors, the latest versions of all dependencies and catches up with new API
features added by Discord. It also contains a couple of bugfixes.

## Feature name changes

The `native` feature in all crates that had one was renamed to `native-tls` to
avoid potential misconceptions about its purpose. Similarly, the `trust-dns`
feature exposed in HTTP was renamed to `hickory` to account for the project's
rebranding.

## Generic in-memory cache

Our in-memory cache implementation is now generic, meaning you can now write
custom cached representations of all the models. There are a couple of trait
bounds that need to be met for the models, however. The new [cache-optimization]
example demonstrates how to write your own cache models and implement the
traits. These changes will let you drop fields that you don't need to store
for your bot to function and save on memory.

Since [`InMemoryCache`] is now a generic type, existing code will have to be
updated to instead use [`DefaultInMemoryCache`], which is a drop-in replacement
for the old type.

## Gateway queue rewrite

The gateway queue crate was rewritten from scratch. The [`Queue`] trait no
longer returns an opaque future type, instead it makes use of channels now.

The three separate queue implementations were merged into one, the
[`InMemoryQueue`]. It is recommended to fetch the data for instantiating one
from the Discord API via [`Client::gateway`] to avoid getting ratelimited.

The old [`NoOpQueue`] can be replicated by setting `max_concurrency` to 0.

## Gateway refactors

The gateway crate has seen several changes as well. Alongside the gateway
queue rewrite, the [`Queue`] on the shards is now stored as a generic to avoid
an allocation. It defaults to an [`InMemoryQueue`].

A major pitfall with twilight's gateway pre-0.16 was that [`Shard::next_event`]
and [`Shard::next_message`] were not cancellation-safe. This has been addressed
by implementing [`Stream`] for the shard and updating the internals to be
cancellation-safe. [`futures_util::StreamExt::next`] now serves as the
replacement for [`Shard::next_message`], while
[`twilight_gateway::StreamExt::next_event`] replaces [`Shard::next_event`].

Additionally, the [`Config`] struct now no longer stores the
[`EventTypeFlags`], those have to be passed to
[`twilight_gateway::StreamExt::next_event`] now.

The [`Shard::command`], [`Shard::send`] and [`Shard::close`] methods now also
queue their action into a channel, like [`MessageSender`], and are therefore no
longer async and now infallible.

The [`create_range`] method was renamed to [`create_iterator`] and takes an
iterator over shard IDs instead of ranges. The `create_*` methods were also
moved to the top of the crate.

We also reworked the error types. [`ProcessError`] was removed entirely, while
[`SendError`] was renamed to [`ChannelError`]. [`ReceiveMessageErrorType`] now
only has four variants.

The [`ConnectionStatus`] enum was renamed to [`ShardState`] and its
[`Connected`] variant to [`Active`]. The close code is no longer stored and a
few methods were removed. Analogously, the method to retrieve it was renamed to
[`Shard::state`].

In order to protect against future API changes, the [`parse`] method no longer
errors upon encountering unknown events.

Putting it all together, the basic example of iterating over all events for a
single shard now looks like this:

<details>
<summary>Twilight 0.15</summary>

```rust,ignore
let intents = Intents::GUILDS | Intents::GUILD_MODERATION;
let mut shard = Shard::new(ShardId::ONE, env::var("DISCORD_TOKEN")?, intents);

loop {
    let event = match shard.next_event().await {
        Ok(event) => event,
        Err(source) => {
            tracing::warn!(?source, "error receiving event");

            if source.is_fatal() {
                break;
            }

            continue;
        }
    };

    println!("Event: {event:?}");
}
```
</details>
<br />
<details>
<summary>Twilight 0.16</summary>

```rust,ignore
use twilight_gateway::StreamExt;

let intents = Intents::GUILDS | Intents::GUILD_MODERATION;
let mut shard = Shard::new(ShardId::ONE, env::var("DISCORD_TOKEN")?, intents);

while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
    let Ok(event) = item else {
        tracing::warn!(source = ?item.unwrap_err(), "error receiving event");

        continue;
    };

    println!("Event: {event:?}");
}
```
</details>

## HTTP errors

The HTTP request builders now return their errors upon finalization instead of
each stage of the building process. The validation errors previously
encountered in the builder are now returned as [`Validation`] errors.

<details>
<summary>Twilight 0.15</summary>

```rust,ignore
let response = client.create_message(channel_id)
    .content("I am a message!")?
    .embeds(&embeds)?
    .tts(true)
    .await?;
```
</details>
<br />
<details>
<summary>Twilight 0.16</summary>

```rust,ignore
let response = client.create_message(channel_id)
    .content("I am a message!")
    .embeds(&embeds)
    .tts(true)
    .await?;
```
</details>

## Select menu support

Twilight now supports all select menu types. This involves multiple breaking
changes to the [`SelectMenu`] struct, since not all types of select menus
contain all fields. Most notably, the type of the select menu can be checked
via the `kind` field, which is a [`SelectMenuType`].

Support for select menu default values was added via
[`SelectMenu::default_values`].

## Discord API catchups

Twilight now supports super reactions via the `burst_colors`, `count_details`
and `me_burst` fields on [`Reaction`].

Auto moderation rule creation now supports setting regex patterns and allow
list. See [`CreateAutoModerationRule::with_keyword`] for the new validation
errors returned.

Channel creation and updating now supports specifying a default thread timeout
via [`CreateGuildChannel::default_thread_rate_limit_per_user`] and
[`UpdateChannel::default_thread_rate_limit_per_user`] respectively.

The guild onboarding flow can now be modified via the
[`UpgradeGuildOnboarding`] request.

Creating a stage instance now allows specifying a guild scheduled event via
[`CreateStageInstance::guild_scheduled_event_id`].

The current user application can now be edited with the
[`UpdateCurrentUserApplication`] request and missing fields were added to the
[`Application`] struct.

The [`Member::joined_at`] field is now marked as optional.

The [`GuildMedia`] channel type was added.

The unused [`UserProfile`] struct was removed from twilight-model, it served
no purpose.

Premium apps are now supported in both the HTTP client and websocket gateway.

Message forwarding is supported with [`CreateMessage::forward`].

Application emojis are supported with [`Client::get_application_emojis`],
[`Client::add_application_emoji`], [`Client::update_application_emoji`], and
[`Client::delete_application_emoji`].

Get guild role endpoint to make it possible to get a role from a guild easily: [`Client::role`].

Get voice state endpoint support with [`Client::current_user_voice_state`] and [`Client::user_voice_state`].

Support for [`Poll`s].

## Ratelimiter http dependency removal

The HTTP ratelimiter now no longer exposes a dependency on [http].
[`Method::to_http`] was changed to [`Method::name`] and now returns a string.

## Ecosystem dependency upgrades

The HTTP crate was updated to make use of [hyper]'s latest 1.x version.
Gateway, HTTP and Lavalink now use [rustls] 0.23, up from 0.20. The [bitflags]
crate was updated to 2.x, which changes the methods available on all types
generated by it.

## Switch to tokio-websockets and fastrand

The lavalink and gateway crates were rewritten internally to switch to the
[tokio-websockets] library, away from [tokio-tungstenite]. This change should
nearly double throughput and efficiency and tightens down on dependency count.
We also changed the RNG used by our crates to [fastrand].

## Deprecated API removal

All APIs deprecated since 0.14.x were removed.

## Removal of support for undocumented gateway events

Support for the undocumented `GIFT_CODE_UPDATE` and `PRESENCES_REPLACE` events
was removed.

[`Active`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/enum.ShardState.html#variant.Active
[`Application`]: https://docs.rs/twilight-model/0.16.0-rc.1/twilight_model/oauth/struct.Application.html
[`ChannelError`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/error/struct.ChannelError.html
[`Client::gateway`]: https://docs.rs/twilight-http/0.16.0-rc.1/twilight_http/client/struct.Client.html#method.gateway
[`Config`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/struct.Config.html
[`Connected`]: https://docs.rs/twilight-gateway/0.15.4/twilight_gateway/enum.ConnectionStatus.html#variant.Connected
[`ConnectionStatus`]: https://docs.rs/twilight-gateway/0.15.4/twilight_gateway/enum.ConnectionStatus.html
[`create_iterator`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/fn.create_iterator.html
[`create_range`]: https://docs.rs/twilight-gateway/0.15.4/twilight_gateway/stream/fn.create_range.html
[`CreateAutoModerationRule::with_keyword`]: https://docs.rs/twilight-http/0.16.0-rc.1/twilight_http/request/guild/auto_moderation/struct.CreateAutoModerationRule.html#method.with_keyword
[`CreateGuildChannel::default_thread_rate_limit_per_user`]: https://docs.rs/twilight-http/0.16.0-rc.1/twilight_http/request/guild/struct.CreateGuildChannel.html#method.default_thread_rate_limit_per_user
[`CreateStageInstance::guild_scheduled_event_id`]: https://docs.rs/twilight-http/0.16.0-rc.1/twilight_http/request/channel/stage/struct.CreateStageInstance.html#method.guild_scheduled_event_id
[`DefaultInMemoryCache`]: https://docs.rs/twilight-cache-inmemory/0.16.0-rc.1/twilight_cache_inmemory/type.DefaultInMemoryCache.html
[`EventTypeFlags`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/struct.EventTypeFlags.html
[`futures_util::StreamExt::next`]: https://docs.rs/futures-util/latest/futures_util/stream/trait.StreamExt.html#method.next
[`GuildMedia`]: https://docs.rs/twilight-model/0.16.0-rc.1/twilight_model/channel/enum.ChannelType.html#variant.GuildMedia
[`InMemoryCache`]: https://docs.rs/twilight-cache-inmemory/0.16.0-rc.1/twilight_cache_inmemory/struct.InMemoryCache.html
[`InMemoryQueue`]: https://docs.rs/twilight-gateway-queue/0.16.0-rc.1/twilight_gateway_queue/struct.InMemoryQueue.html
[`Member::joined_at`]: https://docs.rs/twilight-model/0.16.0-rc.1/twilight_model/guild/struct.Member.html#structfield.joined_at
[`MessageSender`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/struct.MessageSender.html
[`Method::name`]: https://docs.rs/twilight-http-ratelimiting/0.16.0-rc.1/twilight_http_ratelimiting/request/enum.Method.html#method.name
[`Method::to_http`]: https://docs.rs/twilight-http-ratelimiting/0.15.3/twilight_http_ratelimiting/request/enum.Method.html#method.to_http
[`NoOpQueue`]: https://docs.rs/twilight-gateway-queue/0.15.4/twilight_gateway_queue/struct.NoOpQueue.html
[`parse`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/fn.parse.html
[`ProcessError`]: https://docs.rs/twilight-gateway/0.15.4/twilight_gateway/error/struct.ProcessError.html
[`Queue`]: https://docs.rs/twilight-gateway-queue/0.16.0-rc.1/twilight_gateway_queue/trait.Queue.html
[`Reaction`]: https://docs.rs/twilight-model/0.16.0-rc.1/twilight_model/channel/message/struct.Reaction.html
[`ReceiveMessageErrorType`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/error/enum.ReceiveMessageErrorType.html
[`SelectMenu::default_values`]: https://docs.rs/twilight-model/0.16.0-rc.1/twilight_model/channel/message/component/struct.SelectMenu.html#structfield.default_values
[`SelectMenu`]: https://docs.rs/twilight-model/0.16.0-rc.1/twilight_model/channel/message/component/struct.SelectMenu.html
[`SelectMenuType`]: https://docs.rs/twilight-model/0.16.0-rc.1/twilight_model/channel/message/component/enum.SelectMenuType.html
[`SendError`]: https://docs.rs/twilight-gateway/0.15.4/twilight_gateway/error/struct.SendError.html
[`Shard::close`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/struct.Shard.html#method.close
[`Shard::command`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/struct.Shard.html#method.command
[`Shard::next_event`]: https://docs.rs/twilight-gateway/0.15.4/twilight_gateway/struct.Shard.html#method.next_event
[`Shard::next_message`]: https://docs.rs/twilight-gateway/0.15.4/twilight_gateway/struct.Shard.html#method.next_message
[`Shard::send`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/struct.Shard.html#method.send
[`Shard::state`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/struct.Shard.html#method.state
[`ShardState`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/enum.ShardState.html
[`Stream`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/struct.Shard.html#impl-Stream-for-Shard%3CQ%3E
[`twilight_gateway::StreamExt::next_event`]: https://docs.rs/twilight-gateway/0.16.0-rc.1/twilight_gateway/trait.StreamExt.html#method.next_event
[`UpdateChannel::default_thread_rate_limit_per_user`]: https://docs.rs/twilight-http/0.16.0-rc.1/twilight_http/request/channel/struct.UpdateChannel.html#method.default_thread_rate_limit_per_user
[`UpdateCurrentUserApplication`]: https://docs.rs/twilight-http/0.16.0-rc.1/twilight_http/request/struct.UpdateCurrentUserApplication.html
[`UpgradeGuildOnboarding`]: https://docs.rs/twilight-http/0.16.0-rc.1/twilight_http/request/guild/update_guild_onboarding/struct.UpdateGuildOnboarding.html
[`UserProfile`]: https://docs.rs/twilight-model/0.15.4/twilight_model/user/struct.UserProfile.html
[`Validation`]: https://docs.rs/twilight-http/0.16.0-rc.1/twilight_http/error/enum.ErrorType.html#variant.Validation
[`CreateMessage::forward`]: https://api.twilight.rs/twilight_http/request/channel/message/create_message/struct.CreateMessage.html#method.forward
[`Client::get_application_emojis`]: https://api.twilight.rs/twilight_http/client/struct.Client.html#method.get_application_emojis
[`Client::add_application_emoji`]: https://api.twilight.rs/twilight_http/client/struct.Client.html#method.add_application_emoji
[`Client::update_application_emoji`]: https://api.twilight.rs/twilight_http/client/struct.Client.html#method.update_application_emoji
[`Client::delete_application_emoji`]: https://api.twilight.rs/twilight_http/client/struct.Client.html#method.delete_application_emoji
[`Poll`s]: https://api.twilight.rs/twilight_model/poll/struct.Poll.html
[`Client::role`]: https://api.twilight.rs/twilight_http/client/struct.Client.html#method.role
[`Client::current_user_voice_state`]: https://api.twilight.rs/twilight_http/client/struct.Client.html#method.current_user_voice_state
[`Client::user_voice_state`]: https://api.twilight.rs/twilight_http/client/struct.Client.html#method.user_voice_state
[bitflags]: https://docs.rs/bitflags/2.6.0/bitflags/index.html
[cache-optimization]: https://github.com/twilight-rs/twilight/tree/twilight-cache-inmemory-0.16.0-rc.1/examples/cache-optimization
[fastrand]: https://docs.rs/fastrand/2.0.1/fastrand/index.html
[http]: https://docs.rs/http/1.2.0/http/index.html
[hyper]: https://docs.rs/hyper/1.5.2/hyper/index.html
[rustls]: https://docs.rs/rustls/0.23.20/rustls/index.html
[tokio-tungstenite]: https://docs.rs/tokio-tungstenite/0.21.0/tokio_tungstenite/index.html
[tokio-websockets]: https://docs.rs/tokio-websockets/0.11.0/tokio_websockets/index.html
