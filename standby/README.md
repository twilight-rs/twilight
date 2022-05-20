# twilight-standby

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

Standby is a utility to wait for an event to happen based on a predicate
check. For example, you may have a command that has a reaction menu of ✅
and ❌. If you want to handle a reaction to these, using something like an
application-level state or event stream may not suit your use case. It may
be cleaner to wait for a reaction inline to your function. This is where
Twilight Standby comes in.

Standby allows you to wait for things like an event in a certain guild
([`Standby::wait_for`]), a new message in a channel
([`Standby::wait_for_message`]), a new reaction on a message
([`Standby::wait_for_reaction`]), and any event that might not take place in
a guild, such as a new `Ready` event ([`Standby::wait_for_event`]). Each
method also has a stream variant.

To use Standby it must process events, such as in an event loop of events
received by the gateway. Check out the [`Standby::process`] method.

## When to use futures and streams

[`Standby`] has two variants of each method: a future variant and a stream
variant. An example is [`Standby::wait_for_message`], which also has a
[`Standby::wait_for_message_stream`] variant. The future variant is useful
when you want to oneshot an event that you need to wait for. This means that
if you only need to wait for one message in a channel to come in, you'd use
the future variant. If you need to wait for multiple messages, such as maybe
all of the messages within a minute's timespan, you'd use the
[`Standby::wait_for_message_stream`] method.

The difference is that if you use the futures variant in a loop then you may
miss some events while processing a received event. By using a stream, you
won't miss any events.

## Examples

### At a glance

Wait for a message in channel 123 by user 456 with the content "test":

```rust,no_run
use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    id::Id,
};
use twilight_standby::Standby;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let standby = Standby::new();

    let channel_id = Id::new(123);

    let message = standby.wait_for_message(channel_id, |event: &MessageCreate| {
        event.author.id.get() == 456 && event.content == "test"
    }).await?;

    Ok(())
}
```

### A full example

A full sample bot connecting to the gateway, processing events, and
including a handler to wait for reactions:

```rust,no_run
use futures_util::StreamExt;
use std::{env, error::Error, sync::Arc};
use twilight_gateway::{Event, Intents, Shard};
use twilight_model::{
    channel::Message,
    gateway::payload::incoming::ReactionAdd,
};
use twilight_standby::Standby;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = env::var("DISCORD_TOKEN")?;

    // Start a shard connected to the gateway to receive events.
    let intents = Intents::GUILD_MESSAGES | Intents::GUILD_MESSAGE_REACTIONS;
    let (shard, mut events) = Shard::new(token, intents).await?;
    shard.start().await?;

    let standby = Arc::new(Standby::new());

    while let Some(event) = events.next().await {
        // Have standby process the event, which will fulfill any futures
        // that are waiting for an event.
        standby.process(&event);

        match event {
            Event::MessageCreate(msg) if msg.content == "!react" => {
                tokio::spawn(react(msg.0, Arc::clone(&standby)));
            },
            _ => {},
        }
    }

    Ok(())
}

// Wait for a reaction from the user who sent the message, and then print it
// once they react.
async fn react(
    msg: Message,
    standby: Arc<Standby>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let author_id = msg.author.id;

    let reaction = standby.wait_for_reaction(msg.id, move |event: &ReactionAdd| {
        event.user_id == author_id
    }).await?;

    println!("user reacted with {:?}", reaction.emoji);

    Ok(())
}
```

For more examples, check out each of the methods on [`Standby`].

[codecov badge]: https://img.shields.io/codecov/c/gh/twilight-rs/twilight?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
[codecov link]: https://app.codecov.io/gh/twilight-rs/twilight/
[discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
[discord link]: https://discord.gg/7jj8n7D
[github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
[github link]: https://github.com/twilight-rs/twilight
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
[license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[rust badge]: https://img.shields.io/badge/rust-1.60+-93450a.svg?style=for-the-badge&logo=rust
