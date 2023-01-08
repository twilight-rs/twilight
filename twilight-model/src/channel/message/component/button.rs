use crate::channel::message::ReactionType;
use serde::{Deserialize, Serialize};

/// Clickable [`Component`] below messages.
///
/// [`Component`]: super::Component
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Button {
    /// User defined identifier for the button.
    ///
    /// This field is required when using the following [`ButtonStyle`]s:
    ///
    /// - [`ButtonStyle::DANGER`]
    /// - [`ButtonStyle::PRIMARY`]
    /// - [`ButtonStyle::SECONDARY`]
    /// - [`ButtonStyle::SUCCESS`]
    pub custom_id: Option<String>,
    /// Whether the button is disabled.
    ///
    /// Defaults to `false`.
    pub disabled: bool,
    /// Visual emoji for clients to display with the button.
    pub emoji: Option<ReactionType>,
    /// Text appearing on the button.
    pub label: Option<String>,
    /// Style variant of the button.
    pub style: ButtonStyle,
    /// URL for buttons of a [`ButtonStyle::LINK`] style.
    pub url: Option<String>,
}

/// Style of a [`Button`].
// Keep in sync with `twilight-validate::component`!
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ButtonStyle(u8);

impl ButtonStyle {
    /// Button indicates a primary action.
    ///
    /// Selecting this button style requires specifying the
    /// [`Button::custom_id`] field.
    pub const PRIMARY: Self = Self::new(1);

    /// Button indicates a secondary action.
    ///
    /// Selecting this button style requires specifying the
    /// [`Button::custom_id`] field.
    pub const SECONDARY: Self = Self::new(2);

    /// Button indicates a successful action.
    ///
    /// Selecting this button style requires specifying the
    /// [`Button::custom_id`] field.
    pub const SUCCESS: Self = Self::new(3);

    /// Button indicates a dangerous action.
    ///
    /// Selecting this button style requires specifying the
    /// [`Button::custom_id`] field.
    pub const DANGER: Self = Self::new(4);

    /// Button indicates an action with a link.
    ///
    /// Selecting this button style requires specifying the [`Button::url`]
    /// field.
    pub const LINK: Self = Self::new(5);

    /// Create a new button style from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`DANGER`][`Self::DANGER`].
    pub const fn new(button_style: u8) -> Self {
        Self(button_style)
    }

    /// Retrieve the value of the button style.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::message::component::ButtonStyle;
    ///
    /// assert_eq!(3, ButtonStyle::SUCCESS.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for ButtonStyle {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<ButtonStyle> for u8 {
    fn from(value: ButtonStyle) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(Button: custom_id, disabled, emoji, label, style, url);
    assert_impl_all!(Button: Clone, Debug, Eq, Hash, PartialEq, Send, Sync);

    assert_impl_all!(
        ButtonStyle: Clone,
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
    fn button_style_variants() {
        const MAP: &[(ButtonStyle, u8)] = &[
            (ButtonStyle::PRIMARY, 1),
            (ButtonStyle::SECONDARY, 2),
            (ButtonStyle::SUCCESS, 3),
            (ButtonStyle::DANGER, 4),
            (ButtonStyle::LINK, 5),
        ];

        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "ButtonStyle",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, ButtonStyle::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
