# Contributing

Thank you for considering adding your contribution to Twilight! This project
would not be what it is without the support of the community. This document
contains rules and guidelines that should be followed when making contributions
to Twilight. 

## Project Management

### Issues

There are two major kinds of issues in Twilight, user-created and
developer-created. Of the user-created issues, they are usually bug reports,
feature requests, or support requests. If reporting a bug, please include the
operating system used, any relevant information about the tech stack, and the
feature flags used. Before making a support issue, be sure to consider joining
the [Twilight Discord] and making a thread for your question in the `#support`
channel.

Developer-created issues are most often bugs or tracking issues for supporting
Discord API features. We aim for 100% coverage of the Discord API; we will wait
until a new feature is supported in the [Discord API] before adding its support
to Twilight.

### Pull Requests

In order to ease developer cooperation, please make a pull request from a
feature branch on your own fork instead of from `main` or `next`. Additionally,
since we squash every commit before merging, force-pushing a branch with new
changes or updates is not always required. This helps with maintaining a
chronological history in the PR story.

Contributors must add tests and documentation that reflects their changes.

#### Format

Twilight follows a [Conventional Commit] style for pull request titles and
commits, using [Angular]'s types.

The title of the PR must follow this style:

`<type>(<scope>)<! if breaking>: <short summary>`

Acceptable values of the `type` field:
- `build`: Changes that affect the build system or external dependencies (example
  scopes: gulp, broccoli, npm)
- `ci`: Changes to our CI configuration files and scripts (examples: CircleCi,
  SauceLabs)
- `docs`: Documentation only changes
- `feat`: A new feature
- `fix`: A bug fix
- `perf`: A code change that improves performance
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `test`: Adding missing tests or correcting existing tests

Acceptable scopes are any crate in the project. Add an exclamation point after
the scope (outside the parentheses) if the change is breaking.

The PR description must contain a short description of the changes.

#### Labeling

Labels that must be added manually:
- `d-api`: for changes that correspond with the [Discord API]
- `d-breaking`: for API changes that cause a breaking change in library code
- `d-unmerged`: changes that have not yet been merged into the documentation
- `w-do-not-merge`: merge is blocked on another PR, or is not ready for review
- `w-needs-more-docs`: a change needs more documentation in the code itself or
  in the PR description
- `w-needs-testing`: a change needs more tests added or proof of manual testing
- `w-unapproved`: a design change has not yet been approved by library
  maintainers

#### Branches

Any kind of change can be made to the `main` branch, unless it is breaking.
`fix` changes may target `main` if the current functionality is broken.
Generally, `ci`, `docs`, `feat`, `fix`, `perf`, and `test` pull requests are
made targeting the `main` branch, while larger `refactor` pull requests target
`next`. 

#### Merging

Pull requests require two approvals before merging.

### Releases

Twilight uses `git-cliff` and `cargo-release` for releases. These are both
available as binaries from `crates.io`. They are executed via
`gen-changelogs.sh` and `release.sh` respectively. Invoking `release.sh` with
extra arguments passes them to the saved `cargo-release` command.

Steps to create a patch release across the workspace:
- generate the changelogs: `$ ./gen-changelogs.sh`
- customize the changelogs as necessary, and commit them
- preview the release process: `$ ./release.sh patch -vv`
- execute the release: `$ ./release.sh patch --execute`

## Code Style

Not all of our codebase follows this style. But, in an effort to reach
consistency and clarity, we require all new PRs to follow it. For parts of the
codebase that do not yet follow this document, they may be updated
module-by-module or crate-by-crate, depending on the size of the target.

### Enums and Structs

Struct fields should be sorted alphabetically. Enum variants should be as well,
except in some cases, where the enum represents an integer value, in which case
the enum variants should be sorted by that value.

### Errors

Twilight's `Error` design is a struct with one required field (`kind`) and one
optional field (`source`). It includes three methods (`kind`, `into_source`, and
`into_parts`) which allow the user to access data within the error. The return
types for any source errors are `dyn Error`, which allows us to update
dependencies without breaking the public API.

Normally, the fields of the error struct are not public, as they can be accessed
through the provided methods. However, in some cases, you may need to make the
struct fields `pub(crate)`.

New error implementations must follow this pattern:

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

### Formatters

Macros like `format_args!` and `write!` have runtime performance hits. Instead,
use `core::fmt::Formatter` methods such as `Formatter::write_str`, or call
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

## Tests

Feature and bugfix commits must include unit tests to ensure the correctness of
the relevant feature and prevent breakage. Enhancements to existing features
without tests must include new unit tests, especially when the implementation of
something is being modified.

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

We strive to follow these guidelines, however, some parts of the code have not
been updated to do so.

## Documentation

We are slowly approaching a standard documentation style. Changes that are
needed will be requested, on a path towards eventual consistency. Below are some
examples of what we require.

### Structs

```rust
/// Short description of the struct, limited to one sentence.
///
/// Some more information about the struct, specifying all behavior. This can be
/// more than one sentence, and can span multiple lines. It can also contain
/// [shortcut reference links].
///
/// When documenting struct fields, don't prefix with "The", otherwise most
/// documentation lines would start with "the".
///
/// [shortcut reference links]: https://spec.commonmark.org/0.30/#shortcut-reference-link
struct Structy {
    /// Something.
    pub some: String,
    /// Something else.
    pub stuff: String,
}
```

### Methods

```rust
impl Structy {
    /// Short description of the method, limited to one sentence.
    ///
    /// More important information or clarification.
    /// 
    /// # Errors
    /// 
    /// Returns an error of type [`SomethingWentWrong`] if something went wrong.
    /// 
    /// [`SomethingWentWrong`]: SomethingErrorType::SomethingWentWrong
    pub fn method(&self) -> Result<Something, SomethingError> {

    }
}
```

### Discord API Docs links

When linking to the Discord API documentation, the link must be prefixed with
the word "See", and the anchor must be formatted as "Discord Docs/Page Title":
```rust
/// Edit a global command, by ID.
///
/// You must specify a name and description. See
/// [Discord Docs/Edit Global Application Command].
///
/// [Discord Docs/Edit Global Application Command]: https://discord.com/developers/docs/interactions/application-commands#edit-global-application-command
pub const fn update_global_command(
```

[`static_assertions`]: https://crates.io/crates/static_assertions
[Angular]: https://github.com/angular/angular/blob/13.2.x/CONTRIBUTING.md#type
[Conventional Commit]: https://www.conventionalcommits.org/en/v1.0.0/
[Discord API]: https://github.com/discord/discord-api-docs
[Twilight Discord]: https://discord.gg/twilight-rs
