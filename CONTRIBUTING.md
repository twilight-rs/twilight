# Issues

Issues have three types: bug, feature request, and support. When reporting a bug, you must include
the operating system used, any relevant information about the tech stack, and the feature flags
used, as specified in the issue template. Feature requests also have an issue template, containing
questions that should be answered in the issue. We aim for 100% coverage of the Discord API; we will
wait until a new feature is supported in the [Discord API documentation] before adding its support
to Twilight. Before making an issue, be sure to consider joining the [Twilight Discord] and bringing
up your topic in the `#support` channel.

# Pull Requests

Pull requests must be named with the format `{crate}: {short description of change}`, and should use
lower case letters. If the change spans more than one crate, separate the crate names with a comma
and a space: `{crate1}, {crate2}: {short description of change}`. Pull requests must be made from a
new branch. Please avoid making pull requests from the `trunk` branch. If adding a feature or
enhancement, use the term `add` or something sufficiently similar. If fixing a bug, use the term
`fix`, or something sufficiently similar. Avoid force-pushing to a pull request branch, as this
erases review comment history.

Contributors should add tests and documentation that reflects their changes.

## Tests

Feature and bugfix commits must always include unit tests to ensure the correctness of the relevant
feature and prevent breakage. Enhancements to existing features without tests should include new
unit tests, especially when the implementation of something is being modified.

Public API types must be tested with the [`static_assertions`] crate.  `static_assertion`'s
`assert_fields`, `assert_impl_all`, and `assert_obj_safe` functionality are notable. Asserting the
implementation of `Send` and `Sync` are of particular importance.

An example of assertions on an Enum may look like this:

```rust
#[derive(Clone, Copy, Debug)]
pub enum PublicEnumType {
    Foo {
        bar: u64,
    },
    Baz {
        qux: i64,
        quz: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::PublicEnumType;
    use static_assertions::{assert_fields, assert_impl_all};

    assert_fields!(PublicEnumType::Foo: bar);
    assert_fields!(PublicEnumType::Baz: qux, quz);
    assert_impl_all!(PublicEnumType: Clone, Copy, Debug, Send, Sync);
}
```

Tests should be concise and non-repetitive: a function doesn't need to be tested with different
inputs multiple times if variance in the input doesn't affect the functionality. The logic of a code
path only needs to be uniquely tested once; testing the same conditions multiple times has no
benefit.

## Documentation

Structs are to be documented as follows:
```rust
/// Short description of the struct.
///
/// Some more information about the struct, specifying all behavior. This can be more than one
/// sentence, and can span multiple lines. It can also contain [named anchors], which should be
/// specified below.
///
/// When documenting struct fields, don't prefix with "The", otherwise most documentation lines
/// would start with "the".
///
/// [named anchors]: https://api.twilight.rs
struct Structy {
    /// Something.
    pub some: String,
    /// Something else.
    pub stuff: String,
}
```

Methods are to be documented as follows:
```rust
impl Structy {
    /// Short decription of the method.
    ///
    /// More important information or clarification.
    pub fn method(&self) -> Option<Something> {

    }
}
```

Examples of other documentation can be found throughout the project. There isn't an exact standard,
but changes that are needed will be requested, on a path towards eventual consistency.

# Labeling

If you are able, you must label your issues and pull requests appropriately. This includes adding a
label for each applicable crate, or if the issue/change is project-wide, using `c-all`. `feature`s
are new additions, and they are distinct from `enhancement`s, which are improvements on existing
features. `bugfix`es are self-evident. Any change relating to documentation must use the `docs`
label. The `discord api` label is used for changes that must be verified against the Discord API for
correctness.

# Merging

Pull requests require two approvals before merging. The only possible merge option is squash and
merge. The commit must be named with the format `{pr name} (#{pr number})`. When merging, add
headers to the commit message that show who approved, merge, and authored the commit. The
`Approved-by` and `Merged-by` headers are self-evident. The header `Signed-off-by` is used to
specify the commit author. Refer to [this example commit] for proper formatting.  Contributors can
use the `-s` flag on `git commit` to automatically sign off their commits.

[Discord API documentation]: https://github.com/discord/discord-api-docs
[Twilight Discord]: https://discord.gg/7jj8n7D
[this example commit]: https://github.com/twilight-rs/twilight/commit/bbab4a39769eac9f7f2d3878184f518a95645966
