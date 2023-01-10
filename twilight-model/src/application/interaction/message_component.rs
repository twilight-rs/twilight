//! [`MessageComponent`] interaction.
//!
//! [`MessageComponent`]: crate::application::interaction::InteractionType::MessageComponent

use crate::{
    application::interaction::resolved::InteractionDataResolved,
    channel::message::component::ComponentType,
};
use serde::{Deserialize, Serialize};

/// Data received when an [`MessageComponent`] interaction is executed.
///
/// See [Discord Docs/Message Component Data Structure].
///
/// [`MessageComponent`]: crate::application::interaction::InteractionType::MessageComponent
/// [Discord Docs/Message Component Data Structure]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-message-component-data-structure
#[derive(
    Clone,
    Debug,
    Deserialize,
    Eq,
    // Hash,
    PartialEq,
    Serialize,
)]
pub struct MessageComponentInteractionData {
    /// User defined identifier for the component.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// [Discord Docs/Custom ID]: https://discord.com/developers/docs/interactions/message-components#custom-id
    pub custom_id: String,
    /// Type of the component.
    pub component_type: ComponentType,
    /// Values selected by the user.
    ///
    /// Only used for Select Menu components.
    #[serde(default)]
    pub values: Vec<String>,
    /// Resolved data from the interaction's select menu.
    ///
    /// Only used for Select Menu components.
    pub resolved: Option<InteractionDataResolved>,
}

#[cfg(test)]
mod tests {
    use super::MessageComponentInteractionData;
    use crate::{
        application::interaction::resolved::InteractionDataResolved,
        channel::message::component::ComponentType,
        guild::{Permissions, Role},
        id::Id,
    };
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{collections::HashMap, fmt::Debug};

    assert_fields!(
        MessageComponentInteractionData: custom_id,
        component_type,
        values,
        resolved
    );
    assert_impl_all!(
        MessageComponentInteractionData: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        // Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn message_component_interaction_data() {
        let value = MessageComponentInteractionData {
            custom_id: "test".to_owned(),
            component_type: ComponentType::Button,
            values: Vec::from(["1".to_owned(), "2".to_owned()]),
            resolved: Some(InteractionDataResolved {
                attachments: HashMap::new(),
                channels: HashMap::new(),
                members: HashMap::new(),
                messages: HashMap::new(),
                roles: IntoIterator::into_iter([(
                    Id::new(400),
                    Role {
                        color: 0,
                        hoist: true,
                        icon: None,
                        id: Id::new(400),
                        managed: false,
                        mentionable: true,
                        name: "test".to_owned(),
                        permissions: Permissions::ADMINISTRATOR,
                        position: 12,
                        tags: None,
                        unicode_emoji: None,
                    },
                )])
                .collect(),
                users: HashMap::new(),
            }),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageComponentInteractionData",
                    len: 4,
                },
                Token::String("custom_id"),
                Token::String("test"),
                Token::String("component_type"),
                Token::U8(ComponentType::Button.into()),
                Token::String("values"),
                Token::Seq { len: Some(2) },
                Token::String("1"),
                Token::String("2"),
                Token::SeqEnd,
                Token::String("resolved"),
                Token::Some,
                Token::Struct {
                    name: "InteractionDataResolved",
                    len: 1,
                },
                Token::Str("roles"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("400"),
                Token::Struct {
                    name: "Role",
                    len: 8,
                },
                Token::Str("color"),
                Token::U32(0),
                Token::Str("hoist"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("400"),
                Token::Str("managed"),
                Token::Bool(false),
                Token::Str("mentionable"),
                Token::Bool(true),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("permissions"),
                Token::Str("8"),
                Token::Str("position"),
                Token::I64(12),
                Token::StructEnd,
                Token::MapEnd,
                Token::StructEnd,
                Token::StructEnd,
            ],
        )
    }
}
