use super::{ComponentValidationError, ComponentValidationErrorType};
use twilight_model::channel::message::component::TextDisplay;
use twilight_model::channel::message::Component;

/// Maximum number of root [`Component`]s in a message in Components V2.
///
/// This is defined in Discord's documentation, per
/// [Discord Docs][1].
///
/// [1]: https://discord.com/developers/docs/components/reference#component-reference
pub const COMPONENT_COUNT_TOTAL_V2: usize = 40;

/// Maximum length of text display content.
pub const TEXT_DISPLAY_CONTENT_LENGTH_MAX: usize = 2000;

// TODO: rewrite comment
/// Ensure that a top-level request component is correct in V2.
///
/// Intended to ensure that a fully formed top-level component for requests
/// is an action row.
///
/// Refer to other validators like [`button`] if you need to validate other
/// components.
pub fn component_v2(component: &Component) -> Result<(), ComponentValidationError> {
    match component {
        Component::ActionRow(action_row) => super::action_row(action_row, true)?,
        Component::Button(button) => super::button(button)?,
        Component::SelectMenu(select_menu) => super::select_menu(select_menu)?,
        Component::TextInput(text_input) => super::text_input(text_input)?,
        Component::TextDisplay(text_display) => self::text_display(text_display)?,
        _ => todo!(),
    }

    Ok(())
}

pub fn text_display(text_display: &TextDisplay) -> Result<(), ComponentValidationError> {
    let content_len = text_display.content.len();
    if content_len > TEXT_DISPLAY_CONTENT_LENGTH_MAX {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::TextDisplayContentTooLong { len: content_len },
        });
    }

    Ok(())
}
