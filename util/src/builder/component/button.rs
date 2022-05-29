//! Create a [`Button`] with a builder.
//!
//! # Example
//! ```
//! use twilight_model::application::component::Component;
//! use twilight_util::builder::component::ButtonBuilder;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let component = Component::Button(
//!     ButtonBuilder::primary("button_id".to_string())
//!         .label("Button label".to_string())
//!         .validate()?.build()
//! );
//! # Ok(()) }
//! ```

use std::convert::TryFrom;

use twilight_model::{
    application::component::{button::ButtonStyle, Button},
    channel::ReactionType,
};
use twilight_validate::component::{button as validate_button, ComponentValidationError};

/// Create a [`Button`] with a builder.
///
/// # Example
/// ```
/// use twilight_model::application::component::Component;
/// use twilight_util::builder::component::ButtonBuilder;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let component = Component::Button(
///     ButtonBuilder::primary("button_id".to_string())
///         .label("Button label".to_string())
///         .validate()?.build()
/// );
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "builders have no effect if unused"]
pub struct ButtonBuilder(Button);

impl ButtonBuilder {
    /// Create a new builder to construct a [`ButtonStyle::Primary`] styled [`Button`].
    pub const fn primary(custom_id: String) -> Self {
        Self(Button {
            style: ButtonStyle::Primary,
            emoji: None,
            label: None,
            custom_id: Some(custom_id),
            url: None,
            disabled: false,
        })
    }

    /// Create a new builder to construct a [`ButtonStyle::Secondary`] styled [`Button`].
    pub const fn secondary(custom_id: String) -> Self {
        Self(Button {
            style: ButtonStyle::Secondary,
            emoji: None,
            label: None,
            custom_id: Some(custom_id),
            url: None,
            disabled: false,
        })
    }

    /// Create a new builder to construct a [`ButtonStyle::Success`] styled [`Button`].
    pub const fn success(custom_id: String) -> Self {
        Self(Button {
            style: ButtonStyle::Success,
            emoji: None,
            label: None,
            custom_id: Some(custom_id),
            url: None,
            disabled: false,
        })
    }

    /// Create a new builder to construct a [`ButtonStyle::Danger`] styled [`Button`].
    pub const fn danger(custom_id: String) -> Self {
        Self(Button {
            style: ButtonStyle::Danger,
            emoji: None,
            label: None,
            custom_id: Some(custom_id),
            url: None,
            disabled: false,
        })
    }

    /// Create a new builder to construct a [`ButtonStyle::Link`] styled [`Button`].
    pub const fn link(url: String) -> Self {
        Self(Button {
            style: ButtonStyle::Link,
            emoji: None,
            label: None,
            custom_id: None,
            url: Some(url),
            disabled: false,
        })
    }

    /// Consume the builder, returning a [`Button`].
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> Button {
        self.0
    }

    /// Ensure the button is valid.
    ///
    /// # Errors
    ///
    /// Refer to the documentation of [`twilight_validate::component::button`] for
    /// possible errors.
    pub fn validate(self) -> Result<Self, ComponentValidationError> {
        if let Err(source) = validate_button(&self.0) {
            return Err(source);
        }

        Ok(self)
    }

    /// Set whether the button is disabled or not.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::ButtonBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let button = ButtonBuilder::primary("unique-id".into())
    ///     .label("disabled button".into())
    ///     .disable(true)
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    pub const fn disable(mut self, disabled: bool) -> Self {
        self.0.disabled = disabled;

        self
    }

    /// Set the emoji of the button.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_model::channel::ReactionType;
    /// use twilight_util::builder::component::ButtonBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let button = ButtonBuilder::primary("unique-id".into())
    ///     .emoji(ReactionType::Unicode {
    ///         name: "🙂".into()
    ///     })
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn emoji(mut self, emoji: ReactionType) -> Self {
        self.0.emoji = Some(emoji);

        self
    }

    /// Set the label of the button.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::ButtonBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let button = ButtonBuilder::primary("unique-id".into())
    ///     .label("twilight".into())
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn label(mut self, label: String) -> Self {
        self.0.label = Some(label);

        self
    }
}

impl TryFrom<ButtonBuilder> for Button {
    type Error = ComponentValidationError;

    /// Convert a button builder into a button, validating its contents.
    ///
    /// This is equivalent to calling [`ButtonBuilder::validate`], then
    /// [`ButtonBuilder::build`].
    fn try_from(builder: ButtonBuilder) -> Result<Self, Self::Error> {
        Ok(builder.validate()?.build())
    }
}

#[cfg(test)]
mod tests {
    use super::ButtonBuilder;
    use static_assertions::assert_impl_all;
    use std::{convert::TryFrom, fmt::Debug};
    use twilight_model::{
        application::component::{button::ButtonStyle, Button},
        channel::ReactionType,
    };

    assert_impl_all!(ButtonBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Button: TryFrom<ButtonBuilder>);

    #[test]
    fn test_builder_primary() {
        let button = ButtonBuilder::primary("primary-button".to_string())
            .label("primary button".to_string())
            .build();

        let expected = Button {
            style: ButtonStyle::Primary,
            emoji: None,
            label: Some("primary button".to_string()),
            custom_id: Some("primary-button".to_string()),
            url: None,
            disabled: false,
        };

        assert_eq!(button, expected);
    }

    #[test]
    fn test_builder_secondary() {
        let button = ButtonBuilder::secondary("secondary-button".to_string())
            .label("secondary button".to_string())
            .build();

        let expected = Button {
            style: ButtonStyle::Secondary,
            emoji: None,
            label: Some("secondary button".to_string()),
            custom_id: Some("secondary-button".to_string()),
            url: None,
            disabled: false,
        };

        assert_eq!(button, expected);
    }

    #[test]
    fn test_builder_success() {
        let button = ButtonBuilder::success("success-button".to_string())
            .label("success button".to_string())
            .build();

        let expected = Button {
            style: ButtonStyle::Success,
            emoji: None,
            label: Some("success button".to_string()),
            custom_id: Some("success-button".to_string()),
            url: None,
            disabled: false,
        };

        assert_eq!(button, expected);
    }

    #[test]
    fn test_builder_danger() {
        let button = ButtonBuilder::danger("danger-button".to_string())
            .label("danger button".to_string())
            .build();

        let expected = Button {
            style: ButtonStyle::Danger,
            emoji: None,
            label: Some("danger button".to_string()),
            custom_id: Some("danger-button".to_string()),
            url: None,
            disabled: false,
        };

        assert_eq!(button, expected);
    }

    #[test]
    fn test_builder_link() {
        let button = ButtonBuilder::link("https://twilight.rs".to_string())
            .label("link button".to_string())
            .build();

        let expected = Button {
            style: ButtonStyle::Link,
            emoji: None,
            label: Some("link button".to_string()),
            custom_id: None,
            url: Some("https://twilight.rs".to_string()),
            disabled: false,
        };

        assert_eq!(button, expected);
    }

    #[test]
    fn test_builder_disabled_button() {
        let button = ButtonBuilder::primary("disabled-button".to_string())
            .label("disabled button".to_string())
            .disable(true)
            .build();

        let expected = Button {
            style: ButtonStyle::Primary,
            emoji: None,
            label: Some("disabled button".to_string()),
            custom_id: Some("disabled-button".to_string()),
            url: None,
            disabled: true,
        };

        assert_eq!(button, expected);
    }

    #[test]
    fn test_builder_explicit_enabled_button() {
        let button = ButtonBuilder::primary("enabled-button".to_string())
            .label("enabled button".to_string())
            .disable(false)
            .build();

        let expected = Button {
            style: ButtonStyle::Primary,
            emoji: None,
            label: Some("enabled button".to_string()),
            custom_id: Some("enabled-button".to_string()),
            url: None,
            disabled: false,
        };

        assert_eq!(button, expected);
    }

    #[test]
    fn test_builder_with_emoji() {
        let button = ButtonBuilder::primary("emoji-button".to_string())
            .emoji(ReactionType::Unicode {
                name: "\u{1f9ea}".to_string(),
            })
            .build();

        let expected = Button {
            style: ButtonStyle::Primary,
            emoji: Some(ReactionType::Unicode {
                name: "\u{1f9ea}".to_string(),
            }),
            label: None,
            custom_id: Some("emoji-button".to_string()),
            url: None,
            disabled: false,
        };

        assert_eq!(button, expected);
    }

    #[test]
    fn test_builder_try_from() {
        let button = Button::try_from(
            ButtonBuilder::primary("primary-button".to_string()).label("primary button".to_owned()),
        )
        .unwrap();

        let expected = Button {
            style: ButtonStyle::Primary,
            emoji: None,
            label: Some("primary button".to_string()),
            custom_id: Some("primary-button".to_string()),
            url: None,
            disabled: false,
        };

        assert_eq!(button, expected);
    }
}
