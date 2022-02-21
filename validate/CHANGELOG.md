# Changelog

Changelog for `twilight-validate`.

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
[@laralove143]: https://github.com/laralove143

[0.9.2]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.9.2
[0.9.1]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.9.1
[0.9.0]: https://github.com/twilight-rs/twilight/releases/tag/validate-0.9.0
