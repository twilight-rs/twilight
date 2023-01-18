# Migration guide

Shards are no longer thin clients being easily passed to event handler tasks.
They must instead be actively polled for events, requiring `&mut self`, in a
loop. State, such as the available ratelimit tokens, can be retrieved for event
handler tasks between polling for the next event but sending gateway commands is
a bit more troublesome.
Fortunately, shards expose a `sender()` method returning a `MessageSender`
struct where commands can be queued up and sent back to the shard for it to
relay it to Discord. The `Cluster` type should therefore be replacable by a
`HashMap<ShardId, MessageSender>` together with passing any required state with
events.

This more flexible API makes it much easier to start and shutdown at runtime and
improves performance due to removing a lot of internal complexity.

## Elevator pitch

The new API exposes more advanced toggles in 33% fewer LOC.
The new gateway is 33% fewer LOC

No background task with double linked channels. (performance/control)
Rewritten ratelimiter. (accordance)
Simplified internal code. (mention LOC?)
Improved gateway implementation. (accordance)
    Randomized first heartbeat
    Zombie detection (any event)
    Log incorrect gateway responses
`twilight_http` optional
Documentation
    Reshard example

## The new API

Shards no longer return an additional event stream (ran by a background task)
Shards are now driven through `next_message` or `next_event`.
Sending through `MessageSender`.
Multiple shards through `stream` module.

Shards are no longer thin clients for an actor based background task managed through double ended unbound channels.
This hugely simplifies the internal API and complexity but offloads the polling of shards to users instead of events being provided in an event stream.
Shards no longer spawn a background task
