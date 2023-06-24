use crate::channel::{message::ReactionType, ChannelType};
use serde::{Deserialize, Serialize};

/// Dropdown-style [`Component`] that renders below messages.
///
/// [`Component`]: super::Component
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SelectMenu {
    /// An optional list of channel types.
    ///
    /// This option is only used for [channel select menus](SelectMenuType::Channel).
    pub channel_types: Option<Vec<ChannelType>>,
    /// Developer defined identifier.
    pub custom_id: String,
    /// Whether the select menu is disabled.
    ///
    /// Defaults to `false`.
    pub disabled: bool,
    /// This select menu's type.
    ///
    /// [Text select menus](SelectMenuType::Text) *must* also set the `options` field.
    pub kind: SelectMenuType,
    /// Maximum number of options that may be chosen.
    pub max_values: Option<u8>,
    /// Minimum number of options that must be chosen.
    pub min_values: Option<u8>,
    /// A list of available options.
    ///
    /// This value is only used and required by [text select menus](SelectMenuType::Text).
    pub options: Option<Vec<SelectMenuOption>>,
    /// Custom placeholder text if no option is selected.
    pub placeholder: Option<String>,
}

/// A [`SelectMenu`]'s type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SelectMenuType {
    /// Select menus with a text-based `options` list.
    ///
    /// Select menus of this `kind` *must* set the `options` field to specify the options users
    /// can pick from.
    Text,
    /// User select menus.
    User,
    /// Role select menus.
    Role,
    /// Mentionable select menus.
    Mentionable,
    /// Channel select menus.
    ///
    /// Select menus of this `kind` *can* use the `channel_types` field to specify which types of
    /// channels are selectable.
    Channel,
}

/// Dropdown options that are part of [`SelectMenu`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SelectMenuOption {
    /// Whether the option will be selected by default.
    #[serde(default)]
    pub default: bool,
    /// Additional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Emoji associated with the option. Appears left of the label and
    /// description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<ReactionType>,
    /// User-facing name.
    pub label: String,
    /// Developer defined value.
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        SelectMenu: channel_types,
        custom_id,
        disabled,
        kind,
        max_values,
        min_values,
        options,
        placeholder
    );
    assert_impl_all!(SelectMenu: Clone, Debug, Eq, Hash, PartialEq, Send, Sync);

    assert_impl_all!(
        SelectMenuType: Clone,
        Debug,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );

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
}
