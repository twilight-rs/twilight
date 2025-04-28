use super::Component;
use serde::{Deserialize, Serialize};

/// A component representing a section of a message.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Section {
    /// Optional identifier for the section.
    pub id: Option<i32>,
    /// An array of components making up this section.
    ///
    /// Currently, only the TextDisplay component is supported here.
    pub components: Vec<Component>,
    /// An accessory component for this section.
    ///
    /// Currently, only Thumbnail or Button components are supported here.
    pub accessory: Box<Component>,
}
