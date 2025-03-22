use serde::{Deserialize, Serialize};

use super::Component;

/// A container is a top-level component that is used to organize other
/// child [`Component`]s.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Container {
    /// The unique identifier of the container.
    pub id: Option<i32>,
    /// The color of the vertical bar on the left side of the container.
    pub accent_color: Option<u32>,
    /// Whether the container should present content as a spoiler.
    pub spoiler: Option<bool>,
    // At the moment this can only be ActionRow, TextDisplay, Section,
    // MediaGallery, Separator, File.
    /// The components inside this container.
    pub components: Vec<Component>,
}
