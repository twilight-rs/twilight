use twilight_model::channel::message::Component;

use super::ComponentValidationError;

// TODO: rewrite comment
/// Ensure that a top-level request component is correct in V2.
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
pub fn component_v2(component: &Component) -> Result<(), ComponentValidationError> {
    todo!()
}
