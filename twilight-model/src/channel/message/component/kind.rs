use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Type of [`Component`].
///
/// [`Component`]: super::Component
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
    /// Component is an [`SelectMenu`].
    ///
    /// [`SelectMenu`]: super::SelectMenu
    SelectMenu,
    /// Component is an [`TextInput`].
    ///
    /// [`TextInput`]: super::TextInput
    TextInput,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for ComponentType {
    fn from(value: u8) -> Self {
        match value {
            1 => ComponentType::ActionRow,
            2 => ComponentType::Button,
            3 => ComponentType::SelectMenu,
            4 => ComponentType::TextInput,
            unknown => ComponentType::Unknown(unknown),
        }
    }
}

impl From<ComponentType> for u8 {
    fn from(value: ComponentType) -> Self {
        match value {
            ComponentType::ActionRow => 1,
            ComponentType::Button => 2,
            ComponentType::SelectMenu => 3,
            ComponentType::TextInput => 4,
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
            Self::SelectMenu => "SelectMenu",
            Self::TextInput => "TextInput",
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
        serde_test::assert_tokens(&ComponentType::SelectMenu, &[Token::U8(3)]);
        serde_test::assert_tokens(&ComponentType::TextInput, &[Token::U8(4)]);
        serde_test::assert_tokens(&ComponentType::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn names() {
        assert_eq!("ActionRow", ComponentType::ActionRow.name());
        assert_eq!("Button", ComponentType::Button.name());
        assert_eq!("SelectMenu", ComponentType::SelectMenu.name());
        assert_eq!("TextInput", ComponentType::TextInput.name());
        assert_eq!("Unknown", ComponentType::Unknown(99).name());
    }
}
