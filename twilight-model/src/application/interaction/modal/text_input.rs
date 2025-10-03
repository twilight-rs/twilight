use serde::{Deserialize, Serialize};

/// User filled in modal component.
///
/// See [Discord Docs/Message Components].
///
/// [Discord Docs/Message Components]: https://discord.com/developers/docs/interactions/message-components
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModalInteractionTextInput {
    /// User defined identifier for the component.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// [Discord Docs/Custom ID]: https://discord.com/developers/docs/interactions/message-components#custom-id
    pub custom_id: String,
    pub id: i32,
    /// Value submitted by the user.
    pub value: String,
}
