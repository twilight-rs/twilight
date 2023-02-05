# Version 0.15 - 2023-02-05

Version 0.15 of the Twilight ecosystem brings a new implementation of the
Gateway undertaken over the last year, with quality of life improvements and
bugfixes made in other areas.

With the new gateway implementation finer controls over shards, improved
performance, and new documentation have been introduced. Although overall usage
of the gateway crate is not very dissimilar from existing usage for most use
cases, the core event loop and setting up of shards is different. Besides the
gateway, a number quality of life improvements have been made in the model
crate, with a sprinkling of bugfixes across the ecosystem.

The changelog [is available here](./api_changelog.md).

## New Gateway

We have rewritten the internals of the gateway from scratch, with focuses on
three key areas: performance, control, and simplicity. In the pursuit of
**performance**, the model of awaiting a stream of events from a background task
has been shelved in favor of direct asynchronous polling. Essentially,
background channels and tasks have been removed, and thus removing layers of
asynchronous tasks depending on each other. Everyone always wants to
**control** more with the APIs they're provided, which is why we've dedicated
time to making the gateway API extensible and concise, yet powerful.
Receiving websocket messages, manual message payloads, manual control of groups
of shards, and more is now possible. Being able to understand the implementation
when debugging a problem is vital, which is why we've **simplified** the
implementation. The control flow has been significantly simplified and
documented, while the size of implementation has been slimmed down by 30%.

### Shards

The core usage of a shard is not very dissimilar. While creating a shard without
specialized configuration is still done via [`Shard::new`], creating a shard
with specialized configuration is now done via the [`ConfigBuilder`] and
[`Shard::with_config`]. An stream of gateway events is no longer returned along
with the new shard; instead of awaiting [`Events::next`] in a loop,
[`Shard::next_event`] can be awaited in a loop. Let's take a look at how basic
usage of a shard has changed:

<details>
<summary>Twilight 0.14</summary>

```rust,ignore
let intents = Intents::GUILDS | Intents::GUILD_MODERATION;
let (shard, mut events) = Shard::new(env::var("DISCORD_TOKEN")?, intents);

shard.start().await?;
println!("Created shard");

while let Some(event) = events.next().await {
    println!("Event: {event:?}");
}
```
</details>
<br />
<details>
<summary>Twilight 0.15</summary>

```rust,no_run
# use std::{env, error::Error};
# use twilight_gateway::{Intents, Shard, ShardId};
#
# #[tokio::main] async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
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
# Ok(()) }
```
</details>

Notably, receiving and sending messages now require a *mutable reference* to the
shard instance, as opposed to Twilight 0.14 which only required an immutable
reference. This makes sharing a reference to a shard across tasks for sending
messages and accessing shard information — such as a
[shard's status][`Shard::status`] or [its configuration][`Shard::config`] —
impossible to achieve the same way as with Twilight 0.14. Instead of sharing the
shard itself to spawned tasks it's recommended to provide necessary information
to tasks when they are spawned, or maintaining a mutex of necessary shard
information that is passed around to tasks.

Sending messages — such as member chunk requests or presence updates — over the
shard from spawned tasks is now different: instead of being able to directly
[send a message (0.14)][0.14:`Shard::command`], a [message sender][`Shard::sender`] can
be obtained and passed to tasks. This will allow the sending of messages from
spawned tasks to the shard, to then be sent to Discord's gateway.

These are the primary changes to shards! Some new additions have been made: the
[message ratelimiter][`Shard::ratelimiter`] can now be accessed, statistics
about the [message inflater][`Shard::inflater`] can now be accessed, and the
[gateway session][`Shard::session`] and [connection latency][`Shard::latency`]
are more concise.

### Clusters

Twilight 0.14 had an API surface on top of shards: clusters. Clusters were
essentially a wrapper over shards with the intention of being used for managing
multiple shards within one type. The Cluster API duplicated most of the shard
API, with an equivalent event stream that wrapped multiple shards' streams,
a `Cluster` type that instantiated and owned multiple shards with methods mostly
equvialent to shards' methods, and errors wrapping shard errors.

With Twilight 0.15 the concept of a "cluster" has largely been done away with
and replaced by the [`stream`] module. Our aim with this change was to create
transparency about what is happening under the hood, reduce the API surface, and
reduce complexity.

The module contains three functions for creating groups of shards:

- [`create_recommended`] to create the number of shards Discord recommends;
- [`create_range`] to create the shards within a range; and
- [`create_bucket`] to create the shards within a bucket, necessary for very
  large bot sharding.

These functions all return an iterator of shards. Implementing loops to receive
events from this group of shards can be difficult, so we've provided two types
for collecting shards and efficiently polling all of them:

- [`ShardEventStream`] to poll a group of shards for the next
  [gateway event][`Event`]; and
- [`ShardMessageStream`] to poll a group of shards for the next
  [WebSocket message][`Message`].

Let's take a look at what starting a range of shards and iterating over their
events looks like:

<details>
<summary>Loop over the events of a group of shards</summary>

```rust,no_run
use futures::StreamExt;
use std::{env, error::Error};
use twilight_gateway::{
    stream::{self, ShardEventStream},
    Config,
    Intents,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let config = Config::new(token, Intents::GUILD_MESSAGES);

    // Create a group of shards with IDs 0 through 10, out of a total of 20
    // shards.
    let mut shards = stream::create_range(
        0..10,
        20,
        config, |_, builder| builder.build(),
    ).collect::<Vec<_>>();

    // Create a stream to collect all of the shards and poll them for their next
    // Discord gateway events.
    let mut stream = ShardEventStream::new(shards.iter_mut());

    while let Some((shard, event)) = stream.next().await {
        let event = match event {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                // An error may be fatal when something like invalid privileged
                // intents are specified or the Discord token is invalid.
                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        tracing::debug!(?event, shard = ?shard.id(), "received event");
    }

    Ok(())
}
```
</details>

In each iteration the next received event and the shard that produced the event
are returned. Implementing this kind of stream manually is somewhat trivial to
do, but there are some hidden aspects that make this API particularly efficient.
The shard is a *mutable reference* to the shard. When an iteration is over and
it loops back over, the shard is re-inserted into the stream. Because the stream
is never re-created, the futures polling shards aren't re-created on each loop.
This allows for a constant and fast iteration over shards.

We hope that this thin yet powerful layer over shards will allow for a greater
level of flexibility while not being cumbersome to use. Be sure to check out
[the documentation][gateway documentation] to see the full picture of how the
gateway looks. If you have questions about how to migrate your application to
the new Gateway, please ask in the #support channel in [our Discord server] or
in our [GitHub Discussions]!

## Token Debugging

Previous versions of Twilight derived [`Debug`] on a few types that contain
tokens, such as the HTTP crate's [`Client`] and the gateway's [`Shard`].
Twilight has taken the step to manually derive `Debug` on types containing
tokens to prevent tokens from being printed in logs. A small but important
improvement for security!

## Removal of Guild IDs on Members

[`Member`] models have always had the ID of the guild the user is a part of
stored on them. Discord doesn't actually send the guild ID as part of member
objects. Twilight has always injected the guild ID into members as an ergonomic
improvement because guild IDs have always been in context when deserializing or
retrieving members, such as in [`MemberChunk`] events or when
[fetching a guild's member list][`GetGuildMembers`]. Because Twilight aims to
map the Discord API 1:1 as closely as possible, we've taken the step to remove
guild IDs from members.

When working with members a guild ID should usually be known. For example, the
guild ID is present in the [`MemberAdd`] event and is required along with the
user ID when fetching a member [from the cache][`InMemoryCache::member`].

In the future, one case where a guild ID won't be known is when fetching the
guild member details about the members of a channel thread. This is because only
the channel ID is known, and a guild ID isn't returned. This problem was a
motivating factor for this change. Check out [issue #2058] for more information.

## Command Option Choice Refactoring

[`CommandOptionChoice`]s have been refactored. They were previously an enum with
variants for each type of choice (integers, numbers, and strings). In turn,
these variants contained a data struct with the name, localized names, and value
of the choice. We've simplified these definitions by making
`CommandOptionChoice` a struct containing the name and localized names, with the
value field being the enum with variants for each type of value. This allows for
direct access of a choice's names.

## Guild Widget Settings Support

Fetching information about a guild widget and updating its settings has always
been supported, but last year Discord documented support for fetching the
settings of a guild widget. We've introduced support for this via the new
[`GetGuildWidgetSettings`] request. This returns whether the widget is enabled
and the channel ID the widget points to.

## Allowed Mentions

[`AllowedMentions`] has seen a small touchup. While its documentation has been
greatly improved, [`ParseTypes` (0.14)][0.14:`ParseTypes`] has been renamed to the
clearer [`MentionType`]. The builder for allowed mentions has been removed and
may be brought back into the utilities crate in the future.

## AFK Timeouts

[`Guild::afk_timeout`]s are now stored as the new [`AfkTimeout`] instead of as
an integer. This allows use of valid values of AFK timeouts, and implements a
conversion into a [`Duration`]. Neat! `AfkTimeout` has a getter for accessing
the raw integer, [`AfkTimeout::get`].

[`AfkTimeout`]: https://docs.rs/twilight-model/0.15.0-rc.1/twilight_model/guild/struct.AfkTimeout.html
[`AfkTimeout::get`]: https://docs.rs/twilight-model/0.15.0-rc.1/twilight_model/guild/struct.AfkTimeout.html#method.get
[`AllowedMentions`]: https://docs.rs/twilight-model/0.15.0-rc.1/twilight_model/channel/message/struct.AllowedMentions.html
[`Client`]: https://docs.rs/twilight-http/0.15.0-rc.1/twilight_http/client/struct.Client.html
[`CommandOptionChoice`]: https://docs.rs/twilight-model/0.15.0-rc.1/twilight_model/application/command/struct.CommandOptionChoice.html
[`ConfigBuilder`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/struct.ConfigBuilder.html
[`Debug`]: https://doc.rust-lang.org/stable/std/fmt/trait.Debug.html
[`Duration`]: https://doc.rust-lang.org/stable/std/time/struct.Duration.html
[`Event`]: https://docs.rs/twilight-model/0.15.0-rc.1/twilight_model/gateway/event/enum.Event.html
[`Events::next`]: https://docs.rs/twilight-gateway/0.14.2/twilight_gateway/shard/struct.Events.html
[`GetGuildMembers`]: https://docs.rs/twilight-http/0.15.0-rc.1/twilight_http/request/guild/member/struct.GetGuildMembers.html
[`GetGuildWidgetSettings`]: https://docs.rs/twilight-http/0.15.0-rc.1/twilight_http/client/struct.Client.html#method.guild_widget_settings
[`Guild::afk_timeout`]: https://docs.rs/twilight-model/0.15.0-rc.1/twilight_model/guild/struct.Guild.html#structfield.afk_timeout
[`GuildIntegration`]: https://docs.rs/twilight-model/0.15.0-rc.1/twilight_model/guild/struct.GuildIntegration.html
[`InMemoryCache::member`]: https://docs.rs/twilight-cache-inmemory/0.15.0-rc.1/twilight_cache_inmemory/struct.InMemoryCache.html#method.member
[`Member`]: https://docs.rs/twilight-model/0.15.0-rc.1/twilight_model/guild/struct.Member.html
[`MemberAdd`]: https://docs.rs/twilight-model/0.15.0-rc.1/twilight_model/gateway/payload/incoming/struct.MemberAdd.html
[`MemberChunk`]: https://docs.rs/twilight-model/0.15.0-rc.1/twilight_model/gateway/payload/incoming/struct.MemberChunk.html
[`MentionType`]: https://docs.rs/twilight-model/0.15.0-rc.1/twilight_model/channel/message/enum.MentionType.html
[`Message`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/enum.Message.html
[`Shard`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/struct.Shard.html
[`Shard::config`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/struct.Shard.html#method.config
[`Shard::inflater`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/struct.Shard.html#method.inflater
[`Shard::latency`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/struct.Shard.html#method.latency
[`Shard::new`]: https://docs.rs/twilight-gateway/0.15.0/twilight_gateway/struct.Shard.html#method.new
[`Shard::next_event`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/struct.Shard.html#method.next_event
[`Shard::ratelimiter`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/struct.Shard.html#method.ratelimiter
[`Shard::sender`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/struct.Shard.html#method.sender
[`Shard::session`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/struct.Shard.html#method.session
[`Shard::status`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/struct.Shard.html#method.status
[`Shard::with_config`]: https://docs.rs/twilight-gateway/0.15.0/twilight_gateway/struct.Shard.html#method.with_config
[`ShardEventStream`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/stream/struct.ShardEventStream.html
[`ShardMessageStream`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/stream/struct.ShardMessageStream.html
[`create_bucket`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/stream/fn.create_bucket.html
[`create_range`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/stream/fn.create_range.html
[`create_recommended`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/stream/fn.create_recommended.html
[`stream`]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/stream/index.html
[0.14:`ParseTypes`]: https://docs.rs/twilight-model/0.14.5/twilight_model/channel/message/allowed_mentions/enum.ParseTypes.html
[0.14:`Shard::command`]: https://docs.rs/twilight-gateway/0.14.2/twilight_gateway/shard/struct.Shard.html#method.command
[GitHub Discussions]: https://github.com/twilight-rs/twilight/discussions
[changelog]: ./changelog.md
[gateway documentation]: https://docs.rs/twilight-gateway/0.15.0-rc.2/twilight_gateway/index.html
[issue #2058]: https://github.com/twilight-rs/twilight/issues/2058
[our Discord server]: https://discord.twilight.rs
[version 0.15]: ./summary.md
