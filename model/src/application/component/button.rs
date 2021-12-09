use std::fmt::{Formatter, Result as FmtResult};

use crate::channel::ReactionType;

use super::ComponentType;
use serde::{
    de::{Error as DeError, IgnoredAny, MapAccess, Unexpected, Visitor},
    ser::{SerializeStruct, Serializer},
    Deserialize, Deserializer, Serialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

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

impl<'de> Deserialize<'de> for Button {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(ButtonVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum ButtonField {
    Type,
    CustomId,
    Disabled,
    Style,
    Label,
    Emoji,
    Url,
}

struct ButtonVisitor;

impl<'de> Visitor<'de> for ButtonVisitor {
    type Value = Button;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("struct Button")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<Self::Value, V::Error> {
        let mut kind: Option<ComponentType> = None;
        let mut custom_id: Option<String> = None;
        let mut disabled: Option<bool> = None;
        let mut style: Option<ButtonStyle> = None;
        let mut label: Option<String> = None;
        let mut emoji: Option<ReactionType> = None;
        let mut url: Option<String> = None;

        #[cfg(feature = "tracing")]
        let span = tracing::trace_span!("deserializing button");
        #[cfg(feature = "tracing")]
        let _span_enter = span.enter();

        loop {
            #[cfg(feature = "tracing")]
            let span_child = tracing::trace_span!("iterating over button");
            #[cfg(feature = "tracing")]
            let _span_child_enter = span_child.enter();

            let key = match map.next_key() {
                Ok(Some(key)) => {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(?key, "found key");

                    key
                }
                Ok(None) => break,
                Err(why) => {
                    // Encountered when we run into an unknown key.
                    map.next_value::<IgnoredAny>()?;

                    #[cfg(feature = "tracing")]
                    tracing::trace!("ran into an unknown key: {:?}", why);

                    continue;
                }
            };

            match key {
                ButtonField::Type => {
                    if kind.is_some() {
                        return Err(DeError::duplicate_field("type"));
                    }

                    let value: ComponentType = map.next_value()?;

                    if value != ComponentType::Button {
                        return Err(DeError::invalid_value(
                            Unexpected::Unsigned(value as u64),
                            &"a button type",
                        ));
                    }

                    kind = Some(value)
                }
                ButtonField::CustomId => {
                    if custom_id.is_some() {
                        return Err(DeError::duplicate_field("custom_id"));
                    }

                    custom_id = Some(map.next_value()?);
                }
                ButtonField::Disabled => {
                    if disabled.is_some() {
                        return Err(DeError::duplicate_field("disabled"));
                    }

                    disabled = Some(map.next_value()?);
                }
                ButtonField::Style => {
                    if style.is_some() {
                        return Err(DeError::duplicate_field("style"));
                    }

                    style = Some(map.next_value()?);
                }

                ButtonField::Label => {
                    if label.is_some() {
                        return Err(DeError::duplicate_field("label"));
                    }

                    label = Some(map.next_value()?);
                }
                ButtonField::Emoji => {
                    if emoji.is_some() {
                        return Err(DeError::duplicate_field("emoji"));
                    }

                    emoji = Some(map.next_value()?);
                }
                ButtonField::Url => {
                    if url.is_some() {
                        return Err(DeError::duplicate_field("url"));
                    }

                    url = Some(map.next_value()?);
                }
            }
        }

        if kind.is_none() {
            return Err(DeError::missing_field("type"));
        }

        // defaults to false
        let disabled = disabled.unwrap_or(false);
        let style = style.ok_or_else(|| DeError::missing_field("style"))?;

        #[cfg(feature = "tracing")]
        tracing::trace!(
            ?disabled,
            ?style,
            ?kind,
            "all required fields of Button exist"
        );

        Ok(Button {
            custom_id,
            disabled,
            emoji,
            label,
            style,
            url,
        })
    }
}

impl Serialize for Button {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Base of 3 to account for the fields that are always present:
        //
        // - `disabled`
        // - `style`
        // - `type`
        let field_count = 3
            + usize::from(self.custom_id.is_some())
            + usize::from(self.emoji.is_some())
            + usize::from(self.label.is_some())
            + usize::from(self.url.is_some());
        let mut state = serializer.serialize_struct("Button", field_count)?;

        if self.custom_id.is_some() {
            state.serialize_field("custom_id", &self.custom_id)?;
        }

        state.serialize_field("disabled", &self.disabled)?;

        if self.emoji.is_some() {
            state.serialize_field("emoji", &self.emoji)?;
        }

        if self.label.is_some() {
            state.serialize_field("label", &self.label)?;
        }

        state.serialize_field("style", &self.style)?;
        state.serialize_field("type", &ComponentType::Button)?;

        if self.url.is_some() {
            state.serialize_field("url", &self.url)?;
        }

        state.end()
    }
}

#[cfg(test)]
mod tests {
    // Required due to the use of a unicode emoji in a constant.
    #![allow(clippy::non_ascii_literal)]

    use super::{Button, ButtonStyle};
    use crate::{application::component::ComponentType, channel::ReactionType};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all, const_assert_eq};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(Button: custom_id, disabled, emoji, label, style, url);
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
    assert_impl_all!(
        Button: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
    const_assert_eq!(1, ButtonStyle::Primary as u8);
    const_assert_eq!(2, ButtonStyle::Secondary as u8);
    const_assert_eq!(3, ButtonStyle::Success as u8);
    const_assert_eq!(4, ButtonStyle::Danger as u8);
    const_assert_eq!(5, ButtonStyle::Link as u8);

    #[test]
    fn test_button_style() {
        serde_test::assert_tokens(&ButtonStyle::Primary, &[Token::U8(1)]);
        serde_test::assert_tokens(&ButtonStyle::Secondary, &[Token::U8(2)]);
        serde_test::assert_tokens(&ButtonStyle::Success, &[Token::U8(3)]);
        serde_test::assert_tokens(&ButtonStyle::Danger, &[Token::U8(4)]);
        serde_test::assert_tokens(&ButtonStyle::Link, &[Token::U8(5)]);
    }

    #[test]
    fn test_button() {
        // Free Palestine.
        //
        // Palestinian Flag.
        const FLAG: &str = "🇵🇸";

        let value = Button {
            custom_id: Some("test".to_owned()),
            disabled: false,
            emoji: Some(ReactionType::Unicode {
                name: FLAG.to_owned(),
            }),
            label: Some("Test".to_owned()),
            style: ButtonStyle::Link,
            url: Some("https://twilight.rs".to_owned()),
        };

        serde_test::assert_ser_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Button",
                    len: 7,
                },
                Token::String("custom_id"),
                Token::Some,
                Token::String("test"),
                Token::String("disabled"),
                Token::Bool(false),
                Token::String("emoji"),
                Token::Some,
                Token::Struct {
                    name: "ReactionType",
                    len: 1,
                },
                Token::String("name"),
                Token::String(FLAG),
                Token::StructEnd,
                Token::String("label"),
                Token::Some,
                Token::String("Test"),
                Token::String("style"),
                Token::U8(ButtonStyle::Link as u8),
                Token::String("type"),
                Token::U8(ComponentType::Button as u8),
                Token::String("url"),
                Token::Some,
                Token::String("https://twilight.rs"),
                Token::StructEnd,
            ],
        );

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Button",
                    len: 7,
                },
                Token::String("custom_id"),
                Token::String("test"),
                Token::String("disabled"),
                Token::Bool(false),
                Token::String("emoji"),
                Token::Struct {
                    name: "ReactionType",
                    len: 1,
                },
                Token::String("name"),
                Token::String(FLAG),
                Token::StructEnd,
                Token::String("label"),
                Token::String("Test"),
                Token::String("style"),
                Token::U8(ButtonStyle::Link as u8),
                Token::String("type"),
                Token::U8(ComponentType::Button as u8),
                Token::String("url"),
                Token::String("https://twilight.rs"),
                Token::StructEnd,
            ],
        );
    }
}
