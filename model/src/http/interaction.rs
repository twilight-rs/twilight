//! Models used when responding to interactions over HTTP.

use crate::{
    application::{command::CommandOptionChoice, component::Component},
    channel::{
        embed::Embed,
        message::{AllowedMentions, MessageFlags},
    },
    http::attachment::Attachment,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Interaction response sent to Discord.
///
/// See [Discord Docs/Interaction Object].
///
/// [Discord Docs/Interaction Object]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-structure
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InteractionResponse {
    /// Type of the response.
    #[serde(rename = "type")]
    pub kind: InteractionResponseType,
    /// Data of the response.
    ///
    /// This is required if the type is any of the following:
    /// - [`ChannelMessageWithSource`]
    /// - [`UpdateMessage`]
    /// - [`ApplicationCommandAutocompleteResult`]
    ///
    /// [`ApplicationCommandAutocompleteResult`]: InteractionResponseType::ApplicationCommandAutocompleteResult
    /// [`ChannelMessageWithSource`]: InteractionResponseType::ChannelMessageWithSource
    /// [`UpdateMessage`]: InteractionResponseType::UpdateMessage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionResponseData>,
}

/// Data included in an interaction response.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InteractionResponseData {
    /// Allowed mentions of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
    /// List of attachments on the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// List of autocomplete alternatives.
    ///
    /// Can only be used with
    /// [`InteractionResponseType::ApplicationCommandAutocompleteResult`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<CommandOptionChoice>>,
    /// List of components on the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
    /// Content of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// For [`InteractionResponseType::Modal`], user defined identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    /// Embeds of the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
    /// Interaction response data flags.
    ///
    /// The supported flags are [`MessageFlags::SUPPRESS_EMBEDS`] and
    /// [`MessageFlags::EPHEMERAL`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    /// For [`InteractionResponseType::Modal`], title of the modal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Whether the response is TTS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
}

/// Type of interaction response.
#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum InteractionResponseType {
    /// Used when responding to a Ping from Discord.
    Pong = 1,
    /// Responds to an interaction with a message.
    ChannelMessageWithSource = 4,
    /// Acknowledges an interaction, showing a loading state, and allowing for
    /// the message to be edited later.
    DeferredChannelMessageWithSource = 5,
    /// Acknowledges a component interaction, allowing for the message to be
    /// edited later.
    ///
    /// This is only valid for components.
    DeferredUpdateMessage = 6,
    /// Acknowledges a component interaction and edits the message.
    ///
    /// This is only valid for components.
    UpdateMessage = 7,
    /// Respond to an autocomplete interaction with suggested choices.
    ApplicationCommandAutocompleteResult = 8,
    /// Respond to an interaction with a popup modal.
    Modal = 9,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        InteractionResponseData: allowed_mentions,
        choices,
        components,
        content,
        embeds,
        flags,
        tts
    );
    assert_impl_all!(
        InteractionResponseData: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn test_interaction_response() {
        let value = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                allowed_mentions: None,
                attachments: None,
                choices: None,
                components: None,
                content: Some("test".into()),
                custom_id: None,
                embeds: None,
                flags: Some(MessageFlags::EPHEMERAL),
                title: None,
                tts: None,
            }),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InteractionResponse",
                    len: 2,
                },
                Token::Str("type"),
                Token::U8(4),
                Token::Str("data"),
                Token::Some,
                Token::Struct {
                    name: "InteractionResponseData",
                    len: 2,
                },
                Token::Str("content"),
                Token::Some,
                Token::Str("test"),
                Token::Str("flags"),
                Token::Some,
                Token::U64(64),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
