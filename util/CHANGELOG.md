# Changelog

Changelog for `twilight-util`.

## [0.5.2] - 2021-07-14

### Additions

Document the `link` feature. This was already present, just not mentioned in any
documentation ([#1011] - [@zeylahellyer]).

[#1011]: https://github.com/twilight-rs/twilight/pull/1011

## [0.5.1] - 2021-07-02

### Additions

Add a calculator for calculating the permissions of a member on a guild level or
in a specific channel, exposed via the `permission-calculator` feature flag
([#834] - [@zeylahellyer]).

Implement the `snowflake::Snowflake` trait on the new `twilight_model::id` types
`ApplicationId`, `CommandId`, `IntegrationId`, and `InteractionId`
([#950] - [@zeylahellyer]).

[#950]: https://github.com/twilight-rs/twilight/pull/950
[#834]: https://github.com/twilight-rs/twilight/pull/834

## [0.5.0] - 2021-06-13

This major version bump of the Util crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no
breaking changes.

## [0.4.1] - 2021-05-30

### Enhancements

`link::webhook::WebhookParseError::kind` is now `const` ([#824] - [@vivian]).

[#824]: https://github.com/twilight-rs/twilight/pull/824

## [0.4.0] - 2021-05-12

### Upgrade Path

The MSRV is now Rust 1.49.

Errors are no longer enums and don't expose their concrete underlying error
source. You can access the underlying error via the implemented
`std::error::Error::source` method or the `into_parts` or `into_source` methods
on each error struct, which will return a boxed `std::error::Error`. To access
the reason for the error use the `kind` or `into_parts` method on error structs;
the returned error type is an enum with variants for each potential reason the
error occurred.

## [0.3.0] - 2021-01-08

This major version bump of the Util crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no
breaking changes.

### Upgrade Path

There is no upgrade path.

### Additions

Add `link::webhook::parse` for parsing webhook IDs and tokens out of webhook
URLs ([#658] - [@vivian]).

[#658]: https://github.com/twilight-rs/twilight/pull/658

## [0.2.0] - 2020-10-30

This major version bump of the Util crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.2.0-beta.0] - 2020-10-10

This major version bump of the Util crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.1.0] - 2020-10-07

Initial release.

[@vivian]: https://github.com/vivian
[@zeylahellyer]: https://github.com/zeylahellyer

[0.5.2]: https://github.com/twilight-rs/twilight/releases/tag/util-0.5.2
[0.5.1]: https://github.com/twilight-rs/twilight/releases/tag/util-0.5.1
[0.5.0]: https://github.com/twilight-rs/twilight/releases/tag/util-0.5.0
[0.4.1]: https://github.com/twilight-rs/twilight/releases/tag/util-0.4.1
[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/util-0.4.0
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/util-v0.3.0
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/util-v0.2.0
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/util-v0.2.0-beta.0
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/util-v0.1.0
