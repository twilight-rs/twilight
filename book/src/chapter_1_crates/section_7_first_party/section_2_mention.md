# Mention

`twilight-mention` is a utility crate to mention [model] resources.

With this library, you can create mentions for various resources, such as users,
emojis, roles, members, or channels.

## Examples

Create a mention formatter for a user ID, and then format it in a message:

```rust
# #[allow(unused_variables)]
# fn main() {
use twilight_mention::Mention;
use twilight_model::id::{Id, marker::UserMarker};

let user_id: Id<UserMarker> = Id::new(123);
let message = format!("Hey there, {}!", user_id.mention());
# }
```

## Links

*source*: <https://github.com/twilight-rs/twilight/tree/main/twilight-mention>

*docs*: <https://docs.rs/twilight-mention>

*crates.io*: <https://crates.io/crates/twilight-mention>

[model]: ../section_1_model.html
