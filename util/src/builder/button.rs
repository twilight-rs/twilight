use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use twilight_model::{
    application::component::{button::ButtonStyle, Button},
    channel::ReactionType,
};

/// Error building a button.
///
/// This is returned from [`ButtonBuilder::build`].
#[derive(Debug)]
pub struct ButtonError {
    kind: ButtonErrorType,
}

impl ButtonError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ButtonErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (ButtonErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }
}

impl Display for ButtonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use ButtonErrorType::*;
        match &self.kind {
            CustomIdEmpty { .. } => f.write_str("the custom_id is empty"),
            CustomIdTooLong { .. } => f.write_str("the custom_id is too long"),
            ProtocolUnsupported { .. } => f.write_str("url uses an unsupported protocol"),
            LabelAndEmojiEmpty { .. } => f.write_str("label and emoji are empty"),
            LabelTooLong { .. } => f.write_str("label is too long"),
        }
    }
}

impl Error for ButtonError {}

/// Type of [`ButtonError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum ButtonErrorType {
    /// custom_id is empty
    CustomIdEmpty {
        /// Provided custom_id although it's empty.
        custom_id: String,
    },
    /// custom_id is too long.
    CustomIdTooLong {
        /// Provided custom_id.
        custom_id: String,
    },
    /// An invalid protocol was used for the link.
    ProtocolUnsupported {
        /// Provided url.
        url: String,
    },
    /// Neither a label nor an emoji has been provided.
    LabelAndEmojiEmpty {
        /// Provided emoji although it's empty
        emoji: Option<ReactionType>,
        /// Provided label although it's empty
        label: Option<String>,
    },
    /// Label is too long.
    LabelTooLong {
        /// Provided label.
        label: String,
    },
}

/// Create a [`Button`] with a builder.
///
/// # Example
/// ```
/// use twilight_util::builder::{ButtonBuilder, CallbackDataBuilder};
/// use twilight_model::{
///     channel::message::MessageFlags,
///     application::component::{button::ButtonStyle, Component, Button}
/// };
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let component = Component::Button(
///     ButtonBuilder::new(ButtonStyle::Primary, "button_id".to_string())
///         .label("Button label".to_string())
///         .build()?
/// );
///
/// let callback_data = CallbackDataBuilder::new()
///     .content("Callback message".to_string())
///     .flags(MessageFlags::EPHEMERAL)
///     .components([component.clone()])
///     .build();
///
/// assert_eq!(callback_data.components, Some(vec![component]));
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "builders have no effect if unused"]
pub struct ButtonBuilder(Button);

impl ButtonBuilder {
    /// The maximum amount of characters which a custom id can have.
    pub const CUSTOM_ID_LENGTH_LIMIT: usize = 100;

    /// The maximum amount of characters which a label can have.
    pub const LABEL_LENGTH_LIMIT: usize = 80;

    /// Create a new builder to construct a [`Button`].
    pub fn new(style: ButtonStyle, custom_id_or_url: String) -> Self {
        if style == ButtonStyle::Link {
            Self(Button {
                style,
                emoji: None,
                label: None,
                custom_id: None,
                url: Some(custom_id_or_url),
                disabled: false,
            })
        } else {
            Self(Button {
                style,
                emoji: None,
                label: None,
                custom_id: Some(custom_id_or_url),
                url: None,
                disabled: false,
            })
        }
    }

    /// Consume the builder, returning a [`Button`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(mut self) -> Result<Button, ButtonError> {
        if let Some(custom_id) = self.0.custom_id.take() {
            if custom_id.is_empty() {
                return Err(ButtonError {
                    kind: ButtonErrorType::CustomIdEmpty { custom_id },
                });
            }

            if custom_id.chars().count() > Self::CUSTOM_ID_LENGTH_LIMIT {
                return Err(ButtonError {
                    kind: ButtonErrorType::CustomIdTooLong { custom_id },
                });
            }

            self.0.custom_id.replace(custom_id);
        } else if let Some(url) = self.0.url.take() {
            if !url.starts_with("https:") && !url.starts_with("http:") {
                return Err(ButtonError {
                    kind: ButtonErrorType::ProtocolUnsupported { url },
                });
            }

            self.0.url.replace(url);
        }

        if self.0.label.is_none() && self.0.emoji.is_none() {
            return Err(ButtonError {
                kind: ButtonErrorType::LabelAndEmojiEmpty {
                    label: self.0.label,
                    emoji: self.0.emoji,
                },
            });
        }

        if let Some(label) = self.0.label.take() {
            if label.is_empty() && self.0.emoji.is_none() {
                return Err(ButtonError {
                    kind: ButtonErrorType::LabelAndEmojiEmpty {
                        label: Some(label),
                        emoji: self.0.emoji,
                    },
                });
            }

            if label.chars().count() > Self::LABEL_LENGTH_LIMIT {
                return Err(ButtonError {
                    kind: ButtonErrorType::LabelTooLong { label },
                });
            }

            self.0.label.replace(label);
        }

        Ok(self.0)
    }

    /// Set the label of the button.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_model::application::component::button::ButtonStyle;
    /// use twilight_util::builder::ButtonBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let button = ButtonBuilder::new(ButtonStyle::Primary, "unique-id".into())
    ///     .label("twilight".into())
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    pub fn label(mut self, label: String) -> Self {
        self.0.label = Some(label);

        self
    }

    /// Set the emoji of the button.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_model::{application::component::button::ButtonStyle, channel::ReactionType,};
    /// use twilight_util::builder::ButtonBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let button = ButtonBuilder::new(ButtonStyle::Primary, "unique-id".into())
    ///     .emoji(ReactionType::Unicode {
    ///         name: "🙂".into()
    ///     })
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    pub fn emoji(mut self, emoji: ReactionType) -> Self {
        self.0.emoji = Some(emoji);

        self
    }

    /// Set whether the button is disabled or not.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_model::application::component::button::ButtonStyle;
    /// use twilight_util::builder::ButtonBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let button = ButtonBuilder::new(ButtonStyle::Primary, "unique-id".into())
    ///     .label("disabled button".into())
    ///     .disabled(true)
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;

        self
    }
}

impl TryFrom<ButtonBuilder> for Button {
    type Error = ButtonError;

    /// Convert a button builder into a button.
    ///
    /// This is equivalent to calling [`ButtonBuilder::build`].
    fn try_from(builder: ButtonBuilder) -> Result<Self, Self::Error> {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::{ButtonBuilder, ButtonError, ButtonErrorType};
    use static_assertions::{assert_fields, assert_impl_all, const_assert};
    use std::{convert::TryFrom, error::Error, fmt::Debug};
    use twilight_model::{
        application::component::{button::ButtonStyle, Button},
        channel::ReactionType,
    };

    assert_impl_all!(ButtonErrorType: Debug, Send, Sync);
    assert_fields!(ButtonErrorType::CustomIdEmpty: custom_id);
    assert_fields!(ButtonErrorType::CustomIdTooLong: custom_id);
    assert_fields!(ButtonErrorType::ProtocolUnsupported: url);
    assert_fields!(ButtonErrorType::LabelAndEmojiEmpty: emoji, label);
    assert_fields!(ButtonErrorType::LabelTooLong: label);
    assert_impl_all!(ButtonError: Error, Send, Sync);

    const_assert!(ButtonBuilder::CUSTOM_ID_LENGTH_LIMIT == 100);
    const_assert!(ButtonBuilder::LABEL_LENGTH_LIMIT == 80);
    assert_impl_all!(ButtonBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Button: TryFrom<ButtonBuilder>);

    #[test]
    fn test_custom_id_empty_error() {
        assert!(matches!(
            ButtonBuilder::new(ButtonStyle::Primary, "".to_owned()).build().unwrap_err().kind(),
            ButtonErrorType::CustomIdEmpty { custom_id }
            if custom_id.is_empty()
        ));
    }

    #[test]
    fn test_custom_id_too_long_error() {
        let custom_id_too_long = ButtonBuilder::CUSTOM_ID_LENGTH_LIMIT + 1;
        assert!(matches!(
            ButtonBuilder::new(ButtonStyle::Primary, "a".repeat(custom_id_too_long)).build().unwrap_err().kind(),
            ButtonErrorType::CustomIdTooLong { custom_id }
            if custom_id.len() == custom_id_too_long
        ));
    }

    #[test]
    fn test_protocol_unsupported_error() {
        assert!(matches!(
            ButtonBuilder::new(ButtonStyle::Link, "foo://bar.baz".to_owned()).label("testing".to_owned()).build().unwrap_err().kind(),
            ButtonErrorType::ProtocolUnsupported { url } if url == "foo://bar.baz"
        ));
    }

    #[test]
    fn test_label_and_emoji_empty() {
        assert!(matches!(
            ButtonBuilder::new(ButtonStyle::Primary, "testing".to_owned()).build().unwrap_err().kind(),
            ButtonErrorType::LabelAndEmojiEmpty { emoji, label } if emoji.is_none() && label.is_none()
        ));

        assert!(matches!(
            ButtonBuilder::new(ButtonStyle::Primary, "testing".to_owned()).label("".to_owned()).build().unwrap_err().kind(),
            ButtonErrorType::LabelAndEmojiEmpty { emoji, label: Some(label) } if emoji.is_none() && label.is_empty()
        ));
    }

    #[test]
    fn test_label_too_long_error() {
        let label_too_long = ButtonBuilder::LABEL_LENGTH_LIMIT + 1;
        assert!(matches!(
            ButtonBuilder::new(ButtonStyle::Primary, "testing".to_owned()).label("a".repeat(label_too_long)).build().unwrap_err().kind(),
            ButtonErrorType::LabelTooLong { label }
            if label.len() == label_too_long
        ));
    }

    #[test]
    fn test_builder_without_link() {
        let button = ButtonBuilder::new(ButtonStyle::Primary, "primary-button".to_owned())
            .emoji(ReactionType::Unicode {
                name: "🧪".to_owned(),
            })
            .label("testing".to_owned())
            .disabled(true)
            .build()
            .unwrap();

        let expected = Button {
            style: ButtonStyle::Primary,
            emoji: Some(ReactionType::Unicode {
                name: "🧪".to_owned(),
            }),
            label: Some("testing".to_owned()),
            custom_id: Some("primary-button".to_owned()),
            url: None,
            disabled: true,
        };

        assert_eq!(button, expected);
    }

    #[test]
    fn test_builder_with_link() {
        let button = ButtonBuilder::new(ButtonStyle::Link, "https://twilight.rs".to_owned())
            .emoji(ReactionType::Unicode {
                name: "🧪".to_owned(),
            })
            .label("testing".to_owned())
            .disabled(false)
            .build()
            .unwrap();

        let expected = Button {
            style: ButtonStyle::Link,
            emoji: Some(ReactionType::Unicode {
                name: "🧪".to_owned(),
            }),
            label: Some("testing".to_owned()),
            custom_id: None,
            url: Some("https://twilight.rs".to_owned()),
            disabled: false,
        };

        assert_eq!(button, expected);
    }
}
