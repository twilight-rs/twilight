use crate::channel::ReactionType;
use serde::{Deserialize, Serialize};

use super::Component;

/// Dropdown-style interactive components that render on messages.
///
/// Refer to [Discord Docs/Message Components] for additional information.
///
/// [Discord Docs/Message Components]: https://discord.com/developers/docs/interactions/message-components#select-menus
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SelectMenu {
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
    /// List of available choices.
    pub options: Vec<SelectMenuOption>,
    /// Custom placeholder text if no option is selected.
    pub placeholder: Option<String>,
}

/// Dropdown options that are part of [`SelectMenu`].
///
/// Refer to [Discord Docs/Message Components] for additional information.
///
/// [Discord Docs/Message Components]: https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SelectMenuOption {
    /// Whether the option will be selected by default.
    #[serde(default)]
    pub default: bool,
    /// Additional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<ReactionType>,
    /// User-facing name.
    pub label: String,
    /// Developer defined value.
    pub value: String,
}

impl From<SelectMenu> for Component {
    fn from(select_menu: SelectMenu) -> Self {
        Self::SelectMenu(select_menu)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        SelectMenu: custom_id,
        disabled,
        max_values,
        min_values,
        options,
        placeholder
    );
    assert_impl_all!(SelectMenu: Clone, Debug, Eq, Hash, PartialEq, Send, Sync);

    assert_impl_all!(
        SelectMenuOption: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
    assert_fields!(SelectMenuOption: default, description, emoji, label, value);

    assert_impl_all!(Component: From<SelectMenu>);
}
