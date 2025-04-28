use serde::{Deserialize, Serialize};

use super::Component;

/// A Container is a top-level layout component that holds up to 10
/// [`Component`]s. Containers are visually distinct from surrounding
/// [`Component`]s and have an optional customizable color bar.
///
/// Containers are only available in messages.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Container {
    /// Optional identifier for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Color for the accent on the container as RGB from `0x000000` to `0xFFFFFF`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,
    /// Whether the container should be a spoiler (or blurred out). Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoiler: Option<bool>,
    /// Up to 10 components of the type action row, text display,
    /// section, media gallery, separator, or file.
    pub components: Vec<Component>,
}
