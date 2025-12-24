use serde::Deserialize;

use super::{InteractionCallback, resource::InteractionCallbackResource};

/// Included when creating an interaction response with a response
///
/// See [Discord Docs/Interaction Callback Response Object]
///
/// [Discord Docs/Interaction Callback Response Object]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-callback-interaction-callback-response-object
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct InteractionCallbackResponse {
    /// The interaction object associated with the interaction response.
    pub interaction: InteractionCallback,
    /// The resource that was created by the interaction response.
    pub resource: InteractionCallbackResource,
}

#[cfg(test)]
mod tests {

    use serde_test::Token;

    use crate::{
        application::interaction::{
            InteractionType,
            callback::{
                InteractionCallback, interaction_callback_type::InteractionCallbackType,
                resource::InteractionCallbackResource,
            },
        },
        id::Id,
    };

    use super::InteractionCallbackResponse;

    #[test]
    #[allow(clippy::too_many_lines, deprecated)]
    fn test_response_full() {
        let value = InteractionCallbackResponse {
            interaction: InteractionCallback {
                id: Id::new(1),
                kind: InteractionType::ApplicationCommand,
                activity_instance_id: None,
                response_message_id: Some(Id::new(1)),
                response_message_loading: Some(false),
                response_message_ephemeral: Some(false),
            },
            resource: InteractionCallbackResource {
                kind: InteractionCallbackType::ChannelMessageWithSource,
                activity_instance: None,
                message: None,
            },
        };

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InteractionCallbackResponse",
                    len: 2,
                },
                Token::Str("interaction"),
                Token::Struct {
                    name: "InteractionCallback",
                    len: 5,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(InteractionType::ApplicationCommand as u8),
                Token::Str("response_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("response_message_loading"),
                Token::Some,
                Token::Bool(false),
                Token::Str("response_message_ephemeral"),
                Token::Some,
                Token::Bool(false),
                Token::StructEnd,
                Token::Str("resource"),
                Token::Struct {
                    name: "InteractionCallbackResource",
                    len: 3,
                },
                Token::Str("type"),
                Token::U8(InteractionCallbackType::ChannelMessageWithSource as u8),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
