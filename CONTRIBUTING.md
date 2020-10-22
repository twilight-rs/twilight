# Issues

Issues have three types: bug, feature request, and support. When reporting a bug, you must include
the operating system used, any relevant information about the tech stack, and the feature flags
used. Feature request issues should answer these questions: 

```
- What would you like implemented? What do you want that Twilight lacks?
- What use case does this request address? This should be more than a single project's use case.
- Are you willing to help towards contributing this feature?
- Is there any other information that we should know?
```

Before making a support issue or bug report, be sure to consider joining the
[Discord](https://discord.gg/7jj8n7D) and bringing up your topic in the #support channel.

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

// TODO

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

# Labeling

If you are able, you must label your issues and pull requests appropriately. This includes adding a
label for each applicable crate, or if the issue/change is project-wide, using `c-all`. `feature`s
are new additions, and they are distinct from `enhancement`s, which are improvements on existing
features. `bugfix`es are self-evident. Any change relating to documentation must use the `docs`
label. The `discord api` label is used for changes that must be verified against the Discord API for
correctness.

# Merging

Pull requests require two approvals before merging. The commit must be named with the format
`{pr name} (#{pr number})`. The only possible merge option is squash and merge.
