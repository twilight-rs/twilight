# Crates

Twilight is, at heart, an ecosystem. These components of the ecosystem don't
depend on each other in unnecessary ways, allowing you to pick and choose and
combine the crates that you need for your use case. The crates for Twilight are
categorised into three groups: the *core crates*, *first-party crates*, and
*third-party crates*.

## Core Crates

Twilight includes a few crates which are the "building blocks" to most peoples'
use cases. You might not need them all, but generally speaking you'll need most
of them. Most of them wrap Discord's various APIs.

- [model]: All of the structs, enums, and bitflags used by the Discord APIs.
- [http]: HTTP client supporting all of the documented features of Discord's
  HTTP API, with support for ratelimiting, proxying, and more.
- [gateway]: Clients supporting Discord's gateway API.
- [cache]: Definitions for implementating a cache. An in-process memory
  implementation is included.
- [standby]: Utility for asynchronously waiting for certain events, like a new
  message in a channel or a new reaction to a message.
- [util]: Provides various utilities for use with twilight such as: builders for
  larger structs, permissing calculator to calculate permission of members and
  various extension traits for snowflakes.

## First-Party Crates

There are some first-party crates maintained by the Twilight organization, but
not included in the core experience. These might be for more advanced or
specific use cases or clients for third-party services. An example of a
first-party crate is [`twilight-lavalink`], a Client for interacting with
[Lavalink].

## Third-Party Crates

Third-party crates are crates that aren't officially supported by the
Twilight organization, but are recognised by it.

[`twilight-lavalink`]: ./section_7_first_party/section_3_lavalink.md
[cache]: ./section_4_cache_inmemory.md
[util]: ./section_7_first_party/section_4_util.md
[gateway]: ./section_3_gateway.md
[http]: ./section_2_http.md
[model]: ./section_1_model.md
[standby]: ./section_6_standby.md
[Lavalink]: https://github.com/Frederikam/Lavalink
