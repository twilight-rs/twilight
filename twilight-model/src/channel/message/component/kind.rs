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
    /// Component is a [`SelectMenu`] with custom string options.
    ///
    /// [`SelectMenu`]: super::SelectMenu
    TextSelectMenu,
    /// Component is an [`TextInput`].
    ///
    /// [`TextInput`]: super::TextInput
    TextInput,
    /// Component is a [`SelectMenu`] for users.
    ///
    /// [`SelectMenu`]: super::SelectMenu
    UserSelectMenu,
    /// Component is a [`SelectMenu`] for roles.
    ///
    /// [`SelectMenu`]: super::SelectMenu
    RoleSelectMenu,
    /// Component is a [`SelectMenu`] for mentionables.
    ///
    /// [`SelectMenu`]: super::SelectMenu
    MentionableSelectMenu,
    /// Component is a [`SelectMenu`] for channels.
    ///
    /// [`SelectMenu`]: super::SelectMenu
    ChannelSelectMenu,
    /// Component is a [`Container`] to display text alongside an accessory component.
    ///
    /// [`Container`]: super::Container
    Section,
    /// Component is a [`TextDisplay`] containing markdown text.
    ///
    /// [`TextDisplay`]: super::TextDisplay
    TextDisplay,
    /// Component is a [`Thumbnail`] that can be used as an accessory.
    ///
    /// [`Thumbnail`]: super::Thumbnail
    Thumbnail,
    /// Component is a [`MediaGallery`] that display images and other media.
    ///
    /// [`MediaGallery`]: super::MediaGallery
    MediaGallery,
    /// Component is a [`FileDisplay`] that displays an attached file.
    ///
    /// [`FileDisplay`]: super::FileDisplay
    File,
    /// Component is a [`Separator`] that adds vertical padding between other components.
    ///
    /// [`Separator`]: super::Separator
    Separator,
    /// Component is a [`Container`] that visually groups a set of components.
    ///
    /// [`Container`]: super::Container
    Container,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for ComponentType {
    fn from(value: u8) -> Self {
        match value {
            1 => ComponentType::ActionRow,
            2 => ComponentType::Button,
            3 => ComponentType::TextSelectMenu,
            4 => ComponentType::TextInput,
            5 => ComponentType::UserSelectMenu,
            6 => ComponentType::RoleSelectMenu,
            7 => ComponentType::MentionableSelectMenu,
            8 => ComponentType::ChannelSelectMenu,
            9 => ComponentType::Section,
            10 => ComponentType::TextDisplay,
            11 => ComponentType::Thumbnail,
            12 => ComponentType::MediaGallery,
            13 => ComponentType::File,
            14 => ComponentType::Separator,
            17 => ComponentType::Container,
            unknown => ComponentType::Unknown(unknown),
        }
    }
}

impl From<ComponentType> for u8 {
    fn from(value: ComponentType) -> Self {
        match value {
            ComponentType::ActionRow => 1,
            ComponentType::Button => 2,
            ComponentType::TextSelectMenu => 3,
            ComponentType::TextInput => 4,
            ComponentType::UserSelectMenu => 5,
            ComponentType::RoleSelectMenu => 6,
            ComponentType::MentionableSelectMenu => 7,
            ComponentType::ChannelSelectMenu => 8,
            ComponentType::Section => 9,
            ComponentType::TextDisplay => 10,
            ComponentType::Thumbnail => 11,
            ComponentType::MediaGallery => 12,
            ComponentType::File => 13,
            ComponentType::Separator => 14,
            ComponentType::Container => 17,
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
            ComponentType::ActionRow => "ActionRow",
            ComponentType::Button => "Button",
            ComponentType::TextSelectMenu
            | ComponentType::UserSelectMenu
            | ComponentType::RoleSelectMenu
            | ComponentType::MentionableSelectMenu
            | ComponentType::ChannelSelectMenu => "SelectMenu",
            ComponentType::TextInput => "TextInput",
            ComponentType::Section => "Section",
            ComponentType::TextDisplay => "TextDisplay",
            ComponentType::Thumbnail => "Thumbnail",
            ComponentType::MediaGallery => "MediaGallery",
            ComponentType::File => "File",
            ComponentType::Separator => "Separator",
            ComponentType::Container => "Container",
            ComponentType::Unknown(_) => "Unknown",
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
        serde_test::assert_tokens(&ComponentType::TextSelectMenu, &[Token::U8(3)]);
        serde_test::assert_tokens(&ComponentType::TextInput, &[Token::U8(4)]);
        serde_test::assert_tokens(&ComponentType::UserSelectMenu, &[Token::U8(5)]);
        serde_test::assert_tokens(&ComponentType::RoleSelectMenu, &[Token::U8(6)]);
        serde_test::assert_tokens(&ComponentType::MentionableSelectMenu, &[Token::U8(7)]);
        serde_test::assert_tokens(&ComponentType::ChannelSelectMenu, &[Token::U8(8)]);
        serde_test::assert_tokens(&ComponentType::Section, &[Token::U8(9)]);
        serde_test::assert_tokens(&ComponentType::TextDisplay, &[Token::U8(10)]);
        serde_test::assert_tokens(&ComponentType::Thumbnail, &[Token::U8(11)]);
        serde_test::assert_tokens(&ComponentType::MediaGallery, &[Token::U8(12)]);
        serde_test::assert_tokens(&ComponentType::File, &[Token::U8(13)]);
        serde_test::assert_tokens(&ComponentType::Separator, &[Token::U8(14)]);
        serde_test::assert_tokens(&ComponentType::Container, &[Token::U8(17)]);
        serde_test::assert_tokens(&ComponentType::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn names() {
        assert_eq!("ActionRow", ComponentType::ActionRow.name());
        assert_eq!("Button", ComponentType::Button.name());
        assert_eq!("SelectMenu", ComponentType::TextSelectMenu.name());
        assert_eq!("SelectMenu", ComponentType::UserSelectMenu.name());
        assert_eq!("SelectMenu", ComponentType::RoleSelectMenu.name());
        assert_eq!("SelectMenu", ComponentType::MentionableSelectMenu.name());
        assert_eq!("SelectMenu", ComponentType::ChannelSelectMenu.name());
        assert_eq!("TextInput", ComponentType::TextInput.name());
        assert_eq!("Unknown", ComponentType::Unknown(99).name());
    }
}
