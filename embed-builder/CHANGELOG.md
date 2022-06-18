# Changelog

Changelog for `twilight-embed-builder`.

## [0.11.0] - 2022-05-15

MSRV has been bumped to 1.60.

This major version bump of the Embed Builder crate is done to match all
of the other crates in the ecosystem receiving a major version bump.
There are no changes.

## [0.10.1] - 2022-03-20

The Embed Builder crate has been deprecated in favor of
[`twilight_util::builder::embed`] ([#1539] - [@7596ff], [@vilgotf]).

[`twilight_util::builder::embed`]: https://docs.rs/twilight-util/latest/twilight_util/builder/embed/index.html

[#1539]: https://github.com/twilight-rs/twilight/pull/1539

## [0.10.0] - 2022-03-10

This major version bump of the Embed Builder crate is done to match all
of the other crates in the ecosystem receiving a major version bump.
There are no changes.

## [0.9.0] - 2022-01-22

### Changes

The MSRV has been updated to 1.57 ([#1402] - [@zeylahellyer]).

The Rust edition has been updated to 2021 ([#1412] - [@vilgotf]).

[#1402]: https://github.com/twilight-rs/twilight/pull/1402
[#1412]: https://github.com/twilight-rs/twilight/pull/1412

## [0.8.0] - 2021-12-03

### Changes

`EmbedAuthorBuilder` now properly requires the `name` field ([#1290] -
[@itohatweb]).

[#1290]: https://github.com/twilight-rs/twilight/pull/1290

## [0.7.1] - 2021-10-29

### Changes

Fixes some spelling errors in documentation ([#1223] - [@7596ff]).

[#1223]: https://github.com/twilight-rs/twilight/pull/1223

## [0.7.0] - 2021-10-21

### Changes

The MSRV has been updated to 1.53 ([#1161] - [@7596ff]).

`EmbedBuilder::timestamp` now takes a `Timestamp` instead of a `String`
([#1164] - [@zeylahellyer]).

[#1161]: https://github.com/twilight-rs/twilight/pull/1161
[#1164]: https://github.com/twilight-rs/twilight/pull/1164

## [0.6.0] - 2021-07-31

This major version bump of the Embed Builder crate is done to match all
of the other crates in the ecosystem receiving a major version bump.
There are no changes.

## [0.5.2] - 2021-07-14

### Changes

The description is now validated to a length of 4096 ([#1024] -
[@zeylahellyer]).

[#1024]: https://github.com/twilight-rs/twilight/pull/1024

## [0.5.1] - 2021-07-02

### Enhancements

Improve the `Display` implementation performance on the `EmbedError` by calling
`Formatter` methods directly instead of calling the `format_args!` and `write!`
macros ([#944] - [@zeylahellyer]).

[#944]: https://github.com/twilight-rs/twilight/pull/944

## [0.5.0] - 2021-06-13

This major version bump of the Embed Builder crate is done to match all of the
other crates in the ecosystem receiving a major version bump. There are no
changes.

## [0.4.1] - 2021-05-30

### Enhancements

The following functions are now `const`:

- `EmbedAuthorBuilder::new`
- `EmbedBuilder::new`
- `EmbedFieldBuilder::new`
- `EmbedFieldBuilder::inline`
- `EmbedFooterBuilder::new`
- `EmbedError::kind`
- `image_source::ImageSourceAttachmentError::kind`
- `image_source::ImageSourceUrlError::kind`

([#824] - [@vivian]).

[#824]: https://github.com/twilight-rs/twilight/pull/824

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

[@7596ff]: https://github.com/7596ff
[@itohatweb]: https://github.com/itohatweb
[@vilgotf]: https://github.com/vilgotf
[@zeylahellyer]: https://github.com/zeylahellyer

[0.11.0]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.11.0
[0.10.1]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.10.1
[0.10.0]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.10.0
[0.9.0]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.9.0
[0.8.0]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.8.0
[0.7.1]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.7.1
[0.7.0]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.7.0
[0.6.0]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.6.0
[0.5.2]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.5.2
[0.5.1]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.5.1
[0.5.0]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.5.0
[0.4.1]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.4.1
[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-0.4.0
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.3.0
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.2.0
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/embed-builder-v0.2.0-beta.0
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
