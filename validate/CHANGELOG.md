# Changelog

Changelog for `twilight-validate`.

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
[@itohatweb]: https://github.com/itohatweb
[@Erk-]: https://github.com/Erk-
[@laralove143]: https://github.com/laralove143

[0.10.1]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.10.1
[0.10.0]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.10.0
[0.9.2]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.9.2
[0.9.1]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.9.1
[0.9.0]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.9.0
