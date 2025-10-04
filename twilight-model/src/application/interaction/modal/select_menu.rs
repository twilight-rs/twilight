use crate::id::Id;
use crate::id::marker::{ChannelMarker, GenericMarker, RoleMarker, UserMarker};
use serde::{Deserialize, Serialize};

/// User filled in String Select.
///
/// See [Discord Docs/String Select Interaction Response Structure].
///
/// [Discord Docs/String Select Interaction Response Structure]: https://discord.com/developers/docs/components/reference#string-select-string-select-interaction-response-structure
pub type ModalInteractionStringSelect = ModalInteractionSelectMenu<String>;
/// User filled in User Select.
///
/// See [Discord Docs/User Select Interaction Response Structure].
///
/// [Discord Docs/User Select Interaction Response Structure]: https://discord.com/developers/docs/components/reference#user-select-user-select-interaction-response-structure
pub type ModalInteractionUserSelect = ModalInteractionSelectMenu<Id<UserMarker>>;
/// User filled in Role Select.
///
/// See [Discord Docs/Role Select Interaction Response Structure].
///
/// [Discord Docs/Role Select Interaction Response Structure]: https://discord.com/developers/docs/components/reference#role-select-role-select-interaction-response-structure
pub type ModalInteractionRoleSelect = ModalInteractionSelectMenu<Id<RoleMarker>>;
/// User filled in Mentionable Select.
///
/// See [Discord Docs/Mentionable Select Interaction Response Structure].
///
/// [Discord Docs/Mentionable Select Interaction Response Structure]: https://discord.com/developers/docs/components/reference#mentionable-select-mentionable-select-interaction-response-structure
pub type ModalInteractionMentionableSelect = ModalInteractionSelectMenu<Id<GenericMarker>>;
/// User filled in Channel Select.
///
/// See [Discord Docs/Channel Select Interaction Response Structure].
///
/// [Discord Docs/Channel Select Interaction Response Structure]: https://discord.com/developers/docs/components/reference#channel-select-channel-select-interaction-response-structure
pub type ModalInteractionChannelSelect = ModalInteractionSelectMenu<Id<ChannelMarker>>;

/// User filled in [`SelectMenu`].
///
/// The `ValueType` generic parameter defines the type of the selected value
/// (e.g. text, user id, etc.).
/// See also [`ModalInteractionStringSelect`], [`ModalInteractionUserSelect`],
/// [`ModalInteractionRoleSelect`], [`ModalInteractionMentionableSelect`],
/// and [`ModalInteractionChannelSelect`].
///
/// [`SelectMenu`]: crate::channel::message::component::SelectMenu
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModalInteractionSelectMenu<ValueType> {
    /// Unique identifier for the component.
    pub id: i32,
    /// User defined identifier for the component.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// [Discord Docs/Custom ID]: https://discord.com/developers/docs/components/reference#anatomy-of-a-component-custom-id
    pub custom_id: String,
    /// The selected values.
    pub values: Vec<ValueType>,
}
