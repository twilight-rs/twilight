<!-- cargo-sync-readme start -->

# twilight-embed-builder

Builders for creating an embed, useful when creating or updating messages.

If uploading an image as an attachment, set as the image or thumbnail with
`attachment://{filename}.{extension}`. Refer to [the discord docs] for more information.

# Examples

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
use twilight_embed_builder::EmbedBuilder;

let embed = EmbedBuilder::new()
    .description("Here's a cool image of Twilight Sparkle")?
    .image("attachment://bestpony.png")
    .build();

```

[the discord docs]: https://discord.com/developers/docs/resources/channel#create-message-using-attachments-within-embeds

<!-- cargo-sync-readme end -->
