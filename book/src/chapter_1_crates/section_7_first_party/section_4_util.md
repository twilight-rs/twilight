# Util

`twilight-util` is a utility crate that adds utilities to the twilight
ecosystem that do not fit in any other crate. One example feature of the crate
is a trait to make extracting data from Discord identifiers (snowflakes) easier.

## Features

`twilight-util` by default exports nothing. Features must be individually
enabled via feature flags.

### Builder

The `builder` feature enables builders for large structs. At the time of
writing, it contains the following builders:
- [`CommandBuilder`]
- [`EmbedBuilder`]
- [`InteractionResponseData`]

#### Command example

Create a command that can be used to send a animal picture in a
certain category:

```rust
# fn main() {
use twilight_model::application::command::CommandType;
use twilight_util::builder::command::{BooleanBuilder, CommandBuilder, StringBuilder};

CommandBuilder::new(
    "blep",
    "Send a random adorable animal photo",
    CommandType::CHAT_INPUT,
)
.option(
    StringBuilder::new("animal", "The type of animal")
        .required(true)
        .choices([
            ("Dog", "animal_dog"),
            ("Cat", "animal_cat"),
            ("Penguin", "animal_penguin"),
        ]),
)
.option(BooleanBuilder::new(
    "only_smol",
    "Whether to show only baby animals",
));
# }
```

#### Embed examples

Build a simple embed:

```rust
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

let embed = EmbedBuilder::new()
    .description("Here's a list of reasons why Twilight is the best pony:")
    .field(EmbedFieldBuilder::new("Wings", "She has wings.").inline())
    .field(EmbedFieldBuilder::new("Horn", "She can do magic, and she's really good at it.").inline())
    .build();
#     Ok(())
# }
```

Build an embed with an image:

```rust
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use twilight_util::builder::embed::{EmbedBuilder, ImageSource};

let embed = EmbedBuilder::new()
    .description("Here's a cool image of Twilight Sparkle")
    .image(ImageSource::attachment("bestpony.png")?)
    .build();
#     Ok(())
# }
```

### Link

The `link` feature enables the parsing and formatting of URLs to resources, such
as parsing and formatting webhook links or links to a user's avatar.

#### Examples

Parse a webhook URL with a token:

```rust,no_run
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use twilight_model::id::Id;
use twilight_util::link::webhook;

let url = "https://discord.com/api/webhooks/794590023369752587/tjxHaPHLKp9aEdSwJuLeHhHHGEqIxt1aay4I67FOP9uzsYEWmj0eJmDn-2ZvCYLyOb_K";

let (id, token) = webhook::parse(url)?;
assert_eq!(Id::new(794590023369752587), id);
assert_eq!(
    Some("tjxHaPHLKp9aEdSwJuLeHhHHGEqIxt1aay4I67FOP9uzsYEWmj0eJmDn-2ZvCYLyOb_K"),
    token,
);
# Ok(()) }
```

### Permission Calculator

The `permission-calculator` feature is used for calculating the permissions
of a member in a channel, taking into account its roles and permission
overwrites.

### Snowflake

The `snowflake` feature calculates information out of snowflakes, such as the
timestamp or the ID of the worker that created it.

#### Examples

Retrieve the timestamp of a snowflake in milliseconds from the Unix epoch as a
64-bit integer:

```rust
# #[allow(unused_variables)]
# fn main() {
use twilight_util::snowflake::Snowflake;
use twilight_model::id::{Id, marker::UserMarker};

let user: Id<UserMarker> = Id::new(123456);
let timestamp = user.timestamp();
# }
```


## Links

*source*: <https://github.com/twilight-rs/twilight/tree/main/util>

*docs*: <https://docs.rs/twilight-util>

*crates.io*: <https://crates.io/crates/twilight-util>

[`CommandBuilder`]: https://api.twilight.rs/twilight_util/builder/command/struct.CommandBuilder.html
[`EmbedBuilder`]: https://api.twilight.rs/twilight_util/builder/embed/struct.EmbedBuilder.html
[`InteractionResponseDataBuilder`]: https://api.twilight.rs/twilight_util/builder/struct.InteractionResponseDataBuilder.html
