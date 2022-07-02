# Standby

Standby is a utility to wait for an event to happen based on a predicate check.
For example, you may have a command that makes a reaction menu of ✅ and ❌. If
you want to handle a reaction to these, using something like an
application-level state or event stream may not suit your use case. It may be
cleaner to wait for a reaction inline to your function. This is where Standby
comes in.

## Examples

Wait for a message in channel 123 by user 456 with the content "test":

```rust,no_run
# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    id::Id,
};
use twilight_standby::Standby;

let standby = Standby::new();

// Later on in the application...
let message = standby
    .wait_for_message(
        Id::new(123),
        |event: &MessageCreate| {
            event.author.id == Id::new(456) && event.content == "test"
        },
    )
    .await?;
#     Ok(())
# }
```

## Links

*source*: <https://github.com/twilight-rs/twilight/tree/main/standby>

*docs*: <https://docs.rs/twilight-standby>

*crates.io*: <https://crates.io/crates/twilight-standby>
