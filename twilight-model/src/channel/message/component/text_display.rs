use serde::{Deserialize, Serialize};

/// Top-level component that allows markdown formatted text.
///
/// Text Displays are only available in messages.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct TextDisplay {
    /// Optional id for the text display component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Text that will be displayed similar to a message.
    pub content: String,
}
