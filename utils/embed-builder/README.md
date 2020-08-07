<!-- cargo-sync-readme start -->

# twilight-embed-builder

`twilight-embed-builder` is a set of builder for the [`twilight-rs`]
ecosystem to create a message embed, useful when creating or updating
messages.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
twilight-embed-builder = { branch = "trunk", git = "https://github.com/twilight-rs/twilight" }
```

## Examples

Build a simple embed:

```rust,no_run
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder};

let embed = EmbedBuilder::new()
    .description("Here's a list of reasons why Twilight is the best pony:")?
    .field(EmbedFieldBuilder::new("Wings", "She has wings.")?.inline())
    .field(EmbedFieldBuilder::new("Horn", "She can do magic, and she's really good at it.")?.inline())
    .build();
```

Build an embed with an image:

```rust,no_run
use twilight_embed_builder::{EmbedBuilder, ImageSource};

let embed = EmbedBuilder::new()
    .description("Here's a cool image of Twilight Sparkle")?
    .image(ImageSource::attachment("bestpony.png")?)
    .build();

```

[`twilight-rs`]: https://github.com/twilight-rs/twilight
[the discord docs]: https://discord.com/developers/docs/resources/channel#create-message-using-attachments-within-embeds

<!-- cargo-sync-readme end -->
