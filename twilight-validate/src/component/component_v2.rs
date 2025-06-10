use super::{ComponentValidationError, ComponentValidationErrorType};
use twilight_model::channel::message::component::{
    MediaGallery, MediaGalleryItem, Section, TextDisplay,
};
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
        Component::TextDisplay(text_display) => self::text_display(text_display)?,
        Component::MediaGallery(media_gallery) => self::media_gallery(media_gallery)?,
        Component::Section(section) => self::section(section)?,
        // note(HTGAzureX1212): do we need to validate these?
        Component::Separator(_) | Component::File(_) => (),
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

pub fn media_gallery(media_gallery: &MediaGallery) -> Result<(), ComponentValidationError> {
    let items = media_gallery.items.len();
    if !(MEDIA_GALLERY_ITEMS_MIN..=MEDIA_GALLERY_ITEMS_MAX).contains(&items) {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::MediaGalleryItemCountOutOfRange { count: items },
        });
    }

    for item in media_gallery.items.iter() {
        media_gallery_item(&item)?;
    }

    Ok(())
}

pub fn section(section: &Section) -> Result<(), ComponentValidationError> {
    let components = section.components.len();
    if !(SECTION_COMPONENTS_MIN..=SECTION_COMPONENTS_MAX).contains(&components) {
        return Err(ComponentValidationError {
            kind: ComponentValidationErrorType::SectionComponentCountOutOfRange {
                count: components,
            },
        });
    }

    for component in section.components.iter() {
        match component {
            Component::TextDisplay(text_display) => self::text_display(text_display)?,
            _ => {
                return Err(ComponentValidationError {
                    kind: ComponentValidationErrorType::DisallowedChildren,
                })
            }
        }
    }

    match section.accessory.as_ref() {
        Component::Button(button) => super::button(button)?,
        Component::Thumbnail(_) => todo!(),
        _ => {
            return Err(ComponentValidationError {
                kind: ComponentValidationErrorType::DisallowedChildren,
            })
        }
    }

    Ok(())
}

fn media_gallery_item(item: &MediaGalleryItem) -> Result<(), ComponentValidationError> {
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
