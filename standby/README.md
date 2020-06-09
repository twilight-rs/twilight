<!-- cargo-sync-readme start -->

# twilight-standby

Standby is a utility to wait for an event to happen based on a predicate
check. For example, you may have a command that has a reaction menu of ✅ and
❌. If you want to handle a reaction to these, using something like an
application-level state or event stream may not suit your use case. It may
be cleaner to wait for a reaction inline to your function. This is where
Twilight Standby comes in.

Standby allows you to wait for things like an event in a certain guild
([`Standby::wait_for`]), a new message in a channel
([`Standby::wait_for_message`]), a new reaction on a message
([`Standby::wait_for_reaction`]), and any event that might not take place in
a guild, such as a new `Ready` event ([`Standby::wait_for_event`]).

To use Standby, you must process events with it in your main event loop.
Check out the [`Standby::process`] method.

# Examples

Wait for a message in channel 123 by user 456 with the content "test":

```no_run
use futures_util::future;
use twilight_model::{gateway::payload::MessageCreate, id::{ChannelId, UserId}};
use twilight_standby::Standby;

let standby = Standby::new();

let message = standby.wait_for_message(ChannelId(123), |event: &MessageCreate| {
    event.author.id == UserId(456) && event.content == "test"
}).await?;
```

For more examples, check out each method.

[`Standby::process`]: struct.Standby.html#method.process
[`Standby::wait_for`]: struct.Standby.html#method.wait_for
[`Standby::wait_for_event`]: struct.Standby.html#method.wait_for_event
[`Standby::wait_for_message`]: struct.Standby.html#method.wait_for_message
[`Standby::wait_for_reaction`]: struct.Standby.html#method.wait_for_reaction

<!-- cargo-sync-readme end -->
