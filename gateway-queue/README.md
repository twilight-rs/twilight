# twilight-gateway-queue

Ratelimiting functionality for queueing new gateway sessions.

The gateway ratelimits how often clients can initialize new sessions.
Instances of a queue are given to shards so that they can request to
initialize a session.

Queue implementations must point to the same broker so that all shards
across all clusters, processes, and other forms of multi-serviced
applications, can work together and use the same ratelimiting source. That
is, if you for example have two clusters in two different processes, then
the two processes must use some unified form of ratelimiting: this can
either mean using IPC to communicate ratelimiting or a broker.

## Provided queues

Most users only need the [`LocalQueue`]: it's a single-process queue for
smaller bots. Larger bots need the [`LargeBotQueue`], which supports
single-process [Sharding for Very Large Bots] through the use of bucket
releasing.

By default, the gateway's `Cluster` and `Shard`s use the [`LocalQueue`]. You
can override this in the `ClusterBuilder::queue` and `ShardBuilder::queue`
configuration methods.

## Advanced use cases

Large bots, and smaller bots out of design, may need to implement their own
queue. The most common reason to need this is if you have clusters in
multiple processes. You'll need a broker to manage ratelimiting across them
all so a [`Queue`] trait is provided that shards can use to make requests to
create sessions.

## Features

### Twilight-HTTP

The `twilight-http` feature brings in support for [`LargeBotQueue`].

This is enabled by default.

### Tracing

The `tracing` feature enables logging via the [`tracing`] crate.

This is enabled by default.

[`tracing`]: https://crates.io/crates/tracing
[Sharding for Very Large Bots]: https://discord.com/developers/docs/topics/gateway#sharding-for-very-large-bots
