use serde::{Deserialize, Serialize};

/// Pop-up [`Component`] that renders on modals.
///
/// [`Component`]: super::Component
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TextInput {
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
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TextInputStyle(u8);

impl TextInputStyle {
    /// Intended for short single-line text.
    pub const SHORT: Self = Self::new(1);

    /// Intended for much longer inputs.
    pub const PARAGRAPH: Self = Self::new(2);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::PARAGRAPH => "PARAGRAPH",
            Self::SHORT => "SHORT",
            _ => return None,
        })
    }
}

impl_typed!(TextInputStyle, u8);

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
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

    #[test]
    fn text_input_style() {
        serde_test::assert_tokens(
            &TextInputStyle::SHORT,
            &[
                Token::NewtypeStruct {
                    name: "TextInputStyle",
                },
                Token::U8(1),
            ],
        );
        serde_test::assert_tokens(
            &TextInputStyle::PARAGRAPH,
            &[
                Token::NewtypeStruct {
                    name: "TextInputStyle",
                },
                Token::U8(2),
            ],
        );
    }
}
