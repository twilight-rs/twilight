use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Type of [`Component`].
///
/// See [Discord Docs/Component Types]
///
/// [`Component`]: super::Component
/// [Discord Docs/Component Types]: https://discord.com/developers/docs/interactions/message-components#component-object-component-types
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum ComponentType {
    /// Component is an [`ActionRow`].
    ///
    /// [`ActionRow`]: super::ActionRow
    ActionRow,
    /// Component is an [`Button`].
    ///
    /// [`Button`]: super::Button
    Button,
    /// Component is an [`StringSelectMenu`].
    ///
    /// [`StringSelectMenu`]: super::StringSelectMenu
    StringSelectMenu,
    /// Component is an [`TextInput`].
    ///
    /// [`TextInput`]: super::TextInput
    TextInput,
    /// Component is an [`TypeSelectMenu`]
    ///
    /// [`TypeSelectMenu`]: super::TypeSelectMenu
    UserSelectMenu,
    /// Component is an [`RoleSelectMenu`]
    ///
    /// [`RoleSelectMenu`]: super::TypeSelectMenu
    RoleSelectMenu,
    /// Component is an [`MentionableSelectMenu`]
    ///
    /// [`MentionableSelectMenu`]: super::TypeSelectMenu
    MentionableSelectMenu,
    /// Component is an [`ChannelSelectMenu`]
    ///
    /// [`ChannelSelectMenu`]: super::ChannelSelectMenu
    ChannelSelectMenu,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for ComponentType {
    fn from(value: u8) -> Self {
        match value {
            1 => ComponentType::ActionRow,
            2 => ComponentType::Button,
            3 => ComponentType::StringSelectMenu,
            4 => ComponentType::TextInput,
            5 => ComponentType::UserSelectMenu,
            6 => ComponentType::RoleSelectMenu,
            7 => ComponentType::MentionableSelectMenu,
            8 => ComponentType::ChannelSelectMenu,
            unknown => ComponentType::Unknown(unknown),
        }
    }
}

impl From<ComponentType> for u8 {
    fn from(value: ComponentType) -> Self {
        match value {
            ComponentType::ActionRow => 1,
            ComponentType::Button => 2,
            ComponentType::StringSelectMenu => 3,
            ComponentType::TextInput => 4,
            ComponentType::UserSelectMenu => 5,
            ComponentType::RoleSelectMenu => 6,
            ComponentType::MentionableSelectMenu => 7,
            ComponentType::ChannelSelectMenu => 8,
            ComponentType::Unknown(unknown) => unknown,
        }
    }
}

impl ComponentType {
    /// Name of the component type.
    ///
    /// Variants have a name equivalent to the variant name itself.
    ///
    /// # Examples
    ///
    /// Check the [`ActionRow`] variant's name:
    ///
    /// ```
    /// use twilight_model::channel::message::component::ComponentType;
    ///
    /// assert_eq!("ActionRow", ComponentType::ActionRow.name());
    /// ```
    ///
    /// [`ActionRow`]: Self::ActionRow
    pub const fn name(self) -> &'static str {
        match self {
            Self::ActionRow => "ActionRow",
            Self::Button => "Button",
            Self::StringSelectMenu => "StringSelectMenu",
            Self::TextInput => "TextInput",
            Self::UserSelectMenu => "UserSelectMenu",
            Self::RoleSelectMenu => "RoleSelectMenu",
            Self::MentionableSelectMenu => "MentionableSelectMenu",
            Self::ChannelSelectMenu => "ChannelSelectMenu",
            Self::Unknown(_) => "Unknown",
        }
    }
}

impl Display for ComponentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        ComponentType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn variants() {
        serde_test::assert_tokens(&ComponentType::ActionRow, &[Token::U8(1)]);
        serde_test::assert_tokens(&ComponentType::Button, &[Token::U8(2)]);
        serde_test::assert_tokens(&ComponentType::StringSelectMenu, &[Token::U8(3)]);
        serde_test::assert_tokens(&ComponentType::TextInput, &[Token::U8(4)]);
        serde_test::assert_tokens(&ComponentType::UserSelectMenu, &[Token::U8(5)]);
        serde_test::assert_tokens(&ComponentType::RoleSelectMenu, &[Token::U8(6)]);
        serde_test::assert_tokens(&ComponentType::MentionableSelectMenu, &[Token::U8(7)]);
        serde_test::assert_tokens(&ComponentType::ChannelSelectMenu, &[Token::U8(8)]);
        serde_test::assert_tokens(&ComponentType::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn names() {
        assert_eq!("ActionRow", ComponentType::ActionRow.name());
        assert_eq!("Button", ComponentType::Button.name());
        assert_eq!("StringSelectMenu", ComponentType::StringSelectMenu.name());
        assert_eq!("TextInput", ComponentType::TextInput.name());
        assert_eq!("UserSelectMenu", ComponentType::UserSelectMenu.name());
        assert_eq!("RoleSelectMenu", ComponentType::RoleSelectMenu.name());
        assert_eq!(
            "MentionableSelectMenu",
            ComponentType::MentionableSelectMenu.name()
        );
        assert_eq!("ChannelSelectMenu", ComponentType::ChannelSelectMenu.name());
        assert_eq!("Unknown", ComponentType::Unknown(99).name());
    }
}
