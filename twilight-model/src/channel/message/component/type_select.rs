/// Dropdown-style [`Component`] that renders belew messages with pre-populated data.
///
/// [`Component`]: super::Component
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypeSelectMenu {
    /// Developer defined identifier.
    pub custom_id: String,
    /// Whether the select menu is disabled.
    ///
    /// Defaults to `false`.
    pub disabled: bool,
    /// Maximum number of options that may be chosen.
    pub max_values: Option<u8>,
    /// Minimum number of options that must be chosen.
    pub min_values: Option<u8>,
    /// Custom placeholder text if no option is selected.
    pub placeholder: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        TypeSelectMenu: custom_id,
        disabled,
        max_values,
        min_values,
        placeholder
    );
    assert_impl_all!(
        TypeSelectMenu: Clone,
        Debug,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );
}
