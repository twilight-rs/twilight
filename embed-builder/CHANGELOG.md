# Changelog

Changelog for `twilight-embed-builder`.

## [0.4.0] - 2021-05-12

### Upgrade Path

The MSRV is now Rust 1.49.

Individual builder methods' errors have been combined into one and now lazily
error when calling `EmbedBuilder::build`. The following code:

```rust
use twilight_embed_builder::{EmbedBuilder, ImageSource};

let embed = EmbedBuilder::new()
    .description("Here's a cool image of Twilight Sparkle")?
    .image(ImageSource::attachment("bestpony.png")?)
    .build();
```

is now written like:

```rust
use twilight_embed_builder::{EmbedBuilder, ImageSource};

let embed = EmbedBuilder::new()
    .description("Here's a cool image of Twilight Sparkle")
    .image(ImageSource::attachment("bestpony.png")?)
    .build()?;
```

This is much more concise with larger embed builders.

Errors are no longer enums and don't expose their concrete underlying error
source. You can access the underlying error via the implemented
`std::error::Error::source` method or the `into_parts` or `into_source` methods
on each error struct, which will return a boxed `std::error::Error`. To access
the reason for the error use the `kind` or `into_parts` method on error structs;
the returned error type is an enum with variants for each potential reason the
error occurred.

### Changes

Simplify error handling by moving all results into the final
`EmbedBuilder::build` method ([#687] - [@vivian]).

[#687]: https://github.com/twilight-rs/twilight/pull/687

## [0.3.0] - 2020-01-08

This major version bump of the Embed Builder is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

### Upgrade Path

There is no upgrade path.

## [0.2.0] - 2020-10-30

This major version bump of the Embed Builder is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.2.0-beta.0] - 2020-10-10

This major version bump of the Embed Builder is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.1.0] - 2020-09-13

Initial release.

[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.4.0
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.3.0
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.2.0
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-v0.2.0-beta.0
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
