use crate::channel::ReactionType;
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::Component;

/// Clickable interactive components that render on messages.
///
/// See [Discord Docs/Message Components].
///
/// [Discord Docs/Message Components]: https://discord.com/developers/docs/interactions/message-components#button-object-button-structure
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Button {
    /// User defined identifier for the button.
    ///
    /// This field is required when using the following [`ButtonStyle`]s:
    ///
    /// - [`ButtonStyle::Danger`]
    /// - [`ButtonStyle::Primary`]
    /// - [`ButtonStyle::Secondary`]
    /// - [`ButtonStyle::Success`]
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
    /// URL for buttons of a [`ButtonStyle::Link`] style.
    pub url: Option<String>,
}

/// Style of a [`Button`].
///
/// Refer to [the Discord Docs/Button Object] for additional information.
///
/// [the Discord Docs/Button Object]: https://discord.com/developers/docs/interactions/message-components#button-object-button-styles
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Serialize_repr)]
#[repr(u8)]
pub enum ButtonStyle {
    /// Button indicates a primary action.
    ///
    /// Selecting this button style requires specifying the
    /// [`Button::custom_id`] field.
    Primary = 1,
    /// Button indicates a secondary action.
    ///
    /// Selecting this button style requires specifying the
    /// [`Button::custom_id`] field.
    Secondary = 2,
    /// Button indicates a successful action.
    ///
    /// Selecting this button style requires specifying the
    /// [`Button::custom_id`] field.
    Success = 3,
    /// Button indicates a dangerous action.
    ///
    /// Selecting this button style requires specifying the
    /// [`Button::custom_id`] field.
    Danger = 4,
    /// Button indicates an action with a link.
    ///
    /// Selecting this button style requires specifying the [`Button::url`]
    /// field.
    Link = 5,
}

impl From<Button> for Component {
    fn from(button: Button) -> Self {
        Self::Button(button)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all, const_assert_eq};
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
        PartialOrd,
        Send,
        Serialize,
        Sync
    );
    const_assert_eq!(1, ButtonStyle::Primary as u8);
    const_assert_eq!(2, ButtonStyle::Secondary as u8);
    const_assert_eq!(3, ButtonStyle::Success as u8);
    const_assert_eq!(4, ButtonStyle::Danger as u8);
    const_assert_eq!(5, ButtonStyle::Link as u8);

    assert_impl_all!(Component: From<Button>);

    #[test]
    fn test_button_style() {
        serde_test::assert_tokens(&ButtonStyle::Primary, &[Token::U8(1)]);
        serde_test::assert_tokens(&ButtonStyle::Secondary, &[Token::U8(2)]);
        serde_test::assert_tokens(&ButtonStyle::Success, &[Token::U8(3)]);
        serde_test::assert_tokens(&ButtonStyle::Danger, &[Token::U8(4)]);
        serde_test::assert_tokens(&ButtonStyle::Link, &[Token::U8(5)]);
    }
}
