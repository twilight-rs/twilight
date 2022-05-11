<!-- cargo-sync-readme start -->

# twilight-http-ratelimiting

Ratelimiting functionality for HTTP requests.

Discord ratelimits requests to the HTTP API both globally and per-route.
For more information on the specifics, please take a look at
[Discord's documentation].

This crate provides a common [`Ratelimiter`] trait that all ratelimiter
implementations need to implement.

It also ships a default implementation, [`InMemoryRatelimiter`], that manages
the bucket states in memory.

[Discord's documentation]: https://discord.com/developers/docs/topics/rate-limits

<!-- cargo-sync-readme end -->
