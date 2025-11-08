# Changelog

## [unreleased]

### Bug Fixes

- resolve clippy 1.86 lints ([#2426](https://github.com/twilight-rs/twilight/issues/2426))
- Quick Fix for Unnecessary Parentheses ([#2440](https://github.com/twilight-rs/twilight/issues/2440))
- Some items were mistyped or private ([#2459](https://github.com/twilight-rs/twilight/issues/2459))
- use smaller and unsigned integer for get-pins limit ([#2477](https://github.com/twilight-rs/twilight/issues/2477))

### Features

- Components V2 ([#2422](https://github.com/twilight-rs/twilight/issues/2422))
- add support for new fields on modify_current_member ([#2473](https://github.com/twilight-rs/twilight/issues/2473))
- [**breaking**] New modal components ([#2461](https://github.com/twilight-rs/twilight/issues/2461))
- new pin endpoints and pagination for Get Channel Pins ([#2475](https://github.com/twilight-rs/twilight/issues/2475))

### Chore

- Update all dependencies ([#2450](https://github.com/twilight-rs/twilight/issues/2450))

## [0.16.0] - 2025-01-12

### Bug Fixes

- Misleading error message validating commands. ([#2329](https://github.com/twilight-rs/twilight/issues/2329))

### Features

- Implement additional select menu types ([#2219](https://github.com/twilight-rs/twilight/issues/2219))
- Add `regex_patterns` and `allow_list` ([#2189](https://github.com/twilight-rs/twilight/issues/2189))
- Implement select menu default values ([#2281](https://github.com/twilight-rs/twilight/issues/2281))
- Add support for premium apps ([#2282](https://github.com/twilight-rs/twilight/issues/2282))
- [**breaking**] Add support for super reaction types ([#2347](https://github.com/twilight-rs/twilight/issues/2347))
- Implement Premium Button Style ([#2363](https://github.com/twilight-rs/twilight/issues/2363))
- Implement user applications ([#2323](https://github.com/twilight-rs/twilight/issues/2323))

### Refactor

- Remove redundant imports ([#2316](https://github.com/twilight-rs/twilight/issues/2316))

### Chore

- resolve rust 1.83 issues ([#2391](https://github.com/twilight-rs/twilight/issues/2391))
- Clarify that MSRV may change in semver-compatible releases ([#2408](https://github.com/twilight-rs/twilight/issues/2408))

## [0.15.2] - 2023-09-10

### Bug Fixes

- clippy 1.72 lints ([#2268](https://github.com/twilight-rs/twilight/issues/2268))
- improve validation of command option descriptions ([#2269](https://github.com/twilight-rs/twilight/issues/2269))

### Documentation

- Fix broken intra-doc links ([#2220](https://github.com/twilight-rs/twilight/issues/2220))

## [0.15.1] - 2023-02-26

### Features

- get thread members request pagination ([#2119](https://github.com/twilight-rs/twilight/issues/2119))
- command option choice validation ([#2123](https://github.com/twilight-rs/twilight/issues/2123))
- support automod custom messages ([#2161](https://github.com/twilight-rs/twilight/issues/2161))

### Refactor

- change deny lints to warn ([#2144](https://github.com/twilight-rs/twilight/issues/2144))

## [0.15.0] - 2023-02-05

### Features

- [**breaking**] channel user limits ([#2077](https://github.com/twilight-rs/twilight/issues/2077))
- [**breaking**] bulk delete message count ([#2078](https://github.com/twilight-rs/twilight/issues/2078))
- [**breaking**] flatten `CommandOptionChoice` ([#2081](https://github.com/twilight-rs/twilight/issues/2081))

## [0.14.2] - 2023-01-20

### Bug Fixes

- properly validate context menu commands ([#2069](https://github.com/twilight-rs/twilight/issues/2069))
- check for option name uniqueness ([#2073](https://github.com/twilight-rs/twilight/issues/2073))

### Features

- add total command char validation ([#1920](https://github.com/twilight-rs/twilight/issues/1920))

## [0.14.1] - 2023-01-07

### Features

- support nsfw commands ([#2019](https://github.com/twilight-rs/twilight/issues/2019))

## [0.14.0] - 2022-11-14

MSRV has been bumped to 1.64 ([#1897] - [@vilgotf]).

`validate` has been affected by the following `model` changes:

- [**breaking**] move related modules under `message` ([#1831](https://github.com/twilight-rs/twilight/issues/1831))
- [**breaking**] update `ChannelType` names ([#1909](https://github.com/twilight-rs/twilight/issues/1909))

### Changes

Due to [**breaking**] flatten `CommandOption` ([#1819] - [@vilgotf]), the logic
of `command::option` and `command::options` have been simplified.

### Features

Validate attachment descriptions ([#1890] - [@itohatweb]). Adds
`message::{attachment, attachment_description}`.

Due to [**breaking**] add create guild ban delete message seconds ([#1884] -
[@itohatweb]), `request::create_guild_ban_delete_message_days` has been replaced
with `..._seconds`, and corresponding error types have been updated
appropriately.

[#1819]: https://github.com/twilight-rs/twilight/pull/1819
[#1884]: https://github.com/twilight-rs/twilight/pull/1884
[#1890]: https://github.com/twilight-rs/twilight/pull/1890
[#1897]: https://github.com/twilight-rs/twilight/pull/1897

## [0.13.2] - 2022-10-28

### Features

- auto moderation http methods and mention spam ([#1846](https://github.com/twilight-rs/twilight/issues/1846))
- forum channels ([#1864](https://github.com/twilight-rs/twilight/issues/1864))

## [0.13.1] - 2022-09-01

### Refactor

- mark c-style enums `#[non_exhaustive]` ([#1862](https://github.com/twilight-rs/twilight/issues/1862))

## [0.12.1] - 2022-07-26

### Documentation

- format doc examples ([#1847](https://github.com/twilight-rs/twilight/issues/1847))

## [0.12.0] - 2022-07-17

### Features

- initial pass at dealing with unknown enum variants ([#1550](https://github.com/twilight-rs/twilight/issues/1550))

## [0.11.1] - 2022-07-07

### Refactor

- remove `test_` prexif from tests ([#1767](https://github.com/twilight-rs/twilight/issues/1767))
- standardize clippy lints ([#1788](https://github.com/twilight-rs/twilight/issues/1788))
- add `#[non_exhaustive]` to c-style enums ([#1795](https://github.com/twilight-rs/twilight/issues/1795))

Changelog for `twilight-validate`.

## [0.11.0] - 2022-05-15

MSRV has been bumped to 1.60.

### Additions

Support validating webhook usernames under the `request` module
([#1586] - [@7596ff]).

### Changes

Rename `COMPONENT_LABEL_LENGTH` constant to `COMPONENT_BUTTON_LABEL_LENGTH`
([#1634] - [@itohatweb]).

[#1634]: https://github.com/twilight-rs/twilight/pull/1634
[#1586]: https://github.com/twilight-rs/twilight/pull/1586

## [0.10.3] - 2022-05-15

### Additions

Support Get Guild Bans request pagination ([#1657] - [@zeylahellyer]).

[#1657]: https://github.com/twilight-rs/twilight/pull/1657

## [0.10.2] - 2022-04-15

### Additions

Add more particular validation for `TextInput::label` ([#1633] - [@itohatweb]).

[#1633]: https://github.com/twilight-rs/twilight/pull/1633

## [0.10.1] - 2022-03-20

### Additions

Validate the maximum hex color for embeds ([#1539] - [@7596ff], [@vilgotf]).

Add validation for `Button` required fields, adding
`ComponentValidationErrorType::{ButtonConflict, ButtonStyle}` ([#1591] -
[@zeylahellyer]).

Separate out the validation logic for each type of component from the
`component` function to individual `action_row`, `button`, `select_menu`, and
`text_input` functions ([#1592] - [@zeylahellyer]). This allows users to
validate components that aren't wrapped in action rows.

### Changes

Update `SELECT_PLACEHOLDER_LENGTH` to 150 ([#1566] - [@itohatweb]).

[#1539]: https://github.com/twilight-rs/twilight/pull/1539
[#1566]: https://github.com/twilight-rs/twilight/pull/1566
[#1591]: https://github.com/twilight-rs/twilight/pull/1591
[#1592]: https://github.com/twilight-rs/twilight/pull/1592

## [0.10.0] - 2022-03-10

### Additions

Add validation for `TextInput`s ([#1300] - [@itohatweb], [@7596ff]):
- add `component_text_input_max`, `component_text_input_min`,
  `component_text_input_placeholder`, `component_text_input_value`
- add `TEXT_INPUT_LENGTH_MAX`, `TEXT_INPUT_LENGTH_MIN`,
  `TEXT_INPUT_PLACEHOLDER_MAX`
- add `ValidationErrorType::{TextInputMaxLength, TextInputMinLength,
  TextInputPlaceholderLength, TextInputValueLength}`

Add validation for audit logs ([#1527] - [@7596ff]):
- add `AUDIT_REASON_MAX`
- add `audit_reason`
- add `ValidationErrorType::AuditReason`

Add validation for attachment filenames ([#1530] - [@7596ff]):
- add `attachment_filename`
- add `MessageValidationErrorType::AttachmentFilename`

### Changes

Rename `message::stickers` to `sticker_ids` ([#1354] - [@7596ff]).

Many integer sizes such as `CREATE_GUILD_BAN_DELETE_MESSAGE_DAYS_MAX` have been
reduced to `u32`s or `u16`s based on their documented maximum value ([#1505] -
[@laralove143]).

[#1300]: https://github.com/twilight-rs/twilight/pull/1300
[#1354]: https://github.com/twilight-rs/twilight/pull/1354
[#1505]: https://github.com/twilight-rs/twilight/pull/1505
[#1527]: https://github.com/twilight-rs/twilight/pull/1527
[#1530]: https://github.com/twilight-rs/twilight/pull/1530

## [0.9.2] - 2022-02-21

### Changes

Support the new `Attachment` variant of `CommandOption` in validation ([#1537] -
[@Erk-]).

[#1537]: https://github.com/twilight-rs/twilight/pull/1537

## [0.9.1] - 2022-02-12

### Additions

Embed validation has two changes ([#1504] - [@laralove143]):
- Add `embed::chars`, and call it from `embed::embed`
- In `message::embeds`, count each embed as comes in and error out if the total
  length is too long

[#1504]: https://github.com/twilight-rs/twilight/pull/1504

## [0.9.0] - 2022-01-22

Initial release ([#1331], [#1395] - [@7596ff], [@baptiste0928]).

[#1331]: https://github.com/twilight-rs/twilight/pull/1331
[#1395]: https://github.com/twilight-rs/twilight/pull/1395

[@7596ff]: https://github.com/7596ff
[@baptiste0928]: https://github.com/baptiste0928
[@Erk-]: https://github.com/Erk-
[@itohatweb]: https://github.com/itohatweb
[@laralove143]: https://github.com/laralove143
[@vilgotf]: https://github.com/vilgotf
[@zeylahellyer]: https://github.com/zeylahellyer

[0.11.0]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.11.0
[0.10.3]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.10.3
[0.10.1]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.10.1
[0.10.0]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.10.0
[0.9.2]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.9.2
[0.9.1]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.9.1
[0.9.0]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.9.0
