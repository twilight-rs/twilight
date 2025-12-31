use serde::{Deserialize, Serialize};

use super::Component;

/// Non-interactive [`Component`] container of other (non action row) components.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct ActionRow {
    /// List of components in the action row.
    pub components: Vec<Component>,
    /// Optional identifier for the action row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(ActionRow: components, id);
    assert_impl_all!(ActionRow: Clone, Debug, Eq, Hash, PartialEq, Send, Sync);
}
