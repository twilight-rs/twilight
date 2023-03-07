# twilight-gateway-queue

Ratelimiting functionality for queueing gateway `IDENTIFY` commands.

Discord ratelimits how often shards can send `IDENTIFY` commands to once every 5
seconds per bucket, with a daily limit. This crate provides the [`Queue`] trait
and three default types implementing it as an abstraction for shards to interact
with. Shards must request to identify via [`Queue::request`] before sending the
`IDENTIFY` command.

The three provided implementations of [`Queue`] are:

* [`LargeBotQueue`], for bots with `max_concurrency` > 1
* [`LocalQueue`], for bots with `max_concurrency` = 1
* [`NoOpQueue`], for bots behind a gateway proxy

These implementations do not synchronize across processes, so multi-process bots
must wrap them and, for example, interact with them via HTTP (see
[`gateway-queue`]). An alternative is to implement [`Queue`] yourself with
synchronization across processes, for example, by storing state in redis.

By default, twilight-gateway's `stream` module and `Shard`s use [`LocalQueue`].
This can be overridden via the `ShardBuilder::queue` configuration method.

## Features

* `twilight-http` (*default*): enable the [`LargeBotQueue`] type

[`gateway-queue`]: https://github.com/twilight-rs/gateway-queue
