use crate::channel::message::EmojiReactionType;
use crate::id::marker::SkuMarker;
use crate::id::Id;
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
    pub emoji: Option<EmojiReactionType>,
    /// Text appearing on the button.
    pub label: Option<String>,
    /// Style variant of the button.
    pub style: ButtonStyle,
    /// URL for buttons of a [`ButtonStyle::Link`] style.
    pub url: Option<String>,
    /// The ID of the SKU that is attached to the button.
    ///
    /// This field is required when using the [`ButtonStyle::Premium`] style.
    pub sku_id: Option<Id<SkuMarker>>,
}

/// Style of a [`Button`].
// Keep in sync with `twilight-validate::component`!
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum ButtonStyle {
    /// Button indicates a primary action.
    ///
    /// Selecting this button style requires specifying the
    /// [`Button::custom_id`] field.
    Primary,
    /// Button indicates a secondary action.
    ///
    /// Selecting this button style requires specifying the
    /// [`Button::custom_id`] field.
    Secondary,
    /// Button indicates a successful action.
    ///
    /// Selecting this button style requires specifying the
    /// [`Button::custom_id`] field.
    Success,
    /// Button indicates a dangerous action.
    ///
    /// Selecting this button style requires specifying the
    /// [`Button::custom_id`] field.
    Danger,
    /// Button indicates an action with a link.
    ///
    /// Selecting this button style requires specifying the [`Button::url`]
    /// field.
    Link,
    /// Button indicates a premium upgrade action.
    ///
    /// Selecting this button style requires specifying the [`Button::sku_id`]
    /// field.
    /// The following fields are not available for this button style: [`Button::custom_id`], [`Button::label`], [`Button::url`] & [`Button::emoji`].
    /// Premium button styles do not fire an interaction event.
    Premium,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for ButtonStyle {
    fn from(value: u8) -> Self {
        match value {
            1 => ButtonStyle::Primary,
            2 => ButtonStyle::Secondary,
            3 => ButtonStyle::Success,
            4 => ButtonStyle::Danger,
            5 => ButtonStyle::Link,
            6 => ButtonStyle::Premium,
            unknown => ButtonStyle::Unknown(unknown),
        }
    }
}

impl From<ButtonStyle> for u8 {
    fn from(value: ButtonStyle) -> Self {
        match value {
            ButtonStyle::Primary => 1,
            ButtonStyle::Secondary => 2,
            ButtonStyle::Success => 3,
            ButtonStyle::Danger => 4,
            ButtonStyle::Link => 5,
            ButtonStyle::Premium => 6,
            ButtonStyle::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn button_style() {
        serde_test::assert_tokens(&ButtonStyle::Primary, &[Token::U8(1)]);
        serde_test::assert_tokens(&ButtonStyle::Secondary, &[Token::U8(2)]);
        serde_test::assert_tokens(&ButtonStyle::Success, &[Token::U8(3)]);
        serde_test::assert_tokens(&ButtonStyle::Danger, &[Token::U8(4)]);
        serde_test::assert_tokens(&ButtonStyle::Link, &[Token::U8(5)]);
        serde_test::assert_tokens(&ButtonStyle::Premium, &[Token::U8(6)]);
        serde_test::assert_tokens(&ButtonStyle::Unknown(99), &[Token::U8(99)]);
    }
}
