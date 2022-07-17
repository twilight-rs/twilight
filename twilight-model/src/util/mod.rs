//! Utilities for efficiently parsing and representing data from Discord's API.

pub mod datetime;
pub mod image_hash;

pub use self::{datetime::Timestamp, image_hash::ImageHash};

#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn is_false(value: &bool) -> bool {
    !value
}
