use std::fmt::{Formatter, Result as FmtResult};

use super::ComponentType;
use serde::{
    de::{Error as DeError, IgnoredAny, MapAccess, Unexpected, Visitor},
    ser::{SerializeStruct, Serializer},
    Deserialize, Deserializer, Serialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Modal component to prompt users for a text input.
///
/// Refer to [Discord Docs/Input Text] for additional information.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InputText {
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
    pub style: InputTextStyle,
    /// Pre-filled value for input text.
    pub value: Option<String>,
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

impl<'de> Deserialize<'de> for InputText {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(InputTextVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum InputTextField {
    CustomId,
    Label,
    MaxLength,
    MinLength,
    Placeholder,
    Required,
    Style,
    Type,
    Value,
}

struct InputTextVisitor;

impl<'de> Visitor<'de> for InputTextVisitor {
    type Value = InputText;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct InputText")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut custom_id: Option<String> = None;
        let mut kind: Option<ComponentType> = None;
        let mut label: Option<String> = None;
        let mut max_length: Option<u16> = None;
        let mut min_length: Option<u16> = None;
        let mut placeholder: Option<String> = None;
        let mut required: Option<bool> = None;
        let mut style: Option<InputTextStyle> = None;
        let mut value: Option<String> = None;

        #[cfg(feature = "tracing")]
        let span = tracing::trace_span!("deserializing input text");
        #[cfg(feature = "tracing")]
        let _span_enter = span.enter();

        loop {
            #[cfg(feature = "tracing")]
            let span_child = tracing::trace_span!("iterating over input text");
            #[cfg(feature = "tracing")]
            let _span_child_enter = span_child.enter();

            let key = match map.next_key() {
                Ok(Some(key)) => {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(?key, "found key");

                    key
                }
                Ok(None) => break,
                #[cfg(feature = "tracing")]
                Err(why) => {
                    map.next_value::<IgnoredAny>()?;

                    tracing::trace!("ran into an unknown key: {:?}", why);

                    continue;
                }
                #[cfg(not(feature = "tracing"))]
                Err(_) => {
                    map.next_value::<IgnoredAny>()?;

                    continue;
                }
            };

            match key {
                InputTextField::CustomId => {
                    if custom_id.is_some() {
                        return Err(DeError::duplicate_field("custom_id"));
                    }

                    custom_id = Some(map.next_value()?);
                }
                InputTextField::Label => {
                    if label.is_some() {
                        return Err(DeError::duplicate_field("label"));
                    }

                    label = Some(map.next_value()?)
                }
                InputTextField::MaxLength => {
                    if max_length.is_some() {
                        return Err(DeError::duplicate_field("max_length"));
                    }

                    max_length = Some(map.next_value()?)
                }
                InputTextField::MinLength => {
                    if min_length.is_some() {
                        return Err(DeError::duplicate_field("min_length"));
                    }

                    min_length = Some(map.next_value()?)
                }
                InputTextField::Placeholder => {
                    if placeholder.is_some() {
                        return Err(DeError::duplicate_field("placeholder"));
                    }

                    placeholder = Some(map.next_value()?)
                }
                InputTextField::Required => {
                    if required.is_some() {
                        return Err(DeError::duplicate_field("required"));
                    }

                    required = Some(map.next_value()?)
                }
                InputTextField::Style => {
                    if style.is_some() {
                        return Err(DeError::duplicate_field("style"));
                    }

                    style = Some(map.next_value()?);
                }
                InputTextField::Type => {
                    if kind.is_some() {
                        return Err(DeError::duplicate_field("type"));
                    }

                    let value: ComponentType = map.next_value()?;

                    if value != ComponentType::InputText {
                        return Err(DeError::invalid_value(
                            Unexpected::Unsigned(value as u64),
                            &"an input text type",
                        ));
                    }

                    kind = Some(value)
                }
                InputTextField::Value => {
                    if value.is_some() {
                        return Err(DeError::duplicate_field("value"));
                    }

                    value = Some(map.next_value()?)
                }
            }
        }

        if kind.is_none() {
            return Err(DeError::missing_field("kind"));
        }

        let custom_id = custom_id.ok_or_else(|| DeError::missing_field("custom_id"))?;
        let label = label.ok_or_else(|| DeError::missing_field("label"))?;
        let style = style.ok_or_else(|| DeError::missing_field("style"))?;

        #[cfg(feature = "tracing")]
        tracing::trace!(
            %custom_id,
            ?kind,
            %label,
            ?style,
            "all fields of InputText exist"
        );

        Ok(InputText {
            custom_id,
            label,
            max_length,
            min_length,
            placeholder,
            required,
            style,
            value,
        })
    }
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
            + usize::from(self.max_length.is_some())
            + usize::from(self.required.is_some());
        let mut state = serializer.serialize_struct("InputText", field_count)?;

        state.serialize_field("custom_id", &self.custom_id)?;
        state.serialize_field("label", &self.label)?;
        
        if self.max_length.is_some() {
            state.serialize_field("max_length", &self.max_length)?;
        }

        if self.min_length.is_some() {
            state.serialize_field("min_length", &self.min_length)?;
        }

        if self.placeholder.is_some() {
            state.serialize_field("placeholder", &self.placeholder)?;
        }

        if self.required.is_some() {
            state.serialize_field("required", &self.required)?;
        }
        
        state.serialize_field("style", &self.style)?;
        state.serialize_field("type", &ComponentType::InputText)?;

        if self.value.is_some() {
            state.serialize_field("value", &self.value)?;
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
        max_length,
        value
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
            max_length: Some(100),
            min_length: Some(1),
            placeholder: Some("Taking this place".to_owned()),
            required: Some(true),
            style: InputTextStyle::Short,
            value: Some("Hello World!".to_owned()),
        };

        serde_test::assert_ser_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InputText",
                    len: 8,
                },
                Token::String("custom_id"),
                Token::String("test"),
                Token::String("label"),
                Token::String("The label"),
                Token::String("max_length"),
                Token::Some,
                Token::U16(100),
                Token::String("min_length"),
                Token::Some,
                Token::U16(1),
                Token::String("placeholder"),
                Token::Some,
                Token::String("Taking this place"),
                Token::String("required"),
                Token::Some,
                Token::Bool(true),
                Token::String("style"),
                Token::U8(InputTextStyle::Short as u8),
                Token::String("type"),
                Token::U8(ComponentType::InputText as u8),
                Token::String("value"),
                Token::Some,
                Token::String("Hello World!"),
                Token::StructEnd,
            ],
        );

        serde_test::assert_ser_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InputText",
                    len: 8,
                },
                Token::String("custom_id"),
                Token::String("test"),
                Token::String("label"),
                Token::String("The label"),
                Token::String("max_length"),
                Token::Some,
                Token::U16(100),
                Token::String("min_length"),
                Token::Some,
                Token::U16(1),
                Token::String("placeholder"),
                Token::Some,
                Token::String("Taking this place"),
                Token::String("required"),
                Token::Some,
                Token::Bool(true),
                Token::String("style"),
                Token::U8(InputTextStyle::Short as u8),
                Token::String("type"),
                Token::U8(ComponentType::InputText as u8),
                Token::String("value"),
                Token::Some,
                Token::String("Hello World!"),
                Token::StructEnd,
            ],
        );
    }
}
