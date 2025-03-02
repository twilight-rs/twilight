use super::Component;

/// Non-interactive [`Component`] container of other (non action row) components.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActionRow {
    pub id: Option<i32>,
    /// List of components in the action row.
    pub components: Vec<Component>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(ActionRow: components);
    assert_impl_all!(ActionRow: Clone, Debug, Eq, Hash, PartialEq, Send, Sync);
}
