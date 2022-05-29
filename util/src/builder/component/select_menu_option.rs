//! Create a [`SelectMenuOption`] with a builder.
//!
//! # Example
//! ```
//! use twilight_model::{application::component::Component, channel::ReactionType, id::Id};
//! use twilight_util::builder::component::{SelectMenuBuilder, SelectMenuOptionBuilder};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let component = Component::SelectMenu(
//!     SelectMenuBuilder::new("characters".to_string())
//!         .add_options(
//!             &mut vec![
//!                 SelectMenuOptionBuilder::new("twilight-sparkle".to_string(), "Twilight Sparkle".to_string())
//!                     .default(true)
//!                     .emoji(ReactionType::Custom {
//!                         animated: false,
//!                         id: Id::new(754728776402993173),
//!                         name: Some("sparkle".to_string()),
//!                     })
//!                     .build(),
//!                 SelectMenuOptionBuilder::new("rarity".to_string(), "Rarity".to_string())
//!                     .emoji(ReactionType::Custom {
//!                         animated: false,
//!                         id: Id::new(765306914153299978),
//!                         name: Some("rarsmile".to_string()),
//!                     })
//!                     .build(),
//!             ]
//!         ).validate()?.build()
//! );
//! # Ok(()) }
//! ```

use std::convert::TryFrom;

use twilight_model::{
    application::component::select_menu::SelectMenuOption, channel::ReactionType,
};
use twilight_validate::component::{
    select_menu_option as validate_select_menu_option, ComponentValidationError,
};

/// Create a [`SelectMenuOption`] with a builder.
///
/// # Example
/// ```
/// use twilight_model::{application::component::Component, channel::ReactionType, id::Id};
/// use twilight_util::builder::component::{SelectMenuBuilder, SelectMenuOptionBuilder};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let component = Component::SelectMenu(
///     SelectMenuBuilder::new("characters".to_string())
///         .add_options(
///             &mut vec![
///                 SelectMenuOptionBuilder::new("twilight-sparkle".to_string(), "Twilight Sparkle".to_string())
///                     .default(true)
///                     .emoji(ReactionType::Custom {
///                         animated: false,
///                         id: Id::new(754728776402993173),
///                         name: Some("sparkle".to_string()),
///                     })
///                     .build(),
///                 SelectMenuOptionBuilder::new("rarity".to_string(), "Rarity".to_string())
///                     .emoji(ReactionType::Custom {
///                         animated: false,
///                         id: Id::new(765306914153299978),
///                         name: Some("rarsmile".to_string()),
///                     })
///                     .build(),
///             ]
///         ).validate()?.build()
/// );
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "builders have no effect if unused"]
pub struct SelectMenuOptionBuilder(SelectMenuOption);

impl SelectMenuOptionBuilder {
    /// Create a new builder to construct a [`SelectMenuOption`].
    pub const fn new(value: String, label: String) -> Self {
        Self(SelectMenuOption {
            default: false,
            description: None,
            emoji: None,
            label,
            value,
        })
    }

    /// Consume the builder, returning a [`SelectMenuOption`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> SelectMenuOption {
        self.0
    }

    /// Ensure the select menu option is valid.
    ///
    /// # Errors
    ///
    /// Refer to the documentation of [`twilight_validate::component::select_menu_option`] for
    /// possible errors.
    pub fn validate(self) -> Result<Self, ComponentValidationError> {
        if let Err(source) = validate_select_menu_option(&self.0) {
            return Err(source);
        }

        Ok(self)
    }

    /// Set whether this option is selected by default.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::SelectMenuOptionBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = SelectMenuOptionBuilder::new("option-1".into(), "Option One".into())
    ///     .default(true)
    ///     .validate()?.build();
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
    /// use twilight_util::builder::component::SelectMenuOptionBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = SelectMenuOptionBuilder::new("option-1".into(), "Option One".into())
    ///     .description("The first option.".into())
    ///     .validate()?.build();
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
    /// use twilight_util::builder::component::SelectMenuOptionBuilder;
    /// use twilight_model::channel::ReactionType;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let option = SelectMenuOptionBuilder::new("option-1".into(), "Option One".into())
    ///     .emoji(ReactionType::Unicode {
    ///         name: "1️⃣".into()
    ///     })
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn emoji(mut self, emoji: ReactionType) -> Self {
        self.0.emoji = Some(emoji);

        self
    }
}

impl TryFrom<SelectMenuOptionBuilder> for SelectMenuOption {
    type Error = ComponentValidationError;

    /// Convert a `SelectMenuOptionBuilder` into a `SelectMenuOption`.
    ///
    /// This is equivalent to calling [`SelectMenuOptionBuilder::validate`]
    /// then [`SelectMenuOptionBuilder::build`].
    fn try_from(builder: SelectMenuOptionBuilder) -> Result<Self, Self::Error> {
        Ok(builder.validate()?.build())
    }
}

#[cfg(test)]
mod tests {
    use super::SelectMenuOptionBuilder;
    use static_assertions::assert_impl_all;
    use std::{convert::TryFrom, fmt::Debug};
    use twilight_model::{
        application::component::select_menu::SelectMenuOption, channel::ReactionType,
    };

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
    fn test_normal() {
        let select_menu =
            SelectMenuOptionBuilder::new("value".to_string(), "label".to_owned()).build();

        let expected = SelectMenuOption {
            default: false,
            description: None,
            emoji: None,
            label: "label".to_string(),
            value: "value".to_string(),
        };

        assert_eq!(select_menu, expected);
    }

    #[test]
    fn test_description() {
        let select_menu = SelectMenuOptionBuilder::new("value".to_string(), "label".to_owned())
            .description("description".to_string())
            .build();

        let expected = SelectMenuOption {
            default: false,
            description: Some("description".to_string()),
            emoji: None,
            label: "label".to_string(),
            value: "value".to_string(),
        };

        assert_eq!(select_menu, expected);
    }

    #[test]
    fn test_default() {
        let select_menu = SelectMenuOptionBuilder::new("value".to_string(), "label".to_owned())
            .default(true)
            .build();

        let expected = SelectMenuOption {
            default: true,
            description: None,
            emoji: None,
            label: "label".to_string(),
            value: "value".to_string(),
        };

        assert_eq!(select_menu, expected);
    }

    #[test]
    fn test_emoji() {
        let select_menu = SelectMenuOptionBuilder::new("value".to_string(), "label".to_owned())
            .emoji(ReactionType::Unicode {
                name: "\u{1f9ea}".to_string(),
            })
            .build();

        let expected = SelectMenuOption {
            default: false,
            description: None,
            emoji: Some(ReactionType::Unicode {
                name: "\u{1f9ea}".to_string(),
            }),
            label: "label".to_string(),
            value: "value".to_string(),
        };

        assert_eq!(select_menu, expected);
    }

    #[test]
    fn test_builder_try_from() {
        let select_menu = SelectMenuOption::try_from(
            SelectMenuOptionBuilder::new("value".to_string(), "label".to_owned())
                .description("testing".to_string()),
        )
        .unwrap();

        let expected = SelectMenuOption {
            default: false,
            description: Some("testing".to_string()),
            emoji: None,
            label: "label".to_string(),
            value: "value".to_string(),
        };

        assert_eq!(select_menu, expected);
    }
}
