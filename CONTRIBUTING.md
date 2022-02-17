# Issues

Issues have three types: bug, feature request, and support. When reporting a
bug, you must include the operating system used, any relevant information about
the tech stack, and the feature flags used, as specified in the issue template.
Feature requests also have an issue template, containing questions that should
be answered in the issue. We aim for 100% coverage of the Discord API; we will
wait until a new feature is supported in the [Discord API documentation] before
adding its support to Twilight. Before making an issue, be sure to consider
joining the [Twilight Discord] and bringing up your topic in the `#support`
channel.

# Errors

Twilight's `Error` system is a struct with one required field (`kind`) and one
optional field (`source`). It includes three methods (`kind`, `into_source`, and
`into_parts`) which allow the user to access data within the error. The return
types for any source errors are `dyn Error`, which allows us to update
dependencies without breaking the public API.

Normally, the fields of the error struct are not public, as they can be accessed
through the provided methods. However, in some cases, you may need to make the
struct fields `pub` or `pub(crate)`.

Any new error implementations must follow this pattern:

```rust
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Error created when something happens.
#[derive(Debug)]
pub struct TwilightError {
    kind: TwilightErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl TwilightError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &TwilightErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (TwilightErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for TwilightError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            TwilightErrorType::AnError => f.write_str("something went wrong"),
            TwilightErrorType::AnotherError { mistake_count } => {
                f.write_str("something else went wrong, ")?;
                Display::fmt(&mistake_count, f)?;

                f.write_str(" mistakes")
            }
        }
    }
}

impl Error for TwilightError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`TwilightError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum TwilightErrorType {
    /// Something went wrong.
    AnError,
    /// Something else went wrong.
    AnotherError {
        /// Amount of mistakes.
        mistake_count: u64
    },
}
```

# Formatters

Macros like `format_args!` and `write!` have runtime performance hits. Instead,
use `core::fmt::Formatter` methods such as `Formatter::write_str` and calling
`Display::fmt` directly.

An example of what *not* to do:

```rust
use std::fmt::{Display, Formatter, Result as FmtResult};

struct Foo {
    number: u64,
}

impl Display for Foo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "the number {} is too high", self.number)
    }
}
```

Instead, write the `Display` implementation like this:

```rust
impl Display for Foo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("the number ")?;
        Display::fmt(&self.number, f)?;

        f.write_str(" is too high")
    }
}
```

# Pull Requests

Pull requests must be named with a short description of the contained changes.
Pull requests must be made from a new branch. Please avoid making pull requests
from the HEAD branch.

Avoid force-pushing to a pull request branch, as this erases review comment
history. You can merge the HEAD branch into your feature branch instead.

Contributors should add tests and documentation that reflects their changes.

## Tests

Feature and bugfix commits must always include unit tests to ensure the
correctness of the relevant feature and prevent breakage. Enhancements to
existing features without tests should include new unit tests, especially when
the implementation of something is being modified.

Public API types must be tested with the [`static_assertions`] crate.
`static_assertions`' `assert_fields`, `assert_impl_all`, and `assert_obj_safe`
functionality are notable. Asserting the implementation of `Send` and `Sync` are
of particular importance.

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

Tests should be concise and non-repetitive: a function doesn't need to be tested
with different inputs multiple times if variance in the input doesn't affect the
functionality. The logic of a code path only needs to be uniquely tested once;
testing the same conditions multiple times has no benefit.

## Documentation

Structs are to be documented as follows:
```rust
/// Short description of the struct, limited to one sentence.
///
/// Some more information about the struct, specifying all behavior. This can be
/// more than one sentence, and can span multiple lines. It can also contain
/// [named anchors], which should be specified below.
///
/// When documenting struct fields, don't prefix with "The", otherwise most
/// documentation lines would start with "the".
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
    /// Short description of the method, limited to one sentence.
    ///
    /// More important information or clarification.
    pub fn method(&self) -> Option<Something> {

    }
}
```

Examples of other documentation can be found throughout the project. There isn't
an exact standard, but changes that are needed will be requested, on a path
towards eventual consistency.

# Labeling

If you are able, you must label your issues and pull requests appropriately.
This includes adding a label for each applicable crate, or if the issue/change
is project-wide, using `c-all`. `t-feature`s are new additions, and they are
distinct from `t-enhancement`s, which are improvements on existing features.
`t-bugfix`es are self-evident. Changes that aren't features, enhancements, or
bugfixes are marked as `t-chore`. Any change relating to documentation must use
the `t-docs` label. The `discord api` label is used for changes that must be
verified against the Discord API for correctness. The `d-unmerged` label is used
when writing functionality based on an unmerged PR in the 
[Discord API documentation].

# Merging

Pull requests require two approvals before merging. The only possible merge
option is squash and merge. Commits must be named with the format
`type({crate}): {short description of change} (#PR)`, and should use lower case
letters. If the change spans more than one crate, separate the crate names with
a comma and a space: `type({crate1},{crate2}): {short description of change}
(#PR)`. In this format, `type` is the type of the commit according to the `t-*`
label set.

[Discord API documentation]: https://github.com/discord/discord-api-docs
[Twilight Discord]: https://discord.gg/twilight-rs
[`static_assertions`]: https://crates.io/crates/static_assertions
