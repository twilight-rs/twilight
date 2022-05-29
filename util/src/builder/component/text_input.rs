//! Create a [`TextInput`] with a builder.
//!
//! # Example
//! ```
//! use twilight_model::application::component::Component;
//! use twilight_util::builder::component::TextInputBuilder;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let component = Component::TextInput(
//!     TextInputBuilder::paragraph("input-1".to_string(), "Input".to_owned())
//!         .min_length(20)
//!         .required(true)
//!         .validate()?.build()
//! );
//! # Ok(()) }
//! ```

use std::convert::TryFrom;

use twilight_model::application::component::{text_input::TextInputStyle, TextInput};
use twilight_validate::component::{text_input as validate_text_input, ComponentValidationError};

/// Create a [`TextInput`] with a builder.
///
/// # Example
/// ```
/// use twilight_model::application::component::Component;
/// use twilight_util::builder::component::TextInputBuilder;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let component = Component::TextInput(
///     TextInputBuilder::paragraph("input-1".to_string(), "Input".to_owned())
///         .min_length(20)
///         .required(true)
///         .validate()?.build()
/// );
/// # Ok(()) }
/// ```

#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "builders have no effect if unused"]
pub struct TextInputBuilder(TextInput);

impl TextInputBuilder {
    /// Create a new builder to construct a [`TextInputStyle::Short`] styled [`TextInput`].
    pub const fn short(custom_id: String, label: String) -> Self {
        Self(TextInput {
            custom_id,
            label,
            max_length: None,
            min_length: None,
            placeholder: None,
            required: None,
            style: TextInputStyle::Short,
            value: None,
        })
    }

    /// Create a new builder to construct a [`TextInputStyle::Paragraph`] styled [`TextInput`].
    pub const fn paragraph(custom_id: String, label: String) -> Self {
        Self(TextInput {
            custom_id,
            label,
            max_length: None,
            min_length: None,
            placeholder: None,
            required: None,
            style: TextInputStyle::Paragraph,
            value: None,
        })
    }

    /// Consume the builder, returning a [`TextInput`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> TextInput {
        self.0
    }

    /// Ensure the text input is valid.
    ///
    /// # Errors
    ///
    /// Refer to the documentation of [`twilight_validate::component::select_menu_option`] for
    /// possible errors.
    pub fn validate(self) -> Result<Self, ComponentValidationError> {
        if let Err(source) = validate_text_input(&self.0) {
            return Err(source);
        }

        Ok(self)
    }

    /// Set the maximum amount of characters allowed to be entered in this text input.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::TextInputBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = TextInputBuilder::short("input-1".into(), "Input One".into())
    ///     .max_length(100)
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    pub const fn max_length(mut self, max_length: u16) -> Self {
        self.0.max_length = Some(max_length);

        self
    }

    /// Set the minimum amount of characters necessary to be entered in this text input.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::TextInputBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = TextInputBuilder::short("input-1".into(), "Input One".into())
    ///     .min_length(10)
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    pub const fn min_length(mut self, min_length: u16) -> Self {
        self.0.min_length = Some(min_length);

        self
    }

    /// Set the placeholder for this text input.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::TextInputBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = TextInputBuilder::short("input-1".into(), "Input One".into())
    ///     .placeholder("This is the first input".into())
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    pub fn placeholder(mut self, placeholder: String) -> Self {
        self.0.placeholder = Some(placeholder);

        self
    }

    /// Set whether this text input is required or not.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::TextInputBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = TextInputBuilder::short("input-1".into(), "Input One".into())
    ///     .required(true)
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    pub const fn required(mut self, required: bool) -> Self {
        self.0.required = Some(required);

        self
    }

    /// Set the pre-filled value for this text input.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::TextInputBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = TextInputBuilder::short("input-1".into(), "Input One".into())
    ///     .value("This is the first input".into())
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    pub fn value(mut self, value: String) -> Self {
        self.0.value = Some(value);

        self
    }
}

impl TryFrom<TextInputBuilder> for TextInput {
    type Error = ComponentValidationError;

    /// Convert a `TextInputBuilder` into a `TextInput`.
    ///
    /// This is equivalent to calling [`TextInputBuilder::validate`]
    /// then [`TextInputBuilder::build`].
    fn try_from(builder: TextInputBuilder) -> Result<Self, Self::Error> {
        Ok(builder.validate()?.build())
    }
}

#[cfg(test)]
mod tests {
    use super::TextInputBuilder;
    use static_assertions::assert_impl_all;
    use std::{convert::TryFrom, fmt::Debug};
    use twilight_model::application::component::{text_input::TextInputStyle, TextInput};

    assert_impl_all!(TextInputBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(TextInput: TryFrom<TextInputBuilder>);

    #[test]
    fn test_text_input_builder_short() {
        let expected = TextInput {
            custom_id: "input".into(),
            label: "label".into(),
            max_length: None,
            min_length: None,
            placeholder: None,
            required: None,
            style: TextInputStyle::Short,
            value: None,
        };

        let actual = TextInputBuilder::short("input".to_string(), "label".to_owned()).build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_text_input_builder_paragraph() {
        let expected = TextInput {
            custom_id: "input".into(),
            label: "label".into(),
            max_length: None,
            min_length: None,
            placeholder: None,
            required: None,
            style: TextInputStyle::Paragraph,
            value: None,
        };

        let actual = TextInputBuilder::paragraph("input".to_string(), "label".to_owned()).build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_text_input_builder_max_length() {
        let expected = TextInput {
            custom_id: "input".into(),
            label: "label".into(),
            max_length: Some(100),
            min_length: None,
            placeholder: None,
            required: None,
            style: TextInputStyle::Short,
            value: None,
        };

        let actual = TextInputBuilder::short("input".to_string(), "label".to_owned())
            .max_length(100)
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_text_input_builder_min_length() {
        let expected = TextInput {
            custom_id: "input".into(),
            label: "label".into(),
            max_length: None,
            min_length: Some(10),
            placeholder: None,
            required: None,
            style: TextInputStyle::Short,
            value: None,
        };

        let actual = TextInputBuilder::short("input".to_string(), "label".to_owned())
            .min_length(10)
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_text_input_builder_placeholder() {
        let expected = TextInput {
            custom_id: "input".into(),
            label: "label".into(),
            max_length: None,
            min_length: None,
            placeholder: Some("Enter some text".into()),
            required: None,
            style: TextInputStyle::Short,
            value: None,
        };

        let actual = TextInputBuilder::short("input".to_string(), "label".to_owned())
            .placeholder("Enter some text".into())
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_text_input_builder_required() {
        let expected = TextInput {
            custom_id: "input".into(),
            label: "label".into(),
            max_length: None,
            min_length: None,
            placeholder: None,
            required: Some(true),
            style: TextInputStyle::Short,
            value: None,
        };

        let actual = TextInputBuilder::short("input".to_string(), "label".to_owned())
            .required(true)
            .build();

        assert_eq!(actual, expected);
    }
}
