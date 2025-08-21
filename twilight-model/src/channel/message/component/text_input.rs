use serde_repr::{Deserialize_repr, Serialize_repr};

/// Pop-up [`Component`] that renders on modals.
///
/// [`Component`]: super::Component
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TextInput {
    /// Optional id for the text input.
    pub id: Option<i32>,
    /// User defined identifier for the input text.
    pub custom_id: String,
    /// Text appearing over the input field.
    pub label: String,
    /// The maximum length of the text.
    pub max_length: Option<u16>,
    /// The minimum length of the text.
    ///
    /// Defaults to `0`.
    pub min_length: Option<u16>,
    /// Placeholder for the text input.
    pub placeholder: Option<String>,
    /// Whether the user is required to input a text.
    ///
    /// Defaults to `true`.
    pub required: Option<bool>,
    /// Style variant of the input text.
    pub style: TextInputStyle,
    /// Pre-filled value for input text.
    pub value: Option<String>,
}

/// Style of an [`TextInput`].
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[non_exhaustive]
#[repr(u8)]
pub enum TextInputStyle {
    /// Intended for short single-line text.
    Short = 1,
    /// Intended for much longer inputs.
    Paragraph = 2,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all, const_assert_eq};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        TextInput: custom_id,
        label,
        style,
        placeholder,
        min_length,
        max_length,
        value
    );
    assert_impl_all!(TextInput: Clone, Debug, Eq, Hash, PartialEq, Send, Sync);

    assert_impl_all!(
        TextInputStyle: Clone,
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
    const_assert_eq!(1, TextInputStyle::Short as u8);
    const_assert_eq!(2, TextInputStyle::Paragraph as u8);

    #[test]
    fn text_input_style() {
        serde_test::assert_tokens(&TextInputStyle::Short, &[Token::U8(1)]);
        serde_test::assert_tokens(&TextInputStyle::Paragraph, &[Token::U8(2)]);
    }
}
