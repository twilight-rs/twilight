//! Constants, error types, and functions for validating [`Component`]s.

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};
use twilight_model::channel::message::component::{
    ActionRow, Button, ButtonStyle, Component, ComponentType, SelectMenu, SelectMenuOption,
    SelectMenuType, TextInput,
};

/// Maximum number of [`Component`]s allowed inside an [`ActionRow`].
///
/// This is defined in Discord's documentation, per
/// [Discord Docs/Action Rows][1].
///
/// [1]: https://discord.com/developers/docs/interactions/message-components#action-rows
pub const ACTION_ROW_COMPONENT_COUNT: usize = 5;

/// Maximum number of root [`Component`]s in a message.
///
/// This is defined in Discord's documentation, per
/// [Discord Docs/Action Row][1].
///
/// [1]: https://discord.com/developers/docs/interactions/message-components#action-rows
pub const COMPONENT_COUNT: usize = 5;

/// Maximum length of a [`Component`] custom ID in codepoints.
///
/// An example of a component with a custom ID is the
/// [`Button`][`Button::custom_id`].
///
/// This is defined in Discord's documentation, per
/// [Discord Docs/Components][1].
///
/// [1]: https://discord.com/developers/docs/interactions/message-components#component-object-component-structure
pub const COMPONENT_CUSTOM_ID_LENGTH: usize = 100;

/// Maximum [`Component`] label length in codepoints.
///
/// An example of a component with a label is the [`Button`][`Button::label`].
///
/// This is defined in Discord's documentation, per
/// [Discord Docs/Components][1].
///
/// [1]: https://discord.com/developers/docs/interactions/message-components#component-object-component-structure
pub const COMPONENT_BUTTON_LABEL_LENGTH: usize = 80;

/// Maximum number of [`SelectMenuOption`]s that can be chosen in a
/// [`SelectMenu`].
///
/// This is defined in Dicsord's documentation, per
/// [Discord Docs/Select Menu][1].
///
/// [1]: https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-menu-structure
pub const SELECT_MAXIMUM_VALUES_LIMIT: usize = 25;

/// Minimum number of [`SelectMenuOption`]s that can be chosen in a
/// [`SelectMenu`].
///
/// This is defined in Dicsord's documentation, per
/// [Discord Docs/Select Menu][1].
///
/// [1]: https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-menu-structure
pub const SELECT_MAXIMUM_VALUES_REQUIREMENT: usize = 1;

/// Maximum number of [`SelectMenuOption`]s that must be chosen in a
/// [`SelectMenu`].
///
/// This is defined in Dicsord's documentation, per
/// [Discord Docs/Select Menu][1].
///
/// [1]: https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-menu-structure
pub const SELECT_MINIMUM_VALUES_LIMIT: usize = 25;

/// Maximum number of [`SelectMenuOption`]s in a [`SelectMenu`].
///
/// This is defined in Discord's documentation, per
/// [Discord Docs/Select Menu][1].
///
/// [1]: https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-menu-structure
pub const SELECT_OPTION_COUNT: usize = 25;

/// Maximum length of a [`SelectMenuOption::description`] in codepoints.
///
/// This is defined in Discord's documentation, per
/// [Discord Docs/Select Menu Option][1].
///
/// [1]: https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure
pub const SELECT_OPTION_DESCRIPTION_LENGTH: usize = 100;

/// Maximum length of a [`SelectMenuOption::label`] in codepoints.
///
/// This is defined in Discord's documentation, per
/// [Discord Docs/Select Menu Option][1].
///
/// [1]: https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure
pub const SELECT_OPTION_LABEL_LENGTH: usize = 100;

/// Maximum length of a [`SelectMenuOption::value`] in codepoints.
///
/// This is defined in Discord's documentation, per
/// [Discord Docs/Select Menu Option][1].
///
/// [1]: https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-option-structure
pub const SELECT_OPTION_VALUE_LENGTH: usize = 100;

/// Maximum length of a [`SelectMenu::placeholder`] in codepoints.
///
/// This is defined in Discord's documentation, per
/// [Discord Docs/Select Menu][1].
///
/// [1]: https://discord.com/developers/docs/interactions/message-components#select-menu-object-select-menu-structure
pub const SELECT_PLACEHOLDER_LENGTH: usize = 150;

/// Maximum length of [`TextInput::label`].
///
/// This is based on [Discord Docs/Text Inputs].
///
/// [Discord Docs/Text Inputs]: https://discord.com/developers/docs/interactions/message-components#text-inputs
pub const TEXT_INPUT_LABEL_MAX: usize = 45;

/// Minimum length of [`TextInput::label`].
///
/// This is based on [Discord Docs/Text Inputs].
///
/// [Discord Docs/Text Inputs]: https://discord.com/developers/docs/interactions/message-components#text-inputs
pub const TEXT_INPUT_LABEL_MIN: usize = 1;

/// Maximum length of [`TextInput::value`].
///
/// This is based on [Discord Docs/Text Inputs].
///
/// [Discord Docs/Text Inputs]: https://discord.com/developers/docs/interactions/message-components#text-inputs
pub const TEXT_INPUT_LENGTH_MAX: usize = 4000;

/// Minimum length of [`TextInput::value`].
///
/// This is based on [Discord Docs/Text Inputs].
///
/// [Discord Docs/Text Inputs]: https://discord.com/developers/docs/interactions/message-components#text-inputs
pub const TEXT_INPUT_LENGTH_MIN: usize = 1;

/// Maximum length of a [`TextInput::placeholder`] in codepoints.
///
/// This is based on [Discord Docs/Text Inputs].
///
/// [Discord Docs/Text Inputs]: https://discord.com/developers/docs/interactions/message-components#text-inputs
pub const TEXT_INPUT_PLACEHOLDER_MAX: usize = 100;

/// A provided [`Component`] is invalid.
///
/// While multiple components may be invalid, validation will short-circuit on
/// the first invalid component.
#[derive(Debug)]
pub struct ComponentValidationError {
    /// Type of error that occurred.
    kind: ComponentValidationErrorType,
}

impl ComponentValidationError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ComponentValidationErrorType {
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
        ComponentValidationErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for ComponentValidationError {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ComponentValidationErrorType::ActionRowComponentCount { count } => {
                f.write_str("an action row has ")?;
                Display::fmt(&count, f)?;
                f.write_str(" children, but the max is ")?;

                Display::fmt(&ACTION_ROW_COMPONENT_COUNT, f)
            }
            ComponentValidationErrorType::ButtonConflict => {
                f.write_str("button has both a custom id and url, which is never valid")
            }
            ComponentValidationErrorType::ButtonStyle { style } => {
                f.write_str("button has a type of ")?;
                Debug::fmt(style, f)?;
                f.write_str(", which must have a ")?;

                f.write_str(if *style == ButtonStyle::Link {
                    "url"
                } else {
                    "custom id"
                })?;

                f.write_str(" configured")
            }
            ComponentValidationErrorType::ComponentCount { count } => {
                Display::fmt(count, f)?;
                f.write_str(" components were provided, but the max is ")?;

                Display::fmt(&COMPONENT_COUNT, f)
            }
            ComponentValidationErrorType::ComponentCustomIdLength { chars } => {
                f.write_str("a component's custom id is ")?;
                Display::fmt(&chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&COMPONENT_CUSTOM_ID_LENGTH, f)
            }
            ComponentValidationErrorType::ComponentLabelLength { chars } => {
                f.write_str("a component's label is ")?;
                Display::fmt(&chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&COMPONENT_BUTTON_LABEL_LENGTH, f)
            }
            ComponentValidationErrorType::InvalidChildComponent { kind } => {
                f.write_str("a '")?;
                Display::fmt(&kind, f)?;

                f.write_str(" component was provided, but can not be a child component")
            }
            ComponentValidationErrorType::InvalidRootComponent { kind } => {
                f.write_str("a '")?;
                Display::fmt(kind, f)?;

                f.write_str("' component was provided, but can not be a root component")
            }
            ComponentValidationErrorType::SelectMaximumValuesCount { count } => {
                f.write_str("maximum number of values that can be chosen is ")?;
                Display::fmt(count, f)?;
                f.write_str(", but must be greater than or equal to ")?;
                Display::fmt(&SELECT_MAXIMUM_VALUES_REQUIREMENT, f)?;
                f.write_str("and less than or equal to ")?;

                Display::fmt(&SELECT_MAXIMUM_VALUES_LIMIT, f)
            }
            ComponentValidationErrorType::SelectMinimumValuesCount { count } => {
                f.write_str("maximum number of values that must be chosen is ")?;
                Display::fmt(count, f)?;
                f.write_str(", but must be less than or equal to ")?;

                Display::fmt(&SELECT_MAXIMUM_VALUES_LIMIT, f)
            }
            ComponentValidationErrorType::SelectNotEnoughDefaultValues { provided, min } => {
                f.write_str("a select menu provided ")?;
                Display::fmt(provided, f)?;
                f.write_str(" values, but it requires at least ")?;
                Display::fmt(min, f)?;
                f.write_str(" values")
            }
            ComponentValidationErrorType::SelectOptionsMissing => {
                f.write_str("a text select menu doesn't specify the required options field")
            }
            ComponentValidationErrorType::SelectOptionDescriptionLength { chars } => {
                f.write_str("a select menu option's description is ")?;
                Display::fmt(&chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&SELECT_OPTION_DESCRIPTION_LENGTH, f)
            }
            ComponentValidationErrorType::SelectOptionLabelLength { chars } => {
                f.write_str("a select menu option's label is ")?;
                Display::fmt(&chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&SELECT_OPTION_LABEL_LENGTH, f)
            }
            ComponentValidationErrorType::SelectOptionValueLength { chars } => {
                f.write_str("a select menu option's value is ")?;
                Display::fmt(&chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&SELECT_OPTION_VALUE_LENGTH, f)
            }
            ComponentValidationErrorType::SelectPlaceholderLength { chars } => {
                f.write_str("a select menu's placeholder is ")?;
                Display::fmt(&chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&SELECT_PLACEHOLDER_LENGTH, f)
            }
            ComponentValidationErrorType::SelectOptionCount { count } => {
                f.write_str("a select menu has ")?;
                Display::fmt(&count, f)?;
                f.write_str(" options, but the max is ")?;

                Display::fmt(&SELECT_OPTION_COUNT, f)
            }
            ComponentValidationErrorType::SelectTooManyDefaultValues { provided, max } => {
                f.write_str("a select menu provided ")?;
                Display::fmt(provided, f)?;
                f.write_str(" values, but it allows at most ")?;
                Display::fmt(max, f)?;
                f.write_str(" values")
            }
            ComponentValidationErrorType::SelectUnsupportedDefaultValues { kind } => {
                f.write_str("a select menu has defined default_values, but its type, ")?;
                Debug::fmt(kind, f)?;
                f.write_str(", does not support them")
            }
            ComponentValidationErrorType::TextInputLabelLength { len: count } => {
                f.write_str("a text input label length is ")?;
                Display::fmt(count, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&TEXT_INPUT_LABEL_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&TEXT_INPUT_LABEL_MAX, f)
            }
            ComponentValidationErrorType::TextInputMaxLength { len: count } => {
                f.write_str("a text input max length is ")?;
                Display::fmt(count, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&TEXT_INPUT_LENGTH_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&TEXT_INPUT_LENGTH_MAX, f)
            }
            ComponentValidationErrorType::TextInputMinLength { len: count } => {
                f.write_str("a text input min length is ")?;
                Display::fmt(count, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&TEXT_INPUT_LENGTH_MAX, f)
            }
            ComponentValidationErrorType::TextInputPlaceholderLength { chars } => {
                f.write_str("a text input's placeholder is ")?;
                Display::fmt(&chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&TEXT_INPUT_PLACEHOLDER_MAX, f)
            }
            ComponentValidationErrorType::TextInputValueLength { chars } => {
                f.write_str("a text input's value is ")?;
                Display::fmt(&chars, f)?;
                f.write_str(" characters long, but the max is ")?;

                Display::fmt(&TEXT_INPUT_PLACEHOLDER_MAX, f)
            }
        }
    }
}

impl Error for ComponentValidationError {}

/// Type of [`ComponentValidationError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum ComponentValidationErrorType {
    /// Number of components a provided [`ActionRow`] is larger than
    /// [the maximum][`ACTION_ROW_COMPONENT_COUNT`].
    ActionRowComponentCount {
        /// Number of components within the action row.
        count: usize,
    },
    /// Button has both a custom ID and URL set.
    ButtonConflict,
    /// Button does not have the required field based on its style.
    ///
    /// A button with a style of [`ButtonStyle::Link`] must have a URL set,
    /// while buttons of other styles must have a custom ID set.
    ButtonStyle {
        /// Style of the button.
        style: ButtonStyle,
    },
    /// Number of components provided is larger than
    /// [the maximum][`COMPONENT_COUNT`].
    ComponentCount {
        /// Number of components that were provided.
        count: usize,
    },
    /// Component custom ID is larger than the
    /// [the maximum][`COMPONENT_CUSTOM_ID_LENGTH`].
    ComponentCustomIdLength {
        /// Number of codepoints that were provided.
        chars: usize,
    },
    /// Component label is larger than [the maximum][`COMPONENT_BUTTON_LABEL_LENGTH`].
    ComponentLabelLength {
        /// Number of codepoints that were provided.
        chars: usize,
    },
    /// Provided component cannot be a child component.
    InvalidChildComponent {
        /// Type of provided component.
        kind: ComponentType,
    },
    /// Provided component cannot be a root component.
    InvalidRootComponent {
        /// Type of provided component.
        kind: ComponentType,
    },
    /// Maximum number of items that can be chosen is smaller than
    /// [the minimum][`SELECT_MAXIMUM_VALUES_REQUIREMENT`] or larger than
    /// [the maximum][`SELECT_MAXIMUM_VALUES_LIMIT`].
    SelectMaximumValuesCount {
        /// Number of options that were provided.
        count: usize,
    },
    /// Minimum number of items that must be chosen is larger than
    /// [the maximum][`SELECT_MINIMUM_VALUES_LIMIT`].
    SelectMinimumValuesCount {
        /// Number of options that were provided.
        count: usize,
    },
    /// The select menu specifies less default values than its own minimum values requirement.
    SelectNotEnoughDefaultValues {
        /// Number of default values provided.
        provided: usize,
        /// Select menu's minimum number of default values.
        min: usize,
    },
    /// The `options` field is `None` for a [text select menu][text-select].
    ///
    /// [text-select]: SelectMenuType::Text
    SelectOptionsMissing,
    /// Number of select menu options provided is larger than
    /// [the maximum][`SELECT_OPTION_COUNT`].
    SelectOptionCount {
        /// Number of options that were provided.
        count: usize,
    },
    /// Description of a select menu option is larger than
    /// [the maximum][`SELECT_OPTION_DESCRIPTION_LENGTH`].
    SelectOptionDescriptionLength {
        /// Number of codepoints that were provided.
        chars: usize,
    },
    /// Label of a select menu option is larger than
    /// [the maximum][`SELECT_OPTION_LABEL_LENGTH`].
    SelectOptionLabelLength {
        /// Number of codepoints that were provided.
        chars: usize,
    },
    /// Value of a select menu option is larger than
    /// [the maximum][`SELECT_OPTION_VALUE_LENGTH`].
    SelectOptionValueLength {
        /// Number of codepoints that were provided.
        chars: usize,
    },
    /// Placeholder of a component is larger than the
    /// [maximum][`SELECT_PLACEHOLDER_LENGTH`].
    SelectPlaceholderLength {
        /// Number of codepoints that were provided.
        chars: usize,
    },
    /// The select menu specifies less default values than its own minimum values requirement.
    SelectTooManyDefaultValues {
        /// Number of default values provided.
        provided: usize,
        /// Select menu's maximum number of values.
        max: usize,
    },
    /// The select menu type doesn't support the `default_values` field.
    SelectUnsupportedDefaultValues {
        /// The select menu's type.
        kind: SelectMenuType,
    },
    /// [`TextInput::label`] is invalid.
    TextInputLabelLength {
        /// Provided length.
        len: usize,
    },
    /// [`TextInput::max_length`] is invalid.
    TextInputMaxLength {
        /// Provided length.
        len: usize,
    },
    /// [`TextInput::min_length`] is too long.
    TextInputMinLength {
        /// Provided length.
        len: usize,
    },
    /// Placeholder of a [`TextInput`] component is larger than
    /// [`TEXT_INPUT_PLACEHOLDER_MAX`].
    TextInputPlaceholderLength {
        /// Provided number of codepoints.
        chars: usize,
    },
    /// Value of a [`TextInput`] component is larger than
    /// [`TEXT_INPUT_LENGTH_MAX`].
    TextInputValueLength {
        /// Provided number of codepoints.
        chars: usize,
    },
}

/// Ensure that a top-level request component is correct.
///
/// Intended to ensure that a fully formed top-level component for requests
/// is an action row.
///
/// Refer to other validators like [`button`] if you need to validate other
/// components.
///
/// # Errors
///
/// Returns an error of type [`InvalidRootComponent`] if the component is not an
/// [`ActionRow`].
///
/// Refer to [`action_row`] for potential errors when validating an action row
/// component.
///
/// [`InvalidRootComponent`]: ComponentValidationErrorType::InvalidRootComponent
pub fn component(component: &Component) -> Result<(), ComponentValidationError> {
    match component {
        Component::ActionRow(action_row) => self::action_row(action_row)?,
        other => {
            return Err(ComponentValidationError {
                kind: ComponentValidationErrorType::InvalidRootComponent { kind: other.kind() },
            });
        }
    }

    Ok(())
}

/// Ensure that an action row is correct.
///
/// # Errors
///
/// Returns an error of type [`ActionRowComponentCount`] if the action row has
/// too many components in it.
///
/// Returns an error of type [`InvalidChildComponent`] if the provided nested
/// component is an [`ActionRow`]. Action rows can not contain another action
/// row.
///
/// Refer to [`button`] for potential errors when validating a button in the
/// action row.
///
/// Refer to [`select_menu`] for potential errors when validating a select menu
/// in the action row.
///
/// Refer to [`text_input`] for potential errors when validating a text input in
/// the action row.
///
/// [`ActionRowComponentCount`]: ComponentValidationErrorType::ActionRowComponentCount
/// [`InvalidChildComponent`]: ComponentValidationErrorType::InvalidChildComponent
pub fn action_row(action_row: &ActionRow) -> Result<(), ComponentValidationError> {
    self::component_action_row_components(&action_row.components)?;

    for component in &action_row.components {
        match component {
            Component::ActionRow(_) => {
                return Err(ComponentValidationError {
                    kind: ComponentValidationErrorType::InvalidChildComponent {
                        kind: ComponentType::ActionRow,
                    },
                });
            }
            Component::Button(button) => self::button(button)?,
            Component::SelectMenu(select_menu) => self::select_menu(select_menu)?,
            Component::TextInput(text_input) => self::text_input(text_input)?,
            Component::Unknown(unknown) => {
                return Err(ComponentValidationError {
                    kind: ComponentValidationErrorType::InvalidChildComponent {
                        kind: ComponentType::Unknown(*unknown),
                    },
                })
            }
        }
    }

    Ok(())
}

/// Ensure that a button is correct.
///
/// # Errors
///
/// Returns an error of type [`ButtonConflict`] if both a custom ID and URL are
/// specified.
///
/// Returns an error of type
/// [`ButtonStyle`][`ComponentValidationErrorType::ButtonStyle`] if
/// [`ButtonStyle::Link`] is provided and a URL is provided, or if the style is
/// not [`ButtonStyle::Link`] and a custom ID is not provided.
///
/// Returns an error of type [`ComponentCustomIdLength`] if the provided custom
/// ID is too long.
///
/// Returns an error of type [`ComponentLabelLength`] if the provided button
/// label is too long.
///
/// [`ButtonConflict`]: ComponentValidationErrorType::ButtonConflict
/// [`ComponentCustomIdLength`]: ComponentValidationErrorType::ComponentCustomIdLength
/// [`ComponentLabelLength`]: ComponentValidationErrorType::ComponentLabelLength
pub fn button(button: &Button) -> Result<(), ComponentValidationError> {
    let has_custom_id = button.custom_id.is_some();
    let has_url = button.url.is_some();

    // First check if a custom ID and URL are both set. If so this
    // results in a conflict, as no valid button may have both set.
    if has_custom_id && has_url {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::ButtonConflict,
        });
    }

    // Next, we check if the button is a link and a URL is not set.
    //
    // Lastly, we check if the button is not a link and a custom ID is
    // not set.
    let is_link = button.style == ButtonStyle::Link;

    if (is_link && !has_url) || (!is_link && !has_custom_id) {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::ButtonStyle {
                style: button.style,
            },
        });
    }

    if let Some(custom_id) = button.custom_id.as_ref() {
        self::component_custom_id(custom_id)?;
    }

    if let Some(label) = button.label.as_ref() {
        self::component_button_label(label)?;
    }

    Ok(())
}

/// Ensure that a select menu is correct.
///
/// # Errors
///
/// Returns an error of type [`ComponentCustomIdLength`] if the provided custom
/// ID is too long.
///
/// Returns an error of type [`ComponentLabelLength`] if the provided button
/// label is too long.
///
/// Returns an error of type [`SelectMaximumValuesCount`] if the provided number
/// of select menu values that can be chosen is smaller than the minimum or
/// larger than the maximum.
///
/// Returns an error of type [`SelectMinimumValuesCount`] if the provided number
/// of select menu values that must be chosen is larger than the maximum.
///
/// Returns an error of type [`SelectOptionDescriptionLength`] if a provided
/// select option description is too long.
///
/// Returns an error of type [`SelectOptionLabelLength`] if a provided select
/// option label is too long.
///
/// Returns an error of type [`SelectOptionValueLength`] error type if
/// a provided select option value is too long.
///
/// Returns an error of type [`SelectPlaceholderLength`] if a provided select
/// placeholder is too long.
///
/// Returns an error of type [`SelectUnsupportedDefaultValues`] if the select menu's type doesn't
/// support the `default_values` field.
///
/// Returns an error of type [`SelectNotEnoughDefaultValues`] if the select menu specifies fewer
/// default values than its minimum values property.
///
/// Returns an error of type [`SelectTooManyDefaultValues`] if the select menu specifies more
/// default values than its maximum values property.
///
/// [`ComponentCustomIdLength`]: ComponentValidationErrorType::ComponentCustomIdLength
/// [`ComponentLabelLength`]: ComponentValidationErrorType::ComponentLabelLength
/// [`SelectMaximumValuesCount`]: ComponentValidationErrorType::SelectMaximumValuesCount
/// [`SelectMinimumValuesCount`]: ComponentValidationErrorType::SelectMinimumValuesCount
/// [`SelectOptionDescriptionLength`]: ComponentValidationErrorType::SelectOptionDescriptionLength
/// [`SelectOptionLabelLength`]: ComponentValidationErrorType::SelectOptionLabelLength
/// [`SelectOptionValueLength`]: ComponentValidationErrorType::SelectOptionValueLength
/// [`SelectPlaceholderLength`]: ComponentValidationErrorType::SelectPlaceholderLength
/// [`SelectUnsupportedDefaultValues`]: ComponentValidationErrorType::SelectUnsupportedDefaultValues
/// [`SelectNotEnoughDefaultValues`]: ComponentValidationErrorType::SelectNotEnoughDefaultValues
/// [`SelectTooManyDefaultValues`]: ComponentValidationErrorType::SelectTooManyDefaultValues
pub fn select_menu(select_menu: &SelectMenu) -> Result<(), ComponentValidationError> {
    self::component_custom_id(&select_menu.custom_id)?;

    // There aren't any requirements for channel_types that we could validate here
    if let SelectMenuType::Text = &select_menu.kind {
        let options = select_menu
            .options
            .as_ref()
            .ok_or(ComponentValidationError {
                kind: ComponentValidationErrorType::SelectOptionsMissing,
            })?;
        for option in options {
            component_select_option_label(&option.label)?;
            component_select_option_value(&option.value)?;

            if let Some(description) = option.description.as_ref() {
                component_option_description(description)?;
            }
        }
        component_select_options(options)?;
    }

    if let Some(placeholder) = select_menu.placeholder.as_ref() {
        self::component_select_placeholder(placeholder)?;
    }

    if let Some(max_values) = select_menu.max_values {
        self::component_select_max_values(usize::from(max_values))?;
    }

    if let Some(min_values) = select_menu.min_values {
        self::component_select_min_values(usize::from(min_values))?;
    }

    if let Some(default_values) = select_menu.default_values.as_ref() {
        component_select_default_values_supported(select_menu.kind)?;
        component_select_default_values_count(
            select_menu.min_values,
            select_menu.max_values,
            default_values.len(),
        )?;
    }

    Ok(())
}

/// Ensure that a text input is correct.
///
/// # Errors
///
/// Returns an error of type [`ComponentCustomIdLength`] if the provided custom
/// ID is too long.
///
/// Returns an error of type [`ComponentLabelLength`] if the provided button
/// label is too long.
///
/// Returns an error of type [`TextInputMaxLength`] if the length is invalid.
///
/// Returns an error of type [`TextInputMinLength`] if the length is invalid.
///
/// Returns an error of type [`TextInputPlaceholderLength`] if the provided
/// placeholder is too long.
///
/// Returns an error of type [`TextInputValueLength`] if the length is invalid.
///
/// [`ComponentCustomIdLength`]: ComponentValidationErrorType::ComponentCustomIdLength
/// [`ComponentLabelLength`]: ComponentValidationErrorType::ComponentLabelLength
/// [`TextInputMaxLength`]: ComponentValidationErrorType::TextInputMaxLength
/// [`TextInputMinLength`]: ComponentValidationErrorType::TextInputMinLength
/// [`TextInputPlaceholderLength`]: ComponentValidationErrorType::TextInputPlaceholderLength
/// [`TextInputValueLength`]: ComponentValidationErrorType::TextInputValueLength
pub fn text_input(text_input: &TextInput) -> Result<(), ComponentValidationError> {
    self::component_custom_id(&text_input.custom_id)?;
    self::component_text_input_label(&text_input.label)?;

    if let Some(max_length) = text_input.max_length {
        self::component_text_input_max(max_length)?;
    }

    if let Some(min_length) = text_input.min_length {
        self::component_text_input_min(min_length)?;
    }

    if let Some(placeholder) = text_input.placeholder.as_ref() {
        self::component_text_input_placeholder(placeholder)?;
    }

    if let Some(value) = text_input.value.as_ref() {
        self::component_text_input_value(value)?;
    }

    Ok(())
}

/// Validate that an [`ActionRow`] does not contain too many components.
///
/// [`ActionRow`]s may only have so many components within it, defined by
/// [`ACTION_ROW_COMPONENT_COUNT`].
///
/// # Errors
///
/// Returns an error of type [`ActionRowComponentCount`] if the provided list of
/// components is too many for an [`ActionRow`].
///
/// [`ActionRowComponentCount`]: ComponentValidationErrorType::ActionRowComponentCount
/// [`ActionRow`]: twilight_model::application::component::ActionRow
const fn component_action_row_components(
    components: &[Component],
) -> Result<(), ComponentValidationError> {
    let count = components.len();

    if count > COMPONENT_COUNT {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::ActionRowComponentCount { count },
        });
    }

    Ok(())
}

/// Validate that a [`Component`]'s label is not too long.
///
/// # Errors
///
/// Returns an error of type [`ComponentLabelLength`] if the provided component
/// label is too long.
///
/// [`ComponentLabelLength`]: ComponentValidationErrorType::ComponentLabelLength
fn component_button_label(label: impl AsRef<str>) -> Result<(), ComponentValidationError> {
    let chars = label.as_ref().chars().count();

    if chars > COMPONENT_BUTTON_LABEL_LENGTH {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::ComponentLabelLength { chars },
        });
    }

    Ok(())
}

/// Validate that a custom ID is not too long.
///
/// # Errors
///
/// Returns an error of type [`ComponentCustomIdLength`] if the provided custom
/// ID is too long.
///
/// [`ComponentCustomIdLength`]: ComponentValidationErrorType::ComponentCustomIdLength
fn component_custom_id(custom_id: impl AsRef<str>) -> Result<(), ComponentValidationError> {
    let chars = custom_id.as_ref().chars().count();

    if chars > COMPONENT_CUSTOM_ID_LENGTH {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::ComponentCustomIdLength { chars },
        });
    }

    Ok(())
}

/// Validate a [`SelectMenuOption::description`]'s length.
///
/// # Errors
///
/// Returns an error of type [`SelectOptionDescriptionLength`] if the provided
/// select option description is too long.
///
/// [`SelectMenuOption::description`]: twilight_model::application::component::select_menu::SelectMenuOption::description
/// [`SelectOptionDescriptionLength`]: ComponentValidationErrorType::SelectOptionDescriptionLength
fn component_option_description(
    description: impl AsRef<str>,
) -> Result<(), ComponentValidationError> {
    let chars = description.as_ref().chars().count();

    if chars > SELECT_OPTION_DESCRIPTION_LENGTH {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::SelectOptionDescriptionLength { chars },
        });
    }

    Ok(())
}

/// Validate a [`SelectMenuType`] supports the `default_values` field.
///
/// # Errors
///
/// Returns an error of type [`SelectUnsupportedDefaultValues`] if the provided component type
/// doesn't support the `default_values` field.
const fn component_select_default_values_supported(
    menu_type: SelectMenuType,
) -> Result<(), ComponentValidationError> {
    if !matches!(
        menu_type,
        SelectMenuType::User
            | SelectMenuType::Role
            | SelectMenuType::Mentionable
            | SelectMenuType::Channel
    ) {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::SelectUnsupportedDefaultValues { kind: menu_type },
        });
    }

    Ok(())
}

/// Validate a [`SelectMenu`]'s `default_values` field has the correct number of values.
///
/// # Errors
///
/// Returns an error of the type [`SelectTooManyDefaultValues`] if the provided list of default
/// values exceeds the provided `max_values` (if present).
///
/// Alternatively, this returns an error of the type [`SelectNotEnoughDefaultValues`] if the
/// provided list of default values doesn't meet the provided `min_values` requirement (if present).
const fn component_select_default_values_count(
    min_values: Option<u8>,
    max_values: Option<u8>,
    default_values: usize,
) -> Result<(), ComponentValidationError> {
    if let Some(min) = min_values {
        let min = min as usize;
        if default_values < min {
            return Err(ComponentValidationError {
                kind: ComponentValidationErrorType::SelectNotEnoughDefaultValues {
                    provided: default_values,
                    min,
                },
            });
        }
    }
    if let Some(max) = max_values {
        let max = max as usize;
        if default_values > max {
            return Err(ComponentValidationError {
                kind: ComponentValidationErrorType::SelectTooManyDefaultValues {
                    provided: default_values,
                    max,
                },
            });
        }
    }

    Ok(())
}

/// Validate a [`SelectMenu::max_values`] amount.
///
/// # Errors
///
/// Returns an error of type [`SelectMaximumValuesCount`] if the provided number
/// of values that can be chosen is smaller than
/// [the minimum][`SELECT_MAXIMUM_VALUES_REQUIREMENT`] or larger than
/// [the maximum][`SELECT_MAXIMUM_VALUES_LIMIT`].
///
/// [`SelectMenu::max_values`]: twilight_model::application::component::select_menu::SelectMenu::max_values
/// [`SelectMaximumValuesCount`]: ComponentValidationErrorType::SelectMaximumValuesCount
const fn component_select_max_values(count: usize) -> Result<(), ComponentValidationError> {
    if count > SELECT_MAXIMUM_VALUES_LIMIT {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::SelectMaximumValuesCount { count },
        });
    }

    if count < SELECT_MAXIMUM_VALUES_REQUIREMENT {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::SelectMaximumValuesCount { count },
        });
    }

    Ok(())
}

/// Validate a [`SelectMenu::min_values`] amount.
///
/// # Errors
///
/// Returns an error of type [`SelectMinimumValuesCount`] if the provided number
/// of values that must be chosen is larger than
/// [the maximum][`SELECT_MINIMUM_VALUES_LIMIT`].
///
/// [`SelectMenu::min_values`]: twilight_model::application::component::select_menu::SelectMenu::min_values
/// [`SelectMinimumValuesCount`]: ComponentValidationErrorType::SelectMinimumValuesCount
const fn component_select_min_values(count: usize) -> Result<(), ComponentValidationError> {
    if count > SELECT_MINIMUM_VALUES_LIMIT {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::SelectMinimumValuesCount { count },
        });
    }

    Ok(())
}

/// Validate a [`SelectMenuOption::label`]'s length.
///
/// # Errors
///
/// Returns an error of type [`SelectOptionLabelLength`] if the provided select
/// option label is too long.
///
/// [`SelectMenuOption::label`]: twilight_model::application::component::select_menu::SelectMenuOption::label
/// [`SelectOptionLabelLength`]: ComponentValidationErrorType::SelectOptionLabelLength
fn component_select_option_label(label: impl AsRef<str>) -> Result<(), ComponentValidationError> {
    let chars = label.as_ref().chars().count();

    if chars > SELECT_OPTION_LABEL_LENGTH {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::SelectOptionLabelLength { chars },
        });
    }

    Ok(())
}

/// Validate a [`SelectMenuOption::value`]'s length.
///
/// # Errors
///
/// Returns an error of type [`SelectOptionValueLength`] if the provided select
/// option value is too long.
///
/// [`SelectMenuOption::value`]: twilight_model::application::component::select_menu::SelectMenuOption::value
/// [`SelectOptionValueLength`]: ComponentValidationErrorType::SelectOptionValueLength
fn component_select_option_value(value: impl AsRef<str>) -> Result<(), ComponentValidationError> {
    let chars = value.as_ref().chars().count();

    if chars > SELECT_OPTION_VALUE_LENGTH {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::SelectOptionValueLength { chars },
        });
    }

    Ok(())
}

/// Validate a [`SelectMenu`]s number of [`options`].
///
/// [`Component::SelectMenu`]s may only have so many options within it, defined
/// by [`SELECT_OPTION_COUNT`].
///
/// # Errors
///
/// Returns an error of type [`SelectOptionCount`] if the provided list of
/// [`SelectMenuOption`]s is too many for a [`SelectMenu`].
///
/// [`SelectMenu::options`]: twilight_model::application::component::select_menu::SelectMenu::options
/// [`SelectMenuOption`]: twilight_model::application::component::select_menu::SelectMenuOption
/// [`SelectMenu`]: twilight_model::application::component::select_menu::SelectMenu
/// [`SelectOptionCount`]: ComponentValidationErrorType::SelectOptionCount
const fn component_select_options(
    options: &[SelectMenuOption],
) -> Result<(), ComponentValidationError> {
    let count = options.len();

    if count > SELECT_OPTION_COUNT {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::SelectOptionCount { count },
        });
    }

    Ok(())
}

/// Validate a [`SelectMenu::placeholder`]'s length.
///
/// # Errors
///
/// Returns an error of type [`SelectPlaceholderLength`] if the provided select
/// placeholder is too long.
///
/// [`SelectMenu::placeholder`]: twilight_model::application::component::select_menu::SelectMenu::placeholder
/// [`SelectPlaceholderLength`]: ComponentValidationErrorType::SelectPlaceHolderLength
fn component_select_placeholder(
    placeholder: impl AsRef<str>,
) -> Result<(), ComponentValidationError> {
    let chars = placeholder.as_ref().chars().count();

    if chars > SELECT_PLACEHOLDER_LENGTH {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::SelectPlaceholderLength { chars },
        });
    }

    Ok(())
}

/// Ensure a [`TextInput::label`]'s length is correct.
///
/// The length must be at most [`TEXT_INPUT_LABEL_MAX`].
///
/// # Errors
///
/// Returns an error of type [`TextInputLabelLength`] if the provided
/// label is too long.
///
/// [`TextInput::label`]: twilight_model::application::component::text_input::TextInput::label
/// [`TextInputLabelLength`]: ComponentValidationErrorType::TextInputLabelLength
fn component_text_input_label(label: impl AsRef<str>) -> Result<(), ComponentValidationError> {
    let len = label.as_ref().len();

    if (TEXT_INPUT_LABEL_MIN..=TEXT_INPUT_LABEL_MAX).contains(&len) {
        Ok(())
    } else {
        Err(ComponentValidationError {
            kind: ComponentValidationErrorType::TextInputLabelLength { len },
        })
    }
}

/// Ensure a [`TextInput::max_length`]'s value is correct.
///
/// # Errors
///
/// Returns an error of type [`TextInputMaxLength`] if the length is invalid.
///
/// [`TextInput::max_length`]: twilight_model::application::component::text_input::TextInput::max_length
/// [`TextInputMaxLength`]: ComponentValidationErrorType::TextInputMaxLength
const fn component_text_input_max(len: u16) -> Result<(), ComponentValidationError> {
    let len = len as usize;

    if len >= TEXT_INPUT_LENGTH_MIN && len <= TEXT_INPUT_LENGTH_MAX {
        Ok(())
    } else {
        Err(ComponentValidationError {
            kind: ComponentValidationErrorType::TextInputMaxLength { len },
        })
    }
}

/// Ensure a [`TextInput::min_length`]'s value is correct.
///
/// # Errors
///
/// Returns an error of type [`TextInputMinLength`] if the length is invalid.
///
/// [`TextInput::min_length`]: twilight_model::application::component::text_input::TextInput::min_length
/// [`TextInputMinLength`]: ComponentValidationErrorType::TextInputMinLength
const fn component_text_input_min(len: u16) -> Result<(), ComponentValidationError> {
    let len = len as usize;

    if len <= TEXT_INPUT_LENGTH_MAX {
        Ok(())
    } else {
        Err(ComponentValidationError {
            kind: ComponentValidationErrorType::TextInputMinLength { len },
        })
    }
}

/// Ensure a [`TextInput::placeholder`]'s length is correct.
///
/// The length must be at most [`TEXT_INPUT_PLACEHOLDER_MAX`].
///
/// # Errors
///
/// Returns an error of type [`TextInputPlaceholderLength`] if the provided
/// placeholder is too long.
///
/// [`TextInput::placeholder`]: twilight_model::application::component::text_input::TextInput::placeholder
/// [`TextInputPlaceholderLength`]: ComponentValidationErrorType::TextInputPlaceholderLength
fn component_text_input_placeholder(
    placeholder: impl AsRef<str>,
) -> Result<(), ComponentValidationError> {
    let chars = placeholder.as_ref().chars().count();

    if chars <= TEXT_INPUT_PLACEHOLDER_MAX {
        Ok(())
    } else {
        Err(ComponentValidationError {
            kind: ComponentValidationErrorType::TextInputPlaceholderLength { chars },
        })
    }
}

/// Ensure a [`TextInput::value`]'s length is correct.
///
/// # Errors
///
/// Returns an error of type [`TextInputValueLength`] if the length is invalid.
///
/// [`TextInput::value_length`]: twilight_model::application::component::text_input::TextInput::value
/// [`TextInputValueLength`]: ComponentValidationErrorType::TextInputValueLength
fn component_text_input_value(value: impl AsRef<str>) -> Result<(), ComponentValidationError> {
    let chars = value.as_ref().chars().count();

    if chars <= TEXT_INPUT_LENGTH_MAX {
        Ok(())
    } else {
        Err(ComponentValidationError {
            kind: ComponentValidationErrorType::TextInputValueLength { chars },
        })
    }
}

#[allow(clippy::non_ascii_literal)]
#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::{assert_fields, assert_impl_all};
    use twilight_model::channel::message::EmojiReactionType;

    assert_fields!(ComponentValidationErrorType::ActionRowComponentCount: count);
    assert_fields!(ComponentValidationErrorType::ComponentCount: count);
    assert_fields!(ComponentValidationErrorType::ComponentCustomIdLength: chars);
    assert_fields!(ComponentValidationErrorType::ComponentLabelLength: chars);
    assert_fields!(ComponentValidationErrorType::InvalidChildComponent: kind);
    assert_fields!(ComponentValidationErrorType::InvalidRootComponent: kind);
    assert_fields!(ComponentValidationErrorType::SelectMaximumValuesCount: count);
    assert_fields!(ComponentValidationErrorType::SelectMinimumValuesCount: count);
    assert_fields!(ComponentValidationErrorType::SelectOptionDescriptionLength: chars);
    assert_fields!(ComponentValidationErrorType::SelectOptionLabelLength: chars);
    assert_fields!(ComponentValidationErrorType::SelectOptionValueLength: chars);
    assert_fields!(ComponentValidationErrorType::SelectPlaceholderLength: chars);
    assert_impl_all!(ComponentValidationErrorType: Debug, Send, Sync);
    assert_impl_all!(ComponentValidationError: Debug, Send, Sync);

    // All styles of buttons.
    const ALL_BUTTON_STYLES: &[ButtonStyle] = &[
        ButtonStyle::Primary,
        ButtonStyle::Secondary,
        ButtonStyle::Success,
        ButtonStyle::Danger,
        ButtonStyle::Link,
        ButtonStyle::Premium,
    ];

    #[test]
    fn component_action_row() {
        let button = Button {
            custom_id: None,
            disabled: false,
            emoji: Some(EmojiReactionType::Unicode {
                name: "📚".into()
            }),
            label: Some("Read".into()),
            style: ButtonStyle::Link,
            url: Some("https://abebooks.com".into()),
            sku_id: None,
        };

        let select_menu = SelectMenu {
            channel_types: None,
            custom_id: "custom id 2".into(),
            disabled: false,
            default_values: None,
            kind: SelectMenuType::Text,
            max_values: Some(2),
            min_values: Some(1),
            options: Some(Vec::from([SelectMenuOption {
                default: true,
                description: Some("Book 1 of the Expanse".into()),
                emoji: None,
                label: "Leviathan Wakes".into(),
                value: "9780316129084".into(),
            }])),
            placeholder: Some("Choose a book".into()),
        };

        let action_row = ActionRow {
            components: Vec::from([
                Component::SelectMenu(select_menu.clone()),
                Component::Button(button),
            ]),
        };

        assert!(component(&Component::ActionRow(action_row.clone())).is_ok());

        assert!(component(&Component::SelectMenu(select_menu.clone())).is_err());

        assert!(super::action_row(&action_row).is_ok());

        let invalid_action_row = Component::ActionRow(ActionRow {
            components: Vec::from([
                Component::SelectMenu(select_menu.clone()),
                Component::SelectMenu(select_menu.clone()),
                Component::SelectMenu(select_menu.clone()),
                Component::SelectMenu(select_menu.clone()),
                Component::SelectMenu(select_menu.clone()),
                Component::SelectMenu(select_menu),
            ]),
        });

        assert!(component(&invalid_action_row).is_err());
    }

    // Test that a button with both a custom ID and URL results in a
    // [`ComponentValidationErrorType::ButtonConflict`] error type.
    #[test]
    fn button_conflict() {
        let button = Button {
            custom_id: Some("a".to_owned()),
            disabled: false,
            emoji: None,
            label: None,
            style: ButtonStyle::Primary,
            url: Some("https://twilight.rs".to_owned()),
            sku_id: None,
        };

        assert!(matches!(
            super::button(&button),
            Err(ComponentValidationError {
                kind: ComponentValidationErrorType::ButtonConflict,
            }),
        ));
    }

    // Test that all button styles with no custom ID or URL results in a
    // [`ComponentValidationErrorType::ButtonStyle`] error type.
    #[test]
    fn button_style() {
        for style in ALL_BUTTON_STYLES {
            let button = Button {
                custom_id: None,
                disabled: false,
                emoji: None,
                label: Some("some label".to_owned()),
                style: *style,
                url: None,
                sku_id: None,
            };

            assert!(matches!(
                super::button(&button),
                Err(ComponentValidationError {
                    kind: ComponentValidationErrorType::ButtonStyle {
                        style: error_style,
                    }
                })
                if error_style == *style
            ));
        }
    }

    #[test]
    fn component_label() {
        assert!(component_button_label("").is_ok());
        assert!(component_button_label("a").is_ok());
        assert!(component_button_label("a".repeat(80)).is_ok());

        assert!(component_button_label("a".repeat(81)).is_err());
    }

    #[test]
    fn component_custom_id_length() {
        assert!(component_custom_id("").is_ok());
        assert!(component_custom_id("a").is_ok());
        assert!(component_custom_id("a".repeat(100)).is_ok());

        assert!(component_custom_id("a".repeat(101)).is_err());
    }

    #[test]
    fn component_option_description_length() {
        assert!(component_option_description("").is_ok());
        assert!(component_option_description("a").is_ok());
        assert!(component_option_description("a".repeat(100)).is_ok());

        assert!(component_option_description("a".repeat(101)).is_err());
    }

    #[test]
    fn component_select_default_values_support() {
        assert!(component_select_default_values_supported(SelectMenuType::User).is_ok());
        assert!(component_select_default_values_supported(SelectMenuType::Role).is_ok());
        assert!(component_select_default_values_supported(SelectMenuType::Mentionable).is_ok());
        assert!(component_select_default_values_supported(SelectMenuType::Channel).is_ok());

        assert!(component_select_default_values_supported(SelectMenuType::Text).is_err());
    }

    #[test]
    fn component_select_num_default_values() {
        assert!(component_select_default_values_count(None, None, 0).is_ok());
        assert!(component_select_default_values_count(None, None, 1).is_ok());
        assert!(component_select_default_values_count(Some(1), None, 5).is_ok());
        assert!(component_select_default_values_count(Some(5), None, 5).is_ok());
        assert!(component_select_default_values_count(None, Some(5), 5).is_ok());
        assert!(component_select_default_values_count(None, Some(10), 5).is_ok());
        assert!(component_select_default_values_count(Some(5), Some(5), 5).is_ok());
        assert!(component_select_default_values_count(Some(1), Some(10), 5).is_ok());

        assert!(component_select_default_values_count(Some(2), None, 1).is_err());
        assert!(component_select_default_values_count(None, Some(1), 2).is_err());
        assert!(component_select_default_values_count(Some(1), Some(1), 2).is_err());
        assert!(component_select_default_values_count(Some(2), Some(2), 1).is_err());
    }

    #[test]
    fn component_select_max_values_count() {
        assert!(component_select_max_values(1).is_ok());
        assert!(component_select_max_values(25).is_ok());

        assert!(component_select_max_values(0).is_err());
        assert!(component_select_max_values(26).is_err());
    }

    #[test]
    fn component_select_min_values_count() {
        assert!(component_select_min_values(1).is_ok());
        assert!(component_select_min_values(25).is_ok());

        assert!(component_select_min_values(26).is_err());
    }

    #[test]
    fn component_select_option_value_length() {
        assert!(component_select_option_value("a").is_ok());
        assert!(component_select_option_value("a".repeat(100)).is_ok());

        assert!(component_select_option_value("a".repeat(101)).is_err());
    }

    #[test]
    fn component_select_options_count() {
        let select_menu_options = Vec::from([SelectMenuOption {
            default: false,
            description: None,
            emoji: None,
            label: "label".into(),
            value: "value".into(),
        }]);

        assert!(component_select_options(&select_menu_options).is_ok());

        let select_menu_options_25 = select_menu_options
            .iter()
            .cloned()
            .cycle()
            .take(25)
            .collect::<Vec<SelectMenuOption>>();

        assert!(component_select_options(&select_menu_options_25).is_ok());

        let select_menu_options_26 = select_menu_options
            .iter()
            .cloned()
            .cycle()
            .take(26)
            .collect::<Vec<SelectMenuOption>>();

        assert!(component_select_options(&select_menu_options_26).is_err());
    }

    #[test]
    fn component_select_placeholder_length() {
        assert!(component_select_placeholder("").is_ok());
        assert!(component_select_placeholder("a").is_ok());
        assert!(component_select_placeholder("a".repeat(150)).is_ok());

        assert!(component_select_placeholder("a".repeat(151)).is_err());
    }

    #[test]
    fn component_text_input_label_length() {
        assert!(component_text_input_label("a").is_ok());
        assert!(component_text_input_label("a".repeat(45)).is_ok());

        assert!(component_text_input_label("").is_err());
        assert!(component_text_input_label("a".repeat(46)).is_err());
    }

    #[test]
    fn component_text_input_max_count() {
        assert!(component_text_input_max(1).is_ok());
        assert!(component_text_input_max(4000).is_ok());

        assert!(component_text_input_max(0).is_err());
        assert!(component_text_input_max(4001).is_err());
    }

    #[test]
    fn component_text_input_min_count() {
        assert!(component_text_input_min(0).is_ok());
        assert!(component_text_input_min(1).is_ok());
        assert!(component_text_input_min(4000).is_ok());

        assert!(component_text_input_min(4001).is_err());
    }

    #[test]
    fn component_text_input_placeholder_length() {
        assert!(component_text_input_placeholder("").is_ok());
        assert!(component_text_input_placeholder("a").is_ok());
        assert!(component_text_input_placeholder("a".repeat(100)).is_ok());

        assert!(component_text_input_placeholder("a".repeat(101)).is_err());
    }

    #[test]
    fn component_text_input_value() {
        assert!(component_text_input_min(0).is_ok());
        assert!(component_text_input_min(1).is_ok());
        assert!(component_text_input_min(4000).is_ok());

        assert!(component_text_input_min(4001).is_err());
    }
}
