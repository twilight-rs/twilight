use crate::component::ComponentType;
use serde::{self, Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ButtonInteractionData {
    /// Custom id of the button that was clicked
    /// This is currently NOT validated by the discord api and can be spoofed by malicious users
    pub custom_id: String,
    // Component type, always 2
    pub component_type: ComponentType,
}
