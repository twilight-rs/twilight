//! Validates components V2.

use super::{
    ComponentValidationError, ComponentValidationErrorType, action_row, button,
    component_custom_id, select_menu, text_input,
};
use twilight_model::channel::message::Component;
use twilight_model::channel::message::component::{
    ComponentType, Container, FileUpload, Label, MediaGallery, MediaGalleryItem, Section,
    TextDisplay, Thumbnail,
};

/// Maximum length of text display content.
pub const TEXT_DISPLAY_CONTENT_LENGTH_MAX: usize = 2000;

/// Minimum amount of items in a media gallery.
pub const MEDIA_GALLERY_ITEMS_MIN: usize = 1;

/// Maximum amount of items in a media gallery.
pub const MEDIA_GALLERY_ITEMS_MAX: usize = 10;

/// Maximum length of a description of a media gallery item.
pub const MEDIA_GALLERY_ITEM_DESCRIPTION_LENGTH_MAX: usize = 1024;

/// Minimum amount of components in a section.
pub const SECTION_COMPONENTS_MIN: usize = 1;

/// Maximum amount of components in a section.
pub const SECTION_COMPONENTS_MAX: usize = 3;

/// Maximum length of a thumbnail description.
pub const THUMBNAIL_DESCRIPTION_LENGTH_MAX: usize = 1024;

/// Maximum length of the label text of a label component.
pub const LABEL_LABEL_LENGTH_MAX: usize = 45;

/// Maximum length of a label description.
pub const LABEL_DESCRIPTION_LENGTH_MAX: usize = 100;

/// Maximum value of [`FileUpload::max_values`].
///
/// [`FileUpload::max_values`]: FileUpload::max_values
pub const FILE_UPLOAD_MAXIMUM_VALUES_LIMIT: u8 = 10;

/// Maximum value of [`FileUpload::min_values`].
///
/// [`FileUpload::min_values`]: FileUpload::min_values
pub const FILE_UPLOAD_MINIMUM_VALUES_LIMIT: u8 = 10;

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
/// Returns an error of type [`InvalidRootComponent`] if the component cannot be a root
/// component in both modals and messages.
///
/// For other errors refer to the errors of the following functions:
/// - [`action_row`]
/// - [`label`]
/// - [`button`]
/// - [`container`]
/// - [`media_gallery`]
/// - [`section`]
/// - [`select_menu`]
/// - [`text_display`]
/// - [`text_input`]
/// - [`thumbnail`]
///
/// [`InvalidRootComponent`]: ComponentValidationErrorType::InvalidRootComponent
pub fn component_v2(component: &Component) -> Result<(), ComponentValidationError> {
    match component {
        Component::ActionRow(ar) => action_row(ar, true)?,
        Component::Label(l) => label(l)?,
        Component::Button(b) => button(b)?,
        Component::Container(c) => container(c)?,
        Component::MediaGallery(mg) => media_gallery(mg)?,
        Component::Section(s) => section(s)?,
        Component::SelectMenu(sm) => select_menu(sm, true)?,
        Component::TextDisplay(td) => text_display(td)?,
        Component::TextInput(_) | Component::FileUpload(_) => {
            return Err(ComponentValidationError {
                kind: ComponentValidationErrorType::InvalidRootComponent {
                    kind: ComponentType::TextInput,
                },
            });
        }
        Component::Thumbnail(t) => thumbnail(t)?,
        Component::Separator(_) | Component::File(_) | Component::Unknown(_) => (),
    }

    Ok(())
}

/// Ensure that a label is correct.
///
/// # Errors
///
/// Returns an error of type [`InvalidChildComponent`] if the provided nested
/// component is an [`ActionRow`] or a [`Label`]. Labels cannot contain other top-level
/// components.
///
/// Returns an error of type [`DisallowedChildren`] if the label contains components
/// that are disallowed in labels.
///
/// Refer to [`select_menu`] for potential errors when validating a select menu in the label.
///
/// Refer to [`text_input`] for potential errors when validating a text input in the label.
///
/// Refer to [`text_display`] for potential errors when validating a text display in the label.
///
/// [`InvalidChildComponent`]: ComponentValidationErrorType::InvalidChildComponent
/// [`DisallowedChildren`]: ComponentValidationErrorType::DisallowedChildren
/// [`ActionRow`]: twilight_model::channel::message::component::ActionRow
pub fn label(label: &Label) -> Result<(), ComponentValidationError> {
    self::label_label(&label.label)?;

    if let Some(description) = &label.description {
        self::label_description(description)?;
    }

    match &*label.component {
        Component::ActionRow(_) | Component::Label(_) => Err(ComponentValidationError {
            kind: ComponentValidationErrorType::InvalidChildComponent {
                kind: label.component.kind(),
            },
        }),
        Component::SelectMenu(select_menu) => self::select_menu(select_menu, false),
        Component::TextInput(text_input) => self::text_input(text_input, false),
        Component::FileUpload(file_upload) => self::file_upload(file_upload),
        Component::Unknown(unknown) => Err(ComponentValidationError {
            kind: ComponentValidationErrorType::InvalidChildComponent {
                kind: ComponentType::Unknown(*unknown),
            },
        }),

        Component::Button(_)
        | Component::TextDisplay(_)
        | Component::MediaGallery(_)
        | Component::Separator(_)
        | Component::File(_)
        | Component::Section(_)
        | Component::Container(_)
        | Component::Thumbnail(_) => Err(ComponentValidationError {
            kind: ComponentValidationErrorType::DisallowedChildren,
        }),
    }
}

/// Validates a text display component.
///
/// # Errors
///
/// This will error with [`TextDisplayContentTooLong`] if the content is longer
/// than [`TEXT_DISPLAY_CONTENT_LENGTH_MAX`].
///
/// [`TextDisplayContentTooLong`]: ComponentValidationErrorType::TextDisplayContentTooLong
pub const fn text_display(text_display: &TextDisplay) -> Result<(), ComponentValidationError> {
    let content_len = text_display.content.len();
    if content_len > TEXT_DISPLAY_CONTENT_LENGTH_MAX {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::TextDisplayContentTooLong { len: content_len },
        });
    }

    Ok(())
}

/// Validates a media gallery component.
///
/// # Errors
///
/// This will error with [`MediaGalleryItemCountOutOfRange`] if the amount of
/// media items is less than [`MEDIA_GALLERY_ITEMS_MIN`] or greater than
/// [`MEDIA_GALLERY_ITEMS_MAX`].
///
/// For errors for validation of induvidual items see the documentation for [`media_gallery_item`].
///
/// [`MediaGalleryItemCountOutOfRange`]: ComponentValidationErrorType::MediaGalleryItemCountOutOfRange
pub fn media_gallery(media_gallery: &MediaGallery) -> Result<(), ComponentValidationError> {
    let items = media_gallery.items.len();
    if !(MEDIA_GALLERY_ITEMS_MIN..=MEDIA_GALLERY_ITEMS_MAX).contains(&items) {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::MediaGalleryItemCountOutOfRange { count: items },
        });
    }

    for item in &media_gallery.items {
        media_gallery_item(item)?;
    }

    Ok(())
}

/// Validates a section component.
///
/// # Errors
///
/// This will error with [`SectionComponentCountOutOfRange`] if the amount of
/// section components is less than [`SECTION_COMPONENTS_MIN`] or greater than
/// [`SECTION_COMPONENTS_MAX`].
///
/// For validation of specific components see:
/// - [`button`]
/// - [`text_display`]
/// - [`thumbnail`]
///
/// [`SectionComponentCountOutOfRange`]: ComponentValidationErrorType::SectionComponentCountOutOfRange
pub fn section(section: &Section) -> Result<(), ComponentValidationError> {
    let components = section.components.len();
    if !(SECTION_COMPONENTS_MIN..=SECTION_COMPONENTS_MAX).contains(&components) {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::SectionComponentCountOutOfRange {
                count: components,
            },
        });
    }

    for component in &section.components {
        match component {
            Component::TextDisplay(td) => text_display(td)?,
            _ => {
                return Err(ComponentValidationError {
                    kind: ComponentValidationErrorType::DisallowedChildren,
                });
            }
        }
    }

    match section.accessory.as_ref() {
        Component::Button(b) => button(b)?,
        Component::Thumbnail(t) => thumbnail(t)?,
        _ => {
            return Err(ComponentValidationError {
                kind: ComponentValidationErrorType::DisallowedChildren,
            });
        }
    }

    Ok(())
}

/// Validates a container component.
///
/// The only allowed components that are allowed are: `action_row`, `file`,
/// `media_gallery`, `section`, `separator` and `text_display`.
///
/// # Errors
///
/// For errors for specific components refer to the errors of the following functions:
/// - [`action_row`]
/// - [`media_gallery`]
/// - [`text_display`]
/// - [`section`]
///
/// If any except the allowed components are used if will fail with [`DisallowedChildren`].
///
/// [`DisallowedChildren`]: ComponentValidationErrorType::DisallowedChildren
pub fn container(container: &Container) -> Result<(), ComponentValidationError> {
    for component in &container.components {
        match component {
            Component::ActionRow(ar) => action_row(ar, true)?,
            Component::TextDisplay(td) => text_display(td)?,
            Component::Section(s) => section(s)?,
            Component::MediaGallery(mg) => media_gallery(mg)?,
            Component::Separator(_) | Component::File(_) => (),
            _ => {
                return Err(ComponentValidationError {
                    kind: ComponentValidationErrorType::DisallowedChildren,
                });
            }
        }
    }

    Ok(())
}

/// Validates a thumbnail component.
///
/// # Errors
///
/// This will error with [`ThumbnailDescriptionTooLong`] if the description is longer
/// than [`THUMBNAIL_DESCRIPTION_LENGTH_MAX`].
///
/// [`ThumbnailDescriptionTooLong`]: ComponentValidationErrorType::ThumbnailDescriptionTooLong
pub const fn thumbnail(thumbnail: &Thumbnail) -> Result<(), ComponentValidationError> {
    let Some(Some(desc)) = thumbnail.description.as_ref() else {
        return Ok(());
    };

    let len = desc.len();
    if len > THUMBNAIL_DESCRIPTION_LENGTH_MAX {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::ThumbnailDescriptionTooLong { len },
        });
    }

    Ok(())
}

/// Validates a file upload component.
///
/// # Errors
///
/// Returns an error of type [`ComponentCustomIdLength`] if the provided custom
/// ID is too long.
///
/// Returns an error of type [`FileUploadMaximumValuesCount`] if the provided number
/// of files that can be uploaded is larger than the maximum.
///
/// Returns an error of type [`FileUploadMinimumValuesCount`] if the provided number
/// of files that must be uploaded is larger than the maximum.
///
/// [`ComponentCustomIdLength`]: ComponentValidationErrorType::ComponentCustomIdLength
/// [`FileUploadMaximumValuesCount`]: ComponentValidationErrorType::FileUploadMaximumValuesCount
/// [`FileUploadMaximumValuesCount`]: ComponentValidationErrorType::FileUploadMaximumValuesCount
pub fn file_upload(file_upload: &FileUpload) -> Result<(), ComponentValidationError> {
    component_custom_id(&file_upload.custom_id)?;

    if let Some(min_values) = file_upload.min_values {
        component_file_upload_min_values(min_values)?;
    }

    if let Some(max_value) = file_upload.max_values {
        component_file_upload_max_values(max_value)?;
    }

    Ok(())
}

/// Validates a media gallery item
///
/// # Errors
///
/// This will error with [`MediaGalleryItemDescriptionTooLong`] if the description is longer
/// than [`MEDIA_GALLERY_ITEM_DESCRIPTION_LENGTH_MAX`].
///
/// [`MediaGalleryItemDescriptionTooLong`]: ComponentValidationErrorType::MediaGalleryItemDescriptionTooLong
pub const fn media_gallery_item(item: &MediaGalleryItem) -> Result<(), ComponentValidationError> {
    let Some(desc) = item.description.as_ref() else {
        return Ok(());
    };

    let len = desc.len();
    if len > MEDIA_GALLERY_ITEM_DESCRIPTION_LENGTH_MAX {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::MediaGalleryItemDescriptionTooLong { len },
        });
    }

    Ok(())
}

/// Ensure a [`Label::label`]'s length is correct.
///
/// # Errors
///
/// Returns an error of type [`LabelLabelTooLong`] if the length is invalid.
///
/// [`Label::label`]: twilight_model::channel::message::component::Label::label
/// [`LabelLabelTooLong`]: ComponentValidationErrorType::LabelLabelTooLong
fn label_label(value: impl AsRef<str>) -> Result<(), ComponentValidationError> {
    let chars = value.as_ref().chars().count();

    if chars <= LABEL_LABEL_LENGTH_MAX {
        Ok(())
    } else {
        Err(ComponentValidationError {
            kind: ComponentValidationErrorType::LabelLabelTooLong { len: chars },
        })
    }
}

/// Ensure a [`Label::description`]'s length is correct.
///
/// # Errors
///
/// Returns an error of type [`LabelDescriptionTooLong`] if the length is invalid.
///
/// [`Label::label`]: twilight_model::channel::message::component::Label::description
/// [`LabelDescriptionTooLong`]: ComponentValidationErrorType::LabelDescriptionTooLong
fn label_description(value: impl AsRef<str>) -> Result<(), ComponentValidationError> {
    let chars = value.as_ref().chars().count();

    if chars <= LABEL_DESCRIPTION_LENGTH_MAX {
        Ok(())
    } else {
        Err(ComponentValidationError {
            kind: ComponentValidationErrorType::LabelDescriptionTooLong { len: chars },
        })
    }
}

/// Validate a [`FileUpload::max_values`] amount.
///
/// # Errors
///
/// Returns an error of type [`FileUploadMaximumValuesCount`] if the provided number
/// of files that can be uploaded is larger than
/// [the maximum][`FILE_UPLOAD_MAXIMUM_VALUES_LIMIT`].
///
/// [`FileUpload::max_values`]: twilight_model::channel::message::component::FileUpload::max_values
/// [`FileUploadMaximumValuesCount`]: ComponentValidationErrorType::FileUploadMaximumValuesCount
const fn component_file_upload_max_values(count: u8) -> Result<(), ComponentValidationError> {
    if count > FILE_UPLOAD_MAXIMUM_VALUES_LIMIT {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::FileUploadMaximumValuesCount { count },
        });
    }

    Ok(())
}

/// Validate a [`FileUpload::min_values`] amount.
///
/// # Errors
///
/// Returns an error of type [`FileUploadMinimumValuesCount`] if the provided number
/// of files that must be uploaded is larger than
/// [the maximum][`FILE_UPLOAD_MINIMUM_VALUES_LIMIT`].
///
/// [`FileUpload::min_values`]: twilight_model::channel::message::component::FileUpload::min_values
/// [`FileUploadMinimumValuesCount`]: ComponentValidationErrorType::FileUploadMinimumValuesCount
const fn component_file_upload_min_values(count: u8) -> Result<(), ComponentValidationError> {
    if count > FILE_UPLOAD_MINIMUM_VALUES_LIMIT {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::FileUploadMinimumValuesCount { count },
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;
    use twilight_model::channel::message::Component;
    use twilight_model::channel::message::component::{
        Button, ButtonStyle, Label, SelectMenu, SelectMenuType, TextInput, TextInputStyle,
    };

    fn wrap_in_label(component: Component) -> Component {
        Component::Label(Label {
            id: None,
            label: "label".to_owned(),
            description: None,
            component: Box::new(component),
        })
    }

    #[test]
    fn component_label() {
        let button = Component::Button(Button {
            custom_id: None,
            disabled: false,
            emoji: None,
            label: Some("Press me".to_owned()),
            style: ButtonStyle::Danger,
            url: None,
            sku_id: None,
            id: None,
        });

        let text_display = Component::TextDisplay(TextDisplay {
            id: None,
            content: "Text display".to_owned(),
        });

        let valid_select_menu = SelectMenu {
            channel_types: None,
            custom_id: "my_select".to_owned(),
            default_values: None,
            disabled: false,
            kind: SelectMenuType::User,
            max_values: None,
            min_values: None,
            options: None,
            placeholder: None,
            id: None,
            required: None,
        };

        let disabled_select_menu = SelectMenu {
            disabled: true,
            ..valid_select_menu.clone()
        };

        let valid_label = Label {
            id: None,
            label: "Label".to_owned(),
            description: Some("This is a description".to_owned()),
            component: Box::new(Component::SelectMenu(valid_select_menu)),
        };

        let label_invalid_child_button = Label {
            component: Box::new(button),
            ..valid_label.clone()
        };

        let label_invalid_child_text_display = Label {
            id: None,
            label: "Another label".to_owned(),
            description: None,
            component: Box::new(text_display),
        };

        let label_invalid_child_disabled_select = Label {
            component: Box::new(Component::SelectMenu(disabled_select_menu)),
            ..valid_label.clone()
        };

        let label_too_long_description = Label {
            description: Some(iter::repeat_n('a', 101).collect()),
            ..valid_label.clone()
        };

        let label_too_long_label = Label {
            label: iter::repeat_n('a', 46).collect(),
            ..valid_label.clone()
        };

        assert!(label(&valid_label).is_ok());
        assert!(component_v2(&Component::Label(valid_label)).is_ok());
        assert!(label(&label_invalid_child_button).is_err());
        assert!(component_v2(&Component::Label(label_invalid_child_button)).is_err());
        assert!(label(&label_invalid_child_text_display).is_err());
        assert!(component_v2(&Component::Label(label_invalid_child_text_display)).is_err());
        assert!(label(&label_invalid_child_disabled_select).is_err());
        assert!(component_v2(&Component::Label(label_invalid_child_disabled_select)).is_err());
        assert!(label(&label_too_long_description).is_err());
        assert!(component_v2(&Component::Label(label_too_long_description)).is_err());
        assert!(label(&label_too_long_label).is_err());
        assert!(component_v2(&Component::Label(label_too_long_label)).is_err());
    }

    #[test]
    fn no_text_input_label_in_label_component() {
        #[allow(deprecated)]
        let text_input_with_label = Component::TextInput(TextInput {
            id: None,
            custom_id: "The custom id".to_owned(),
            label: Some("The text input label".to_owned()),
            max_length: None,
            min_length: None,
            placeholder: None,
            required: None,
            style: TextInputStyle::Short,
            value: None,
        });

        let invalid_label_component = Label {
            id: None,
            label: "Label".to_owned(),
            description: None,
            component: Box::new(text_input_with_label),
        };

        assert!(label(&invalid_label_component).is_err());
        assert!(component_v2(&Component::Label(invalid_label_component)).is_err());
    }

    #[test]
    fn component_file_upload() {
        let valid = FileUpload {
            id: Some(42),
            custom_id: "custom_id".to_owned(),
            max_values: Some(10),
            min_values: Some(10),
            required: None,
        };

        assert!(file_upload(&valid).is_ok());
        assert!(component_v2(&wrap_in_label(Component::FileUpload(valid.clone()))).is_ok());

        let invalid_custom_id = FileUpload {
            custom_id: iter::repeat_n('a', 101).collect(),
            ..valid.clone()
        };

        assert!(file_upload(&invalid_custom_id).is_err());
        assert!(component_v2(&wrap_in_label(Component::FileUpload(invalid_custom_id))).is_err());

        let invalid_min_values = FileUpload {
            min_values: Some(11),
            ..valid.clone()
        };

        assert!(file_upload(&invalid_min_values).is_err());
        assert!(component_v2(&wrap_in_label(Component::FileUpload(invalid_min_values))).is_err());

        let invalid_max_values_too_high = FileUpload {
            max_values: Some(11),
            ..valid
        };

        assert!(file_upload(&invalid_max_values_too_high).is_err());
        assert!(
            component_v2(&wrap_in_label(Component::FileUpload(
                invalid_max_values_too_high
            )))
            .is_err()
        );
    }
}
