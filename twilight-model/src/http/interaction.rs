//! Models used when responding to interactions over HTTP.

use super::attachment::Attachment;
use crate::{
    application::command::CommandOptionChoice,
    channel::message::{AllowedMentions, Component, Embed, MessageFlags},
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Interaction response sent to Discord.
///
/// See [Discord Docs/Interaction Object].
///
/// [Discord Docs/Interaction Object]: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-structure
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InteractionResponse {
    /// Type of the response.
    #[serde(rename = "type")]
    pub kind: InteractionResponseType,
    /// Data of the response.
    ///
    /// This is required if the type is any of the following:
    /// - [`ChannelMessageWithSource`]
    /// - [`UpdateMessage`]
    /// - [`Modal`]
    /// - [`ApplicationCommandAutocompleteResult`]
    ///
    /// [`ApplicationCommandAutocompleteResult`]: InteractionResponseType::ApplicationCommandAutocompleteResult
    /// [`ChannelMessageWithSource`]: InteractionResponseType::ChannelMessageWithSource
    /// [`Modal`]: InteractionResponseType::Modal
    /// [`UpdateMessage`]: InteractionResponseType::UpdateMessage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionResponseData>,
}

/// Data included in an interaction response.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[non_exhaustive]
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
    /// This is only valid for components and modal submits.
    DeferredUpdateMessage = 6,
    /// Acknowledges a component interaction and edits the message.
    ///
    /// This is only valid for components and modal submits.
    UpdateMessage = 7,
    /// Respond to an autocomplete interaction with suggested choices.
    ApplicationCommandAutocompleteResult = 8,
    /// Respond to an interaction with a popup modal.
    Modal = 9,
    /// Respond to an interaction with an upgrade button, only available
    /// for apps with monetization enabled
    ///
    /// Please send a [`InteractionResponseType::ChannelMessageWithSource`]
    /// with an [`Button`](crate::channel::message::component::Button) with the style [`ButtonStyle::Premium`](crate::channel::message::component::ButtonStyle)
    /// instead.
    PremiumRequired = 10,
}

#[cfg(test)]
mod tests {
    use crate::{
        channel::message::MessageFlags,
        http::{
            attachment::Attachment,
            interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
        },
    };
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

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
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn interaction_response() {
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

    #[test]
    fn interaction_response_with_attachments() {
        let value = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionResponseData {
                attachments: Some(Vec::from([Attachment {
                    description: None,
                    file: "file data".into(),
                    filename: "filename.jpg".into(),
                    id: 1,
                }])),
                ..InteractionResponseData::default()
            }),
        };

        serde_test::assert_ser_tokens(
            &value,
            &[
                Token::Struct {
                    name: "InteractionResponse",
                    len: 2,
                },
                Token::Str("type"),
                Token::U8(InteractionResponseType::ChannelMessageWithSource as u8),
                Token::Str("data"),
                Token::Some,
                Token::Struct {
                    name: "InteractionResponseData",
                    len: 1,
                },
                Token::Str("attachments"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "Attachment",
                    len: 2,
                },
                Token::Str("filename"),
                Token::Str("filename.jpg"),
                Token::Str("id"),
                Token::U64(1),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
