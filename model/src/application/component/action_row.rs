use crate::application::component::Component;

/// A non-interactive component that acts as a container for other components.
///
/// Refer to [Discord Docs/Message Components] for additional information.
///
/// [Discord Docs/Message Components]: https://discord.com/developers/docs/interactions/message-components#action-rows
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct ActionRow {
    /// List of components in the action row.
    pub components: Vec<Component>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(ActionRow: components);
    assert_impl_all!(
        ActionRow: Clone,
        Debug,
        Default,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );
}
