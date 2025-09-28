//! Validates components V2.

use super::{
    action_row, button, select_menu, text_input, ComponentValidationError,
    ComponentValidationErrorType,
};
use twilight_model::channel::message::component::{
    Container, MediaGallery, MediaGalleryItem, Section, TextDisplay, Thumbnail,
};
use twilight_model::channel::message::Component;

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

/// Maximum length of a thumbnail description
pub const THUMBNAIL_DESCRIPTION_LENGTH_MAX: usize = 1024;

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
/// For errors refer to the errors of the following functions:
/// - [`action_row`]
/// - [`button`]
/// - [`container`]
/// - [`media_gallery`]
/// - [`section`]
/// - [`select_menu`]
/// - [`text_display`]
/// - [`text_input`]
/// - [`thumbnail`]
pub fn component_v2(component: &Component) -> Result<(), ComponentValidationError> {
    match component {
        Component::ActionRow(ar) => action_row(ar, true)?,
        Component::Button(b) => button(b)?,
        Component::Container(c) => container(c)?,
        Component::MediaGallery(mg) => media_gallery(mg)?,
        Component::Section(s) => section(s)?,
        Component::SelectMenu(sm) => select_menu(sm)?,
        Component::TextDisplay(td) => text_display(td)?,
        Component::TextInput(ti) => text_input(ti)?,
        Component::Thumbnail(t) => thumbnail(t)?,
        Component::Separator(_) | Component::File(_) | Component::Unknown(_) => (),
    }

    Ok(())
}

/// Validates a text display component.
///
/// # Errors
///
/// This will error with [`TextDisplayContentTooLong`] if the content is longer
/// than [`TEXT_DISPLAY_CONTENT_LENGTH_MAX`].
///
/// [`TextDisplayContentTooLong`]: ComponentValidationErrorType::TextDisplayContentTooLong
pub fn text_display(text_display: &TextDisplay) -> Result<(), ComponentValidationError> {
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
/// For errors for validation of induvidual items see the docuumentation for [`media_gallery_item`].
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
                })
            }
        }
    }

    match section.accessory.as_ref() {
        Component::Button(b) => button(b)?,
        Component::Thumbnail(t) => thumbnail(t)?,
        _ => {
            return Err(ComponentValidationError {
                kind: ComponentValidationErrorType::DisallowedChildren,
            })
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
                })
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
pub fn thumbnail(thumbnail: &Thumbnail) -> Result<(), ComponentValidationError> {
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

/// Validates a media gallery item
///
/// # Errors
///
/// This will error with [`MediaGalleryItemDescriptionTooLong`] if the description is longer
/// than [`MEDIA_GALLERY_ITEM_DESCRIPTION_LENGTH_MAX`].
///
/// [`MediaGalleryItemDescriptionTooLong`]: ComponentValidationErrorType::MediaGalleryItemDescriptionTooLong
pub fn media_gallery_item(item: &MediaGalleryItem) -> Result<(), ComponentValidationError> {
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
