//! Create a [`SelectMenuOption`] with a builder.
//!
//! # Example
//! ```
//! use twilight_util::builder::{select_menu_option::SelectMenuOptionBuilder, CallbackDataBuilder};
//! use twilight_model::{
//!     channel::message::MessageFlags,
//!     application::component::{Component, SelectMenu},
//!     channel::ReactionType,
//!     id::EmojiId,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let component = Component::SelectMenu(
//!     SelectMenu {
//!         custom_id: "characters".to_owned(),
//!         disabled: false,
//!         max_values: None,
//!         min_values: None,
//!         options: vec![
//!             SelectMenuOptionBuilder::new("Twilight Sparkle".to_string(), "twilight-sparkle".to_string())
//!                 .default(true)
//!                 .emoji(ReactionType::Custom {
//!                     animated: false,
//!                     id: EmojiId::new(754728776402993173_u64).unwrap(),
//!                     name: Some("sparkle".to_string()),
//!                 })
//!                 .build()?,
//!             SelectMenuOptionBuilder::new("Rarity".to_string(), "rarity".to_string())
//!                 .emoji(ReactionType::Custom {
//!                     animated: false,
//!                     id: EmojiId::new(765306914153299978_u64).unwrap(),
//!                     name: Some("rarsmile".to_string()),
//!                 })
//!                 .build()?,
//!         ],
//!         placeholder: None,
//!     }
//! );
//!
//! let callback_data = CallbackDataBuilder::new()
//!     .content("Callback message".to_string())
//!     .flags(MessageFlags::EPHEMERAL)
//!     .components([component.clone()])
//!     .build();
//!
//! assert_eq!(callback_data.components, Some(vec![component]));
//! # Ok(()) }
//! ```

use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use twilight_model::{
    application::component::select_menu::SelectMenuOption, channel::ReactionType,
};

/// Error building a `SelectMenuOption`.
///
/// This is returned from [`SelectMenuOptionBuilder::build`].
#[derive(Debug)]
pub struct SelectMenuOptionError {
    kind: SelectMenuOptionErrorType,
}

impl SelectMenuOptionError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &SelectMenuOptionErrorType {
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
    pub fn into_parts(
        self,
    ) -> (
        SelectMenuOptionErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for SelectMenuOptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            SelectMenuOptionErrorType::LabelEmpty { .. } => f.write_str("the label is empty"),
            SelectMenuOptionErrorType::LabelTooLong { .. } => f.write_str("the label is too long"),
            SelectMenuOptionErrorType::ValueEmpty { .. } => f.write_str("the value is empty"),
            SelectMenuOptionErrorType::ValueTooLong { .. } => f.write_str("the value is too long"),
            SelectMenuOptionErrorType::DescriptionTooLong { .. } => {
                f.write_str("the description is too long")
            }
        }
    }
}

impl Error for SelectMenuOptionError {}

/// Type of [`SelectMenuOptionError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum SelectMenuOptionErrorType {
    /// Label is empty.
    LabelEmpty {
        /// Provided label although it's empty.
        label: String,
    },
    /// Label is too long.
    LabelTooLong {
        /// Provided label.
        label: String,
    },
    /// Value is empty.
    ValueEmpty {
        /// Provided value although it's empty.
        value: String,
    },
    /// Value is too long.
    ValueTooLong {
        /// Provided value.
        value: String,
    },
    /// Description is too long.
    DescriptionTooLong {
        /// Provided description.
        description: String,
    },
}

/// Create a [`SelectMenuOption`] with a builder.
///
/// # Example
/// ```
/// use twilight_util::builder::{select_menu_option::SelectMenuOptionBuilder, CallbackDataBuilder};
/// use twilight_model::{
///     channel::message::MessageFlags,
///     application::component::{Component, SelectMenu},
///     channel::ReactionType,
///     id::EmojiId,
/// };
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let component = Component::SelectMenu(
///     SelectMenu {
///         custom_id: "characters".into(),
///         disabled: false,
///         max_values: None,
///         min_values: None,
///         options: vec![
///             SelectMenuOptionBuilder::new("Twilight Sparkle".to_string(), "twilight-sparkle".to_string())
///                 .default(true)
///                 .emoji(ReactionType::Custom {
///                     animated: false,
///                     id: EmojiId::new(754728776402993173_u64).unwrap(),
///                     name: Some("sparkle".to_string()),
///                 })
///                 .build()?,
///             SelectMenuOptionBuilder::new("Rarity".to_string(), "rarity".to_string())
///                 .emoji(ReactionType::Custom {
///                     animated: false,
///                     id: EmojiId::new(765306914153299978_u64).unwrap(),
///                     name: Some("rarsmile".to_string()),
///                 })
///                 .build()?,
///         ],
///         placeholder: None,
///     }
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
pub struct SelectMenuOptionBuilder(SelectMenuOption);

impl SelectMenuOptionBuilder {
    /// The maximum amount of characters which a label can have.
    pub const LABEL_LENGTH_LIMIT: usize = 100;

    /// The maximum amount of characters which a value can have.
    pub const VALUE_LENGTH_LIMIT: usize = 100;

    /// The maximum amount of characters which a description can have.
    pub const DESCRIPTION_LENGTH_LIMIT: usize = 100;

    /// Create a new builder to construct a [`SelectMenuOption`].
    pub const fn new(label: String, value: String) -> Self {
        Self(SelectMenuOption {
            default: false,
            description: None,
            emoji: None,
            label,
            value,
        })
    }

    /// Consume the builder, returning a [`SelectMenuOption`].
    ///
    /// # Errors
    ///
    /// Returns an [`SelectMenuOptionErrorType::LabelEmpty`] error type if the provided `label` is empty.
    ///
    /// Returns an [`SelectMenuOptionErrorType::LabelTooLong`] error type if the provided `label` is longer than
    /// the limit defined at [`LABEL_LENGTH_LIMIT`].
    ///
    /// Returns an [`SelectMenuOptionErrorType::ValueEmpty`] error type if the provided `value` is empty.
    ///
    /// Returns an [`SelectMenuOptionErrorType::ValueTooLong`] error type if the provided `value` is longer than
    /// the limit defined at [`VALUE_LENGTH_LIMIT`].
    ///
    /// Returns an [`SelectMenuOptionErrorType::DescriptionTooLong`] error type if the provided `description` is longer than
    /// the limit defined at [`DESCRIPTION_LENGTH_LIMIT`].
    ///
    /// [`LABEL_LENGTH_LIMIT`]: Self::LABEL_LENGTH_LIMIT
    /// [`VALUE_LENGTH_LIMIT`]: Self::VALUE_LENGTH_LIMIT
    /// [`DESCRIPTION_LENGTH_LIMIT`]: Self::DESCRIPTION_LENGTH_LIMIT
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(mut self) -> Result<SelectMenuOption, SelectMenuOptionError> {
        if self.0.label.is_empty() {
            return Err(SelectMenuOptionError {
                kind: SelectMenuOptionErrorType::LabelEmpty {
                    label: self.0.label,
                },
            });
        }

        if self.0.label.chars().count() > Self::LABEL_LENGTH_LIMIT {
            return Err(SelectMenuOptionError {
                kind: SelectMenuOptionErrorType::LabelTooLong {
                    label: self.0.label,
                },
            });
        }

        if self.0.value.is_empty() {
            return Err(SelectMenuOptionError {
                kind: SelectMenuOptionErrorType::ValueEmpty {
                    value: self.0.value,
                },
            });
        }

        if self.0.value.chars().count() > Self::VALUE_LENGTH_LIMIT {
            return Err(SelectMenuOptionError {
                kind: SelectMenuOptionErrorType::ValueTooLong {
                    value: self.0.value,
                },
            });
        }

        if let Some(description) = self.0.description.take() {
            if description.chars().count() > Self::DESCRIPTION_LENGTH_LIMIT {
                return Err(SelectMenuOptionError {
                    kind: SelectMenuOptionErrorType::DescriptionTooLong { description },
                });
            }

            self.0.description.replace(description);
        }

        Ok(self.0)
    }

    /// Set whether this option is selected by default.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::select_menu_option::SelectMenuOptionBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = SelectMenuOptionBuilder::new("Option One".into(), "option-1".into())
    ///     .default(true)
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    pub const fn default(mut self, default: bool) -> Self {
        self.0.default = default;

        self
    }

    /// Set the description of this option.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::select_menu_option::SelectMenuOptionBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = SelectMenuOptionBuilder::new("Option One".into(), "option-1".into())
    ///     .description("The first option.".into())
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn description(mut self, description: String) -> Self {
        self.0.description = Some(description);

        self
    }

    /// Set the emoji of this option.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::select_menu_option::SelectMenuOptionBuilder;
    /// use twilight_model::channel::ReactionType;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = SelectMenuOptionBuilder::new("Option One".into(), "option-1".into())
    ///     .emoji(ReactionType::Unicode {
    ///         name: "1️⃣".into()
    ///     })
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn emoji(mut self, emoji: ReactionType) -> Self {
        self.0.emoji = Some(emoji);

        self
    }
}

impl TryFrom<SelectMenuOptionBuilder> for SelectMenuOption {
    type Error = SelectMenuOptionError;

    /// Convert a `SelectMenuOptionBuilder` into a `SelectMenuOption`.
    ///
    /// This is equivalent to calling [`SelectMenuOptionBuilder::build`].
    fn try_from(builder: SelectMenuOptionBuilder) -> Result<Self, Self::Error> {
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::{SelectMenuOptionBuilder, SelectMenuOptionError, SelectMenuOptionErrorType};
    use static_assertions::{assert_fields, assert_impl_all, const_assert};
    use std::{convert::TryFrom, error::Error, fmt::Debug};
    use twilight_model::{
        application::component::select_menu::SelectMenuOption, channel::ReactionType,
    };

    assert_impl_all!(SelectMenuOptionErrorType: Debug, Send, Sync);
    assert_fields!(SelectMenuOptionErrorType::LabelEmpty: label);
    assert_fields!(SelectMenuOptionErrorType::LabelTooLong: label);
    assert_fields!(SelectMenuOptionErrorType::ValueEmpty: value);
    assert_fields!(SelectMenuOptionErrorType::ValueTooLong: value);
    assert_fields!(SelectMenuOptionErrorType::DescriptionTooLong: description);
    assert_impl_all!(SelectMenuOptionError: Error, Send, Sync);

    const_assert!(SelectMenuOptionBuilder::LABEL_LENGTH_LIMIT == 100);
    const_assert!(SelectMenuOptionBuilder::VALUE_LENGTH_LIMIT == 100);
    const_assert!(SelectMenuOptionBuilder::DESCRIPTION_LENGTH_LIMIT == 100);
    assert_impl_all!(
        SelectMenuOptionBuilder: Clone,
        Debug,
        Eq,
        PartialEq,
        Send,
        Sync
    );
    assert_impl_all!(SelectMenuOption: TryFrom<SelectMenuOptionBuilder>);

    #[test]
    fn test_label_empty_error() {
        assert!(matches!(
            SelectMenuOptionBuilder::new("".to_owned(), "value".to_owned()).build().unwrap_err().kind(),
            SelectMenuOptionErrorType::LabelEmpty { label }
            if label.is_empty()
        ));
    }

    #[test]
    fn test_label_too_long_error() {
        let label_too_long = SelectMenuOptionBuilder::LABEL_LENGTH_LIMIT + 1;
        assert!(matches!(
            SelectMenuOptionBuilder::new("a".repeat(label_too_long), "value".to_owned()).build().unwrap_err().kind(),
            SelectMenuOptionErrorType::LabelTooLong { label }
            if label.len() == label_too_long
        ));
    }

    #[test]
    fn test_value_empty_error() {
        assert!(matches!(
            SelectMenuOptionBuilder::new("label".to_owned(), "".to_owned()).build().unwrap_err().kind(),
            SelectMenuOptionErrorType::ValueEmpty { value }
            if value.is_empty()
        ));
    }

    #[test]
    fn test_value_too_long_error() {
        let value_too_long = SelectMenuOptionBuilder::VALUE_LENGTH_LIMIT + 1;
        assert!(matches!(
            SelectMenuOptionBuilder::new("label".to_owned(), "a".repeat(value_too_long)).build().unwrap_err().kind(),
            SelectMenuOptionErrorType::ValueTooLong { value }
            if value.len() == value_too_long
        ));
    }

    #[test]
    fn test_description_too_long_error() {
        let description_too_long = SelectMenuOptionBuilder::DESCRIPTION_LENGTH_LIMIT + 1;
        assert!(matches!(
            SelectMenuOptionBuilder::new("label".to_owned(), "value".to_owned()).description("a".repeat(description_too_long)).build().unwrap_err().kind(),
            SelectMenuOptionErrorType::DescriptionTooLong { description }
            if description.len() == description_too_long
        ));
    }

    #[test]
    fn test_normal() {
        let select_menu_option =
            SelectMenuOptionBuilder::new("label".to_owned(), "value".to_owned())
                .build()
                .unwrap();

        let expected = SelectMenuOption {
            default: false,
            description: None,
            emoji: None,
            label: "label".to_owned(),
            value: "value".to_owned(),
        };

        assert_eq!(select_menu_option, expected);
    }

    #[test]
    fn test_description() {
        let select_menu_option =
            SelectMenuOptionBuilder::new("label".to_owned(), "value".to_owned())
                .description("description".to_owned())
                .build()
                .unwrap();

        let expected = SelectMenuOption {
            default: false,
            description: Some("description".to_owned()),
            emoji: None,
            label: "label".to_owned(),
            value: "value".to_owned(),
        };

        assert_eq!(select_menu_option, expected);
    }

    #[test]
    fn test_default() {
        let select_menu_option =
            SelectMenuOptionBuilder::new("label".to_owned(), "value".to_owned())
                .default(true)
                .build()
                .unwrap();

        let expected = SelectMenuOption {
            default: true,
            description: None,
            emoji: None,
            label: "label".to_owned(),
            value: "value".to_owned(),
        };

        assert_eq!(select_menu_option, expected);
    }

    #[test]
    fn test_emoji() {
        let select_menu_option =
            SelectMenuOptionBuilder::new("label".to_owned(), "value".to_owned())
                .emoji(ReactionType::Unicode {
                    name: "\u{1f9ea}".to_owned(),
                })
                .build()
                .unwrap();

        let expected = SelectMenuOption {
            default: false,
            description: None,
            emoji: Some(ReactionType::Unicode {
                name: "\u{1f9ea}".to_owned(),
            }),
            label: "label".to_owned(),
            value: "value".to_owned(),
        };

        assert_eq!(select_menu_option, expected);
    }

    #[test]
    fn test_builder_try_from() {
        let select_menu_option = SelectMenuOption::try_from(
            SelectMenuOptionBuilder::new("label".to_owned(), "value".to_owned())
                .description("testing".to_owned()),
        )
        .unwrap();

        let expected = SelectMenuOption {
            default: false,
            description: Some("testing".to_owned()),
            emoji: None,
            label: "label".to_owned(),
            value: "value".to_owned(),
        };

        assert_eq!(select_menu_option, expected);
    }

    #[test]
    fn test_error_into_source() {
        assert!(matches!(
            SelectMenuOptionBuilder::new("".to_owned(), "value".to_owned())
                .build()
                .unwrap_err()
                .into_source(),
            None
        ));
    }

    #[test]
    fn test_error_into_parts() {
        assert!(matches!(
            SelectMenuOptionBuilder::new("".to_owned(), "value".to_owned())
                .build()
                .unwrap_err()
                .into_parts(),
            (SelectMenuOptionErrorType::LabelEmpty { .. }, None)
        ));
    }
}
