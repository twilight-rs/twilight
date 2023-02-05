# Model

`twilight-model` is a crate of models for use with [serde] defining the Discord
APIs with limited implementations on top of them.

These are in a single crate for ease of use, a single point of definition,
and a sort of versioning of the Discord API. Similar to how a database
schema progresses in versions, the definition of the API also progresses in
versions.

Most other Twilight crates use types from this crate. For example, the
[Embed Builder] crate primarily uses types having to do with channel message
embeds, while the [Lavalink] crate works with a few of the events received from
the gateway. These types being in a single versioned definition is beneficial
because it removes the need for crates to rely on other large and unnecessary
crates.

The types in this crate are reproducible: deserializing a payload into a
type, serializing it, and then deserializing it again will result in the same
instance.

Defined are a number of modules defining types returned by or owned by
resource categories. For example, `gateway` contains types used to interact with
and returned by the gateway API. `guild` contains types owned by the Guild
resource category. These types may be directly returned by, built on top of,
or extended by other Twilight crates.

## Id

The `Id` type has a marker depending on which context it is used in. In most
cases it will be possible for the compiler to infer the marker that it needs to
have. This helps ensuring that IDs are not used in the wrong context. If you
need to use the id in a different context you can use the `cast` method which
allow changing the marker type. This is for example helpful to turn a guild ID
into a role ID to get the `@everyone` role. It comes with no additional run-time
cost.

### Example

```rust
use twilight_model::id::{Id, marker::{GuildMarker, RoleMarker}};
# fn main() {

let guild_id: Id<GuildMarker> = Id::new(123);

// To get the everyone role we have to convert the guild id to a role id.
let everyone_role_id: Id<RoleMarker> = guild_id.cast();
# }
```

## Links

*source*: <https://github.com/twilight-rs/twilight/tree/main/twilight-model>

*docs*: <https://docs.rs/twilight-model>

*crates.io*: <https://crates.io/crates/twilight-model>

[Embed Builder]: ./section_7_first_party/section_1_embed_builder.md
[Lavalink]: ./section_7_first_party/section_3_lavalink.md
[serde]: https://serde.rs/
