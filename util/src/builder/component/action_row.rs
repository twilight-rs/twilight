//! Create an [`ActionRow`] with a builder.
//!
//! # Example
//! ```
//! use twilight_model::application::component::Component;
//! use twilight_util::builder::component::{ActionRowBuilder, ButtonBuilder};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let action_row = ActionRowBuilder::new()
//!     .add_component(
//!         Component::Button(
//!             ButtonBuilder::primary("button-1".to_string())
//!                 .label("Button".to_string())
//!                 .validate()?.build()        
//!         )        
//!     )
//!     .validate()?.build();
//! # Ok(()) }
//! ```

use twilight_model::application::component::{action_row::ActionRow, Component};
use twilight_validate::component::{action_row as validate_action_row, ComponentValidationError};

/// Create an [`ActionRow`] with a builder.
///
/// # Example
/// ```
/// use twilight_model::application::component::Component;
/// use twilight_util::builder::component::{ActionRowBuilder, ButtonBuilder};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let action_row = ActionRowBuilder::new()
///     .add_component(
///         Component::Button(
///             ButtonBuilder::primary("button-1".to_string())
///                 .label("Button".to_string())
///                 .validate()?.build()
///         )
///     )
///     .validate()?.build();
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "builders have no effect if unused"]
pub struct ActionRowBuilder(ActionRow);

impl ActionRowBuilder {
    /// Create a new builder to construct a [`SelectMenu`].
    pub const fn new() -> Self {
        Self(ActionRow {
            components: Vec::new(),
        })
    }

    /// Consume the builder, returning a [`SelectMenu`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> ActionRow {
        self.0
    }

    /// Add a component to this action row.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_model::application::component::Component;
    /// use twilight_util::builder::component::{ActionRowBuilder, ButtonBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let action_row = ActionRowBuilder::new()
    ///     .add_component(
    ///         Component::Button(
    ///             ButtonBuilder::primary("button-1".to_string())
    ///                 .label("Button".to_string())
    ///                 .build()
    ///         )
    ///     )
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn add_component(mut self, component: Component) -> Self {
        self.0.components.push(component);

        self
    }

    /// Add multiple components to this action row.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_model::application::component::Component;
    /// use twilight_util::builder::component::{ActionRowBuilder, ButtonBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let action_row = ActionRowBuilder::new()
    ///     .add_components(
    ///         &mut vec![
    ///             Component::Button(
    ///                 ButtonBuilder::primary("button-1".to_string())
    ///                     .label("First".to_string())
    ///                     .build()
    ///             ),
    ///             Component::Button(
    ///                 ButtonBuilder::secondary("button-2".to_string())
    ///                     .label("Second".to_string())
    ///                     .build()
    ///             )
    ///         ]
    ///     )
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn add_components(mut self, components: &mut Vec<Component>) -> Self {
        self.0.components.append(components);

        self
    }

    /// Ensure the action row is valid.
    ///
    /// # Errors
    ///
    /// Refer to the documentation of [`twilight_validate::component::action_row`] for
    /// possible errors.
    pub fn validate(self) -> Result<Self, ComponentValidationError> {
        if let Err(source) = validate_action_row(&self.0) {
            return Err(source);
        }

        Ok(self)
    }
}

impl TryFrom<ActionRowBuilder> for ActionRow {
    type Error = ComponentValidationError;

    /// Convert a select menu builder into a select menu, validating its contents.
    ///
    /// This is equivalent to calling [`SelectMenuBuilder::validate`], then
    /// [`SelectMenuBuilder::build`].
    fn try_from(builder: ActionRowBuilder) -> Result<Self, Self::Error> {
        Ok(builder.validate()?.build())
    }
}

#[cfg(test)]
mod test {
    use super::ActionRowBuilder;
    use static_assertions::assert_impl_all;
    use std::{convert::TryFrom, fmt::Debug};
    use twilight_model::application::component::{
        button::ButtonStyle, ActionRow, Button, Component,
    };

    assert_impl_all!(ActionRowBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(ActionRow: TryFrom<ActionRowBuilder>);

    #[test]
    fn test_action_row_builder() {
        let expected = ActionRow {
            components: Vec::new(),
        };
        let actual = ActionRowBuilder::new().build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_action_row_builder_add_component() {
        let expected = ActionRow {
            components: Vec::from([Component::Button(Button {
                custom_id: Some("button".into()),
                disabled: false,
                emoji: None,
                label: Some("label".into()),
                style: ButtonStyle::Primary,
                url: None,
            })]),
        };
        let actual = ActionRowBuilder::new()
            .add_component(Component::Button(Button {
                custom_id: Some("button".into()),
                disabled: false,
                emoji: None,
                label: Some("label".into()),
                style: ButtonStyle::Primary,
                url: None,
            }))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_action_row_builder_add_components() {
        let expected = ActionRow {
            components: Vec::from([
                Component::Button(Button {
                    custom_id: Some("button".into()),
                    disabled: false,
                    emoji: None,
                    label: Some("label".into()),
                    style: ButtonStyle::Primary,
                    url: None,
                }),
                Component::Button(Button {
                    custom_id: Some("button-2".into()),
                    disabled: false,
                    emoji: None,
                    label: Some("label".into()),
                    style: ButtonStyle::Primary,
                    url: None,
                }),
            ]),
        };
        let actual = ActionRowBuilder::new()
            .add_components(&mut Vec::from([
                Component::Button(Button {
                    custom_id: Some("button".into()),
                    disabled: false,
                    emoji: None,
                    label: Some("label".into()),
                    style: ButtonStyle::Primary,
                    url: None,
                }),
                Component::Button(Button {
                    custom_id: Some("button-2".into()),
                    disabled: false,
                    emoji: None,
                    label: Some("label".into()),
                    style: ButtonStyle::Primary,
                    url: None,
                }),
            ]))
            .build();

        assert_eq!(actual, expected);
    }
}
