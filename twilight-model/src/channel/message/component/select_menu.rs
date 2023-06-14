use crate::channel::{message::ReactionType, ChannelType};
use serde::{Deserialize, Serialize};

/// Dropdown-style [`Component`] that renders below messages.
///
/// Use the `data` field to determine which kind of select menu you want. The kinds available at the moment are listed
/// as [`SelectMenuData`]'s variants.
///
/// [`Component`]: super::Component
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SelectMenu {
    /// Developer defined identifier.
    pub custom_id: String,
    /// Data specific to this select menu's kind.
    pub data: SelectMenuData,
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

/// Data specific to a kind of [`SelectMenu`].
///
/// Choosing a variant of this enum implicitly sets the select menu's kind.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SelectMenuData {
    /// Data specific to text select menus.
    ///
    /// Choosing this variant for your select menu makes the menu a [`ComponentType::TextSelectMenu`].
    ///
    /// [`ComponentType::TextSelectMenu`]: super::ComponentType::TextSelectMenu
    Text(TextSelectMenuData),
    /// Data specific to user select menus.
    ///
    /// Choosing this variant for your select menu makes the menu a [`ComponentType::UserSelectMenu`].
    ///
    /// [`ComponentType::UserSelectMenu`]: super::ComponentType::UserSelectMenu
    User,
    /// Data specific to role select menus.
    ///
    /// Choosing this variant for your select menu makes the menu a [`ComponentType::RoleSelectMenu`].
    ///
    /// [`ComponentType::RoleSelectMenu`]: super::ComponentType::RoleSelectMenu
    Role,
    /// Data specific to mentionable select menus.
    ///
    /// Choosing this variant for your select menu makes the menu a [`ComponentType::MentionableSelectMenu`].
    ///
    /// [`ComponentType::MentionableSelectMenu`]: super::ComponentType::MentionableSelectMenu
    Mentionable,
    /// Data specific to channel select menus.
    ///
    /// Choosing this variant for your select menu makes the menu a [`ComponentType::ChannelSelectMenu`].
    ///
    /// [`ComponentType::ChannelSelectMenu`]: super::ComponentType::ChannelSelectMenu
    Channel(ChannelSelectMenuData),
}

/// Data specific to text select menus.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TextSelectMenuData {
    /// A list of available choices for this select menu.
    pub options: Vec<SelectMenuOption>,
}

/// Data specific to channel select menus.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ChannelSelectMenuData {
    /// An optional list of channel types to include in this select menu.
    ///
    /// If `None`, the select menu will display all channel types.
    pub channel_types: Option<Vec<ChannelType>>,
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
        SelectMenu: custom_id,
        data,
        disabled,
        max_values,
        min_values,
        placeholder
    );
    assert_impl_all!(SelectMenu: Clone, Debug, Eq, Hash, PartialEq, Send, Sync);

    assert_impl_all!(
        SelectMenuData: Clone,
        Debug,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );
    assert_impl_all!(
        TextSelectMenuData: Clone,
        Debug,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );
    assert_impl_all!(
        ChannelSelectMenuData: Clone,
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
