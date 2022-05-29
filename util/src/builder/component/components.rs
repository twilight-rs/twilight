//! Create a [`Vec<Components>`] with a builder.
//!
//! # Example
//! ```
//! use twilight_util::builder::component::{ButtonBuilder, ComponentsBuilder};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let components = ComponentsBuilder::new()
//!     .button(
//!         ButtonBuilder::primary("button-1".to_string())
//!             .label("Button".to_string())
//!             .build()
//!     )
//!     .validate()?.build();
//! # Ok(()) }
//! ```

use twilight_model::application::component::{ActionRow, Button, Component, SelectMenu, TextInput};
use twilight_validate::component::{
    action_row as validate_action_row, ComponentValidationError, ACTION_ROW_COMPONENT_COUNT,
    COMPONENT_COUNT,
};

/// Create a [`Vec<Components>`] with a builder.
///
/// # Example
/// ```
/// use twilight_util::builder::component::{ButtonBuilder, ComponentsBuilder};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let components = ComponentsBuilder::new()
///     .button(
///         ButtonBuilder::primary("button-1".to_string())
///             .label("Button".to_string())
///             .build()
///     )
///     .validate()?.build();
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use = "builders have no effect if unused"]
pub struct ComponentsBuilder(Vec<ActionRow>);

impl ComponentsBuilder {
    /// Create a new builder to construct a Vec<[`Component`]>.
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Add a new action row to this builder.
    ///
    /// If the builder is already full,
    /// the action row won't be added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_model::application::component::Component;
    /// use twilight_util::builder::component::{ActionRowBuilder, ButtonBuilder, ComponentsBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let components = ComponentsBuilder::new()
    ///     .action_row(
    ///         ActionRowBuilder::new()
    ///             .add_component(
    ///                 Component::Button(
    ///                     ButtonBuilder::primary("button-1".to_string())
    ///                         .label("Button".to_string())
    ///                         .build()
    ///                 )
    ///             )
    ///         .build()
    ///     )
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    pub fn action_row(mut self, action_row: ActionRow) -> Self {
        if self.is_full() {
            return self;
        }

        self.0.push(action_row);

        self
    }

    /// Consume the builder, returning a Vec<[`Component`]>.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> Vec<Component> {
        self.0.into_iter().map(Component::ActionRow).collect()
    }

    /// Ensure the Components are valid.
    ///
    /// # Errors
    ///
    /// Refer to the documentation of [`twilight_validate::component::validate_action_row`] for
    /// possible errors.
    pub fn validate(self) -> Result<Self, ComponentValidationError> {
        for action_row in &self.0 {
            if let Err(source) = validate_action_row(action_row) {
                return Err(source);
            }
        }

        Ok(self)
    }

    /// Add a button to this builder.
    ///
    /// If there is an action row available the button will be added to it
    /// else a new action row will be created.
    ///
    /// If all action rows are full the button won't be added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::{ComponentsBuilder, ButtonBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let components = ComponentsBuilder::new()
    ///     .button(
    ///         ButtonBuilder::primary("button-1".to_string())
    ///         .label("Button".to_string())
    ///         .build()
    ///     )
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```  
    pub fn button(mut self, button: Button) -> Self {
        let action_row: Option<&mut ActionRow> = self.0.iter_mut().last();

        match action_row {
            Some(action_row) => {
                if action_row
                    .components
                    .iter()
                    .any(|c| !matches!(c, Component::Button(_)))
                    || action_row.components.len() == ACTION_ROW_COMPONENT_COUNT
                {
                    if self.is_full() {
                        return self;
                    }

                    return self.action_row_components(Vec::from([Component::Button(button)]));
                }

                action_row.components.push(Component::Button(button));
                self
            }
            None => self.action_row_components(Vec::from([Component::Button(button)])),
        }
    }

    /// Add a select menu to this builder.
    ///
    /// If there is an empty action row available the select menu will be added to it
    /// else a new action row will be created.
    ///
    /// If all action rows are full the action row won't be added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::{ComponentsBuilder, SelectMenuBuilder, SelectMenuOptionBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let components = ComponentsBuilder::new()
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
        self.action_row_components(Vec::from([Component::SelectMenu(select_menu)]))
    }

    /// Add a text input to this builder.
    ///
    /// If there is an action row available the text input will be added to it
    /// else a new action row will be created.
    ///
    /// If all action rows are full the text input won't be added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_util::builder::component::{ComponentsBuilder, TextInputBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let components = ComponentsBuilder::new()
    ///     .text_input(
    ///         TextInputBuilder::short("input-1".to_string(), "Input One".to_owned())
    ///         .build()
    ///     )
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```  
    pub fn text_input(self, text_input: TextInput) -> Self {
        self.action_row_components(Vec::from([Component::TextInput(text_input)]))
    }

    fn action_row_components(mut self, components: Vec<Component>) -> Self {
        if self.is_full() {
            return self;
        }

        match self.0.iter_mut().last() {
            Some(action_row) if action_row.components.is_empty() => {
                action_row.components = components;
            }
            _ => {
                self.0.push(ActionRow { components });
            }
        }

        self
    }

    fn is_full(&self) -> bool {
        self.0.len() == COMPONENT_COUNT
    }
}

impl TryFrom<ComponentsBuilder> for Vec<Component> {
    type Error = ComponentValidationError;

    /// Convert a components builder into a `Vec<Components>`, validating its contents.
    ///
    /// This is equivalent to calling [`Components::validate`], then
    /// [`ComponentsBuilder::build`].
    fn try_from(builder: ComponentsBuilder) -> Result<Self, Self::Error> {
        Ok(builder.validate()?.build())
    }
}

#[cfg(test)]
mod test {
    use super::ComponentsBuilder;
    use static_assertions::assert_impl_all;
    use std::{convert::TryFrom, fmt::Debug};
    use twilight_model::application::component::{
        button::ButtonStyle, text_input::TextInputStyle, ActionRow, Button, Component, SelectMenu,
        TextInput,
    };

    assert_impl_all!(ComponentsBuilder: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Vec<Component>: TryFrom<ComponentsBuilder>);

    fn action_row(components: Vec<Component>) -> ActionRow {
        ActionRow { components }
    }

    fn button(custom_id: &str) -> Button {
        Button {
            custom_id: Some(custom_id.into()),
            disabled: false,
            emoji: None,
            label: Some("label".into()),
            style: ButtonStyle::Primary,
            url: None,
        }
    }

    fn select_menu(custom_id: &str) -> SelectMenu {
        SelectMenu {
            custom_id: custom_id.into(),
            disabled: false,
            min_values: None,
            max_values: None,
            options: Vec::new(),
            placeholder: None,
        }
    }

    fn text_input(custom_id: &str) -> TextInput {
        TextInput {
            custom_id: custom_id.into(),
            label: "label".into(),
            max_length: None,
            min_length: None,
            placeholder: None,
            required: None,
            style: TextInputStyle::Short,
            value: None,
        }
    }

    #[test]
    fn builder() {
        let expected: Vec<Component> = Vec::new();
        let actual = ComponentsBuilder::new().build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn one_action_row() {
        let expected = Vec::from([Component::ActionRow(action_row(Vec::new()))]);
        let actual = ComponentsBuilder::new()
            .action_row(action_row(Vec::new()))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiple_action_rows() {
        let expected = Vec::from([
            Component::ActionRow(action_row(Vec::new())),
            Component::ActionRow(action_row(Vec::new())),
        ]);
        let actual = ComponentsBuilder::new()
            .action_row(action_row(Vec::new()))
            .action_row(action_row(Vec::new()))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn one_button() {
        let expected = Vec::from([Component::ActionRow(ActionRow {
            components: Vec::from([Component::Button(button("button"))]),
        })]);
        let actual = ComponentsBuilder::new().button(button("button")).build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiple_buttons() {
        let expected = Vec::from([Component::ActionRow(ActionRow {
            components: Vec::from([
                Component::Button(button("button-1")),
                Component::Button(button("button-2")),
            ]),
        })]);
        let actual = ComponentsBuilder::new()
            .button(button("button-1"))
            .button(button("button-2"))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn button_with_existing_action_row() {
        let expected = Vec::from([Component::ActionRow(ActionRow {
            components: Vec::from([Component::Button(button("button"))]),
        })]);
        let actual = ComponentsBuilder::new()
            .action_row(ActionRow {
                components: Vec::new(),
            })
            .button(button("button"))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn overflowing_buttons() {
        let expected = Vec::from([
            Component::ActionRow(ActionRow {
                components: Vec::from([
                    Component::Button(button("button-1")),
                    Component::Button(button("button-2")),
                    Component::Button(button("button-3")),
                    Component::Button(button("button-4")),
                    Component::Button(button("button-5")),
                ]),
            }),
            Component::ActionRow(ActionRow {
                components: Vec::from([Component::Button(button("button-6"))]),
            }),
        ]);
        let actual = ComponentsBuilder::new()
            .button(button("button-1"))
            .button(button("button-2"))
            .button(button("button-3"))
            .button(button("button-4"))
            .button(button("button-5"))
            .button(button("button-6"))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn one_select_menu() {
        let expected = Vec::from([Component::ActionRow(action_row(Vec::from([
            Component::SelectMenu(select_menu("select")),
        ])))]);
        let actual = ComponentsBuilder::new()
            .select_menu(select_menu("select"))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiple_select_menus() {
        let expected = Vec::from([
            Component::ActionRow(action_row(Vec::from([Component::SelectMenu(select_menu(
                "select-1",
            ))]))),
            Component::ActionRow(action_row(Vec::from([Component::SelectMenu(select_menu(
                "select-2",
            ))]))),
        ]);
        let actual = ComponentsBuilder::new()
            .select_menu(select_menu("select-1"))
            .select_menu(select_menu("select-2"))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn one_text_input() {
        let expected = Vec::from([Component::ActionRow(action_row(Vec::from([
            Component::TextInput(text_input("input")),
        ])))]);
        let actual = ComponentsBuilder::new()
            .text_input(text_input("input"))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn multiple_text_inputs() {
        let expected = Vec::from([
            Component::ActionRow(action_row(Vec::from([Component::TextInput(text_input(
                "input-1",
            ))]))),
            Component::ActionRow(action_row(Vec::from([Component::TextInput(text_input(
                "input-2",
            ))]))),
        ]);
        let actual = ComponentsBuilder::new()
            .text_input(text_input("input-1"))
            .text_input(text_input("input-2"))
            .build();

        assert_eq!(actual, expected);
    }
}
