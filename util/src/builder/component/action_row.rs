//! Create an [`ActionRow`] with a builder.
//!
//! # Example
//! ```
//! use twilight_model::application::component::Component;
//! use twilight_util::builder::component::{ActionRowBuilder, ButtonBuilder};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let action_row = ActionRowBuilder::new()
//!     .component(
//!         Component::Button(
//!             ButtonBuilder::primary("button-1".to_string())
//!                 .label("Button".to_string())
//!                 .validate()?.build()        
//!         )        
//!     )
//!     .validate()?.build();
//! # Ok(()) }
//! ```

use twilight_model::application::component::{
    action_row::ActionRow, button::Button, select_menu::SelectMenu, text_input::TextInput,
    Component,
};
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
///     .component(
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
    /// Create a new builder to construct an [`ActionRow`].
    pub const fn new() -> Self {
        Self(ActionRow {
            components: Vec::new(),
        })
    }

    /// Consume the builder, returning an [`ActionRow`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> ActionRow {
        self.0
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

    /// Consume the builder, returning an action row wrapped in
    /// [`Component::ActionRow`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn into_component(self) -> Component {
        Component::ActionRow(self.build())
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
    ///     .component(
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
    pub fn component(mut self, component: Component) -> Self {
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
    ///     .components(
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
    pub fn components(mut self, components: &mut Vec<Component>) -> Self {
        self.0.components.append(components);

        self
    }

    /// Add a button to this action row.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::{ActionRowBuilder, ButtonBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let action_row = ActionRowBuilder::new()
    ///     .button(
    ///         ButtonBuilder::primary("button-1".to_string())
    ///         .label("Button".to_string())
    ///         .build()
    ///     )
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```  
    pub fn button(self, button: Button) -> Self {
        self.component(Component::Button(button))
    }

    /// Add a select menu to this action row.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::{ActionRowBuilder, SelectMenuBuilder, SelectMenuOptionBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let action_row = ActionRowBuilder::new()
    ///     .select_menu(
    ///         SelectMenuBuilder::new("characters".to_string())
    ///             .add_options(
    ///             &mut vec![
    ///                 SelectMenuOptionBuilder::new("twilight-sparkle".to_string(), "Twilight Sparkle".to_string())
    ///                     .default(true)
    ///                     .build(),
    ///                 SelectMenuOptionBuilder::new("rarity".to_string(), "Rarity".to_string())
    ///                     .build(),
    ///             ]
    ///         ).build()
    ///     )
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```  
    pub fn select_menu(self, select_menu: SelectMenu) -> Self {
        self.component(Component::SelectMenu(select_menu))
    }

    /// Add a text input to this action row.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::{ActionRowBuilder, TextInputBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let action_row = ActionRowBuilder::new()
    ///     .text_input(
    ///         TextInputBuilder::short("input-1".to_string(), "Input One".to_owned())
    ///         .build()
    ///     )
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```  
    pub fn text_input(self, text_input: TextInput) -> Self {
        self.component(Component::TextInput(text_input))
    }
}

impl TryFrom<ActionRowBuilder> for ActionRow {
    type Error = ComponentValidationError;

    /// Convert an action row builder into an action row, validating its contents.
    ///
    /// This is equivalent to calling [`ActionRowBuilder::validate`], then
    /// [`ActionRowBuilder::build`].
    fn try_from(builder: ActionRowBuilder) -> Result<Self, Self::Error> {
        Ok(builder.validate()?.build())
    }
}

impl TryFrom<ActionRowBuilder> for Component {
    type Error = ComponentValidationError;

    /// Convert an action row builder into a component, validating its contents.
    ///
    /// This is equivalent to calling [`ActionRowBuilder::validate`], then
    /// [`ActionRowBuilder::into_component`].
    fn try_from(builder: ActionRowBuilder) -> Result<Self, Self::Error> {
        Ok(builder.validate()?.into_component())
    }
}

#[cfg(test)]
mod test {
    use super::ActionRowBuilder;
    use static_assertions::assert_impl_all;
    use std::{convert::TryFrom, fmt::Debug};
    use twilight_model::application::component::{
        button::ButtonStyle, text_input::TextInputStyle, ActionRow, Button, Component, SelectMenu,
        TextInput,
    };

    assert_impl_all!(ActionRowBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(ActionRow: TryFrom<ActionRowBuilder>);
    assert_impl_all!(Component: TryFrom<ActionRowBuilder>);

    #[test]
    fn builder() {
        let expected = ActionRow {
            components: Vec::new(),
        };

        let actual = ActionRowBuilder::new().build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn into_component() {
        let expected = Component::ActionRow(ActionRow {
            components: Vec::new(),
        });

        let actual = ActionRowBuilder::new().into_component();

        assert_eq!(actual, expected);
    }

    #[test]
    fn component() {
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
            .component(Component::Button(Button {
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
    fn components() {
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
            .components(&mut Vec::from([
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

    #[test]
    fn button() {
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
            .button(Button {
                custom_id: Some("button".into()),
                disabled: false,
                emoji: None,
                label: Some("label".into()),
                style: ButtonStyle::Primary,
                url: None,
            })
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn select_menu() {
        let expected = ActionRow {
            components: Vec::from([Component::SelectMenu(SelectMenu {
                custom_id: "select_menu".into(),
                disabled: false,
                max_values: None,
                min_values: None,
                options: Vec::new(),
                placeholder: None,
            })]),
        };

        let actual = ActionRowBuilder::new()
            .select_menu(SelectMenu {
                custom_id: "select_menu".into(),
                disabled: false,
                max_values: None,
                min_values: None,
                options: Vec::new(),
                placeholder: None,
            })
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn input_text() {
        let expected = ActionRow {
            components: Vec::from([Component::TextInput(TextInput {
                custom_id: "input_text".into(),
                label: "label".into(),
                max_length: None,
                min_length: None,
                placeholder: None,
                style: TextInputStyle::Short,
                required: None,
                value: None,
            })]),
        };

        let actual = ActionRowBuilder::new()
            .text_input(TextInput {
                custom_id: "input_text".into(),
                label: "label".into(),
                max_length: None,
                min_length: None,
                placeholder: None,
                style: TextInputStyle::Short,
                required: None,
                value: None,
            })
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn action_row_try_from() {
        let expected = ActionRow {
            components: Vec::new(),
        };

        let actual = ActionRow::try_from(ActionRowBuilder::new()).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]

    fn component_try_from() {
        let expected = Component::ActionRow(ActionRow {
            components: Vec::new(),
        });

        let actual = Component::try_from(ActionRowBuilder::new()).unwrap();

        assert_eq!(actual, expected);
    }
}
