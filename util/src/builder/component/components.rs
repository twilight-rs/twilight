use twilight_model::application::component::{ActionRow, Button, Component, SelectMenu, TextInput};
use twilight_validate::component::{
    action_row as validate_action_row, ComponentValidationError, ACTION_ROW_COMPONENT_COUNT,
    COMPONENT_COUNT,
};

///
#[derive(Clone, Debug)]
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
    /// use twilight_util::builder::component::ComponentsBuilder;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let components = ComponentsBuilder::new()
    ///     .action_row(Vec::new())
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```
    pub fn action_row(mut self, components: Vec<Component>) -> Self {
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

    /// Consume the builder, returning a Vec<[`Component`]>.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "builders have no effect if unused"]
    pub fn build(self) -> Vec<Component> {
        self.0
            .into_iter()
            .map(|action_row| Component::ActionRow(action_row))
            .collect()
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
    ///         ButtonBuilder::primary("button-1".to_owned())
    ///         .label("Button".to_owned())
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

                    return self.action_row(Vec::from([Component::Button(button)]));
                }

                action_row.components.push(Component::Button(button));
                self
            }
            None => self.action_row(Vec::from([Component::Button(button)])),
        }
    }

    fn is_full(&self) -> bool {
        self.0.len() == COMPONENT_COUNT
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
    ///         SelectMenuBuilder::new("characters".to_owned())
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
        self.action_row(Vec::from([Component::SelectMenu(select_menu)]))
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
    ///         TextInputBuilder::short("input-1".to_owned(), "Input One".to_owned())
    ///         .build()
    ///     )
    ///     .validate()?.build();
    /// # Ok(()) }
    /// ```  
    pub fn text_input(self, text_input: TextInput) -> Self {
        self.action_row(Vec::from([Component::TextInput(text_input)]))
    }

    /// Consume the builder, ensure that the action rows and their components are valid.
    ///
    /// # Errors
    ///
    /// Refer to the documentation of [`twilight_validate::component::validate_action_row`] for
    /// possible errors.
    pub fn validate(self) -> Result<Self, ComponentValidationError> {
        for action_row in self.0.iter() {
            if let Err(source) = validate_action_row(action_row) {
                return Err(source);
            }
        }

        Ok(self)
    }
}
