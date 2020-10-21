# Issues

Issues have three types: bug, feature request, and support. When reporting a bug, you must include
the operating system used, any relevant information about the tech stack, and the feature flags
used. 

# Pull Requests

Pull requests must be named with the format `crate: short description of change`, and should use
lower case letters. If the change spans more than one crate, separate the crate names with a comma
and a space: `crate1, crate2: short description of change`. Always make a pull request from a new
branch, that is named similarly, but with only a few words: `crate-short-description`, or
`crate1-crate2-short-description`. If adding a feature or enhancement, use the term `add` or
something sufficiently similar. If fixing a bug, use the term `fix`, or something sufficiently
similar. Avoid force-pushing to a pull request branch, as this erases review comment history.

# Labeling

If you are able, you must label your issues and pull requests appropriately. This includes adding a
label for each applicable crate, or if the issue/change is project-wide, using `c-all`. `feature`s
are new additions, and they are distinct from `enhancement`s, which are improvements on existing
features.  `bugfix`es are self-evident. Any change relating to documentation must use the `docs`
label. The `discord api` label is used for changes that must be verified against the Discord API for
correctness.

# Merging

Pull requests require two approvals before merging. They must be merged with the same name as the
pull request, and as such must follow the naming rules for pull requests. The only possible merge
option is squash and merge. Add the headers `Approved-by`, `Merged-by`, and `Signed-off-by` (the
author of the pull request) to the end of the merge description. These headers must contain data in
the format `Name <email@example.com>`, unless that data is not present.
