use super::Component;
use serde::{Deserialize, Serialize};

/// A component representing a section of a message.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Section {
    /// Optional identifier for the section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// One to three [text components].
    ///
    /// [text components]: super::TextDisplay
    pub components: Vec<Component>,
    /// An accessory component for this section.
    ///
    /// Currently, only [Thumbnail] and [Button] components are supported here.
    ///
    /// [Thumbnail]: super::Thumbnail
    /// [Button]: super::Button
    pub accessory: Box<Component>,
}
