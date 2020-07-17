# Development

## Making a commit

Before working on the project, be sure to run `cargo test` at least once from
the workspace root. This will install [`rusty-hook`], which will add some git
hooks. One of these will be a pre-commit hook, which will run tests, clippy, and
rustfmt before allowing you to commit. This will help make sure pull requests
usually build successfully the first time around.

[`rusty-hook`]: https://github.com/swellaby/rusty-hook
