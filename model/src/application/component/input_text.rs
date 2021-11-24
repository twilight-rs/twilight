use super::ComponentType;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Modal component to prompt users for a text input.
///
/// Refer to [Discord Docs/Input Text] for additional information.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct InputText {
    /// User defined identifier for the input text.
    pub custom_id: String,
    /// Text appearing over the input field.
    pub label: String,
    /// Style variant of the input text.
    pub style: InputTextStyle,
    /// Placeholder for the text input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// The minimum length of the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    /// The maximum length of the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
}

/// Style of an [`InputText`].
///
/// Refer to [the discord docs] for additional information.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Serialize_repr)]
#[repr(u8)]
pub enum InputTextStyle {
    /// Intended for short single-line text.
    Short = 1,
    /// Intended for much longer inputs.
    Paragraph = 2,
}

impl Serialize for InputText {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Base of 4 to account for the fields that are always present:
        //
        // - `custom_id`
        // - `label`
        // - `style`
        // - `type`
        let field_count = 4
            + usize::from(self.placeholder.is_some())
            + usize::from(self.min_length.is_some())
            + usize::from(self.max_length.is_some());
        let mut state = serializer.serialize_struct("InputText", field_count)?;

        state.serialize_field("custom_id", &self.custom_id)?;
        state.serialize_field("label", &self.label)?;
        state.serialize_field("style", &self.style)?;
        state.serialize_field("type", &ComponentType::InputText)?;

        if self.placeholder.is_some() {
            state.serialize_field("placeholder", &self.placeholder)?;
        }

        if self.min_length.is_some() {
            state.serialize_field("min_length", &self.min_length)?;
        }

        if self.max_length.is_some() {
            state.serialize_field("max_length", &self.max_length)?;
        }

        state.end()
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all, const_assert_eq};

    use crate::application::component::{input_text::InputTextStyle, ComponentType};

    use super::InputText;
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        InputText: custom_id,
        label,
        style,
        placeholder,
        min_length,
        max_length
    );
    assert_impl_all!(
        InputText: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
    assert_impl_all!(
        InputTextStyle: Clone,
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
    const_assert_eq!(1, InputTextStyle::Short as u8);
    const_assert_eq!(2, InputTextStyle::Paragraph as u8);

    #[test]
    fn test_input_text_style() {
        serde_test::assert_tokens(&InputTextStyle::Short, &[Token::U8(1)]);
        serde_test::assert_tokens(&InputTextStyle::Paragraph, &[Token::U8(2)]);
    }

    #[test]
    fn test_input_text() {
        let value = InputText {
            custom_id: "test".to_owned(),
            label: "The label".to_owned(),
            style: InputTextStyle::Short,
            placeholder: Some("Taking this place".to_owned()),
            min_length: Some(1),
            max_length: Some(100),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InputText",
                    len: 7,
                },
                Token::String("custom_id"),
                Token::String("test"),
                Token::String("label"),
                Token::String("The label"),
                Token::String("style"),
                Token::U8(InputTextStyle::Short as u8),
                Token::String("type"),
                Token::U8(ComponentType::InputText as u8),
                Token::String("placeholder"),
                Token::Some,
                Token::String("Taking this place"),
                Token::String("min_length"),
                Token::Some,
                Token::I32(1),
                Token::String("max_length"),
                Token::Some,
                Token::I32(100),
                Token::StructEnd,
            ],
        )
    }
}
